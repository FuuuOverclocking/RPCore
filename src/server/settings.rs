use std::time::Duration;

pub trait HasPolling {
    fn polling(&self) -> &Option<Duration>;
}
