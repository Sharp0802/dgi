use crate::resource::context::adapter::Adapter;
use crate::resource::context::device::Device;
use crate::resource::context::config::HotConfig;
use crate::resource::context::instance::Instance;
use std::ops::Deref;
use std::sync::Arc;
use wgpu::{CompositeAlphaMode, PresentMode, SurfaceConfiguration, TextureUsages};
use winit::dpi::PhysicalSize;
use winit::window::Window;

pub struct Surface {
    surface: wgpu::Surface<'static>,
    format: wgpu::TextureFormat,
}

impl Surface {
    pub async fn new(
        window: Arc<Window>,
        instance: &Instance,
        adapter: &Adapter,
    ) -> Result<Self, wgpu::CreateSurfaceError> {
        let surface = instance.create_surface(window.clone())?;
        let format = surface.get_capabilities(&adapter).formats[0];

        Ok(Self {
            surface,
            format,
        })
    }

    pub fn configure(&self, device: &Device, size: PhysicalSize<u32>, hint: &HotConfig) {
        let present_mode = if hint.vsync {
            PresentMode::AutoVsync
        } else {
            PresentMode::AutoNoVsync
        };

        let config = SurfaceConfiguration {
            usage: TextureUsages::RENDER_ATTACHMENT,
            format: self.format,
            view_formats: vec![self.format.add_srgb_suffix()],
            alpha_mode: CompositeAlphaMode::Auto,
            width: size.width,
            height: size.height,
            desired_maximum_frame_latency: 2,
            present_mode,
        };

        self.surface.configure(&device, &config);
    }
    
    pub fn format(&self) -> wgpu::TextureFormat {
        self.format
    }
}

impl Deref for Surface {
    type Target = wgpu::Surface<'static>;

    fn deref(&self) -> &Self::Target {
        &self.surface
    }
}
