mod extended_io;

use std::os::fd::{AsFd, OwnedFd};

use bytes::{Buf, Bytes};

/// A trait for types that can be encoded into a stream.
pub trait Encode {
    /// Encodes `self` into the given `Encoder`.
    fn encode<C: Encoder>(self, encoder: C);
}

/// A trait for encoding data, including bytes and file descriptors, into a stream.
pub trait Encoder: Sized {
    /// Writes a buffer of bytes to the stream.
    fn write_bytes<B: Buf>(&mut self, bytes: B);

    /// Writes `count` bytes from `fd` at `offset` using a zero-copy mechanism.
    ///
    /// The underlying implementation typically uses `sendfile(2)`.
    fn write_zcopy_from_fd(&mut self, fd: impl AsFd, offset: usize, count: usize);

    /// Writes a list of file descriptors to the stream.
    ///
    /// The underlying implementation uses `SCM_RIGHTS` and is only available on Unix sockets.
    fn write_fds(&mut self, fds: impl IntoIterator<Item = OwnedFd>);

    /// Marks the end of the current message.
    fn finish(self);
}

/// A trait for types that can be decoded from a stream.
pub trait Decode: Sized {
    /// The error type for decoding.
    type Err;

    /// Attempts to decode a message from the `Decoder`.
    ///
    /// # Returns
    ///
    /// - `Ok(Some(Self))`: A complete message was successfully decoded.
    /// - `Ok(None)`: More data is needed to decode a complete message.
    /// - `Err(Self::Err)`: A decoding error occurred.
    fn decode<D: Decoder>(decoder: D) -> Result<Option<Self>, Self::Err>;
}

/// A decoder that reads from an internal, potentially non-contiguous buffer.
///
/// It maintains a cursor to track the current reading position.
pub trait Decoder {
    /// Peeks at `count` bytes starting from `offset` without advancing the internal cursor.
    ///
    /// Requesting smaller chunks may allow the implementation to return a direct reference,
    /// avoiding memory copies.
    fn peak(&self, offset: usize, count: usize) -> Bytes;

    /// Advances the internal cursor by `count` bytes.
    fn advance(&mut self, count: usize);

    /// Returns the number of bytes remaining in the buffer.
    fn remaining(&self) -> usize;

    /// Reads exactly `count` bytes into `buf` and advances the cursor.
    ///
    /// The data may be split across multiple `Bytes` chunks in `buf`.
    fn read(&mut self, buf: &mut Vec<Bytes>, count: usize);

    /// Returns the number of file descriptors remaining in the buffer.
    fn remaining_fds(&self) -> usize;

    /// Reads exactly `count` file descriptors from the buffer into `buf`.
    fn read_fds(&mut self, buf: &mut Vec<OwnedFd>, count: usize);
}
