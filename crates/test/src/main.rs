use dgi::log::impls::Console;
use dgi::log::prelude::*;
use dgi::shell::app::App;

fn main() {
    let logger = builder()
        .writer(Console::new().max_verbosity(Verbosity::Debug))
        .run()
        .unwrap();

    App::new().run();

    logger.stop();
}
