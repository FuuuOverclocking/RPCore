use std::time::Duration;

use rpcore_core::server::settings::{HasHooks, HasPolling};

#[derive(Debug)]
pub(crate) struct Settings<Hooks = ()> {
    pub(crate) polling: Option<Duration>,
    pub(crate) hooks: Hooks,
}

impl<Hooks> HasPolling for Settings<Hooks> {
    fn polling(&self) -> &Option<Duration> {
        &self.polling
    }
}

impl<Hooks> HasHooks for Settings<Hooks>
where
    Hooks: rpcore_core::server::Hooks,
{
    type H = Hooks;

    fn hooks(&mut self) -> &mut Self::H {
        &mut self.hooks
    }
}
