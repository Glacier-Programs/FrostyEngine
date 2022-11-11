use winit::{
    event_loop, window, dpi
};
use wgpu;

use super::render_backend::RenderBackend;

// A stucture that acts as a proxy so that users don't have to
// worry about render_backend 
pub struct Window{
    event_loop: event_loop::EventLoop<()>,
    render_backend: RenderBackend
}

impl Window{
    pub async fn new(width: u32, height: u32) -> Self {
        let event_loop = event_loop::EventLoop::new();
        let window_builder = window::WindowBuilder::new();
        let winit_window = window_builder.with_inner_size(dpi::PhysicalSize::new(width, height)).build(&event_loop).unwrap();  
        
        let render_backend = RenderBackend::new(winit_window).await;

        Self { 
            event_loop,
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