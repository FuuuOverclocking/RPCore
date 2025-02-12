pub trait Subscriber<R: ReactorRegistry> {
    type Event;

    fn init(&mut self, ctl: &mut R);
    fn on_ready(&mut self, events: Self::Event, ctl: &mut R);
}

pub trait ReactorRegistry {
    type Source;
    type Settings;
    type Err;

    fn add(&mut self, source: Self::Source, settings: Self::Settings) -> Result<(), Self::Err>;
    fn modify(&mut self, source: Self::Source, settings: Self::Settings) -> Result<(), Self::Err>;
    fn remove(&mut self, source: Self::Source) -> Result<(), Self::Err>;
}

pub trait EventSource<Reactor> {
    type Token;
    type Err;

    fn register(&mut self, registry: &mut Reactor, token: Self::Token) -> Result<(), Self::Err>;
    fn deregister(&mut self, registry: &mut Reactor, token: Self::Token) -> Result<(), Self::Err>;
}
