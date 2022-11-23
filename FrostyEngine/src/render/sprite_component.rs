use crate::ecs::{Component, Entity};
use super::vertex::VertexTrait;

// A component that can render through a shader
// This component should describe to a shader how to
// Obtain the objects >Vertices< and >Texture<
pub trait RenderableComponent<T>
where T: VertexTrait{
    fn get_vertices(&self) -> Vec<T>;
}