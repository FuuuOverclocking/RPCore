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

    fn handle(&mut self, arg: Arg, callback: impl Callback<Output = Self::Ret>);
}

pub trait Callback: Send + 'static {
    type Output;

    fn call(self, out: Self::Output);
}

pub trait TryHandler<Arg>: Handler<Arg, Ret = Result<Self::Ok, Self::Err>> {
    type Ok;
    type Err;

    fn handle(&mut self, arg: Arg, callback: impl TryCallback<Ok = Self::Ok, Err = Self::Err>);
}

pub trait TryCallback: Callback<Output = Result<Self::Ok, Self::Err>> {
    type Ok;
    type Err;
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

impl<Cb, Ok, Err> TryCallback for Cb
where
    Cb: Callback<Output = Result<Ok, Err>>,
{
    type Ok = Ok;
    type Err = Err;
}

struct FnCallback<F, Output> {
    f: F,
    _phantom: PhantomData<fn(Output)>,
}

impl<F, Output> Callback for FnCallback<F, Output>
where
    F: FnOnce(Output) + Send + 'static,
    Output: 'static,
{
    type Output = Output;

    fn call(self, out: Self::Output) {
        (self.f)(out);
    }
}

fn callback_fn<F, Output>(f: F) -> FnCallback<F, Output>
where
    F: FnOnce(Output) + Send + 'static,
{
    FnCallback {
        f,
        _phantom: Default::default(),
    }
}
