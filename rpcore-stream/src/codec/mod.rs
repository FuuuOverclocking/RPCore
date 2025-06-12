use std::os::fd::{AsFd, OwnedFd};

use bytes::Buf;

pub trait Encode {
    fn encode<C: Encoder>(self, encoder: C);
}

pub trait Encoder: Sized {
    fn write_bytes(&mut self, bytes: impl Buf);

    /// Underlying uses sendfile().
    fn write_zcopy_from_fd(&mut self, fd: impl AsFd, offset: usize, count: usize);

    /// Underlying uses SCM_RIGHT, Unix socket only.
    fn write_fds(&mut self, fds: impl IntoIterator<Item = OwnedFd>);

    fn finish(self);
}

pub trait Decode<'de>: Sized {
    fn decode<D>(decoder: D) -> Result<Option<Self>, D::Error>
    where
        D: Decoder<'de>;
}

pub trait Decoder<'de>: Buf {
    type Error;

}
