mod rx;
pub use rx::Rx;

#[cfg(any(target_os = "linux", target_os = "android"))]
mod rx_with_event_fd;
#[cfg(any(target_os = "linux", target_os = "android"))]
pub use rx_with_event_fd::RxWithEventFd;

mod tx_callback;
pub use tx_callback::TxCallback;

#[cfg(any(target_os = "linux", target_os = "android"))]
mod impl_completion;
#[cfg(any(target_os = "linux", target_os = "android"))]
mod impl_readiness;
mod impl_recv;

#[cfg(feature = "server")]
pub mod mpsc_server;

pub type Invocation<Arg, Ret> = rpcore_core::Invocation<Arg, TxCallback<Ret>>;
