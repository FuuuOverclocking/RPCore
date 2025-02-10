mod single_path;

pub mod settings;

pub struct Server<Src, H, Sets> {
    invocation_source: Src,
    handler: H,
    settings: Sets,
}

trait SinglePath {
    fn run(&mut self, shutdown: Shutdown);
}

trait SinglePathWithPolling {
    fn run(&mut self, shutdown: Shutdown);
}
