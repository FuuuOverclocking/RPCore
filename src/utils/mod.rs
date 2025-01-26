mod oneshot_callback;
pub use oneshot_callback::OneshotCallback;

#[cfg(all(
    feature = "event-manager",
    any(target_os = "linux", target_os = "android")
))]
mod event_manager;
