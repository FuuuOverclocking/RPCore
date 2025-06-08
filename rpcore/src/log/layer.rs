use super::Log;
use crate::layer::Layer;

#[derive(Debug, Clone, Default)]
pub struct LogLayer {
    handler_name: Option<String>,
}

impl<H> Layer<H> for LogLayer {
    type Handler = Log<H>;

    fn layer(&self, inner: H) -> Self::Handler {
        Log::new(inner, self.handler_name.clone())
    }
}
