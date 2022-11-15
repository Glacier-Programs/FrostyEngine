use wgpu::Texture;

use crate::ecs::{Component, Entity};
use crate::rect::{Rect};

#[derive(Debug)]
pub struct Sprite{

}

impl Component for Sprite{
    fn check_required_components(&self, parent: &Entity) {
        todo!();
    }
}