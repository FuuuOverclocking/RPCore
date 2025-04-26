use std::io;
/// Provides a receiver with an associated event file descriptor (eventfd) for
/// asynchronous notifications.
use std::ops;
use std::os::fd::{FromRawFd, OwnedFd};
use std::sync::mpsc;

use crate::mpsc::{Invocation, Rx};

/// A receiver that wraps an `Rx` with an associated event file descriptor.
pub struct RxWithEventFd<Arg, Ret> {
    pub(crate) rx: Rx<Arg, Ret>,
    pub(crate) eventfd: OwnedFd,
}

impl<Arg, Ret> RxWithEventFd<Arg, Ret> {
    /// Creates a new `RxWithEventFd` with a new event file descriptor.
    pub fn new(rx: mpsc::Receiver<Invocation<Arg, Ret>>) -> io::Result<Self> {
        // SAFETY: eventfd is a safe syscall.
        let fd = unsafe { libc::eventfd(0, libc::EFD_NONBLOCK | libc::EFD_CLOEXEC) };
        if fd == -1 {
            return Err(io::Error::last_os_error());
        }
        // SAFETY: fd is valid and we have ownership of it.
        let eventfd = unsafe { OwnedFd::from_raw_fd(fd) };

        Ok(Self {
            rx: Rx::new(rx),
            eventfd,
        })
    }

    /// Creates a new `RxWithEventFd` with an existing event file descriptor.
    pub fn with_eventfd(rx: mpsc::Receiver<Invocation<Arg, Ret>>, eventfd: OwnedFd) -> Self {
        Self {
            rx: Rx::new(rx),
            eventfd,
        }
    }

    /// Consumes the `RxWithEventFd` and returns the inner receiver and event
    /// file descriptor.
    pub fn into_inner(self) -> (mpsc::Receiver<Invocation<Arg, Ret>>, OwnedFd) {
        (self.rx.into_inner(), self.eventfd)
    }
}

impl<Arg, Ret> ops::Deref for RxWithEventFd<Arg, Ret> {
    type Target = Rx<Arg, Ret>;

    fn deref(&self) -> &Self::Target {
        &self.rx
    }
}

impl<Arg, Ret> ops::DerefMut for RxWithEventFd<Arg, Ret> {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.rx
    }
}
