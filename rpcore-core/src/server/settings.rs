use std::time::Duration;

use crate::server::Hooks;

pub trait HasPolling {
    fn polling(&self) -> &Option<Duration>;
}

pub trait HasHooks {
    type H: Hooks;

    fn hooks(&mut self) -> &mut Self::H;
}
