// mod callback;

// use std::sync::mpsc;

// use event_manager::MutEventSubscriber;
// use log::info;

// use crate::defs::{Callback, Invocation, RecvInvocation, TryRecvInvocation};

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

// pub struct ChanInvocationSource<Arg, Cb> {
//     rx: mpsc::Receiver<Invocation<Arg, Cb>>,
// }

// impl<Arg, Ret, Cb> RecvInvocation<Arg, Ret, Cb> for ChanInvocationSource<Arg, Cb>
// where
//     Cb: Callback<Ret>,
// {
//     type Err = mpsc::RecvError;

//     fn recv(&mut self) -> Result<Invocation<Arg, Cb>, Self::Err> {
//         self.rx.recv()
//     }
// }

// impl<Arg, Ret, Cb> TryRecvInvocation<Arg, Ret, Cb> for ChanInvocationSource<Arg, Cb>
// where
//     Cb: Callback<Ret>,
// {
//     type Err = mpsc::TryRecvError;

//     fn try_recv(&mut self) -> Result<Invocation<Arg, Cb>, Self::Err> {
//         self.rx.try_recv()
//     }
// }

// impl<Arg, Ret, Cb> MutEventSubscriber for ChanInvocationSource<Arg, Cb>
// where
//     Cb: Callback<Ret>,
// {
//     fn init(&mut self, ops: &mut event_manager::EventOps) {
        
//     }

//     fn process(&mut self, events: event_manager::Events, ops: &mut event_manager::EventOps) {
//         todo!()
//     }
// }
