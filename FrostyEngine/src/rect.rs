use std::{
    any::TypeId,
    rc::Rc
};
use wgpu::util::DeviceExt;

use crate::color::colors::RED;
use crate::render::gpu_package::GPUPackage;
use crate::render::vertex::DefaultVertex;
use crate::render::{
    vertex::VertexTrait,
    sprite_component::ReturnsBuffer
};
use crate::ecs::{
    Component, 
    Entity, 
    ComponentFlags, 
    component_builder::{
        ComponentBuilder,
        SpriteComponentBuilder
    },
    updating_component::{
        UpdatingComponent,
        UpdateData,
        UpdateDataType
    }
};

/*
 *  Components and their builders for Rect Objects
 *  Rects are 2d bounding boxes that allow for easy 
 *  Collision detection
 */

pub const QUAD_VERTEX_ORDER: [u32; 6] = [0, 2, 1, 1, 2, 3];

//
// Components
//

// a 2-dimensional rectangle. Similiar to rect used in SDL
// also functional as 2d version of transform in 3d engines
#[derive( Copy, Clone, Debug )]
pub struct RectComponent{
    x: f32,
    y: f32,
    width: f32,
    height: f32
}

impl RectComponent{
    pub fn new(x: f32, y: f32, width: f32, height: f32) -> Self{
        Self{
            x,
            y,
            width,
            height
        }
    }

    pub fn collide_rect(&self, other_rect: &RectComponent) -> bool{
        //let x_collision;
        todo!();
    }

    pub fn get_vertices<Vertex: VertexTrait> (&self) -> [Vertex; 6]{
        todo!();
    }
}

impl Component for RectComponent{
    fn get_flags(&self) -> Vec<ComponentFlags> { vec![ComponentFlags::Unflagged] }
    fn id() -> TypeId where Self: Sized { TypeId::of::<RectComponent>() }
    fn get_type_id(&self) -> TypeId { TypeId::of::<RectComponent>() }
    fn as_any(&self) -> &dyn std::any::Any { self }
    fn as_dyn_component(&self) -> &dyn Component { self }
}


// A component that allows a Rect object to render
// This should be basic implementation so that any renderable
// sprite without a Component with ComponentFlags::Renderable in it
// creates a RectRenderComponent which just makes a filled in rectangle
#[derive(core::fmt::Debug)]
pub struct RectRenderComponent{
    // a reference to 
    rect_reference: Rc<RectComponent>
}

impl RectRenderComponent{
    pub fn new(rect: Rc<RectComponent>) -> Self{
        Self{
            rect_reference: rect
        }
    }
}

impl Component for RectRenderComponent{
    fn get_flags(&self) -> Vec<ComponentFlags> { vec![ComponentFlags::Renderable] }
    fn id() -> TypeId{ TypeId::of::<RectRenderComponent>() }
    fn get_type_id(&self) -> TypeId{ TypeId::of::<RectRenderComponent>() }
    fn as_any(&self) -> &dyn std::any::Any{ self }
    fn as_dyn_component(&self) -> &dyn Component { self }
}

impl ReturnsBuffer for RectRenderComponent{
    fn get_buffers(&self, device: &wgpu::Device) -> (wgpu::Buffer, wgpu::Buffer) {
        let index_buffer = device.create_buffer_init(
            &wgpu::util::BufferInitDescriptor {
                label: Some("Index Buffer"),
                contents: bytemuck::cast_slice(&QUAD_VERTEX_ORDER[..]),
                usage: wgpu::BufferUsages::INDEX,
            }
        );

        let verts = [
            DefaultVertex{ scene_coords: [self.rect_reference.x, self.rect_reference.y], color: RED.as_f32() },
            DefaultVertex{ scene_coords: [self.rect_reference.x + self.rect_reference.width, self.rect_reference.y], color: RED.as_f32() },
            DefaultVertex{ scene_coords: [self.rect_reference.x, self.rect_reference.y - self.rect_reference.height], color: RED.as_f32() },
            DefaultVertex{ scene_coords: [self.rect_reference.x + self.rect_reference.width, self.rect_reference.y - self.rect_reference.height], color: RED.as_f32() },
        ];

        let vertex_buffer = device.create_buffer_init(
            &wgpu::util::BufferInitDescriptor {
                label: Some("Vertex Buffer"),
                contents: bytemuck::cast_slice(&verts),
                usage: wgpu::BufferUsages::INDEX,
            }
        );

        (vertex_buffer, index_buffer)
    }
    fn get_num_indices(&self) -> u32 { 6u32 }
    fn get_shader(&self) -> String { "default".into() }
    fn returns_buffer_to_dyn_component(&self) -> &dyn Component { self }
}


//
// Builders
//

#[derive(Copy, Clone, Debug)]
pub struct RectBuilder{
    pub x: f32,
    pub y: f32,
    pub width: f32,
    pub height: f32
}

impl ComponentBuilder for RectBuilder{
    type Output =  RectComponent;
    fn build(&self) -> Self::Output {
        RectComponent{
            x: self.x,
            y: self.y,
            width: self.width,
            height: self.height
        }
    }
    fn check_required_components(&self, parent: &mut Entity) -> Self { *self }
}

#[derive(Clone, Debug)]
pub struct RectRenderComponentBuilder{
    // the actual rect may or may not exist when the builder
    // is constructed
    pub rect_reference: Option<Rc<RectComponent>>
}

impl SpriteComponentBuilder for RectRenderComponentBuilder{
    // outputs a PseudoRectRender since builders cannot
    // implement a way to add components to the parent.
    // PseudoRectRender will create a Rect if it does
    // not exist, add a RectRender, and then deconstruct
    type Output =  RectRenderComponent;
    fn build(&self, gpu_handles: GPUPackage) -> Self::Output {
        // rect should exist since check_required_components will build one
        let rect_ref = self.rect_reference.as_ref()
            .expect("RectSpriteComponent Built Without Rect reference");

        todo!()
        /*
        RectRenderComponent{
            // using a match so that the option passed in contains
            // a new reference to the rect rather than still
            // using the same one as the builder
            rect_reference: match self.rect_reference{
                None => None,
                Some(_) => self.rect_reference.clone()
            }
        }
        */
    }
    fn check_required_components(&self, parent: &mut Entity) -> Self {
        // requires a Rect component
        match parent.get_component::<RectComponent>(){
            Ok(_) => { Self{ rect_reference: self.rect_reference.clone() } }
            Err(_) => { // the error doesn't matter since the rect is unfindable
                // create a rect
                let rect_builder = RectBuilder{ x: 0.0, y: 0.0, width: 10.0, height: 10.0};
                parent.build_component(&rect_builder);
                todo!()
            }
        }
    }
}