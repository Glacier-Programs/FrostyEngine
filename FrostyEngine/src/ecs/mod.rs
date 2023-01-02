pub mod component;
//pub mod ecs;
pub mod entity;
pub mod meta_data_component;

pub use component::{
    Component,
    ComponentFlags,
    ComponentId
};
/*
pub use ecs::{
    MetaDataComponent,
};
*/

pub use entity::{
    Entity
};

pub use meta_data_component::MetaDataComponent;