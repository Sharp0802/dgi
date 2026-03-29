use dgi_log::{error, fatal, warn};
use std::sync::Arc;
use thiserror::Error;
use wgpu::*;
use winit::dpi::PhysicalSize;
use winit::window::Window;

#[derive(Debug, Error)]
pub enum InitError {
    #[error("failed to request adapter: {0}")]
    RequestAdapter(#[from] RequestAdapterError),
    #[error("failed to request device: {0}")]
    RequestDevice(#[from] RequestDeviceError),
    #[error("failed to create surface: {0}")]
    CreateSurface(#[from] CreateSurfaceError),
}

pub struct Surface {
    window: Arc<Window>,
    device: Device,
    queue: Queue,
    size: PhysicalSize<u32>,
    surface: wgpu::Surface<'static>,
    surface_format: TextureFormat,
}

impl Surface {
    pub async fn new(window: Arc<Window>) -> Result<Self, InitError> {
        let instance = Instance::new(InstanceDescriptor::new_without_display_handle());
        let adapter = instance.request_adapter(&Default::default()).await?;

        let (device, queue) = adapter.request_device(&Default::default()).await?;

        let size = window.inner_size();

        let surface = instance.create_surface(window.clone())?;
        let surface_format = surface.get_capabilities(&adapter).formats[0];

        let state = Self {
            window,
            device,
            queue,
            size,
            surface,
            surface_format,
        };

        state.configure();

        Ok(state)
    }

    fn configure(&self) {
        let config = SurfaceConfiguration {
            usage: TextureUsages::RENDER_ATTACHMENT,
            format: self.surface_format,
            view_formats: vec![self.surface_format.add_srgb_suffix()],
            alpha_mode: CompositeAlphaMode::Auto,
            width: self.size.width,
            height: self.size.height,
            desired_maximum_frame_latency: 2,
            present_mode: PresentMode::AutoNoVsync,
        };

        self.surface.configure(&self.device, &config);
    }

    pub fn resize(&mut self, size: PhysicalSize<u32>) {
        self.size = size;
        self.configure();
    }

    fn get_current_texture(&self) -> Option<SurfaceTexture> {
        loop {
            match self.surface.get_current_texture() {
                CurrentSurfaceTexture::Success(texture) => return Some(texture),
                CurrentSurfaceTexture::Occluded => return None,

                CurrentSurfaceTexture::Timeout => {
                    warn!("timed out to get texture");
                }

                CurrentSurfaceTexture::Suboptimal(_) => {
                    warn!("surface returns suboptimal texture");
                    self.configure();
                }
                CurrentSurfaceTexture::Outdated => {
                    warn!("surface was outdated");
                    self.configure();
                }

                CurrentSurfaceTexture::Lost => {
                    fatal!("device lost");
                },
                CurrentSurfaceTexture::Validation => {
                    error!("validation failed for surface");
                },
            }
        }
    }

    pub fn render(&self, bundles: &[RenderBundle]) {
        if self.size.width == 0 || self.size.height == 0 {
            return;
        }

        let Some(texture) = self.get_current_texture() else {
            return;
        };

        let view = texture.texture.create_view(&TextureViewDescriptor {
            format: Some(self.surface_format.add_srgb_suffix()),
            ..Default::default()
        });

        let mut encoder = self.device.create_command_encoder(&Default::default());
        {
            let mut render_pass = encoder.begin_render_pass(&RenderPassDescriptor {
                label: None,
                color_attachments: &[Some(RenderPassColorAttachment {
                    view: &view,
                    depth_slice: None,
                    resolve_target: None,
                    ops: Operations {
                        load: LoadOp::Clear(Color::GREEN),
                        store: StoreOp::Store,
                    },
                })],
                depth_stencil_attachment: None,
                timestamp_writes: None,
                occlusion_query_set: None,
                multiview_mask: None,
            });

            render_pass.execute_bundles(bundles);
        }

        self.queue.submit([encoder.finish()]);
        self.window.pre_present_notify();
        texture.present();
    }
}
