mod callback;
pub use callback::{callback_fn, Callback, FnCallback};

mod handler_builder;
pub use handler_builder::HandlerBuilder;

pub mod invocation_source;
pub mod layer;
pub mod server;

pub trait Handler<Arg> {
    type Ret;

    fn handle(&mut self, arg: Arg, callback: impl Callback<Ret = Self::Ret>);
}

#[derive(Debug)]
pub struct Invocation<Arg, Cb> {
    pub arg: Arg,
    pub callback: Cb,
}
