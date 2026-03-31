use crate::event::{Event, Verbosity};
use crate::fmt::Writer;
use crate::ser::json::Pretty;

pub struct Console {
    max_verbosity: Verbosity,
    colored: bool,
}

impl Console {
    pub const fn new() -> Self {
        Self {
            max_verbosity: Verbosity::Info,
            colored: true,
        }
    }

    pub const fn max_verbosity(mut self, verbosity: Verbosity) -> Self {
        self.max_verbosity = verbosity;
        self
    }

    pub const fn colored(mut self, colored: bool) -> Self {
        self.colored = colored;
        self
    }
    
    
    const fn head(&self, verbosity: Verbosity) -> &'static str {
        if self.colored {
            match verbosity {
                Verbosity::Fatal => "\x1b[1;31mFTL\x1b[0m",
                Verbosity::Error => "\x1b[31mERR\x1b[0m",
                Verbosity::Warn => "\x1b[33mWRN\x1b[0m",
                Verbosity::Info => "\x1b[32mINF\x1b[0m",
                Verbosity::Debug => "\x1b[36mDBG\x1b[0m",
            }
        } else {
            match verbosity {
                Verbosity::Fatal => "FTL",
                Verbosity::Error => "ERR",
                Verbosity::Warn => "WRN",
                Verbosity::Info => "INF",
                Verbosity::Debug => "DBG",
            }
        }
    }
}

impl Writer for Console {
    fn enabled_for(&self, verbosity: Verbosity) -> bool {
        verbosity <= self.max_verbosity
    }

    fn write(&self, event: &Event) {
        println!(
            "{} {} {}[{:02}]: {}",
            event.timestamp.format("%+"),
            self.head(event.verbosity),
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
