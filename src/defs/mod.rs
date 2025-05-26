mod handler_builder;
pub mod invocation_source;
pub mod layer;

use std::marker::PhantomData;

pub use self::handler_builder::HandlerBuilder;

#[derive(Debug)]
pub struct Invocation<Arg, Cb> {
    pub arg: Arg,
    pub callback: Cb,
}

pub trait Handler<Arg> {
    type Ret;

    fn handle(&mut self, arg: Arg, callback: impl Callback<Ret = Self::Ret>);
}

pub trait Callback: Send + 'static {
    type Ret;

    fn call(self, out: Self::Ret);
}

pub struct FnCallback<F, Ret> {
    f: F,
    _phantom: PhantomData<fn(Ret)>,
}

impl<F, Ret> Callback for FnCallback<F, Ret>
where
    F: FnOnce(Ret) + Send + 'static,
    Ret: 'static,
{
    type Ret = Ret;

    fn call(self, out: Self::Ret) {
        (self.f)(out);
    }
}

pub fn callback_fn<F, Ret>(f: F) -> FnCallback<F, Ret>
where
    F: FnOnce(Ret) + Send + 'static,
{
    FnCallback {
        f,
        _phantom: Default::default(),
    }
}

pub trait TryHandler<Arg>: Handler<Arg, Ret = Result<Self::Ok, Self::Err>> {
    type Ok;
    type Err;

    fn handle(&mut self, arg: Arg, callback: impl TryCallback<Ok = Self::Ok, Err = Self::Err>);
}

impl<H, Arg, Ok, Err> TryHandler<Arg> for H
where
    H: Handler<Arg, Ret = Result<Ok, Err>>,
{
    type Ok = Ok;
    type Err = Err;

    fn handle(&mut self, arg: Arg, callback: impl TryCallback<Ok = Self::Ok, Err = Self::Err>) {
        self.handle(arg, callback)
    }
}

pub trait TryCallback: Callback<Ret = Result<Self::Ok, Self::Err>> {
    type Ok;
    type Err;
}

impl<Cb, Ok, Err> TryCallback for Cb
where
    Cb: Callback<Ret = Result<Ok, Err>>,
{
    type Ok = Ok;
    type Err = Err;
}
