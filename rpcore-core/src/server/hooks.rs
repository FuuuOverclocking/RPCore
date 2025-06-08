use std::error::Error;

pub trait Hooks {
    fn on_shutdown(&mut self) {}

    #[allow(unused_variables)]
    fn on_error(&mut self, e: &dyn Error) {}
}

impl Hooks for () {}
