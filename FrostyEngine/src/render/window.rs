use std::{borrow::Cow};
use winit::{
    event_loop, window, dpi, 
    //event::{Event, WindowEvent, VirtualKeyCode, KeyboardInput, ElementState}
};
//use wgpu;

use super::{render_backend::RenderBackend, vertex::VertexTrait};
use crate::input::InputHandler;

// A stucture that acts as a proxy so that users don't have to
// worry about render_backend 
pub struct Window{
    pub(crate) event_loop: event_loop::EventLoop<()>,
    pub(crate) winit_window: window::Window,
    pub(crate) render_backend: RenderBackend
}

impl Window{

    /*
        In the future, a macro should be usd to create a window.
        The macro should be able to take specific attributes
        Attributes not entered should have some default value 
    */

    pub async fn new<'a>(width: u32, height: u32, default_shader: Cow<'a, str>) -> Self {
        // creates a window with requested sizes
        let event_loop = event_loop::EventLoop::new();
        let window_builder = window::WindowBuilder::new();
        let winit_window = window_builder.with_inner_size(dpi::PhysicalSize::new(width, height)).build(&event_loop).unwrap();  
        
        let render_backend = RenderBackend::new(&winit_window, default_shader).await;

        Self { 
            event_loop,
            winit_window,
            render_backend
        }
    }

    pub async fn new_default_size<'a>(default_shader: Cow<'a, str>) -> Self {
        // creates a window with the default size as decided by winit
        let event_loop = event_loop::EventLoop::new();
        let window_builder = window::WindowBuilder::new();
        let winit_window = window_builder.build(&event_loop).unwrap();  
        
        let render_backend = RenderBackend::new(&winit_window, default_shader).await;

        Self { 
            event_loop,
            winit_window,
            render_backend
        }
    }

    pub fn resize(&mut self, width: u32, height: u32){
        self.render_backend.resize(dpi::PhysicalSize::new(width, height));
    }

    pub fn get_event_loop(&self) -> &event_loop::EventLoop<()>{
        &self.event_loop
    }
}