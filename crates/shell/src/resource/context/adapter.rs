use crate::resource::context::config::ColdConfig;
use crate::resource::context::instance::Instance;
use crate::util::{MapAsync, OrElseAsync};
use dgi_log::warn;
use serde::{Deserialize, Serialize};
use std::ops::Deref;

#[derive(Debug, Serialize, Deserialize, Clone, Eq, PartialEq)]
pub struct AdapterCache {
    name: String,
    backend: wgpu::Backend,
}

impl AdapterCache {
    pub fn new(name: String, backend: wgpu::Backend) -> Self {
        Self { name, backend }
    }

    fn is_matched(&self, adapter: &wgpu::AdapterInfo) -> bool {
        self.name == adapter.name && self.backend == adapter.backend
    }
}

pub struct Adapter {
    adapter: wgpu::Adapter,
}

impl Adapter {
    async fn get_adapter(instance: &wgpu::Instance, hint: &ColdConfig) -> Option<wgpu::Adapter> {
        instance
            .request_adapter(&wgpu::RequestAdapterOptions {
                power_preference: hint.power_preference,
                force_fallback_adapter: false,
                compatible_surface: None,
            })
            .await
            .ok()
    }

    async fn find_adapter(
        instance: &wgpu::Instance,
        cache: &AdapterCache,
        deny: &[AdapterCache],
    ) -> Option<wgpu::Adapter> {
        instance
            .enumerate_adapters(wgpu::Backends::all())
            .await
            .into_iter()
            .find(|adapter| {
                let info = adapter.get_info();
                !deny.iter().any(|f| f.is_matched(&info)) && cache.is_matched(&info)
            })
    }

    pub async fn new(instance: &Instance, hint: &ColdConfig, deny: &[AdapterCache]) -> Option<Self> {
        let adapter = hint
            .adapter
            .clone()
            .map_async(async |cache| Self::find_adapter(instance, &cache, deny).await)
            .await
            .flatten()
            .or_else_async(async || {
                if hint.adapter.is_some() {
                    warn!(
                        "cached adapter not found; do fallback...",
                        cache = hint.adapter
                    );
                }

                Self::get_adapter(instance, hint).await
            })
            .await;

        adapter.map(|adapter| Self { adapter })
    }
}

impl Deref for Adapter {
    type Target = wgpu::Adapter;

    fn deref(&self) -> &Self::Target {
        &self.adapter
    }
}
