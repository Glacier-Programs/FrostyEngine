use std::borrow::Cow;

// for runninf window
use winit::{
    event_loop, window, dpi, 
    event::{Event, WindowEvent, VirtualKeyCode, KeyboardInput, ElementState}
};

use crate::{scene::Scene, render::vertex::VertexTrait};
use crate::render::{
    window::Window,
    vertex::DefaultVertex
};
use crate::input::InputHandler;
use crate::ecs::MetaDataComponent;

// a trait for any struct used as main point of a game
pub trait Runnable{ fn run(self) -> !; }

// the root of a game fully using this engine
pub struct App{
    active_scene: Scene,
    window: Window
}

impl App {
    pub async fn default_new<'b>(default_shader: Cow<'b, str>) -> App{
        let window = Window::new_default_size(default_shader).await;
        App{ 
            active_scene: Scene::empty(), 
            window: window
        }
    }

    /* 
    pub fn get_active_scene(&self) -> &Scene{
        &self.active_scene
    }
    */

    pub fn get_mut_active_scene(&mut self) -> &mut Scene{
        &mut self.active_scene
    }
}

impl Runnable for App{
    /*
     * LOOP:
     * - Take Input
     * - Prepare UpdatingComponentData
     * - Complete Pseudo System Updates (ex: calculate forces applied to physics objects)
     * - Handle Input
     * - Apply UpdatingComponent Updates
     * - Finish System Updates (ex: calculate final positions of physics objects)
     * - Tick Down EphemeralComponent Timers
     * - Render
     */
    fn run(mut self) -> !{
        // since self is not borrowed, it will be dropped after this
        // although that shouldn't matter since this method shouldn't return
        let mut input_handle = InputHandler::new_default();
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
                        _ => {input_handle.recieve_window_input(event);}
                    }
                }
                Event::RedrawRequested(window_id) if window_id == self.window.winit_window.id() => {
                    // rendering
                    // get all entities with a render component
                    let renderables = self.active_scene.get_renderable_entities();

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
