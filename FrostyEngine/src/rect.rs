use std::{
    any::TypeId,
    rc::Rc
};

use crate::render::{
    vertex::VertexTrait,
    sprite_component::ReturnsBuffer
};
use crate::ecs::{
    Component, 
    Entity, 
    ComponentFlags, 
    component_builder::ComponentBuilder,
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

//
// Components
//

// a 2-dimensional rectangle. Similiar to rect used in SDL
// also functional as 2d version of transform in 3d engines
#[derive( Copy, Clone, Debug )]
pub struct RectComponent{
    x: i32,
    y: i32,
    width: i32,
    height: i32
}

impl RectComponent{
    pub fn new(x: i32, y: i32, width: i32, height: i32) -> Self{
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
    fn check_required_components(&self, parent: &mut Entity) { /* No components needed */}
    fn get_flags(&self) -> Vec<ComponentFlags> { vec![ComponentFlags::Unflagged] }
    fn id() -> TypeId where Self: Sized { TypeId::of::<RectComponent>() }
    fn get_type_id(&self) -> TypeId { TypeId::of::<RectComponent>() }
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
    fn check_required_components(&self, parent: &mut Entity) {
        // requires a Rect component
        match parent.get_component::<RectComponent>(){
            Some(_) => { /* do nothing if it exists */ }
            None => {
                // create a rect
                let rect_builder = RectBuilder{ x: 0, y: 0, width: 10, height: 10};
                parent.build_component(&rect_builder);
            }
        }
    }
    
    fn get_flags(&self) -> Vec<ComponentFlags> { vec![ComponentFlags::Renderable] }

    fn id() -> TypeId{ TypeId::of::<RectRenderComponent>() }
    fn get_type_id(&self) -> TypeId{ TypeId::of::<RectRenderComponent>() }
}

impl ReturnsBuffer for RectRenderComponent{
    fn get_buffers(&self, device: &wgpu::Device) -> (wgpu::Buffer, wgpu::Buffer) {
        todo!();
    }
    fn get_num_indices(&self) -> u32 { 6u32 }
    fn get_shader(&self) -> String { "default".into() }
}

#[derive(Debug)]
pub struct PseudoRectRenderComponent{
    // this exists so that RectRenderBuilder can construct 
    // a RectRenderComponent without a defined
    // RectComponent which is why it uses the 
    // RectRenderBuilder and not its own
    rect_reference: Option<Rc<RectComponent>>
}

impl Component for PseudoRectRenderComponent{
    fn check_required_components(&self, parent: &mut Entity) { 
        // ensures that RectRender will have a RectComponent
        match parent.get_component::<RectComponent>(){
            Some(rect_ref) => { /*self.rect_reference = Some(rect_ref);*/ }
            None => {
                // create a rect
                let rect_builder = RectBuilder{ x: 0, y: 0, width: 10, height: 10};
                parent.build_component(&rect_builder);
            }
        } 
    }
    fn get_flags(&self) -> Vec<ComponentFlags> { vec![ComponentFlags::Ephemeral(1), ComponentFlags::Renderable] /* should be removed after creating RectRenderComponent */ }
    fn get_type_id(&self) -> TypeId { TypeId::of::<PseudoRectRenderComponent>() }
    fn id() -> TypeId{ TypeId::of::<PseudoRectRenderComponent>() }
}

impl UpdatingComponent for PseudoRectRenderComponent{
    fn update(&mut self, update_data: UpdateData) {
        match &update_data{
            UpdateData::EntityRef(parent) => {
                let rect_component = parent.get_component::<RectComponent>();
                if rect_component.is_none(){  }
            },
            _ => { /* This should never be reached */ panic!("PseudoRectRender recieved unexpected UpdataData") }
        }
    }

    fn get_required_update_data(&self) -> Vec<UpdateDataType> {
        vec![ UpdateDataType::EntityRef ]
    }
}


//
// Builders
//

pub struct RectBuilder{
    pub x: i32,
    pub y: i32,
    pub width: i32,
    pub height: i32
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
}

pub struct RectRenderComponentBuilder{
    // the actual rect may or may not exist when the builder
    // is constructed
    pub rect_reference: Option<Rc<RectComponent>>
}

impl ComponentBuilder for RectRenderComponentBuilder{
    // outputs a PseudoRectRender since builders cannot
    // implement a way to add components to the parent.
    // PseudoRectRender will create a Rect if it does
    // not exist, add a RectRender, and then deconstruct
    type Output =  PseudoRectRenderComponent;
    fn build(&self) -> Self::Output {
        PseudoRectRenderComponent{
            // using a match so that the option passed in contains
            // a new reference to the rect rather than still
            // using the same one as the builder
            rect_reference: match self.rect_reference{
                None => None,
                Some(_) => self.rect_reference.clone()
            }
        }
    }
}