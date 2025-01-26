//! OneshotCallback
//! mpsc + eventfd

mod impl_recv;

mod single_path;
mod readiness;
mod completion {}

// #[cfg(all(
//     feature = "event-manager",
//     any(target_os = "linux", target_os = "android")
// ))]
// mod impl_readiness;

use std::io;
use std::ops::{Deref, DerefMut};
use std::sync::mpsc;

use nix::sys::eventfd::{EfdFlags, EventFd};

use crate::defs::invocation_source;
use crate::utils::OneshotCallback;

type Invocation<Arg, Ret> = invocation_source::Invocation<Arg, OneshotCallback<Ret>>;

pub struct Rx<Arg, Ret> {
    rx: mpsc::Receiver<Invocation<Arg, Ret>>,
}

pub struct RxWithEventFd<Arg, Ret> {
    rx: Rx<Arg, Ret>,
    eventfd: EventFd,
}

impl<Arg, Ret> Deref for RxWithEventFd<Arg, Ret> {
    type Target = Rx<Arg, Ret>;

    fn deref(&self) -> &Self::Target {
        &self.rx
    }
}

impl<Arg, Ret> DerefMut for RxWithEventFd<Arg, Ret> {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.rx
    }
}

impl<Arg, Ret> Rx<Arg, Ret> {
    pub fn new(rx: mpsc::Receiver<Invocation<Arg, Ret>>) -> Self {
        Self { rx }
    }

    pub fn into_inner(self) -> mpsc::Receiver<Invocation<Arg, Ret>> {
        self.rx
    }
}

impl<Arg, Ret> RxWithEventFd<Arg, Ret> {
    pub fn new(rx: mpsc::Receiver<Invocation<Arg, Ret>>) -> io::Result<Self> {
        let eventfd = EventFd::from_flags(EfdFlags::EFD_CLOEXEC | EfdFlags::EFD_NONBLOCK)
            .map_err(io::Error::from)?;
        Ok(Self::with_eventfd(rx, eventfd))
    }

    pub fn with_eventfd(rx: mpsc::Receiver<Invocation<Arg, Ret>>, eventfd: EventFd) -> Self {
        Self {
            rx: Rx::new(rx),
            eventfd,
        }
    }

    pub fn into_inner(self) -> (mpsc::Receiver<Invocation<Arg, Ret>>, EventFd) {
        (self.rx.into_inner(), self.eventfd)
    }
}
