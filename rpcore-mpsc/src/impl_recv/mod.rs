mod recv_error;
pub use recv_error::{RecvError, TryRecvError};
use rpcore_core::invocation_source::recv;

#[cfg(any(target_os = "linux", target_os = "android"))]
use crate::RxWithEventFd;
use crate::{Invocation, Rx, TxCallback};

impl<Arg, Ret> recv::RecvInvocation<Arg, TxCallback<Ret>> for Rx<Arg, Ret>
where
    Ret: Send + 'static,
{
    type RecvErr = RecvError;

    fn recv(&mut self) -> Result<Invocation<Arg, Ret>, Self::RecvErr> {
        self.rx.recv().map_err(RecvError)
    }
}

impl<Arg, Ret> recv::TryRecvInvocation<Arg, TxCallback<Ret>> for Rx<Arg, Ret>
where
    Ret: Send + 'static,
{
    type TryRecvErr = TryRecvError;

    fn try_recv(&mut self) -> Result<Invocation<Arg, Ret>, Self::TryRecvErr> {
        self.rx.try_recv().map_err(TryRecvError)
    }
}

impl<Arg, Ret> recv::RecvInvocation<Arg, TxCallback<Ret>> for RxWithEventFd<Arg, Ret>
where
    Ret: Send + 'static,
{
    type RecvErr = RecvError;

    fn recv(&mut self) -> Result<Invocation<Arg, Ret>, Self::RecvErr> {
        self.rx.recv()
    }
}

impl<Arg, Ret> recv::TryRecvInvocation<Arg, TxCallback<Ret>> for RxWithEventFd<Arg, Ret>
where
    Ret: Send + 'static,
{
    type TryRecvErr = TryRecvError;

    fn try_recv(&mut self) -> Result<Invocation<Arg, Ret>, Self::TryRecvErr> {
        self.rx.try_recv()
    }
}
