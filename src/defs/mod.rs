pub mod layer;

mod handler_builder;
pub use handler_builder::HandlerBuilder;

pub mod invocation_source;

pub trait Handler<Arg> {
    type Ret;

    fn handle(&mut self, arg: Arg, callback: impl Callback<Self::Ret>);
}

pub trait Callback<T>: Send + 'static {
    fn call(self, val: T);
}

pub trait TryHandler<Arg>: Handler<Arg, Ret = Result<Self::Ok, Self::Err>> {
    type Ok;
    type Err;

    fn handle(&mut self, arg: Arg, callback: impl TryCallback<Self::Ok, Self::Err>);
}

pub trait TryCallback<T, E>: Callback<Result<T, E>> {}

/***** 让这些 trait 彼此等价 *****/

impl<H, Arg, Ok, Err> TryHandler<Arg> for H
where
    H: Handler<Arg, Ret = Result<Ok, Err>>,
{
    type Ok = Ok;
    type Err = Err;

    fn handle(&mut self, arg: Arg, callback: impl TryCallback<Self::Ok, Self::Err>) {
        self.handle(arg, callback)
    }
}

impl<Cb, Ok, Err> TryCallback<Ok, Err> for Cb where Cb: Callback<Result<Ok, Err>> {}

/***** Callback = FnOnce(T) + Send + 'static *****/

impl<T, F> Callback<T> for F
where
    F: FnOnce(T) + Send + 'static,
{
    fn call(self, val: T) {
        self(val)
    }
}
