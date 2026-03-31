use crate::resource::context::adapter::{Adapter, AdapterCache};
use crate::resource::context::config::Config;
use crate::resource::context::device::Device;
use crate::resource::context::instance::Instance;
use crate::resource::context::surface::Surface;
use crate::resource::version::Version;
use dgi_log::{error, warn};
use std::sync::Arc;
use thiserror::Error;
use wgpu::{
    Color, CurrentSurfaceTexture, LoadOp, Operations, RenderPassColorAttachment,
    RenderPassDescriptor, StoreOp, TextureViewDescriptor,
};
use winit::dpi::PhysicalSize;
use winit::window::Window;

pub struct Context {
    cold_version: Version,
    hot_version: Version,
    report: Report,

    config: Config,
    window: Arc<Window>,
    size: PhysicalSize<u32>,

    // cold stage
    instance: Instance,
    adapter: Adapter,
    device: Device,

    // hot stage
    surface: Surface,
}

pub struct AdapterError {
    adapter: AdapterCache,
    message: String,
}

pub struct Report {
    adapter_errors: Vec<AdapterError>,
}

#[derive(Debug, Error)]
pub enum InitError {
    #[error("there is no available adapter")]
    NoAvailableAdapter,
}

#[derive(Debug, Error)]
pub enum RenderError {
    #[error("device lost")]
    DeviceLost,
}

impl Context {
    pub async fn new(window: Arc<Window>, mut config: Config) -> Result<Self, InitError> {
        let instance = Instance::new(window.clone(), &config.cold);

        let mut deny: Vec<AdapterCache> = vec![];
        let mut errors: Vec<String> = vec![];

        let (adapter, device, surface) = loop {
            let Some(adapter) = Adapter::new(&instance, &config.cold, &deny).await else {
                return Err(InitError::NoAvailableAdapter);
            };

            let device = match Device::new(&adapter, &config.cold).await {
                Ok(device) => device,
                Err(e) => {
                    error!("failed to create device", error = e);

                    let info = adapter.get_info();
                    deny.push(AdapterCache::new(info.name, info.backend));
                    errors.push(format!("failed to init device: {}", e));

                    continue;
                }
            };

            let surface = match Surface::new(window.clone(), &instance, &adapter).await {
                Ok(surface) => surface,
                Err(e) => {
                    error!("failed to create surface", error = e);

                    let info = adapter.get_info();
                    deny.push(AdapterCache::new(info.name, info.backend));
                    errors.push(format!("failed to init surface: {}", e));

                    continue;
                }
            };

            break (adapter, device, surface);
        };

        let adapter_info = adapter.get_info();
        config.cold.adapter = Some(AdapterCache::new(adapter_info.name, adapter_info.backend));

        let adapter_errors = deny
            .into_iter()
            .zip(errors.into_iter())
            .map(|(adapter, message)| AdapterError { adapter, message })
            .collect();

        let report = Report { adapter_errors };

        let size = window.inner_size();
        surface.configure(&device, size, &config.hot);

        Ok(Self {
            cold_version: Version::new(),
            hot_version: Version::new(),
            report,

            config,
            window,
            size,

            instance,
            adapter,
            device,
            surface,
        })
    }

    pub async fn renew(&mut self, config: Option<Config>) -> Result<(), InitError> {

        async fn hard_renew(ctx: &mut Context, config: Config) -> Result<(), InitError> {
            let cold_v = ctx.cold_version;
            let hot_v = ctx.hot_version;

            *ctx = Context::new(ctx.window.clone(), config).await?;

            ctx.cold_version = cold_v.next();
            ctx.hot_version = hot_v.next();

            Ok(())
        }

        match config {
            Some(config) if self.config.cold != config.cold => {
                hard_renew(self, config).await?;
            }

            Some(config) if self.config.hot != config.hot => {
                self.config = config;
                self.reconfigure_surface();
                self.hot_version = self.hot_version.next();
            }

            Some(_config) => {
                // do nothing: trust `Config: Eq`
            }

            None => {
                hard_renew(self, self.config.clone()).await?;
            }
        }

        Ok(())
    }

    pub fn config(&self) -> &Config {
        &self.config
    }
}

impl Context {
    fn reconfigure_surface(&self) {
        self.surface
            .configure(&self.device, self.size, &self.config.hot);
    }

    pub fn resize(&mut self, size: PhysicalSize<u32>) {
        self.size = size;
        self.surface.configure(&self.device, size, &self.config.hot);
    }

    fn get_current_texture(&self) -> Result<Option<wgpu::SurfaceTexture>, RenderError> {
        match self.surface.get_current_texture() {
            CurrentSurfaceTexture::Success(texture) => Ok(Some(texture)),
            CurrentSurfaceTexture::Occluded => Ok(None),

            CurrentSurfaceTexture::Timeout => {
                warn!("timed out to get texture");
                Ok(None)
            }
            CurrentSurfaceTexture::Suboptimal(_) => {
                warn!("surface returns suboptimal texture");
                self.reconfigure_surface();
                Ok(None)
            }
            CurrentSurfaceTexture::Outdated => {
                warn!("surface was outdated");
                self.reconfigure_surface();
                Ok(None)
            }
            CurrentSurfaceTexture::Validation => {
                error!("surface validation failed");
                Ok(None)
            }

            CurrentSurfaceTexture::Lost => Err(RenderError::DeviceLost),
        }
    }

    pub fn render<'a, I: IntoIterator<Item = &'a wgpu::RenderBundle>>(
        &self,
        bundles: I,
    ) -> Result<(), RenderError> {
        if self.size.width == 0 || self.size.height == 0 {
            return Ok(());
        }

        let Some(texture) = self.get_current_texture()? else {
            return Ok(());
        };

        let view = texture.texture.create_view(&TextureViewDescriptor {
            format: Some(self.surface.format().add_srgb_suffix()),
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

        self.device.queue().submit([encoder.finish()]);
        self.window.pre_present_notify();
        texture.present();

        Ok(())
    }
}
