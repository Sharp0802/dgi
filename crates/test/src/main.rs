use std::fs::File;
use dgi_log::impls::{Console, Json};
use dgi_log::prelude::*;

fn main() {
    let Ok(file) = File::create("./log.json") else {
        eprintln!("cannot open log.json");
        return
    };

    let logger = builder()
        .writer(Console::new().max_level(Level::Debug))
        .writer(Json::new(file))
        .run()
        .unwrap();

    error!("hello! it's error", extra = "extra things here");
    warn!("hello! it's warning");
    info!("hello! it's info");
    debug!("hello! it's debug");

    logger.stop();
}
