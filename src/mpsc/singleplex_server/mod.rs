mod settings;
pub use settings::Settings;

mod builder;
pub use builder::{Bounded, Builder, ClientBuilder, SyncClientBuilder, Unbounded};

mod client;
pub use client::Client;

mod sync_client;
pub use client::SyncClient;

#[derive(Debug, thiserror::Error)]
pub enum Error {
    #[error("Too many clients")]
    TooManyClients,
    #[error("Server closed")]
    ServerClosed,
    #[error("Server internal error")]
    ServerInternalError,
}

pub type Result<T> = std::result::Result<T, Error>;
