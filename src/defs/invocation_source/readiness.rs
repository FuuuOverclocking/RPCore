pub trait EventSource<Reactor> {
    type Token;
    type Err;

    fn register(&mut self, registry: &mut Reactor, token: Self::Token) -> Result<(), Self::Err>;
    fn deregister(&mut self, registry: &mut Reactor) -> Result<(), Self::Err>;
}
