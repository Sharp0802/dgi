use crate::event::{Event, Verbosity};
use crate::fmt::writer::Writer;
use std::sync::mpsc::{sync_channel, Receiver, SyncSender};
use std::sync::OnceLock;

pub struct Writers(Vec<Box<dyn Writer + Send>>);

impl Writer for Writers {
    fn enabled_for(&self, verbosity: Verbosity) -> bool {
        self.0.iter().any(|w| w.enabled_for(verbosity))
    }

    fn write(&self, event: &Event) {
        for writer in &self.0 {
            if writer.enabled_for(event.verbosity) {
                writer.write(event);
            }
        }
    }
}

impl Writers {
    pub fn new() -> Self {
        Self(Vec::new())
    }

    pub fn push(&mut self, w: Box<dyn Writer + Send>) {
        self.0.push(w);
    }
}

pub struct State {
    pub sender: SyncSender<Event>,
    pub max_verbosity: Verbosity,
}

static STATE: OnceLock<State> = OnceLock::new();

impl State {
    pub fn init(writers: &Writers) -> Receiver<Event> {
        let max_verbosity = *Verbosity::all()
            .iter()
            .rfind(|&&l| writers.enabled_for(l))
            .unwrap_or(&Verbosity::Fatal);

        let (sender, rx) = sync_channel(16384);
        let state = Self { sender, max_verbosity };
        if STATE.set(state).is_err() {
            panic!("state is already initialized");
        }

        rx
    }

    pub fn get() -> &'static Self {
        STATE.get().expect("state is not yet initialized")
    }
}
