use std::fmt;

use crate::core::layer::{layer_fn, Identity, Layer, LayerFn, Stack};

#[derive(Clone)]
pub struct HandlerBuilder<L> {
    layer: L,
}

impl Default for HandlerBuilder<Identity> {
    fn default() -> Self {
        Self::new()
    }
}

impl HandlerBuilder<Identity> {
    pub const fn new() -> Self {
        Self {
            layer: Identity::new(),
        }
    }
}

impl<L> HandlerBuilder<L> {
    pub fn layer<T>(self, layer: T) -> HandlerBuilder<Stack<T, L>> {
        HandlerBuilder {
            layer: Stack::new(layer, self.layer),
        }
    }

    pub fn layer_fn<F>(self, f: F) -> HandlerBuilder<Stack<LayerFn<F>, L>> {
        self.layer(layer_fn(f))
    }

    pub fn into_inner(self) -> L {
        self.layer
    }

    pub fn handler<H>(&self, handler: H) -> L::Handler
    where
        L: Layer<H>,
    {
        self.layer.layer(handler)
    }
}

impl<L: fmt::Debug> fmt::Debug for HandlerBuilder<L> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_tuple("HandlerBuilder").field(&self.layer).finish()
    }
}

impl<H, L> Layer<H> for HandlerBuilder<L>
where
    L: Layer<H>,
{
    type Handler = L::Handler;

    fn layer(&self, inner: H) -> Self::Handler {
        self.layer.layer(inner)
    }
}
