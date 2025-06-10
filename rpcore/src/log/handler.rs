use std::any::type_name;
use std::fmt;
use std::time::Instant;

use crate::{callback_fn, Callback, Handler};

#[derive(Debug)]
pub struct Log<H> {
    inner: H,
    handler_name: String,
}

impl<H> Log<H> {
    pub fn new(inner: H, handler_name: Option<String>) -> Self {
        let handler_name = handler_name.unwrap_or(type_name::<H>().to_owned());
        Self {
            inner,
            handler_name,
        }
    }

    pub fn get_ref(&self) -> &H {
        &self.inner
    }

    pub fn get_mut(&mut self) -> &mut H {
        &mut self.inner
    }

    pub fn into_inner(self) -> H {
        self.inner
    }
}

impl<H, Arg> Handler<Arg> for Log<H>
where
    H: Handler<Arg>,
    H::Ret: fmt::Debug + 'static,
    Arg: fmt::Debug,
{
    type Ret = H::Ret;

    fn handle(&mut self, arg: Arg, callback: impl Callback<Ret = Self::Ret>) {
        let begin_at = Instant::now();
        let handler_name = self.handler_name.clone();
        let formatted_arg = format!("{arg:?}");
        log::info!("[{handler_name}] handling {formatted_arg}");

        self.inner.handle(
            arg,
            callback_fn(move |ret| {
                let elapsed = begin_at.elapsed();
                log::info!(
                    "[{handler_name}] handled  {formatted_arg}, ret: {ret:?}, elapsed: {elapsed:?}"
                );
                callback.call(ret);
            }),
        );
    }
}
