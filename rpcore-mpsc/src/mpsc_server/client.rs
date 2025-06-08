use std::sync::mpsc;
use std::time::Duration;

use oneshot::RecvTimeoutError;
use rpcore_core::server::{SetToken, Token};

use crate::mpsc_server::{Error, Result};
use crate::{Invocation, TxCallback};

pub struct Client<Arg, Ret> {
    pub(crate) token: Token,
    pub(crate) tx: mpsc::Sender<Invocation<Arg, Ret>>,
}

pub struct SyncClient<Arg, Ret> {
    pub(crate) token: Token,
    pub(crate) tx: mpsc::SyncSender<Invocation<Arg, Ret>>,
}

macro_rules! impl_call {
    () => {
        pub fn call(&self, mut arg: Arg) -> Result<Ret> {
            arg.set_token(self.token);

            let (tx, rx) = oneshot::channel();
            let inv = Invocation {
                arg,
                callback: TxCallback::new(tx),
            };
            self.tx.send(inv).map_err(|_| Error::ServerClosed)?;
            rx.recv().map_err(|_| Error::ServerInternalError)
        }

        pub fn call_timeout(&self, mut arg: Arg, timeout: Duration) -> Result<Ret> {
            arg.set_token(self.token);

            let (tx, rx) = oneshot::channel();
            let inv = Invocation {
                arg,
                callback: TxCallback::new(tx),
            };
            self.tx.send(inv).map_err(|_| Error::ServerClosed)?;
            match rx.recv_timeout(timeout) {
                Ok(ret) => Ok(ret),
                Err(RecvTimeoutError::Timeout) => Err(Error::ServerTimeout),
                Err(_) => Err(Error::ServerInternalError),
            }
        }

        pub async fn call_async(&self, mut arg: Arg) -> Result<Ret> {
            arg.set_token(self.token);

            let (tx, rx) = oneshot::channel();
            let inv = Invocation {
                arg,
                callback: TxCallback::new(tx),
            };
            self.tx.send(inv).map_err(|_| Error::ServerClosed)?;
            rx.await.map_err(|_| Error::ServerInternalError)
        }
    };
}

impl<Arg, Ret> Client<Arg, Ret>
where
    Arg: SetToken,
{
    impl_call! {}
}

impl<Arg, Ret> SyncClient<Arg, Ret>
where
    Arg: SetToken,
{
    impl_call! {}
}
