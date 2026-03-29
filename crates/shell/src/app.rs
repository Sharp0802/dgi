use dgi_log::{expect, info, warn};
use pollster::block_on;
use std::sync::Arc;
use winit::application::ApplicationHandler;
use winit::event::WindowEvent;
use winit::event_loop::{ActiveEventLoop, ControlFlow, EventLoop};
use winit::window::{Window, WindowId};
use crate::abs::surface::Surface;

pub struct App {
    state: Option<Surface>,
}

impl App {
    pub fn new() -> Self {
        Self { state: None }
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

        let state = expect!(
            block_on(Surface::new(window.clone())),
            "failed to initialize state"
        );
        self.state = Some(state);

        window.request_redraw();
    }

    fn window_event(
        &mut self,
        event_loop: &ActiveEventLoop,
        _window_id: WindowId,
        event: WindowEvent,
    ) {
        let Some(state) = self.state.as_mut() else {
            warn!("state is not yet initialized; ignoring event...");
            return;
        };

        match event {
            WindowEvent::CloseRequested => {
                info!("close requested; exiting...");
                event_loop.exit();
            }

            WindowEvent::RedrawRequested => {
                state.render(&[]);
            }

            WindowEvent::Resized(size) => {
                state.resize(size);
            }

            _ => (),
        }
    }
}
