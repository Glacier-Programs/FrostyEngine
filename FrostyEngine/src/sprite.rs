use wgpu::Texture;
use std::any::TypeId;

use crate::ecs::{Component, Entity, ComponentFlags};

#[derive(Debug)]
pub struct Sprite{

}

impl Component for Sprite{
    fn check_required_components(&self, parent: &mut Entity) {
        todo!();
    }
    fn get_flags(&self) -> Vec<ComponentFlags> {
        todo!();
    }

    fn id() -> TypeId where Self: Sized { todo!(); }
    fn get_type_id(&self) -> TypeId { todo!(); }
    fn as_any(&self) -> &dyn std::any::Any{ self }
    fn as_dyn_component(&self) -> &dyn Component { self }
}