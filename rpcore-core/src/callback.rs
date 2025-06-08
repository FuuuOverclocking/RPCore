use std::marker::PhantomData;

pub trait Callback: Send + 'static {
    type Ret;

    fn call(self, out: Self::Ret);
}

pub fn callback_fn<F, Ret>(f: F) -> FnCallback<F, Ret>
where
    F: FnOnce(Ret) + Send + 'static,
{
    FnCallback {
        f,
        _phantom: Default::default(),
    }
}

pub struct FnCallback<F, Ret> {
    f: F,
    _phantom: PhantomData<fn(Ret)>,
}

impl<F, Ret> Callback for FnCallback<F, Ret>
where
    F: FnOnce(Ret) + Send + 'static,
    Ret: 'static,
{
    type Ret = Ret;

    fn call(self, out: Self::Ret) {
        (self.f)(out);
    }
}
