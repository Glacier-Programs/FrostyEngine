use crate::render::{sprite_component::ReturnsBuffer, gpu_package::GPUPackage};

use super::{Component, Entity};

/*
 * For Building Components:
 * 1) Entity takes in a reference to a builder
 * 2) Entity checks the builders required components
 * This splits paths then
 * ====== Option 1 - All requirements met =======
 * 3) Builder creates Component Object
 * 4) Component Object added to list of components
 * ====== Option 2 - Not all requirements are met ======
 * 3) Proper components are built
 * 4) New, updated builder object is created
 * 5) New builder creates Component Object
 * 6) Component Object added to list of components
 * 
 * Builders can alter the state of entities 
 * exclusively by adding components, even if it 
 * is more than just one component
 */
pub trait ComponentBuilder{
    type Output: Component + 'static;
    // this will be used to construct the component
    // it is taken as reference so that the same 
    // builder can be used multiple times
    fn build(&self) -> Self::Output;
    // here is where more components than just Self::Output can be 
    // added to an entity. This method should fix any issues that may
    // arise during build, either by adding needed components (like 
    // in RectRenderComponentBuilder) or by returning a new builder
    // with updated fields
    fn check_required_components(&self, parent: &mut Entity) -> Self;
}

// Same as ComponentBuilder, but creates renderable components
pub trait SpriteComponentBuilder{
    type Output: Component + ReturnsBuffer + 'static;
    fn build(&self, gpu_handles: GPUPackage) -> Self::Output;
    fn check_required_components(&self, parent: &mut Entity) -> Self;
}