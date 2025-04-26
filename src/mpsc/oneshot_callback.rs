use std::any::type_name;
use std::fmt;

use crate::defs::Callback;

pub struct OneshotCallback<T> {
    tx: oneshot::Sender<T>,
}

impl<T> OneshotCallback<T> {
    pub const fn new(tx: oneshot::Sender<T>) -> Self {
        Self { tx }
    }
}

impl<T> Callback<T> for OneshotCallback<T>
where
    T: Send + 'static,
{
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

impl<T> fmt::Debug for OneshotCallback<T> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_struct(format!("OneshotCallback<{}>", type_name::<T>()).as_str())
            .finish()
    }
}
