pub trait Proactor<Submitter, SQ, CQE, Token> {
    type InitErr;
    type SubmitErr;
    type OnCompleteErr;

    fn init(&mut self, submitter: &Submitter) -> Result<(), Self::InitErr>;
    fn submit(&mut self, sq: &mut SQ, token: Token) -> Result<(), Self::SubmitErr>;
    fn on_complete(&mut self, entry: CQE) -> Result<(), Self::OnCompleteErr>;
}
