mod layer_fn;
pub use layer_fn::{layer_fn, LayerFn};

mod identity;
pub use identity::Identity;

mod stack;
pub use stack::Stack;

mod tuple;

pub trait Layer<H> {
    type Handler;

    fn layer(&self, inner: H) -> Self::Handler;
}

impl<'a, T, H> Layer<H> for &'a T
where
    T: ?Sized + Layer<H>,
{
    type Handler = T::Handler;

    fn layer(&self, inner: H) -> Self::Handler {
        (**self).layer(inner)
    }
}
