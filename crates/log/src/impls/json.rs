use std::cell::RefCell;
use crate::event::{Event, Level};
use crate::fmt::writer::Writer;
use std::io::Write;
use std::ops::DerefMut;
use crate::error;

pub struct Json<W: Write> {
    max_level: Level,
    file: RefCell<W>,
}

impl<W: Write> Json<W> {
    pub const fn new(file: W) -> Self {
        Self {
            max_level: Level::Info,
            file: RefCell::new(file),
        }
    }
    
    pub const fn max_level(mut self, max_level: Level) -> Self {
        self.max_level = max_level;
        self
    }
}

impl<W: Write> Writer for Json<W> {
    fn enabled_for(&self, level: Level) -> bool {
        level <= self.max_level
    }

    fn write(&self, event: &Event) {
        if event.module == module_path!() {
            return;
        }

        let mut borrow = self.file.borrow_mut();

        if let Err(e) = serde_json::to_writer(borrow.deref_mut(), event) {
            error!(ignorable "failed to write event", error = e.to_string());
        }

        if let Err(e) = borrow.write(b"\n") {
            error!(ignorable "failed to write event delimiter", error = e.to_string());
        }
    }
}
