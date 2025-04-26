use crate::mpsc::singleplex_server::Settings;

pub struct Builder<IsBounded> {
    settings: Settings,
    is_bounded: IsBounded,
}

pub struct Unbounded;
pub struct Bounded(usize);

impl Builder<Unbounded> {
    pub fn new() -> Self {
        Self {
            settings: Default::default(),
            is_bounded: Unbounded,
        }
    }

    pub fn build<H, Arg>(self, handler: H) {
        todo!()
    }
}

impl Builder<Bounded> {
    pub fn new(bound: usize) -> Self {
        Self {
            settings: Default::default(),
            is_bounded: Bounded(bound),
        }
    }
}
