use std::sync::mpsc;

use super::{Invocation, Rx, RxWithEventFd};
use crate::defs::invocation_source::recv::{RecvInvocation, TryRecvInvocation};
use crate::utils::OneshotCallback;

impl<Arg, Ret> RecvInvocation<Arg, Ret, OneshotCallback<Ret>> for Rx<Arg, Ret>
where
    Ret: Send + 'static,
{
    type Err = mpsc::RecvError;

    fn recv(&mut self) -> Result<Invocation<Arg, Ret>, Self::Err> {
        self.rx.recv()
    }
}

impl<Arg, Ret> TryRecvInvocation<Arg, Ret, OneshotCallback<Ret>> for Rx<Arg, Ret>
where
    Ret: Send + 'static,
{
    type Err = mpsc::TryRecvError;

    fn try_recv(&mut self) -> Result<Invocation<Arg, Ret>, Self::Err> {
        self.rx.try_recv()
    }
}

impl<Arg, Ret> RecvInvocation<Arg, Ret, OneshotCallback<Ret>> for RxWithEventFd<Arg, Ret>
where
    Ret: Send + 'static,
{
    type Err = mpsc::RecvError;

    fn recv(&mut self) -> Result<Invocation<Arg, Ret>, Self::Err> {
        self.rx.recv()
    }
}

impl<Arg, Ret> TryRecvInvocation<Arg, Ret, OneshotCallback<Ret>> for RxWithEventFd<Arg, Ret>
where
    Ret: Send + 'static,
{
    type Err = mpsc::TryRecvError;

    fn try_recv(&mut self) -> Result<Invocation<Arg, Ret>, Self::Err> {
        self.rx.try_recv()
    }
}
