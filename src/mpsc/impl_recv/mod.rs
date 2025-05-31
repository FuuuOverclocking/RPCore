mod impl_recv_error;

use std::sync::mpsc;

use crate::core::invocation_source::recv;
#[cfg(any(target_os = "linux", target_os = "android"))]
use crate::mpsc::RxWithEventFd;
use crate::mpsc::{Invocation, OneshotCallback, Rx};

impl<Arg, Ret> recv::RecvInvocation<Arg, Ret, OneshotCallback<Ret>> for Rx<Arg, Ret>
where
    Ret: Send + 'static,
{
    type RecvErr = mpsc::RecvError;

    fn recv(&mut self) -> Result<Invocation<Arg, Ret>, Self::RecvErr> {
        self.rx.recv()
    }
}

impl<Arg, Ret> recv::TryRecvInvocation<Arg, Ret, OneshotCallback<Ret>> for Rx<Arg, Ret>
where
    Ret: Send + 'static,
{
    type TryRecvErr = mpsc::TryRecvError;

    fn try_recv(&mut self) -> Result<Invocation<Arg, Ret>, Self::TryRecvErr> {
        self.rx.try_recv()
    }
}

impl<Arg, Ret> recv::RecvInvocation<Arg, Ret, OneshotCallback<Ret>> for RxWithEventFd<Arg, Ret>
where
    Ret: Send + 'static,
{
    type RecvErr = mpsc::RecvError;

    fn recv(&mut self) -> Result<Invocation<Arg, Ret>, Self::RecvErr> {
        self.rx.recv()
    }
}

impl<Arg, Ret> recv::TryRecvInvocation<Arg, Ret, OneshotCallback<Ret>> for RxWithEventFd<Arg, Ret>
where
    Ret: Send + 'static,
{
    type TryRecvErr = mpsc::TryRecvError;

    fn try_recv(&mut self) -> Result<Invocation<Arg, Ret>, Self::TryRecvErr> {
        self.rx.try_recv()
    }
}
