mod rx;
pub use rx::Rx;

#[cfg(any(target_os = "linux", target_os = "android"))]
mod rx_with_event_fd;
#[cfg(any(target_os = "linux", target_os = "android"))]
pub use rx_with_event_fd::RxWithEventFd;

mod oneshot_callback;
pub use oneshot_callback::OneshotCallback;

#[cfg(any(target_os = "linux", target_os = "android"))]
mod impl_completion;
#[cfg(any(target_os = "linux", target_os = "android"))]
mod impl_readiness;
mod impl_recv;

pub mod singleplex_server;

use crate::defs::invocation_source;

pub type Invocation<Arg, Ret> = invocation_source::Invocation<Arg, OneshotCallback<Ret>>;
