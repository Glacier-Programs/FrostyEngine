use crate::render::vertex::VertexTrait;
use crate::ecs::{Component, Entity, ComponentFlags};

// a 2-dimensional rectangle. Similiar to rect used in SDL
// also functional as 2d version of transform in 3d engines
#[derive( Copy, Clone, Debug )]
pub struct Rect{
    x: i32,
    y: i32,
    width: i32,
    height: i32
}

impl Rect{
    pub fn new(x: i32, y: i32, width: i32, height: i32) -> Self{
        Self{
            x,
            y,
            width,
            height
        }
    }

    pub fn collide_rect(&self, other_rect: &Rect) -> bool{
        //let x_collision;
        todo!();
    }

    pub fn get_vertices<Vertex: VertexTrait> (&self) -> [Vertex; 6]{
        todo!();
    }
}

impl Component for Rect{
    fn check_required_components(&self, parent: &Entity) { /* No components needed */}
    fn get_flags(&self) -> Vec<ComponentFlags> { vec![ComponentFlags::Unflagged] }
}


// A component that allows a Rect object to render
// This should be basic implementation so that any renderable
// sprite without a Component with ComponentFlags::Renderable in it
// creates a RectRenderComponent which just makes a filled in rectangle
#[derive(core::fmt::Debug)]
pub struct RectRenderComponent{
}

impl Component for RectRenderComponent{
    fn check_required_components(&self, parent: &Entity) {
        // requires a Rect component
        todo!();
    }
    
    fn get_flags(&self) -> Vec<ComponentFlags> {
        vec![ComponentFlags::Renderable]
    }
}