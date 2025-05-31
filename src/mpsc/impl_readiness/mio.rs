use std::io;
use std::os::fd::AsRawFd;

use mio::unix::SourceFd;

use crate::core::invocation_source::readiness;
use crate::mpsc::RxWithEventFd;

impl<Arg, Ret> readiness::EventSource<mio::Registry> for RxWithEventFd<Arg, Ret> {
    type Token = mio::Token;
    type Err = io::Error;

    fn register(
        &mut self,
        registry: &mut mio::Registry,
        token: Self::Token,
    ) -> Result<(), Self::Err> {
        registry.register(
            &mut SourceFd(&self.eventfd.as_raw_fd()),
            token,
            mio::Interest::READABLE,
        )
    }

    fn deregister(&mut self, registry: &mut mio::Registry) -> Result<(), Self::Err> {
        registry.deregister(&mut SourceFd(&self.eventfd.as_raw_fd()))
    }
}
