use crate::render::vertex::VertexTrait;
use crate::ecs::Component;

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
    fn check_required_components(&self, parent: &crate::ecs::Entity) {}
}