use std::{any::Any, rc::Rc, cell::RefMut};

use crate::ecs::{Component, Entity, component::downcast_component};
use super::vertex::VertexTrait;
use crate::ecs::entity::COMPONENTPOINTER;

// A component that can render through a shader
// This component should describe to a shader how to
// Obtain the objects >Vertices< and >Texture<
pub trait RenderableComponent<V>
where V: VertexTrait{
    fn get_vertices(&self) -> Vec<V>;
}

pub trait ReturnsBuffer: Component {
    fn get_buffers(&self, device: &wgpu::Device) -> (wgpu::Buffer, wgpu::Buffer);
    fn get_num_indices(&self) -> u32;
    fn get_shader(&self) -> String;
}

// Take a dyn Component and convert it to a dyn ReturnsBuffer
// C should be the underlying data type 
pub(crate) unsafe fn comp_to_return_buffer<C: Component + ReturnsBuffer>( component: &C ) -> &dyn ReturnsBuffer{
    component
}