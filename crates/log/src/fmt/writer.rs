use crate::event::{Event, Verbosity};

pub trait Writer {
    fn enabled_for(&self, verbosity: Verbosity) -> bool;
    fn write(&self, event: &Event);
}
