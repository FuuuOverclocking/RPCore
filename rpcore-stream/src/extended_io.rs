use std::cell::RefCell;
use std::io::{self, IoSlice, IoSliceMut};
use std::os::fd::{AsRawFd, FromRawFd, OwnedFd, RawFd};
use std::{mem, ptr, slice};

use libc::{self, ssize_t};

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
    ) -> io::Result<(usize, Vec<OwnedFd>)> {
        // The default implementation simply calls `read_vectored` and reports
        // that zero file descriptors were read.
        self.read_vectored(bufs).map(|n| (n, vec![]))
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

#[repr(C)]
#[repr(align(4096))]
struct AlignedPage {
    data: [u8; 4096],
}

thread_local! {
    /// A thread-local buffer for control messages.
    ///
    /// This buffer is used to avoid allocations for ancillary data in `sendmsg`
    /// and `recvmsg` for common cases. If a larger buffer is needed, a new one
    /// will be allocated on the heap for that specific call.
    static CMSG_BUF: RefCell<AlignedPage> = const { RefCell::new(AlignedPage { data: [0; 4096] }) };
}

/// A helper function to convert a libc return value into a `std::io::Result`.
///
/// A return value of -1 indicates an error, in which case `io::Error::last_os_error`
/// is used to retrieve the error information. Otherwise, the return value is
/// converted to a `usize`.
fn cvt(ret: ssize_t) -> io::Result<usize> {
    if ret == -1 {
        Err(io::Error::last_os_error())
    } else {
        Ok(ret as usize)
    }
}

fn read_vectored_with_fds(
    this_fd: RawFd,
    bufs: &mut [IoSliceMut<'_>],
) -> io::Result<(usize, Vec<OwnedFd>)> {
    CMSG_BUF.with(|buf_cell| {
        let cmsg_buf = &mut buf_cell.borrow_mut().data;

        // Initialize the message header for recvmsg.
        // SAFETY: Create empty msghdr is safe.
        let mut msg: libc::msghdr = unsafe { mem::zeroed() };
        msg.msg_iov = bufs.as_mut_ptr().cast();
        msg.msg_iovlen = bufs.len() as _;
        msg.msg_control = cmsg_buf.as_mut_ptr().cast();
        msg.msg_controllen = cmsg_buf.len();

        // On Linux and Android, set MSG_CMSG_CLOEXEC to atomically set the
        // close-on-exec flag for the received file descriptors.
        #[cfg(any(target_os = "android", target_os = "linux"))]
        let flags = libc::MSG_CMSG_CLOEXEC;
        #[cfg(not(any(target_os = "android", target_os = "linux")))]
        let flags = 0;

        // SAFETY: Safe to call `recvmsg` with a valid msghdr.
        let bytes_read = cvt(unsafe { libc::recvmsg(this_fd, &mut msg, flags) })?;

        // Check if the control message was truncated.
        if msg.msg_flags & libc::MSG_CTRUNC != 0 {
            return Err(io::Error::new(
                io::ErrorKind::InvalidData,
                "received truncated control message",
            ));
        }

        let mut fds: Vec<OwnedFd> = vec![];

        // Iterate over the control messages to find the file descriptors.
        // SAFETY: OS guarantees that cmsg is a valid pointer to a cmsghdr.
        let mut cmsg = unsafe { libc::CMSG_FIRSTHDR(&msg) };
        let mut prev_cmsg: *const libc::cmsghdr = ptr::null();
        while !cmsg.is_null() {
            // A safeguard against buggy CMSG_NXTHDR implementations that could
            // cause an infinite loop by repeatedly returning the same pointer.
            if !prev_cmsg.is_null() && prev_cmsg == cmsg {
                break;
            }

            // SAFETY: OS guarantees that cmsg is a valid pointer to a cmsghdr.
            let cmsg_ref = unsafe { &*cmsg };

            if cmsg_ref.cmsg_level == libc::SOL_SOCKET && cmsg_ref.cmsg_type == libc::SCM_RIGHTS {
                // Calculate how many file descriptors are in this message.
                // SAFETY: Safe to calculate `CMSG_LEN(0)`.
                let data_len = cmsg_ref.cmsg_len - unsafe { libc::CMSG_LEN(0) as usize };
                let num_fds_in_msg = data_len / size_of::<RawFd>();

                if num_fds_in_msg != 0 {
                    // SAFETY: Safe to access the data portion of the control message.
                    let raw_fds = unsafe {
                        slice::from_raw_parts(libc::CMSG_DATA(cmsg) as *const RawFd, num_fds_in_msg)
                    };
                    // SAFETY: Received fds are valid and owned.
                    fds.extend(
                        raw_fds
                            .iter()
                            .map(|&fd| unsafe { OwnedFd::from_raw_fd(fd) }),
                    );
                }
            }
            prev_cmsg = cmsg;
            // Advance to the next control message header.
            cmsg = unsafe { libc::CMSG_NXTHDR(&msg, cmsg) };
        }

        Ok((bytes_read, fds))
    })
}

fn write_vectored_with_fds<T: Write>(
    this: &mut T,
    this_fd: RawFd,
    bufs: &[IoSlice<'_>],
    fds: &[RawFd],
) -> io::Result<usize> {
    if fds.is_empty() {
        return io::Write::write_vectored(this, bufs);
    }

    let cmsg_data_len = size_of_val(fds);
    // SAFETY: Safe to calculate CMSG_SPACE for the data length.
    let cmsg_space = unsafe { libc::CMSG_SPACE(cmsg_data_len as u32) as usize };

    CMSG_BUF.with(|buf_cell| {
        let cmsg_buf = &mut buf_cell.borrow_mut().data;

        let cmsg_buf = if cmsg_space <= cmsg_buf.len() {
            &mut cmsg_buf[..cmsg_space]
        } else {
            return Err(io::Error::new(
                io::ErrorKind::InvalidInput,
                "too many file descriptors to send",
            ));
        };

        // Initialize the message header for sendmsg.
        // SAFETY: Create empty msghdr is safe.
        let mut msg: libc::msghdr = unsafe { mem::zeroed() };
        msg.msg_iov = bufs.as_ptr() as *mut libc::iovec;
        msg.msg_iovlen = bufs.len() as _;
        msg.msg_control = cmsg_buf.as_mut_ptr() as *mut _;
        msg.msg_controllen = cmsg_buf.len();

        // Get a pointer to the first control message header in our buffer.
        // SAFETY: Safe to call CMSG_FIRSTHDR with a valid msghdr.
        let cmsg = unsafe { libc::CMSG_FIRSTHDR(&msg) };
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
        ) -> io::Result<(usize, Vec<OwnedFd>)> {
            read_vectored_with_fds(self.as_raw_fd(), bufs)
        }
    }

    impl Read for &UnixStream {
        fn read_vectored_with_fds(
            &mut self,
            bufs: &mut [IoSliceMut<'_>],
        ) -> io::Result<(usize, Vec<OwnedFd>)> {
            read_vectored_with_fds(self.as_raw_fd(), bufs)
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
        ) -> io::Result<(usize, Vec<OwnedFd>)> {
            read_vectored_with_fds(self.as_raw_fd(), bufs)
        }
    }

    impl Read for &Socket {
        fn read_vectored_with_fds(
            &mut self,
            bufs: &mut [IoSliceMut<'_>],
        ) -> io::Result<(usize, Vec<OwnedFd>)> {
            read_vectored_with_fds(self.as_raw_fd(), bufs)
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
