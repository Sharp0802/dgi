use crate::prelude::json::Pretty;
use crate::fmt::Writer;
use crate::prelude::{Event, Verbosity};

fn alert(message: &str) {
    use msgbox::*;

    create("Error", message, IconType::Error).unwrap();
}

pub struct Alert {
    max_verbosity: Verbosity,
}

impl Alert {
    pub const fn new() -> Self {
        Self {
            max_verbosity: Verbosity::Fatal,
        }
    }

    pub const fn max_verbosity(mut self, verbosity: Verbosity) -> Self {
        self.max_verbosity = verbosity;
        self
    }
}

impl Writer for Alert {
    fn enabled_for(&self, verbosity: Verbosity) -> bool {
        verbosity <= self.max_verbosity
    }

    fn write(&self, event: &Event) {
        let mut message = format!(
            "{} {}[{}] {}",
            event.timestamp, event.module, event.thread_id, event.message,
        );

        for field in &event.fields {
            message += &format!(
                "\n\n{} = {}",
                field.name,
                Pretty::new(0, &field.value, Default::default())
            );
        }

        alert(&event.message);
    }
}
