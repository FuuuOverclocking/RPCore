// use log::info;

// use crate::defs::Callback;

// pub struct OneshotCallback<Ret> {
//     tx: oneshot::Sender<Ret>,
// }

// impl<Ret> Callback<Ret> for OneshotCallback<Ret>
// where
//     Ret: Send + 'static,
// {
//     fn call(self, ret: Ret) {
//         let result = self.tx.send(ret);
//         if result.is_err() {
//             info!("client closed");
//         }
//     }
// }
