use crate::Invocation;

pub trait RecvInvocation<Arg, Cb> {
    type RecvErr: Error;

    fn recv(&mut self) -> Result<Invocation<Arg, Cb>, Self::RecvErr>;
}

pub trait TryRecvInvocation<Arg, Cb>: RecvInvocation<Arg, Cb> {
    type TryRecvErr: Error;

    fn try_recv(&mut self) -> Result<Invocation<Arg, Cb>, Self::TryRecvErr>;
}

pub trait Error: std::error::Error {
    fn is_closed(&self) -> bool;
    fn is_empty(&self) -> bool;
}
