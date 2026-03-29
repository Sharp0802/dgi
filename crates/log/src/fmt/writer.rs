use crate::event::{Event, Level};

pub trait Writer {
    fn enabled_for(&self, level: Level) -> bool;
    fn write(&self, event: &Event);
}
