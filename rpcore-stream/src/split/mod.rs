use std::sync::Arc;

use socket2::Socket;

pub fn split(sock_stream: Socket) -> (ReadHalf, WriteHalf) {
    let inner = Arc::new(sock_stream);
    let read_half = ReadHalf {
        inner: Arc::clone(&inner),
    };
    let write_half = WriteHalf {
        inner,
        close_on_drop: true,
    };

    (read_half, write_half)
}

pub struct ReadHalf {
    inner: Arc<Socket>,
}

pub struct WriteHalf {
    inner: Arc<Socket>,
    close_on_drop: bool,
}
