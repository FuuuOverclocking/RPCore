use std::io;
use std::net::Shutdown;
use std::os::fd::{OwnedFd, RawFd};
use std::sync::Arc;

use socket2::Socket;

use crate::extended_io;

pub fn split(sock_stream: Socket) -> (ReadHalf, WriteHalf) {
    let inner = Arc::new(sock_stream);
    let read_half = ReadHalf {
        inner: Arc::clone(&inner),
    };
    let write_half = WriteHalf {
        inner,
        shutdown_on_drop: true,
    };

    (read_half, write_half)
}

pub fn reunite(read_half: ReadHalf, write_half: WriteHalf) -> Result<Socket, ReuniteError> {
    if Arc::ptr_eq(&read_half.inner, &write_half.inner) {
        write_half.forget();
        // This unwrap cannot fail as the api does not allow creating more than two Arcs,
        // and we just dropped the other half.
        Ok(Arc::try_unwrap(read_half.inner).expect("TcpStream: try_unwrap failed in reunite"))
    } else {
        Err(ReuniteError(read_half, write_half))
    }
}

/// Error indicating that two halves were not from the same socket, and thus could
/// not be reunited.
#[derive(Debug)]
pub struct ReuniteError(pub ReadHalf, pub WriteHalf);

#[derive(Debug)]
pub struct ReadHalf {
    inner: Arc<Socket>,
}

#[derive(Debug)]
pub struct WriteHalf {
    inner: Arc<Socket>,
    shutdown_on_drop: bool,
}

impl WriteHalf {
    /// Destroys the write half, but don't close the write half of the stream
    /// until the read half is dropped. If the read half has already been
    /// dropped, this closes the stream.
    pub fn forget(mut self) {
        self.shutdown_on_drop = false;
        drop(self);
    }
}

impl Drop for WriteHalf {
    fn drop(&mut self) {
        if self.shutdown_on_drop {
            let _ = self.inner.shutdown(Shutdown::Write);
        }
    }
}

impl io::Read for ReadHalf {
    fn read(&mut self, buf: &mut [u8]) -> io::Result<usize> {
        self.inner.as_ref().read(buf)
    }
}

impl io::Read for &ReadHalf {
    fn read(&mut self, buf: &mut [u8]) -> io::Result<usize> {
        self.inner.as_ref().read(buf)
    }
}

impl extended_io::Read for ReadHalf {
    fn read_vectored_with_fds(
        &mut self,
        bufs: &mut [io::IoSliceMut<'_>],
    ) -> io::Result<(usize, Vec<OwnedFd>)> {
        self.inner.as_ref().read_vectored_with_fds(bufs)
    }
}

impl extended_io::Read for &ReadHalf {
    fn read_vectored_with_fds(
        &mut self,
        bufs: &mut [io::IoSliceMut<'_>],
    ) -> io::Result<(usize, Vec<OwnedFd>)> {
        self.inner.as_ref().read_vectored_with_fds(bufs)
    }
}

impl io::Write for WriteHalf {
    fn write(&mut self, buf: &[u8]) -> io::Result<usize> {
        self.inner.as_ref().write(buf)
    }

    fn flush(&mut self) -> io::Result<()> {
        self.inner.as_ref().flush()
    }
}

impl io::Write for &WriteHalf {
    fn write(&mut self, buf: &[u8]) -> io::Result<usize> {
        self.inner.as_ref().write(buf)
    }

    fn flush(&mut self) -> io::Result<()> {
        self.inner.as_ref().flush()
    }
}

impl extended_io::Write for WriteHalf {
    fn write_vectored_with_fds(
        &mut self,
        bufs: &[io::IoSlice<'_>],
        fds: &[RawFd],
    ) -> io::Result<usize> {
        self.inner.as_ref().write_vectored_with_fds(bufs, fds)
    }
}

impl extended_io::Write for &WriteHalf {
    fn write_vectored_with_fds(
        &mut self,
        bufs: &[io::IoSlice<'_>],
        fds: &[RawFd],
    ) -> io::Result<usize> {
        self.inner.as_ref().write_vectored_with_fds(bufs, fds)
    }
}
