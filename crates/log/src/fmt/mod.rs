use crate::event::{Event, Verbosity};
use crate::fmt::builder::Builder;
use crate::fmt::state::State;
use std::sync::mpsc::TrySendError;

mod builder;
mod state;

pub mod writer;

pub use crate::macros::*;


pub fn builder() -> Builder {
    Builder::new()
}

pub fn write(event: Event, force: bool) {
    match State::get().sender.try_send(event) {
        Ok(_) => return,

        Err(TrySendError::Full(event)) if force => {
            if State::get().sender.send(event).is_ok() {
                return;
            }
        }

        Err(TrySendError::Full(_)) => return,

        Err(TrySendError::Disconnected(_)) => {}
    }

    #[cfg(debug_assertions)]
    panic!("channel disconnected");
}

pub fn should_write(verbosity: Verbosity) -> bool {
    State::get().max_verbosity >= verbosity
}
