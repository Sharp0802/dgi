use std::cell::RefCell;
use crate::event::{Event, Verbosity};
use crate::fmt::writer::Writer;
use std::io::Write;
use std::ops::DerefMut;
use crate::error;

pub struct Json<W: Write> {
    max_verbosity: Verbosity,
    file: RefCell<W>,
}

impl<W: Write> Json<W> {
    pub const fn new(file: W) -> Self {
        Self {
            max_verbosity: Verbosity::Info,
            file: RefCell::new(file),
        }
    }
    
    pub const fn max_verbosity(mut self, max_verbosity: Verbosity) -> Self {
        self.max_verbosity = max_verbosity;
        self
    }
}

impl<W: Write> Writer for Json<W> {
    fn enabled_for(&self, verbosity: Verbosity) -> bool {
        verbosity <= self.max_verbosity
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
