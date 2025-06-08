use event_manager::{EventOps, EventSet, Events};
use rpcore_core::invocation_source::readiness;

use crate::RxWithEventFd;

impl<Arg, Ret> readiness::EventSource<EventOps<'_>> for RxWithEventFd<Arg, Ret> {
    type Token = u32;
    type Err = event_manager::Error;

    fn register(
        &mut self,
        registry: &mut EventOps<'_>,
        token: Self::Token,
    ) -> Result<(), Self::Err> {
        let eventset = EventSet::EDGE_TRIGGERED | EventSet::IN;
        let events = Events::with_data(&self.eventfd, token, eventset);
        registry.add(events)
    }

    fn deregister(&mut self, registry: &mut EventOps<'_>) -> Result<(), Self::Err> {
        let events = Events::new(&self.eventfd, EventSet::empty());
        registry.remove(events)
    }
}
