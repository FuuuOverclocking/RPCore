use std::time::Duration;

use crate::server::settings;

#[derive(Debug, Default)]
pub struct Settings {
    polling: Option<Duration>,
}

impl settings::HasPolling for Settings {
    fn polling(&self) -> Option<Duration> {
        self.polling.clone()
    }
}
