use std::future::Future;
use std::sync::mpsc;
use std::time::Duration;

use oneshot::RecvTimeoutError;
use rpcore_core::server::{SetToken, Token};

use crate::mpsc_server::{Error, Result};
use crate::{Invocation, TxCallback};

pub struct MpscClient<Arg, Ret> {
    pub(crate) token: Token,
    pub(crate) tx: mpsc::Sender<Invocation<Arg, Ret>>,
}

pub struct MpscSyncClient<Arg, Ret> {
    pub(crate) token: Token,
    pub(crate) tx: mpsc::SyncSender<Invocation<Arg, Ret>>,
}

pub trait CallSettingToken {
    type Arg;
    type Ret;

    fn call(&self, arg: Self::Arg) -> Result<Self::Ret>;
    fn call_timeout(&self, arg: Self::Arg, timeout: Duration) -> Result<Self::Ret>;
    fn call_async(&self, arg: Self::Arg) -> impl Future<Output = Result<Self::Ret>>;
}

macro_rules! impl_call_setting_token {
    ($client:ident) => {
        impl<Arg, Ret> CallSettingToken for $client<Arg, Ret>
        where
            Arg: SetToken,
        {
            type Arg = Arg;
            type Ret = Ret;

            fn call(&self, mut arg: Arg) -> Result<Ret> {
                arg.set_token(self.token);

                let (tx, rx) = oneshot::channel();
                let inv = Invocation {
                    arg,
                    callback: TxCallback::new(tx),
                };
                self.tx.send(inv).map_err(|_| Error::ServerClosed)?;
                rx.recv().map_err(|_| Error::ServerInternalError)
            }

            fn call_timeout(&self, mut arg: Arg, timeout: Duration) -> Result<Ret> {
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

            async fn call_async(&self, mut arg: Arg) -> Result<Ret> {
                arg.set_token(self.token);

                let (tx, rx) = oneshot::channel();
                let inv = Invocation {
                    arg,
                    callback: TxCallback::new(tx),
                };
                self.tx.send(inv).map_err(|_| Error::ServerClosed)?;
                rx.await.map_err(|_| Error::ServerInternalError)
            }
        }
    };
}

macro_rules! impl_call {
    ($client:ident) => {
        impl<Arg, Ret> $client<Arg, Ret> {
            pub fn call(&self, arg: Arg) -> Result<Ret> {
                let (tx, rx) = oneshot::channel();
                let inv = Invocation {
                    arg,
                    callback: TxCallback::new(tx),
                };
                self.tx.send(inv).map_err(|_| Error::ServerClosed)?;
                rx.recv().map_err(|_| Error::ServerInternalError)
            }

            pub fn call_timeout(&self, arg: Arg, timeout: Duration) -> Result<Ret> {
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

            pub async fn call_async(&self, arg: Arg) -> Result<Ret> {
                let (tx, rx) = oneshot::channel();
                let inv = Invocation {
                    arg,
                    callback: TxCallback::new(tx),
                };
                self.tx.send(inv).map_err(|_| Error::ServerClosed)?;
                rx.await.map_err(|_| Error::ServerInternalError)
            }
        }
    };
}

impl_call_setting_token!(MpscClient);
impl_call_setting_token!(MpscSyncClient);

impl_call!(MpscClient);
impl_call!(MpscSyncClient);
