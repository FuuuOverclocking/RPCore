use crate::defs::{Callback, Invocation};

pub trait RecvInvocation<Arg, Ret, Cb>
where
    Cb: Callback<Ret = Ret>,
{
    type RecvErr: Error;

    fn recv(&mut self) -> Result<Invocation<Arg, Cb>, Self::RecvErr>;
}

pub trait TryRecvInvocation<Arg, Ret, Cb>
where
    Cb: Callback<Ret = Ret>,
{
    type TryRecvErr: Error;

    fn try_recv(&mut self) -> Result<Invocation<Arg, Cb>, Self::TryRecvErr>;
}

pub trait Error {
    fn is_closed(&self) -> bool;
    fn is_empty(&self) -> bool;
}
