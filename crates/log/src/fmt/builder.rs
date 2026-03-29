use crate::fmt::state::{State, Writers};
use crate::fmt::writer::Writer;
use std::sync::atomic::{AtomicBool, Ordering};
use std::sync::Arc;
use std::thread::{spawn, JoinHandle};
use std::time::Duration;

pub struct Builder {
    writers: Writers,
}

impl Builder {
    pub fn new() -> Self {
        Self {
            writers: Writers::new(),
        }
    }

    pub fn writer<T: Writer + Send + 'static>(mut self, writer: T) -> Self {
        self.writers.push(Box::new(writer));
        self
    }

    pub fn run(self) -> Result<Handle, ()> {
        let rx = State::init(&self.writers);

        let token = Arc::new(AtomicBool::new(true));
        
        let token_copy = token.clone();
        let handle = spawn(move || {
            while token_copy.load(Ordering::Acquire) {
                let Ok(event) = rx.recv_timeout(Duration::from_millis(100)) else {
                    continue;
                };
                
                self.writers.write(&event);
            }
            
            while let Ok(event) = rx.try_recv() {
                self.writers.write(&event);
            }
        });

        Ok(Handle {
            join_handle: handle,
            stop_token: token,
        })
    }
}

pub struct Handle {
    join_handle: JoinHandle<()>,
    stop_token: Arc<AtomicBool>,
}

impl Handle {
    pub fn stop(self) {
        self.stop_token.store(false, Ordering::Release);

        // ignore: it's assumed that the logging thread will not be panic
        let _ = self.join_handle.join();
    }
}
