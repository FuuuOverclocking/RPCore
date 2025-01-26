use std::os::fd::RawFd;

use event_manager::{EventOps, EventSet, Events};

use crate::defs::invocation_source::readiness::ReactorRegistry;

impl ReactorRegistry for EventOps<'_> {
    type Source = RawFd;
    type Settings = (EventSet, u32);
    type Err = event_manager::Error;

    fn add(&mut self, source: Self::Source, settings: Self::Settings) -> Result<(), Self::Err> {
        let events = Events::with_data_raw(source, settings.1, settings.0);
        self.add(events)
    }

    fn modify(&mut self, source: Self::Source, settings: Self::Settings) -> Result<(), Self::Err> {
        let events = Events::with_data_raw(source, settings.1, settings.0);
        (self as &Self).modify(events)
    }

    fn remove(&mut self, source: Self::Source) -> Result<(), Self::Err> {
        let events = Events::with_data_raw(source, 0, EventSet::empty());
        self.remove(events)
    }
}
