use crate::event::{Event, Level};
use crate::fmt::writer::Writer;
use crate::ser::json::Pretty;

pub struct Console {
    max_level: Level,
    colored: bool,
}

impl Console {
    pub const fn new() -> Self {
        Self {
            max_level: Level::Info,
            colored: true,
        }
    }

    pub const fn max_level(mut self, level: Level) -> Self {
        self.max_level = level;
        self
    }

    pub const fn colored(mut self, colored: bool) -> Self {
        self.colored = colored;
        self
    }
    
    
    const fn head(&self, level: Level) -> &'static str {
        if self.colored {
            match level {
                Level::Fatal => "\x1b[1;31mFTL\x1b[0m",
                Level::Error => "\x1b[31mERR\x1b[0m",
                Level::Warn => "\x1b[33mWRN\x1b[0m",
                Level::Info => "\x1b[32mINF\x1b[0m",
                Level::Debug => "\x1b[36mDBG\x1b[0m",
            }
        } else {
            match level {
                Level::Fatal => "FTL",
                Level::Error => "ERR",
                Level::Warn => "WRN",
                Level::Info => "INF",
                Level::Debug => "DBG",
            }
        }
    }
}

impl Writer for Console {
    fn enabled_for(&self, level: Level) -> bool {
        level <= self.max_level
    }

    fn write(&self, event: &Event) {
        println!(
            "{} {} {}[{:02}]: {}",
            event.timestamp.format("%+"),
            self.head(event.level),
            event.module,
            event.thread_id,
            event.message,
        );

        for field in &event.fields {
            println!(
                "  {} = {}",
                field.name,
                Pretty::new(2, &field.value, Default::default())
            );
        }
    }
}
