mod hooks;
pub use hooks::Hooks;

mod shutdown;
pub use shutdown::{IsShuttingDown, Shutdown, ShutdownBool};

mod token;
pub use token::{GetToken, SetToken, SyncTokenAllocator, Token, UnsyncTokenAllocator};

pub mod settings;
pub mod singleplex;
