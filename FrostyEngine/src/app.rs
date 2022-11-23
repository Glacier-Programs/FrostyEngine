use std::borrow::Cow;

// for runninf window
use winit::{
    event_loop, window, dpi, 
    event::{Event, WindowEvent, VirtualKeyCode, KeyboardInput, ElementState}
};

use crate::scene::Scene;
use crate::render::{window::Window};
use crate::input::InputHandler;

// a trait for any struct used as main point of a game
pub trait Runnable{ fn run(self) -> !; }

// the root of a game fully using this engine
pub struct App<'a: 'static>{
    active_scene: Scene<'a>,
    window: Window
}

impl<'a: 'static> App<'a>{
    pub async fn default_new<'b>(default_shader: Cow<'b, str>) -> App<'a>{
        let window = Window::new_default_size(default_shader).await;
        App{ 
            active_scene: Scene::empty(), 
            window: window
        }
    }

    pub fn get_active_scene(&self) -> &Scene{
        &self.active_scene
    }

    pub fn get_mut_active_scene<'b: 'a>(&mut self) -> &mut Scene<'b>{
        &mut self.active_scene
    }
}

impl<'a> Runnable for App<'a>{
    // for discution, should this be moved into an app struct? 
    // then users would do app::new(*info).run();
    fn run(mut self) -> !{
        // since self is not borrowed, it will be dropped after this
        // although that shouldn't matter since this method shouldn't return
        let input_handle = InputHandler::new_default();
        self.window.event_loop.run(move |event, _, control_flow|{
            match event {
                Event::WindowEvent { ref event, window_id } if window_id == self.window.winit_window.id() => {
                    match event {
                        WindowEvent::CloseRequested => {
                            *control_flow = event_loop::ControlFlow::Exit
                        },
                        WindowEvent::Resized(physical_size) => {
                            self.window.render_backend.resize(*physical_size);
                        },
                        WindowEvent::ScaleFactorChanged { new_inner_size, .. } => {
                            // new_inner_size is &&mut so w have to dereference it twice
                            self.window.render_backend.resize(**new_inner_size);
                        },
                        _ => {input_handle.recieve_input(event);}
                    }
                }
                Event::RedrawRequested(window_id) if window_id == self.window.winit_window.id() => {
                    match self.window.render_backend.render() {
                        // everything went properly
                        Ok(_) => {}
                        // Reconfigure the surface if it's lost or outdated
                        Err(wgpu::SurfaceError::Lost | wgpu::SurfaceError::Outdated) => self.window.render_backend.resize(self.window.render_backend.size),
                        // The system is out of memory, we should probably quit
                        Err(wgpu::SurfaceError::OutOfMemory) => *control_flow = event_loop::ControlFlow::Exit,
                        Err(wgpu::SurfaceError::Timeout) => log::warn!("Surface timeout"),
                    }
                }
                Event::RedrawEventsCleared => {
                    // RedrawRequested will only trigger once, unless it is requested
                    self.window.winit_window.request_redraw();
                }
                _ => {}
            }
        });
    }
}
