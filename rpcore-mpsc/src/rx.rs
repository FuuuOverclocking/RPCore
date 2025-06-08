use std::any::type_name;
use std::fmt;
use std::sync::mpsc;

use crate::Invocation;

pub struct Rx<Arg, Ret> {
    pub(crate) rx: mpsc::Receiver<Invocation<Arg, Ret>>,
}

impl<Arg, Ret> Rx<Arg, Ret> {
    pub const fn new(rx: mpsc::Receiver<Invocation<Arg, Ret>>) -> Self {
        Self { rx }
    }

    pub fn into_inner(self) -> mpsc::Receiver<Invocation<Arg, Ret>> {
        self.rx
    }
}

impl<Arg, Ret> fmt::Debug for Rx<Arg, Ret> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_struct(&format!(
            "Rx<{}, {}>",
            type_name::<Arg>(),
            type_name::<Ret>()
        ))
        .finish_non_exhaustive()
    }
}
