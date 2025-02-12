pub trait CompletionBased<R: Ring> {
    fn submit(&mut self, token: R::Token, sq: &R::Submitter);
    fn on_complete(&mut self, entry: R::CompletionEntry);
}

pub trait Ring {
    type Submitter;
    type Token;
    type CompletionEntry;
}
