pub mod recv;
pub mod readiness;
pub mod completion;

pub struct Invocation<Arg, Cb> {
    pub arg: Arg,
    pub callback: Cb,
}
