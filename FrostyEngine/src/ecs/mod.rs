//
// The entity component system.
// Entities are structs comprised of a Vec of components and a meta-data component
// Components are structs that have special functionality. They can only be used through 
// an update method right now
//

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