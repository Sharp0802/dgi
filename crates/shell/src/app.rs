use crate::resource::context::*;
use dgi_log::impls::{Alert, Console};
use dgi_log::prelude::{Handle, Verbosity, builder};
use dgi_log::{expect, info, warn};
use pollster::block_on;
use std::sync::Arc;
use winit::application::ApplicationHandler;
use winit::event::WindowEvent;
use winit::event_loop::{ActiveEventLoop, ControlFlow, EventLoop};
use winit::window::{Window, WindowId};

pub struct App {
    context: Option<Context>,
    logger: Option<Handle>,
}

impl App {
    pub fn new() -> Self {
        let logger = builder()
            .writer(Console::new().max_verbosity(Verbosity::Debug))
            .writer(Alert::new())
            .run()
            .unwrap();

        Self {
            context: None,
            logger: Some(logger),
        }
    }

    pub fn run(mut self) {
        let event_loop = expect!(
            EventLoop::with_user_event().build(),
            "failed to initialize event loop"
        );

        event_loop.set_control_flow(ControlFlow::Poll);

        expect!(
            event_loop.run_app(&mut self),
            "error occurred in event loop"
        );
    }
}

impl ApplicationHandler for App {
    fn resumed(&mut self, event_loop: &ActiveEventLoop) {
        let window = expect!(
            event_loop.create_window(Window::default_attributes()),
            "failed to create window"
        );

        let window = Arc::new(window);

        let context = expect!(
            block_on(Context::new(
                window.clone(),
                Config {
                    cold: ColdConfig {
                        debug: false,
                        power_preference: Default::default(),
                        memory_hints: MemoryPreference::Performance,
                        adapter: None,
                    },
                    hot: HotConfig { vsync: false },
                }
            )),
            "failed to initialize context"
        );
        self.context = Some(context);

        window.request_redraw();
    }

    fn window_event(
        &mut self,
        event_loop: &ActiveEventLoop,
        _window_id: WindowId,
        event: WindowEvent,
    ) {
        let Some(state) = self.context.as_mut() else {
            warn!("state is not yet initialized; ignoring event...");
            return;
        };

        match event {
            WindowEvent::CloseRequested => {
                info!("close requested; exiting...");

                self.logger.take().unwrap().stop();
                event_loop.exit();
            }

            WindowEvent::RedrawRequested => {
                state.render(&[]).unwrap();
            }

            WindowEvent::Resized(size) => {
                state.resize(size);
            }

            _ => (),
        }
    }
}
