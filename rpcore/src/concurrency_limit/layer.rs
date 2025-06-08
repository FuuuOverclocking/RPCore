use super::ConcurrencyLimit;
use crate::layer::Layer;

#[derive(Debug, Clone)]
pub struct ConcurrencyLimitLayer {
    limit: u32,
}

impl ConcurrencyLimitLayer {
    pub const fn new(limit: u32) -> Self {
        Self { limit }
    }
}

impl<H> Layer<H> for ConcurrencyLimitLayer {
    type Handler = ConcurrencyLimit<H>;

    fn layer(&self, inner: H) -> Self::Handler {
        ConcurrencyLimit::new(inner, self.limit)
    }
}
