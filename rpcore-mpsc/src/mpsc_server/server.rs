use rpcore_core::server::singleplex::{ServeWithPolling, Server};
use rpcore_core::server::IsShuttingDown;
use rpcore_core::Handler;

use crate::mpsc_server::Settings;
use crate::Rx;

pub struct MpscServer<H, Arg, Hooks>
where
    H: Handler<Arg>,
{
    pub(crate) inner: Server<Rx<Arg, H::Ret>, H, Settings<Hooks>>,
}

impl<H, Arg, Hooks> MpscServer<H, Arg, Hooks>
where
    H: Handler<Arg>,
    H::Ret: Send + 'static,
    Hooks: rpcore_core::server::Hooks,
{
    pub fn serve(&mut self, shutdown: &impl IsShuttingDown) {
        ServeWithPolling::serve(&mut self.inner, shutdown);
    }
}
