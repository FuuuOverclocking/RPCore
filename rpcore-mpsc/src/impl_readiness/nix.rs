use nix::sys::epoll::{Epoll, EpollEvent, EpollFlags};
use rpcore_core::invocation_source::readiness;

use crate::RxWithEventFd;

impl<Arg, Ret> readiness::EventSource<Epoll> for RxWithEventFd<Arg, Ret> {
    type Token = u64;
    type Err = nix::Error;

    fn register(&mut self, registry: &mut Epoll, token: Self::Token) -> Result<(), Self::Err> {
        let flags = EpollFlags::EPOLLET | EpollFlags::EPOLLIN;
        let event = EpollEvent::new(flags, token);
        registry.add(&self.eventfd, event)
    }

    fn deregister(&mut self, registry: &mut Epoll) -> Result<(), Self::Err> {
        registry.delete(&self.eventfd)
    }
}
