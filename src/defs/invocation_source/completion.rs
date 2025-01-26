pub trait CompletionBased {
    type Ring: RingSubmitter;
    type CompletionEntry;

    fn init(&mut self, ring: &Self::Ring);
    fn on_complete(&mut self, entry: Self::CompletionEntry, ring: &Self::Ring);
}

pub trait RingSubmitter {
    type SubmissionEntry;

    fn submit(&self, entry: Self::SubmissionEntry);
}
