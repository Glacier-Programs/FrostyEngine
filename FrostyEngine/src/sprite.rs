use wgpu::Texture;
use uuid;

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

    fn id() -> uuid::Uuid where Self: Sized { todo!(); }

    fn get_type_id(&self) -> uuid::Uuid { todo!(); }
}