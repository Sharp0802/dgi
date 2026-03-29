use dgi_log::impls::Console;
use dgi_log::prelude::*;
use dgi_shell::app::App;

fn main() {
    let logger = builder()
        .writer(Console::new().max_level(Level::Debug))
        .run()
        .unwrap();

    App::new().run();

    logger.stop();
}
