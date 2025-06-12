use std::{io::Read, os::fd::OwnedFd};

use bytes::{BufMut, Bytes, BytesMut};
use rpcore_stream::codec::{Decode, Decoder, Encode, Encoder};

struct Message {
    header: Header,
    fds: Vec<OwnedFd>,
    payload: Bytes,
}

#[derive(Debug, Clone, Copy, Default)]
#[repr(C)]
struct Header {
    magic: [u8; 4],
    version: u8,
    unused: [u8; 3],
    fd_len: u32,
    payload_len: u32,
}

impl Header {
    fn new(fd_len: u32, payload_len: u32,) -> Self {
        let header = Self::default();
        header.magic = *b"42  ";
    }
}

impl Encode for Message {
    fn encode<C: Encoder>(self, mut encoder: C) {
        let mut header = BytesMut::with_capacity(size_of::<Header>());

        header.put(&self.header.magic[..]);
        header.put_u8(self.header.version);
        header.put(&self.header.unused[..]);
        header.put_u32_le(self.header.fd_len);
        header.put_u32_le(self.header.payload_len);

        encoder.write_bytes(header.freeze());
        encoder.write_bytes(self.payload);
        encoder.write_fds(self.fds);
        encoder.finish();
    }
}

impl<'de> Decode<'de> for Message {
    fn decode<D>(decoder: D) -> Result<Option<Self>, D::Error>
    where
        D: Decoder<'de>,
    {
        if decoder.remaining() < size_of::<Header>() {
            return Ok(None);
        }
        let mut header = Header::default();
        decoder.reader().read_exact(buf)
    }
}

fn main() {}
