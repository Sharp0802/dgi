use crate::resource::context::adapter::Adapter;
use std::ops::Deref;
use crate::resource::context::config::ColdConfig;

pub struct Device {
    device: wgpu::Device,
    queue: wgpu::Queue,
}

impl Device {
    pub async fn new(adapter: &Adapter, hint: &ColdConfig) -> Result<Self, wgpu::RequestDeviceError> {
        let (device, queue) = adapter
            .request_device(&wgpu::DeviceDescriptor {
                required_features: Default::default(),
                memory_hints: hint.memory_hints.into(),
                ..Default::default()
            })
            .await?;
        
        Ok(Self { device, queue })
    }
    
    pub fn queue(&self) -> &wgpu::Queue {
        &self.queue
    }
}

impl Deref for Device {
    type Target = wgpu::Device;
    
    fn deref(&self) -> &Self::Target {
        &self.device
    }
}
