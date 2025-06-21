use std::cell::RefCell;
use std::io::{self, IoSlice, IoSliceMut};
use std::os::fd::{AsRawFd, RawFd};

use std::{mem, mem::size_of, ptr};

use libc::{self, c_uint};

/// An extension trait for `io::Read` types that can receive file descriptors.
pub trait Read: io::Read {
    /// Receives data and a set of file descriptors from this reader.
    ///
    /// This method is an extension of `read_vectored` that allows for receiving
    /// file descriptors (ancillary data) alongside the normal data.
    ///
    /// # Returns
    ///
    /// A tuple containing the number of bytes read and the number of file
    /// descriptors received.
    #[allow(unused_variables)]
    fn read_vectored_with_fds(
        &mut self,
        bufs: &mut [IoSliceMut<'_>],
        fds: &mut [RawFd],
    ) -> io::Result<(usize, usize)> {
        // The default implementation simply calls `read_vectored` and reports
        // that zero file descriptors were read.
        self.read_vectored(bufs).map(|n| (n, 0))
    }
}

/// An extension trait for `io::Write` types that can send file descriptors.
pub trait Write: io::Write {
    /// Sends data and a set of file descriptors to this writer.
    ///
    /// This method is an extension of `write_vectored` that allows for sending
    /// file descriptors (ancillary data) alongside the normal data.
    ///
    /// # Returns
    ///
    /// The number of bytes written.
    #[allow(unused_variables)]
    fn write_vectored_with_fds(
        &mut self,
        bufs: &[IoSlice<'_>],
        fds: &[RawFd],
    ) -> io::Result<usize> {
        // The default implementation simply calls 'write_vectored', and reports
        // unsupported when the user attempts to send fds.
        if !fds.is_empty() {
            return Err(io::Error::from_raw_os_error(libc::EOPNOTSUPP));
        }
        self.write_vectored(bufs)
    }
}

impl Read for std::net::TcpStream {}
impl Read for &std::net::TcpStream {}
impl Write for std::net::TcpStream {}
impl Write for &std::net::TcpStream {}

impl Read for std::io::PipeReader {}
impl Read for &std::io::PipeReader {}
impl Write for std::io::PipeWriter {}
impl Write for &std::io::PipeWriter {}

/// The initial size of the control message buffer.
///
/// This size is chosen to be large enough for common cases of file descriptor
/// passing, avoiding heap allocations for most scenarios.
const CMSG_BUFFER_SIZE: usize = 4096;

thread_local! {
    /// A thread-local buffer for control messages.
    ///
    /// This buffer is used to avoid allocations for ancillary data in `sendmsg`
    /// and `recvmsg` for common cases. If a larger buffer is needed, a new one
    /// will be allocated on the heap for that specific call.
    static CMSG_BUF: RefCell<[u8; CMSG_BUFFER_SIZE]> = RefCell::new([0; CMSG_BUFFER_SIZE]);
}

/// A helper function to convert a libc return value into a `std::io::Result`.
///
/// A return value of -1 indicates an error, in which case `io::Error::last_os_error`
/// is used to retrieve the error information. Otherwise, the return value is
/// converted to a `usize`.
fn cvt(ret: libc::ssize_t) -> io::Result<usize> {
    if ret == -1 {
        Err(io::Error::last_os_error())
    } else {
        Ok(ret as usize)
    }
}

fn read_vectored_with_fds<T: Read>(
    this: &mut T,
    this_fd: RawFd,
    bufs: &mut [IoSliceMut<'_>],
    fds: &mut [RawFd],
) -> io::Result<(usize, usize)> {
    // If no file descriptors are requested, use the standard vectored read
    // for efficiency.
    if fds.is_empty() {
        return io::Read::read_vectored(this, bufs).map(|n| (n, 0));
    }

    // Calculate the required space for the control message to hold the fds.
    let cmsg_len = fds.len() * size_of::<RawFd>();
    let cmsg_space = unsafe { libc::CMSG_SPACE(cmsg_len as c_uint) as usize };

    CMSG_BUF.with(|buf_cell| {
        let mut thread_local_buf = buf_cell.borrow_mut();
        let mut cmsg_buf_dynamic;

        // Use the thread-local buffer if it's large enough, otherwise fall
        // back to a heap allocation for this call.
        let cmsg_buf_slice: &mut [u8] = if cmsg_space <= thread_local_buf.len() {
            &mut thread_local_buf[..cmsg_space]
        } else {
            cmsg_buf_dynamic = vec![0u8; cmsg_space];
            &mut cmsg_buf_dynamic
        };

        // Initialize the message header for recvmsg.
        let mut msg: libc::msghdr = unsafe { mem::zeroed() };
        // Set the IO vectors for the data.
        msg.msg_iov = bufs.as_mut_ptr().cast();
        msg.msg_iovlen = bufs.len() as _;
        // Set the buffer for the control message (ancillary data).
        msg.msg_control = cmsg_buf_slice.as_mut_ptr().cast();
        msg.msg_controllen = cmsg_buf_slice.len();

        // On Linux and Android, set MSG_CMSG_CLOEXEC to atomically set the
        // close-on-exec flag for the received file descriptors. This is a
        // security and resource management best practice.
        #[cfg(any(target_os = "android", target_os = "linux"))]
        let flags = libc::MSG_CMSG_CLOEXEC;
        #[cfg(not(any(target_os = "android", target_os = "linux")))]
        let flags = 0;

        // Call recvmsg to receive both data and control messages.
        let bytes_read = cvt(unsafe { libc::recvmsg(this_fd, &mut msg, flags) })?;

        // Check if the control message was truncated.
        if msg.msg_flags & libc::MSG_CTRUNC != 0 {
            return Err(io::Error::new(
                io::ErrorKind::InvalidData,
                "received truncated control message",
            ));
        }

        let mut fds_read = 0;
        // Iterate over the control messages to find the file descriptors.
        let mut cmsg = unsafe { libc::CMSG_FIRSTHDR(&msg) };
        // A safeguard against buggy CMSG_NXTHDR implementations that could
        // cause an infinite loop by repeatedly returning the same pointer.
        let mut prev_cmsg: *const libc::cmsghdr = ptr::null();

        while !cmsg.is_null() {
            if !prev_cmsg.is_null() && prev_cmsg == cmsg {
                break;
            }

            let cmsg_ref = unsafe { &*cmsg };
            // Check if the message is the one we expect: a list of file descriptors.
            if cmsg_ref.cmsg_level == libc::SOL_SOCKET && cmsg_ref.cmsg_type == libc::SCM_RIGHTS {
                // Calculate how many file descriptors are in this message.
                let data_len = cmsg_ref.cmsg_len - unsafe { libc::CMSG_LEN(0) as usize };
                let num_fds_in_msg = data_len / size_of::<RawFd>();

                let space_in_output = fds.len() - fds_read;
                let fds_to_copy = std::cmp::min(num_fds_in_msg, space_in_output);

                if fds_to_copy > 0 {
                    // Get a pointer to the file descriptor data within the control message.
                    let data_ptr = unsafe { libc::CMSG_DATA(cmsg) } as *const RawFd;
                    // Copy the received file descriptors into the output slice.
                    unsafe {
                        ptr::copy_nonoverlapping(
                            data_ptr,
                            fds.as_mut_ptr().add(fds_read),
                            fds_to_copy,
                        );
                    }
                }
                fds_read += fds_to_copy;
            }
            prev_cmsg = cmsg;
            // Advance to the next control message header.
            cmsg = unsafe { libc::CMSG_NXTHDR(&msg, cmsg) };
        }
        // Return the number of bytes read and the number of file descriptors read.
        Ok((bytes_read, fds_read))
    })
}

fn write_vectored_with_fds<T: Write>(
    this: &mut T,
    this_fd: RawFd,
    bufs: &[IoSlice<'_>],
    fds: &[RawFd],
) -> io::Result<usize> {
    // If no file descriptors are being sent, use the standard vectored write.
    if fds.is_empty() {
        return io::Write::write_vectored(this, bufs);
    }

    let cmsg_data_len = fds.len() * size_of::<RawFd>();
    let cmsg_space = unsafe { libc::CMSG_SPACE(cmsg_data_len as u32) as usize };

    CMSG_BUF.with(|buf_cell| {
        let mut thread_local_buf = buf_cell.borrow_mut();
        let mut cmsg_buf_dynamic;

        // Similar to the read path, use a thread-local or heap-allocated buffer.
        let cmsg_buf_slice: &mut [u8] = if cmsg_space <= thread_local_buf.len() {
            &mut thread_local_buf[..cmsg_space]
        } else {
            cmsg_buf_dynamic = vec![0u8; cmsg_space];
            &mut cmsg_buf_dynamic
        };

        cmsg_buf_slice.fill(0);

        // Initialize the message header for sendmsg.
        let mut msg: libc::msghdr = unsafe { mem::zeroed() };
        // Set the IO vectors for the data to be written.
        msg.msg_iov = bufs.as_ptr() as *mut libc::iovec;
        msg.msg_iovlen = bufs.len() as _;
        // Set the buffer for the control message.
        msg.msg_control = cmsg_buf_slice.as_mut_ptr() as *mut _;
        msg.msg_controllen = cmsg_buf_slice.len();

        // Get a pointer to the first control message header in our buffer.
        let cmsg = unsafe { libc::CMSG_FIRSTHDR(&mut msg) };
        assert!(!cmsg.is_null(), "CMSG_FIRSTHDR returned null");
        let cmsg_mut = unsafe { &mut *cmsg };

        // Populate the control message header.
        // Set the level and type to indicate we are sending file descriptors.
        cmsg_mut.cmsg_level = libc::SOL_SOCKET;
        cmsg_mut.cmsg_type = libc::SCM_RIGHTS;
        // Set the length of the control message data.
        cmsg_mut.cmsg_len = unsafe { libc::CMSG_LEN(cmsg_data_len as u32) as _ };

        // Get a pointer to the data portion of the control message.
        let data_ptr = unsafe { libc::CMSG_DATA(cmsg) } as *mut RawFd;
        // Copy the file descriptors into the control message data buffer.
        unsafe {
            ptr::copy_nonoverlapping(fds.as_ptr(), data_ptr, fds.len());
        }

        // The total length of the control message part of the msghdr is the
        // length of the cmsghdr itself, which includes the header and data.
        msg.msg_controllen = cmsg_mut.cmsg_len;

        // Call sendmsg to send both data and the control message with FDs.
        cvt(unsafe { libc::sendmsg(this_fd, &msg, 0) })
    })
}

#[cfg(all(not(target_os = "hermit"), any(unix, doc)))]
mod impl_for_unix_stream {
    use std::os::unix::net::UnixStream;

    use super::*;

    impl Read for UnixStream {
        fn read_vectored_with_fds(
            &mut self,
            bufs: &mut [IoSliceMut<'_>],
            fds: &mut [RawFd],
        ) -> io::Result<(usize, usize)> {
            read_vectored_with_fds(self, self.as_raw_fd(), bufs, fds)
        }
    }

    impl Read for &UnixStream {
        fn read_vectored_with_fds(
            &mut self,
            bufs: &mut [IoSliceMut<'_>],
            fds: &mut [RawFd],
        ) -> io::Result<(usize, usize)> {
            read_vectored_with_fds(self, self.as_raw_fd(), bufs, fds)
        }
    }

    impl Write for UnixStream {
        fn write_vectored_with_fds(
            &mut self,
            bufs: &[IoSlice<'_>],
            fds: &[RawFd],
        ) -> io::Result<usize> {
            write_vectored_with_fds(self, self.as_raw_fd(), bufs, fds)
        }
    }

    impl Write for &UnixStream {
        fn write_vectored_with_fds(
            &mut self,
            bufs: &[IoSlice<'_>],
            fds: &[RawFd],
        ) -> io::Result<usize> {
            write_vectored_with_fds(self, self.as_raw_fd(), bufs, fds)
        }
    }
}

#[cfg(feature = "socket2")]
mod impl_for_socket2 {
    use socket2::Socket;

    use super::*;

    impl Read for Socket {
        fn read_vectored_with_fds(
            &mut self,
            bufs: &mut [IoSliceMut<'_>],
            fds: &mut [RawFd],
        ) -> io::Result<(usize, usize)> {
            read_vectored_with_fds(self, self.as_raw_fd(), bufs, fds)
        }
    }

    impl Read for &Socket {
        fn read_vectored_with_fds(
            &mut self,
            bufs: &mut [IoSliceMut<'_>],
            fds: &mut [RawFd],
        ) -> io::Result<(usize, usize)> {
            read_vectored_with_fds(self, self.as_raw_fd(), bufs, fds)
        }
    }

    impl Write for Socket {
        fn write_vectored_with_fds(
            &mut self,
            bufs: &[IoSlice<'_>],
            fds: &[RawFd],
        ) -> io::Result<usize> {
            write_vectored_with_fds(self, self.as_raw_fd(), bufs, fds)
        }
    }

    impl Write for &Socket {
        fn write_vectored_with_fds(
            &mut self,
            bufs: &[IoSlice<'_>],
            fds: &[RawFd],
        ) -> io::Result<usize> {
            write_vectored_with_fds(self, self.as_raw_fd(), bufs, fds)
        }
    }
}
