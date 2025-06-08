use std::result::Result as StdResult;
use std::sync::mpsc;
use std::time::Duration;

use rpcore_core::server::singleplex::Server;
use rpcore_core::server::SyncTokenAllocator;
use rpcore_core::Handler;

use crate::mpsc_server::{Client, Error, MpscServer, Result, Settings, SyncClient};
use crate::{Invocation, Rx};

pub struct Builder<B, Hooks = ()> {
    settings: Settings<Hooks>,
    bound: B,
}

pub struct Unbounded;

pub struct Bounded {
    cap: usize,
}

impl<B, Hooks> Builder<B, Hooks> {
    pub fn new() -> Builder<Unbounded> {
        Builder {
            settings: Settings {
                polling: None,
                hooks: (),
            },
            bound: Unbounded,
        }
    }

    pub fn new_bounded(cap: usize) -> Builder<Bounded> {
        Builder {
            settings: Settings {
                polling: None,
                hooks: (),
            },
            bound: Bounded { cap },
        }
    }

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
    #[allow(clippy::complexity)]
    pub fn build<H, Arg>(
        self,
        handler: H,
    ) -> (
        MpscServer<H, Arg, Hooks>,
        ClientBuilder<Arg, StdResult<H::Ok, H::Err>>,
    )
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
    #[allow(clippy::complexity)]
    pub fn build<H, Arg>(
        self,
        handler: H,
    ) -> (
        MpscServer<H, Arg, Hooks>,
        SyncClientBuilder<Arg, StdResult<H::Ok, H::Err>>,
    )
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
    pub fn build_client(&self) -> Result<Client<Arg, Ret>> {
        Ok(Client {
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
    pub fn build_client(&self) -> Result<SyncClient<Arg, Ret>> {
        Ok(SyncClient {
            token: self.token_allocator.alloc().ok_or(Error::TooManyClients)?,
            tx: self.tx.clone(),
        })
    }
}
