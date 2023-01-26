use crate::ecs::{Component, Entity};
use super::vertex::VertexTrait;

// A component that can render through a shader
// This component should describe to a shader how to
// Obtain the objects >Vertices< and >Texture<
pub trait RenderableComponent<V>
where V: VertexTrait{
    fn get_vertices(&self) -> Vec<V>;
}

pub trait ReturnsBuffer{
    fn get_buffers(&self, device: &wgpu::Device) -> (wgpu::Buffer, wgpu::Buffer);
    fn get_num_indices(&self) -> u32;
    fn get_shader(&self) -> String;
}