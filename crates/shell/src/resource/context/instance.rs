use std::ops::Deref;
use std::sync::Arc;
use winit::window::Window;
use crate::resource::context::config::ColdConfig;

pub struct Instance {
    instance: wgpu::Instance,
}

impl Instance {
    pub fn new(window: Arc<Window>, hint: &ColdConfig) -> Self {
        let flags = if hint.debug {
            wgpu::InstanceFlags::debugging()
        } else {
            wgpu::InstanceFlags::default()
        };

        let desc = wgpu::InstanceDescriptor {
            backends: Default::default(),
            flags,
            memory_budget_thresholds: Default::default(),
            backend_options: Default::default(),
            display: Some(Box::new(window.clone())),
        };

        let instance = wgpu::Instance::new(desc);

        Self {
            instance,
        }
    }
}

impl Deref for Instance {
    type Target = wgpu::Instance;
    
    fn deref(&self) -> &Self::Target {
        &self.instance
    }
}
