use super::Invocation;
use crate::defs::Callback;

pub trait RecvInvocation<Arg, Ret, Cb>
where
    Cb: Callback<Ret>,
{
    type Err;

    fn recv(&mut self) -> Result<Invocation<Arg, Cb>, Self::Err>;
}

pub trait TryRecvInvocation<Arg, Ret, Cb>
where
    Cb: Callback<Ret>,
{
    type Err;

    fn try_recv(&mut self) -> Result<Invocation<Arg, Cb>, Self::Err>;
}

// TODO: 是否提供 trait 判断 Err::Empty?
