use uuid;
use std::any::TypeId;

use crate::render::vertex::VertexTrait;
use crate::ecs::{Component, Entity, ComponentFlags, component_builder::ComponentBuilder};

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
    fn id() -> TypeId where Self: Sized { todo!(); }
    fn get_type_id(&self) -> TypeId { todo!(); }
}


// A component that allows a Rect object to render
// This should be basic implementation so that any renderable
// sprite without a Component with ComponentFlags::Renderable in it
// creates a RectRenderComponent which just makes a filled in rectangle
#[derive(core::fmt::Debug)]
pub struct RectRenderComponent{
}

impl RectRenderComponent{
    pub fn new() -> Self{
        Self{}
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

    fn id() -> TypeId{todo!();}
    fn get_type_id(&self) -> TypeId{todo!();}
}

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

}

impl ComponentBuilder for RectRenderComponentBuilder{
    type Output =  RectRenderComponent;
    fn build(&self) -> Self::Output {
        RectRenderComponent{
            
        }
    }
}