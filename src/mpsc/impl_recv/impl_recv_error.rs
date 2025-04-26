use std::sync::mpsc;

use crate::defs::invocation_source::recv;

impl recv::Error for mpsc::RecvError {
    fn is_closed(&self) -> bool {
        true
    }

    fn is_empty(&self) -> bool {
        false
    }
}

impl recv::Error for mpsc::TryRecvError {
    fn is_closed(&self) -> bool {
        matches!(self, mpsc::TryRecvError::Disconnected)
    }

    fn is_empty(&self) -> bool {
        matches!(self, mpsc::TryRecvError::Empty)
    }
}
