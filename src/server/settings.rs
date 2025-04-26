use std::time::Duration;

pub trait HasPolling {
    fn polling(&self) -> Option<Duration>;
}

// pub trait HasHooks {
//     fn hooks(&mut self) -> &mut Hooks;
// }
