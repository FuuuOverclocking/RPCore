use std::io;
use std::mem::size_of;
use std::os::fd::OwnedFd;

use bytes::{Buf, BufMut, Bytes, BytesMut};
use rpcore_stream::codec::{Decode, Decoder, Encode, Encoder};

/// Represents a single message, containing a header, a payload, and optional file descriptors.
struct Message {
    header: Header,
    fds: Vec<OwnedFd>,
    payload: Bytes,
}

/// The fixed-size header for a `Message`.
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
    const SIZE: usize = size_of::<Self>();
    const MAGIC: [u8; 4] = *b"\0rpc";
}

impl Encode for Message {
    /// Encodes the message by writing the header, payload, and FDs sequentially.
    fn encode<C: Encoder>(self, mut encoder: C) {
        let mut header = BytesMut::with_capacity(Header::SIZE);

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

impl Decode for Message {
    type Err = io::Error;

    fn decode<D: Decoder>(mut decoder: D) -> Result<Option<Self>, Self::Err> {
        // Check if there's enough data to read the fixed-size header.
        if decoder.remaining() < Header::SIZE {
            return Ok(None); // Not enough data, wait for more.
        }

        // Peek at the header data without consuming it from the buffer.
        let mut header_bytes = decoder.peak(0, Header::SIZE);

        // Parse the magic number from the peeked data.
        let mut magic = [0u8; 4];
        header_bytes.copy_to_slice(&mut magic);

        // Validate the magic number to ensure we're reading a valid message.
        if magic != Header::MAGIC {
            return Err(io::Error::new(
                io::ErrorKind::InvalidData,
                "invalid magic number",
            ));
        }

        // Parse the rest of the header.
        let version = header_bytes.get_u8();
        let mut unused = [0u8; 3];
        header_bytes.copy_to_slice(&mut unused);
        let fd_len = header_bytes.get_u32_le();
        let payload_len = header_bytes.get_u32_le();

        let header = Header {
            magic,
            version,
            unused,
            fd_len,
            payload_len,
        };

        // Using the lengths from the header, check if the full message body has arrived.
        let total_len = Header::SIZE + payload_len as usize;
        if decoder.remaining() < total_len {
            return Ok(None); // Payload data is incomplete.
        }
        if decoder.remaining_fds() < fd_len as usize {
            return Ok(None); // File descriptors are incomplete.
        }

        // At this point, the full message is available in the buffer.

        // Consume the header from the buffer by advancing the cursor.
        decoder.advance(Header::SIZE);

        // Read the payload.
        let payload = {
            let mut payload_chunks = Vec::new();
            decoder.read(&mut payload_chunks, payload_len as usize);
            // The decoder might return the payload in multiple chunks; combine them if needed.
            if payload_chunks.is_empty() {
                Bytes::new()
            } else if payload_chunks.len() == 1 {
                payload_chunks.pop().unwrap()
            } else {
                let mut combined = BytesMut::with_capacity(payload_len as usize);
                for chunk in payload_chunks {
                    combined.put(chunk);
                }
                combined.freeze()
            }
        };

        // Read the file descriptors.
        let mut fds = Vec::with_capacity(fd_len as usize);
        decoder.read_fds(&mut fds, fd_len as usize);

        // Construct and return the complete message.
        Ok(Some(Message {
            header,
            fds,
            payload,
        }))
    }
}

fn main() {}
