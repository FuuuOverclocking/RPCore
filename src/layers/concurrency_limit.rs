use std::sync::{Arc, Condvar, Mutex};

use crate::defs::layer::Layer;
use crate::defs::{callback_fn, Callback, Handler};

#[derive(Debug)]
pub struct ConcurrencyLimit<H> {
    inner: H,
    limit: u32,
    inflight_count: Arc<(Mutex<u32>, Condvar)>,
}

impl<H> ConcurrencyLimit<H> {
    pub fn new(inner: H, limit: u32) -> Self {
        assert!(limit > 0, "Limit must be greater than 0.");

        Self {
            inner,
            limit,
            inflight_count: Default::default(),
        }
    }
}

impl<H, Arg> Handler<Arg> for ConcurrencyLimit<H>
where
    H: Handler<Arg>,
    H::Ret: 'static,
{
    type Ret = H::Ret;

    fn handle(&mut self, arg: Arg, callback: impl Callback<Ret = Self::Ret>) {
        {
            let (count, cvar) = self.inflight_count.as_ref();
            let mut count = count.lock().unwrap();
            while *count >= self.limit {
                count = cvar.wait(count).unwrap();
            }
            *count += 1;
        }

        let cloned = Arc::clone(&self.inflight_count);

        self.inner.handle(
            arg,
            callback_fn(move |ret| {
                let (count, cvar) = cloned.as_ref();
                let mut count = count.lock().unwrap();
                *count -= 1;
                cvar.notify_one();

                callback.call(ret);
            }),
        );
    }
}

#[derive(Debug, Clone)]
pub struct ConcurrencyLimitLayer {
    limit: u32,
}

impl ConcurrencyLimitLayer {
    pub const fn new(limit: u32) -> Self {
        Self { limit }
    }
}

impl<H> Layer<H> for ConcurrencyLimitLayer {
    type Handler = ConcurrencyLimit<H>;

    fn layer(&self, inner: H) -> Self::Handler {
        ConcurrencyLimit::new(inner, self.limit)
    }
}
