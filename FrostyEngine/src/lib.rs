// this allows for easier generics
// ex: accepting only <ComponentBuilder>s which 
//     build ReturnsBuffer Components in 
//    <Entity>.build_sprite_component()
#![feature(associated_type_bounds)]

pub mod color;
pub mod ecs;
pub mod error;
pub(crate) mod resource_manager;
#[macro_use]
pub mod render;
pub mod scene;
pub mod defaults;
pub mod sprite;
pub mod app;
pub mod input;
pub mod draw;
pub mod time_keep;

pub(crate) mod util;