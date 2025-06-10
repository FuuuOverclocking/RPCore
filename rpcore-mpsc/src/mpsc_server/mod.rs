pub mod builder;
pub use builder::Builder;

mod settings;
pub(crate) use settings::Settings;

mod server;
pub use server::MpscServer;

mod client;
pub use client::{CallSettingToken, MpscClient, MpscSyncClient};

#[derive(Debug, thiserror::Error)]
pub enum Error {
    #[error("Too many clients")]
    TooManyClients,

    #[error("Server closed")]
    ServerClosed,

    #[error("Server internal error")]
    ServerInternalError,

    #[error("Server timeout")]
    ServerTimeout,
}

pub type Result<T> = std::result::Result<T, Error>;
