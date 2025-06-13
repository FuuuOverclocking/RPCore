use std::thread;
use std::time::Duration;

use log::{error, info, LevelFilter};
use rpcore::log::LogLayer;
use rpcore::server::{Shutdown, ShutdownBool};
use rpcore_core::server::Hooks;
use rpcore_core::{Callback, Handler, HandlerBuilder};
use rpcore_mpsc::mpsc_server::{self, MpscClient};

struct MyHooks;

impl Hooks for MyHooks {
    fn on_shutdown(&mut self) {
        info!("Server shutdown");
    }

    fn on_error(&mut self, e: &dyn std::error::Error) {
        error!("Server got error: {e}");
    }
}

struct MyHandler;

impl Handler<String> for MyHandler {
    type Ret = String;

    fn handle(&mut self, arg: String, callback: impl Callback<Ret = Self::Ret>) {
        callback.call(arg);
    }
}

fn main() {
    simplelog::SimpleLogger::init(LevelFilter::Info, Default::default()).unwrap();

    let handler = HandlerBuilder::new()
        .layer(LogLayer::default())
        .handler(MyHandler);

    let (mut server, client_builder) = mpsc_server::Builder::new()
        .polling(Some(Duration::from_micros(200)))
        .hooks(MyHooks)
        .build(handler);

    let c1 = client_builder.build_client().unwrap();
    let c2 = client_builder.build_client().unwrap();

    let shutdown = ShutdownBool::new();

    client_random_calling(c1);
    client_random_calling(c2);
    shutdown_after_3s(shutdown.clone());

    server.serve(&shutdown);
}

fn client_random_calling(client: MpscClient<String, String>) {
    thread::spawn(move || loop {
        let sleep_dur = Duration::from_millis(rand::random::<u64>() % 100);
        thread::sleep(sleep_dur);

        let _ = client.call(format!("sleep {sleep_dur:?}")).unwrap();
    });
}

fn shutdown_after_3s(shutdown: ShutdownBool) {
    thread::spawn(move || {
        thread::sleep(Duration::from_secs(3));
        shutdown.shutdown();
    });
}
