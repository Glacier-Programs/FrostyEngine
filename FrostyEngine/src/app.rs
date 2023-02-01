use std::{borrow::Cow, mem::transmute, cell::RefCell, rc::Rc};

// for runninf window
use winit::{
    event_loop::{self, ControlFlow}, window, dpi::{self, PhysicalPosition, PhysicalSize}, 
    event::{Event, WindowEvent, VirtualKeyCode, KeyboardInput, ElementState}
};

use crate::{scene::Scene, render::{vertex::VertexTrait, sprite_component::{RenderableComponent, comp_to_return_buffer}}};
use crate::render::{
    window::Window,
    vertex::DefaultVertex,
    sprite_component::ReturnsBuffer
};
use crate::input::InputHandler;
use crate::ecs::{Component, MetaDataComponent, component::downcast_component};
use crate::time_keep::TimeKeep;

// a trait for any struct used as main point of a game
pub trait Runnable{ fn run<C: Component + ReturnsBuffer>(self) -> !; }

// the root of a game fully using this engine
pub struct App{
    active_scene: Scene,
    window: Window,
    input_handle: InputHandler,
    time_keep: TimeKeep
}

impl App {
    pub async fn default_new<'b>(default_shader: Cow<'b, str>) -> App{
        let window = Window::new_default_size(default_shader).await;
        App{ 
            active_scene: Scene::empty(), 
            window: window,
            input_handle: InputHandler::new_default(),
            time_keep: TimeKeep::new()
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
    fn run<C: Component + ReturnsBuffer>(mut self) -> !{
        // since self is not borrowed, it will be dropped after this
        // although that shouldn't matter since this method shouldn't return
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
                        _ => {self.input_handle.recieve_window_input(event);}
                    }
                }
                Event::RedrawRequested(window_id) if window_id == self.window.winit_window.id() => {
                    // This is the main update section of the game loop
                    let dt = self.time_keep.get_dt_as_secs();
                    self.active_scene.update();
                    
                    // rendering
                    // get all entities with a render component
                    let renderable_indices = self.active_scene.get_renderable_entities();                    
                    /*let render_index = meta_data.renderable_index; */
                    let mut render_components: Vec<Rc<dyn ReturnsBuffer>> = Vec::new();

                    for index in renderable_indices{
                        let entity = self.active_scene.get_entity_by_index(*index);
                        let meta_data = entity.get_meta_data();
                        let render_spot = meta_data.renderable_index;
                        let renderable_component = entity.get_component_at(render_spot).expect("MetaData contains improper render index");
                        let component_casted = unsafe{ downcast_component::<C>(&renderable_component).expect("Wrong Component Indexed as Entity Sprite") };
                        let component_as_rc = unsafe{ Rc::from_raw(component_casted) };
                        render_components.push(component_as_rc);
                    }

                    match self.window.render_backend.render(render_components) {
                        // everything went properly
                        Ok(_) => {}
                        // Reconfigure the surface if it's lost or outdated
                        Err(wgpu::SurfaceError::Lost | wgpu::SurfaceError::Outdated) => self.window.render_backend.resize(self.window.render_backend.size),
                        // The system is out of memory, we should probably quit
                        Err(wgpu::SurfaceError::OutOfMemory) => *control_flow = event_loop::ControlFlow::Exit,
                        Err(wgpu::SurfaceError::Timeout) => log::warn!("Surface timeout"),
                    }
                    self.time_keep.tick();
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
