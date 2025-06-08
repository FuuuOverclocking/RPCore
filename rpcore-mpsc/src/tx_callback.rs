use std::any::type_name;
use std::fmt;

use rpcore_core::Callback;

pub struct TxCallback<T> {
    tx: oneshot::Sender<T>,
}

impl<T> TxCallback<T> {
    pub const fn new(tx: oneshot::Sender<T>) -> Self {
        Self { tx }
    }
}

impl<T> Callback for TxCallback<T>
where
    T: Send + 'static,
{
    type Ret = T;

    fn call(self, val: T) {
        let result = self.tx.send(val);
        if result.is_err() {
            log::warn!(
                "Failed to send, oneshot::Receiver<{}> disconnected.",
                type_name::<T>()
            );
        }
    }
}

impl<T> fmt::Debug for TxCallback<T> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_struct(format!("TxCallback<{}>", type_name::<T>()).as_str())
            .finish()
    }
}
