use std::any::type_name;
use std::fmt;

use super::Layer;

pub fn layer_fn<F>(f: F) -> LayerFn<F> {
    LayerFn { f }
}

#[derive(Clone, Copy)]
pub struct LayerFn<F> {
    f: F,
}

impl<F, H, Out> Layer<H> for LayerFn<F>
where
    F: Fn(H) -> Out,
{
    type Handler = Out;

    fn layer(&self, inner: H) -> Self::Handler {
        (self.f)(inner)
    }
}

impl<F> fmt::Debug for LayerFn<F> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_struct("LayerFn")
            .field("f", &format_args!("{}", type_name::<F>()))
            .finish()
    }
}
