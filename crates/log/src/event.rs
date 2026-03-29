use chrono::{DateTime, Utc};
use dashmap::DashMap;
use serde::Serialize;
use serde_json::Value;
use serde_repr::Serialize_repr;
use std::sync::LazyLock;
use std::sync::atomic::{AtomicUsize, Ordering};

#[derive(Serialize_repr, Copy, Clone, Eq, PartialEq, Ord, PartialOrd)]
#[repr(u8)]
pub enum Level {
    Fatal,
    Error,
    Warn,
    Info,
    Debug,
}

impl Level {
    pub fn all() -> &'static [Self] {
        &[
            Self::Fatal,
            Self::Error,
            Self::Warn,
            Self::Info,
            Self::Debug,
        ]
    }
}

#[derive(Serialize)]
pub struct Field {
    pub name: &'static str,
    pub value: Value,
}

#[derive(Serialize)]
pub struct Event {
    pub timestamp: DateTime<Utc>,
    pub level: Level,
    pub thread_id: usize,
    pub module: &'static str,
    pub message: String,
    pub fields: Vec<Field>,
}

static TID: AtomicUsize = AtomicUsize::new(0);
static TID_MAP: LazyLock<DashMap<usize, usize>> = LazyLock::new(|| DashMap::new());

fn get_tid() -> usize {
    *TID_MAP
        .entry(thread_id::get())
        .or_insert_with(|| TID.fetch_add(1, Ordering::AcqRel))
        .value()
}

impl Event {
    pub fn new(level: Level, module: &'static str, message: String, fields: Vec<Field>) -> Self {
        Self {
            timestamp: Utc::now(),
            level,
            thread_id: get_tid(),
            module,
            message,
            fields,
        }
    }
}
