//
// The entity component system.
// Entities are structs comprised of a Vec of components and a meta-data component
// Components are structs that have special functionality. They can only be used through 
// an update method right now
//

use std::rc::Rc;

pub mod component;
pub mod component_builder;
pub mod entity;
pub mod meta_data_component;
pub mod updating_component;

pub use component::{
    Component,
    ComponentFlags,
};

pub use entity::{
    Entity
};

pub use meta_data_component::MetaDataComponent;
pub use updating_component::UpdatingComponent;

use crate::render::sprite_component::ReturnsBuffer;

#[derive(Debug, Clone)]
pub enum ComponentType{
    Base(Rc<dyn Component>),
    Render(Rc<dyn ReturnsBuffer>),
    Updating(Rc<dyn UpdatingComponent>)
}

impl ComponentType{
    pub fn to_dyn_component(&self) -> &dyn Component{
        match self{
            ComponentType::Base(data) => data.as_ref(),
            ComponentType::Render(data) => data.as_dyn_component(),
            ComponentType::Updating(data) => data.dyn_update_to_dyn_component()
        } 
    }
}