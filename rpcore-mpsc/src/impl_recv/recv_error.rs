use std::fmt;
use std::sync::mpsc;

use rpcore_core::invocation_source::recv;

#[repr(transparent)]
pub struct RecvError(pub mpsc::RecvError);

impl fmt::Debug for RecvError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        self.0.fmt(f)
    }
}

impl fmt::Display for RecvError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        self.0.fmt(f)
    }
}

impl std::error::Error for RecvError {
    fn source(&self) -> Option<&(dyn std::error::Error + 'static)> {
        self.0.source()
    }

    #[allow(deprecated)]
    fn description(&self) -> &str {
        self.0.description()
    }

    fn cause(&self) -> Option<&dyn std::error::Error> {
        self.source()
    }
}

impl recv::Error for RecvError {
    fn is_closed(&self) -> bool {
        true
    }

    fn is_empty(&self) -> bool {
        false
    }
}

#[repr(transparent)]
pub struct TryRecvError(pub mpsc::TryRecvError);

impl fmt::Debug for TryRecvError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        self.0.fmt(f)
    }
}

impl fmt::Display for TryRecvError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        self.0.fmt(f)
    }
}

impl std::error::Error for TryRecvError {
    fn source(&self) -> Option<&(dyn std::error::Error + 'static)> {
        self.0.source()
    }

    #[allow(deprecated)]
    fn description(&self) -> &str {
        self.0.description()
    }

    fn cause(&self) -> Option<&dyn std::error::Error> {
        self.source()
    }
}

impl recv::Error for TryRecvError {
    fn is_closed(&self) -> bool {
        matches!(self.0, mpsc::TryRecvError::Disconnected)
    }

    fn is_empty(&self) -> bool {
        matches!(self.0, mpsc::TryRecvError::Empty)
    }
}
