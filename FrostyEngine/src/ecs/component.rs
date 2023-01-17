use std::{
    cell::RefCell, 
    rc::Rc,
    any::TypeId
};
use super::Entity;

// this functions as a way to reverse the v-tablization of 
// components when stored in entities
pub unsafe fn downcast_component<C: Component>(component: &Rc<RefCell<(dyn Component)>>) -> &mut C{
    // This is based on the information from this question:
    // https://stackoverflow.com/questions/33687447/how-to-get-a-reference-to-a-concrete-type-from-a-trait-object
    // Any cannot be used since it only applies to static lifetimes which components inherently aren't
    // This function should only be applied to (dyn DowncastableComponent)'s with a known true type
    // otherwise there will be ub
    // Also, the component returned from this function should be a copy of the original, so the 
    // returned component cannot affect the initial, probably
    let component_clone = component.clone();
    // this is the super unsafe logic
    // the vptr is unimportant since the v-table is unneeded once the object is instantiated
    let (new_component, _vptr): (&mut C, *const ()) =  std::mem::transmute::<Rc<RefCell<dyn Component>>, (&mut C, *const ())>(component_clone);
    new_component
}

// Flags used to help specify the use of a component
#[derive(PartialEq, Eq)]
pub enum ComponentFlags{
    // These flags indicate to >Scene< that should be updated each scene
    // Input means that the component needs to take in the InputHandler
    // ex:  A CharacterControllerComponent needs to take in a InputHandler
    //      in order to update the character player
    // SelfUpdated means that the component has some functionality
    // that needs to be udpated each frame
    // ex:  An enemy needs to move each frame following its own logic
    Input, // Component impls input::InputComponent 
    SelfUpdated, // Component impls UpdatingComponent
    // This flag indicates that the component needs to be added
    // to a scenes render list each frame
    Renderable, // Component impls render::RenderableComponent<T>
    // This flag means that the component has no special functionality
    // that the Scene needs to know about
    Unflagged, // Component has no special features
}

pub trait Component: core::fmt::Debug{ // debug is required for Vec<Box<dyn Component>>
    // a way for components to depend on other components
    // if no dependancies exist then don't implement any logic
    // otherwise check if a component exists. If it does, then use it
    // if it doesn't, decide what to do
    // an example of it not being used is the Rect component
    // an example of it being used is the sprite component
    fn check_required_components(&self, parent: &mut Entity);
    
    // a way to check the flags of a component type
    fn get_flags(&self) -> Vec<ComponentFlags>;

    // this should not be used by the end user,
    // so some macro for deriving components should be created
    // the id should be unique for each component type
    // ex: all rects should return 3 while all sprites should return 4
    fn id() -> TypeId where Self: Sized;
    // same as id() but applicable on instances of an object
    fn get_type_id(&self) -> TypeId;
}
