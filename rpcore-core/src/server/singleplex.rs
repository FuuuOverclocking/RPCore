use std::time::{Duration, Instant};

use crate::invocation_source::recv::{Error, RecvInvocation, TryRecvInvocation};
use crate::server::settings::{HasHooks, HasPolling};
use crate::server::{Hooks, IsShuttingDown};
use crate::{Callback, Handler, Invocation};

const CHECK_AFTER_POLL_N_TIMES: u32 = 128;

pub struct Server<I, H, S> {
    pub inv_src: I,
    pub handler: H,
    pub settings: S,
}

impl<I, H, S, Arg, Cb> RecvInvocation<Arg, Cb> for Server<I, H, S>
where
    I: RecvInvocation<Arg, Cb>,
    Cb: Callback,
{
    type RecvErr = I::RecvErr;

    fn recv(&mut self) -> Result<Invocation<Arg, Cb>, Self::RecvErr> {
        self.inv_src.recv()
    }
}

impl<I, H, S, Arg, Cb> TryRecvInvocation<Arg, Cb> for Server<I, H, S>
where
    I: TryRecvInvocation<Arg, Cb>,
    Cb: Callback,
{
    type TryRecvErr = I::TryRecvErr;

    fn try_recv(&mut self) -> Result<Invocation<Arg, Cb>, Self::TryRecvErr> {
        self.inv_src.try_recv()
    }
}

impl<I, H, S, Arg> Handler<Arg> for Server<I, H, S>
where
    H: Handler<Arg>,
{
    type Ok = H::Ok;
    type Err = H::Err;

    fn handle(&mut self, arg: Arg, callback: impl Callback<Ret = Result<Self::Ok, Self::Err>>) {
        self.handler.handle(arg, callback)
    }
}

impl<I, H, S> HasPolling for Server<I, H, S>
where
    S: HasPolling,
{
    fn polling(&self) -> &Option<Duration> {
        self.settings.polling()
    }
}

impl<I, H, S> HasHooks for Server<I, H, S>
where
    S: HasHooks,
{
    type H = S::H;

    fn hooks(&mut self) -> &mut Self::H {
        self.settings.hooks()
    }
}

impl<I, H, S, Arg, Cb> Serve<Arg, Cb> for Server<I, H, S>
where
    I: RecvInvocation<Arg, Cb>,
    Cb: Callback<Ret = Result<Self::Ok, Self::Err>>,
    H: Handler<Arg>,
    S: HasHooks,
{
}

impl<I, H, S, Arg, Cb> ServeWithPolling<Arg, Cb> for Server<I, H, S>
where
    I: TryRecvInvocation<Arg, Cb>,
    Cb: Callback<Ret = Result<Self::Ok, Self::Err>>,
    H: Handler<Arg>,
    S: HasHooks + HasPolling,
{
}

pub trait Serve<Arg, Cb>: RecvInvocation<Arg, Cb> + Handler<Arg> + HasHooks
where
    Cb: Callback<Ret = Result<Self::Ok, Self::Err>>,
{
    fn serve(&mut self, shutdown: &impl IsShuttingDown) {
        loop {
            if shutdown.is_shutting_down() {
                self.hooks().on_shutdown();
                return;
            }

            let inv = match self.recv() {
                Ok(inv) => inv,
                Err(e) if e.is_empty() => continue,
                Err(e) if e.is_closed() => {
                    self.hooks().on_error(&e);
                    self.hooks().on_shutdown();
                    return;
                }
                Err(e) => {
                    self.hooks().on_error(&e);
                    continue;
                }
            };

            self.handle(inv.arg, inv.callback);
        }
    }
}

pub trait ServeWithPolling<Arg, Cb>:
    TryRecvInvocation<Arg, Cb> + Handler<Arg> + HasPolling + HasHooks
where
    Cb: Callback<Ret = Result<Self::Ok, Self::Err>>,
{
    fn serve(&mut self, shutdown: &impl IsShuttingDown) {
        loop {
            if shutdown.is_shutting_down() {
                self.hooks().on_shutdown();
                return;
            }

            let inv = match self.try_recv() {
                Ok(inv) => inv,
                Err(e) if e.is_empty() => continue,
                Err(e) if e.is_closed() => {
                    self.hooks().on_error(&e);
                    self.hooks().on_shutdown();
                    return;
                }
                Err(e) => {
                    self.hooks().on_error(&e);
                    continue;
                }
            };

            self.handle(inv.arg, inv.callback);

            let poll_dur = match self.polling() {
                Some(dur) => *dur,
                None => continue,
            };
            let mut poll_until = Instant::now() + poll_dur;
            let mut i = 0;
            loop {
                i = (i + 1) % CHECK_AFTER_POLL_N_TIMES;
                if i == 0 {
                    if Instant::now() >= poll_until {
                        break;
                    }

                    if shutdown.is_shutting_down() {
                        self.hooks().on_shutdown();
                        return;
                    }
                }

                let inv = match self.try_recv() {
                    Ok(inv) => inv,
                    Err(e) if e.is_empty() => continue,
                    Err(e) if e.is_closed() => {
                        self.hooks().on_error(&e);
                        self.hooks().on_shutdown();
                        return;
                    }
                    Err(e) => {
                        self.hooks().on_error(&e);
                        continue;
                    }
                };

                self.handle(inv.arg, inv.callback);

                if shutdown.is_shutting_down() {
                    self.hooks().on_shutdown();
                    return;
                }
                poll_until = Instant::now() + poll_dur;
            }
        }
    }
}
