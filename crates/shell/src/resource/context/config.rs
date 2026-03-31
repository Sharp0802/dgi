use serde::{Deserialize, Serialize};
use crate::resource::context::adapter::AdapterCache;

#[derive(Debug, Serialize, Deserialize, Copy, Clone, Eq, PartialEq)]
#[repr(u8)]
pub enum MemoryPreference {
    Performance,
    MemoryUsage,
}

impl From<MemoryPreference> for wgpu::MemoryHints {
    fn from(value: MemoryPreference) -> Self {
        match value {
            MemoryPreference::Performance => wgpu::MemoryHints::Performance,
            MemoryPreference::MemoryUsage => wgpu::MemoryHints::MemoryUsage,
        }
    }
}

#[derive(Debug, Serialize, Deserialize, Clone, Eq, PartialEq)]
pub struct ColdConfig {
    pub debug: bool,
    pub power_preference: wgpu::PowerPreference,
    pub memory_hints: MemoryPreference,
    pub adapter: Option<AdapterCache>,
}

#[derive(Debug, Serialize, Deserialize, Clone, Eq, PartialEq)]
pub struct HotConfig {
    pub vsync: bool,
}

#[derive(Debug, Serialize, Deserialize, Clone, Eq, PartialEq)]
pub struct Config {
    pub cold: ColdConfig,
    pub hot: HotConfig,
}
