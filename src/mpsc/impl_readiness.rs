use std::sync::mpsc::TryRecvError;

use event_manager::{EventSet, Events};

use super::RxWithEventFd;
use crate::defs::invocation_source::readiness;
use crate::defs::invocation_source::recv::TryRecvInvocation;

impl<'a, Arg, Ret> readiness::Subscriber<event_manager::EventOps<'a>> for RxWithEventFd<Arg, Ret>
where
    Ret: Send + 'static,
{
    type Event = event_manager::Events;

    fn init(&mut self, ctl: &mut event_manager::EventOps<'a>) {
        let ev = Events::new(&self.eventfd, EventSet::IN);
        if let Err(e) = ctl.add(ev) {
            log::error!("mpsc: Failed to register eventfd to the epoll instance: {e}");
        }
    }

    fn on_ready(&mut self, events: Self::Event, ctl: &mut event_manager::EventOps<'a>) {
        loop {
            let inv = match self.rx.try_recv() {
                Ok(inv) => inv,
                Err(TryRecvError::Empty) => break,
                Err(TryRecvError::Disconnected) => {
                    log::info!("mpsc: All clients disconnected.");
                    break;
                }
            };

        }
    }
}
