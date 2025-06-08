use std::fmt;

use super::Layer;

#[derive(Default, Clone)]
pub struct Identity;

impl Identity {
    pub const fn new() -> Self {
        Self
    }
}

impl<H> Layer<H> for Identity {
    type Handler = H;

    fn layer(&self, inner: H) -> Self::Handler {
        inner
    }
}

impl fmt::Debug for Identity {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_struct("Identity").finish()
    }
}
