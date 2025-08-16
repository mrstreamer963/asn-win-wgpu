use std::sync::Arc;

use asn_gui_core::{AsnGuiWindowConfig, TAsnRenderManager, TAsnWindowManager};
use asn_logger::*;

use winit::{
    application::ApplicationHandler, event::WindowEvent, event_loop::ActiveEventLoop,
    window::WindowId,
};

use crate::{WinitWindow, data::LOG_MODULE_NAME, runner_dataset::RunnerDataset};

impl<R> ApplicationHandler for RunnerDataset<R>
where
    R: TAsnRenderManager<Window = WinitWindow>,
{
    fn about_to_wait(&mut self, _event_loop: &ActiveEventLoop) {
        // This method is called when the event loop is about to wait for new events.
        // You can use this to request a redraw if your application needs continuous rendering.
        if let Some(window) = &self.window {
            // trace(LOG_MODULE_NAME, &format!("about_to_wait"));
            window.request_redraw();
        }
    }

    fn resumed(&mut self, event_loop: &winit::event_loop::ActiveEventLoop) {
        if self.window.is_none() {
            let conf = AsnGuiWindowConfig::default();

            let w = self.new_window(event_loop, &conf).unwrap();
            let arc_w = Arc::new(w);

            self.r.init(arc_w.clone()).unwrap();

            self.window = Some(arc_w);
        }
    }

    fn window_event(&mut self, event_loop: &ActiveEventLoop, id: WindowId, event: WindowEvent) {
        match event {
            WindowEvent::CloseRequested => {
                m_trace!("CloseRequested event");
                self.handle_close(event_loop);
            }
            WindowEvent::RedrawRequested => {
                // trace(LOG_MODULE_NAME, &format!("RedrawRequested event"));
                self.handle_redraw();
            }
            WindowEvent::Resized(size) => {
                m_trace!("Resized event: {size:?}");
                self.handle_resize(size.width, size.height);
            }
            WindowEvent::KeyboardInput { event, .. } => {
                m_trace!("KeyboardInput event: {event:?}");
                self.handle_keyboard_input(event_loop, event);
            }
            WindowEvent::Focused(focused) => {
                m_trace!("Window focus changed: {id:?}");
                m_trace!("Window focus changed: {focused}");
            }
            WindowEvent::ScaleFactorChanged { scale_factor, .. } => {
                m_trace!("Scale factor changed: {scale_factor}");
            }
            _ => {
                m_trace!("Window {id:?} event: {event:?}");
            }
        }
    }
}

impl<R> RunnerDataset<R>
where
    R: TAsnRenderManager<Window = WinitWindow>,
{
    /// Handles application close
    fn handle_close(&mut self, event_loop: &ActiveEventLoop) {
        m_info!("Application close requested");
        if self.window.is_some() {
            let _ = self.window.take();
        }
        event_loop.exit();
    }

    /// Handles window resize events
    fn handle_resize(&mut self, width: u32, height: u32) {
        m_trace!("Resizing window to {width}x{height}");

        if let Err(resize_error) = self.r.resize(width, height) {
            m_error!("Resize failed: {resize_error}");
        } else {
            m_info!("Window resized successfully to {width}x{height}");
        }
    }

    fn handle_redraw(&mut self) {
        match &self.window {
            Some(w) => {
                w.request_redraw();
            }
            None => {
                m_error!("window is None");
                return;
            }
        };

        match self.r.draw() {
            Ok(_) => {}
            Err(err) => {
                m_error!("handle_redraw draw failed: {err}");
                return;
            }
        };
    }

    /// Handles keyboard input events
    fn handle_keyboard_input(
        &mut self,
        event_loop: &ActiveEventLoop,
        event: winit::event::KeyEvent,
    ) {
        use winit::event::ElementState;

        if event.state == ElementState::Pressed {
            match event.logical_key.as_ref() {
                winit::keyboard::Key::Character("Escape") => {
                    m_info!("Escape key pressed - closing application");
                    self.handle_close(event_loop);
                }
                winit::keyboard::Key::Named(winit::keyboard::NamedKey::F11) => {
                    m_info!("F11 key pressed - toggling fullscreen");
                    // TODO: Implement fullscreen toggle
                    m_info!("Fullscreen toggle not yet implemented");
                }
                winit::keyboard::Key::Named(winit::keyboard::NamedKey::F1) => {
                    m_info!("F1 key pressed - showing help");
                    // TODO: Implement help system
                }
                winit::keyboard::Key::Character("r") | winit::keyboard::Key::Character("R") => {
                    m_info!("R key pressed...");
                }
                _ => {
                    m_trace!("Key pressed: {:?}", event.logical_key);
                }
            }
        }
    }
}
