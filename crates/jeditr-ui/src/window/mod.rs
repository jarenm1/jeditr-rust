use std::sync::Arc;

use winit::{
    application::ApplicationHandler,
    event_loop::{self, EventLoop},
    window::{Fullscreen, Window, WindowAttributes},
};

use crate::renderer::RenderState;

pub struct App {
    window: Option<Arc<Window>>,
    renderer: Option<RenderState>,
}

impl App {
    pub fn new() -> Self {
        Self {
            window: None,
            renderer: None,
        }
    }
    pub fn run(mut self) {
        let event_loop = EventLoop::new().unwrap();

        event_loop
            .run_app(&mut self)
            .expect("Failed to run event loop")
    }
}

impl ApplicationHandler for App {
    fn resumed(&mut self, event_loop: &event_loop::ActiveEventLoop) {
        if self.window.is_none() {
            let window = {
                let mut window_attributes = WindowAttributes::default();
                window_attributes = window_attributes.with_title("App");

                event_loop
                    .create_window(window_attributes)
                    .expect("Failed to create window")
            };

            let window_arc = Arc::new(window);
            self.window = Some(window_arc.clone());
            self.renderer = Some(pollster::block_on(RenderState::new(window_arc.clone())));

            if let Some(renderer) = self.renderer.as_mut() {
                renderer.resize(renderer.size());
            }
        }
    }
    fn window_event(
        &mut self,
        event_loop: &event_loop::ActiveEventLoop,
        window_id: winit::window::WindowId,
        event: winit::event::WindowEvent,
    ) {
        if self.window.as_ref().map_or(false, |w| w.id() == window_id) {
            let renderer = match self.renderer.as_mut() {
                Some(r) => r,
                None => return,
            };

            match event {
                winit::event::WindowEvent::CloseRequested => {
                    event_loop.exit();
                }
                winit::event::WindowEvent::Resized(physical_size) => renderer.resize(physical_size),
                winit::event::WindowEvent::RedrawRequested => {
                    renderer.update();
                    match renderer.render() {
                        Ok(_) => {}
                        Err(wgpu::SurfaceError::Lost) => renderer.resize(renderer.size()),
                        Err(wgpu::SurfaceError::OutOfMemory) => event_loop.exit(),
                        Err(e) => eprintln!("Error rendering frame: {:?}", e),
                    }
                    if let Some(window) = self.window.as_ref() {
                        window.request_redraw();
                    }
                }
                _ => {}
            }
        }
    }
}
