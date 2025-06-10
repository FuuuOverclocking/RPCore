use std::sync::mpsc;
use std::time::Duration;

use rpcore_core::server::singleplex::Server;
use rpcore_core::server::SyncTokenAllocator;
use rpcore_core::Handler;

use crate::mpsc_server::{Error, MpscClient, MpscServer, MpscSyncClient, Result, Settings};
use crate::{Invocation, Rx};

pub struct Builder<B, Hooks = ()> {
    settings: Settings<Hooks>,
    bound: B,
}

pub struct Unbounded;

pub struct Bounded {
    cap: usize,
}

impl Builder<Unbounded> {
    #[allow(clippy::new_without_default)]
    pub fn new() -> Builder<Unbounded> {
        Builder {
            settings: Settings {
                polling: None,
                hooks: (),
            },
            bound: Unbounded,
        }
    }
}

impl Builder<Bounded> {
    pub fn new_bounded(cap: usize) -> Builder<Bounded> {
        Builder {
            settings: Settings {
                polling: None,
                hooks: (),
            },
            bound: Bounded { cap },
        }
    }
}

impl<B, Hooks> Builder<B, Hooks> {
    pub fn polling(mut self, polling: Option<Duration>) -> Self {
        self.settings.polling = polling;
        self
    }

    pub fn hooks<H2>(self, hooks: H2) -> Builder<B, H2> {
        let settings = Settings {
            polling: self.settings.polling,
            hooks,
        };
        Builder {
            settings,
            bound: self.bound,
        }
    }
}

impl<Hooks> Builder<Unbounded, Hooks> {
    pub fn build<H, Arg>(
        self,
        handler: H,
    ) -> (MpscServer<H, Arg, Hooks>, ClientBuilder<Arg, H::Ret>)
    where
        H: Handler<Arg>,
    {
        let (tx, rx) = mpsc::channel();
        let inner = Server {
            inv_src: Rx::new(rx),
            handler,
            settings: self.settings,
        };
        let client_builder = ClientBuilder {
            token_allocator: SyncTokenAllocator::default(),
            tx,
        };

        (MpscServer { inner }, client_builder)
    }
}

impl<Hooks> Builder<Bounded, Hooks> {
    pub fn build<H, Arg>(
        self,
        handler: H,
    ) -> (MpscServer<H, Arg, Hooks>, SyncClientBuilder<Arg, H::Ret>)
    where
        H: Handler<Arg>,
    {
        let (tx, rx) = mpsc::sync_channel(self.bound.cap);
        let inner = Server {
            inv_src: Rx::new(rx),
            handler,
            settings: self.settings,
        };
        let client_builder = SyncClientBuilder {
            token_allocator: SyncTokenAllocator::default(),
            tx,
        };

        (MpscServer { inner }, client_builder)
    }
}

pub struct ClientBuilder<Arg, Ret> {
    token_allocator: SyncTokenAllocator,
    tx: mpsc::Sender<Invocation<Arg, Ret>>,
}

impl<Arg, Ret> ClientBuilder<Arg, Ret> {
    pub fn build_client(&self) -> Result<MpscClient<Arg, Ret>> {
        Ok(MpscClient {
            token: self.token_allocator.alloc().ok_or(Error::TooManyClients)?,
            tx: self.tx.clone(),
        })
    }
}

pub struct SyncClientBuilder<Arg, Ret> {
    token_allocator: SyncTokenAllocator,
    tx: mpsc::SyncSender<Invocation<Arg, Ret>>,
}

impl<Arg, Ret> SyncClientBuilder<Arg, Ret> {
    pub fn build_client(&self) -> Result<MpscSyncClient<Arg, Ret>> {
        Ok(MpscSyncClient {
            token: self.token_allocator.alloc().ok_or(Error::TooManyClients)?,
            tx: self.tx.clone(),
        })
    }
}
