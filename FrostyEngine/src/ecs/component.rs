use std::{
    any::Any, 
    cell::RefCell, 
    rc::Rc
};
use super::Entity;

// this functions as a way to reverse the v-tablization of 
// components when stored in entities
pub fn downcast_component<C: Component>(component: Rc<RefCell<(dyn DowncastableComponent)>>){
    let component_clone = component.clone();
    let component_as_any: Box<dyn Any> = Box::new(component_clone);
    match component_as_any.downcast_ref::<C>(){
        None => {},
        Some(_) => {}
    }
}

// an id able to identify what type of component a component is
// essentially an easy form of reflection
pub struct ComponentId(u32);

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
    fn check_required_components(&self, parent: &Entity);
    
    // a way to check the flags of a component type
    fn get_flags(&self) -> Vec<ComponentFlags>;

    // this should not be used by the end user,
    // so some macro for deriving components should be created
    // the id should be unique for each component type
    // ex: all rects should return 3 while all sprites should return 4
    fn id() -> uuid::Uuid where Self: Sized;
    // same as id() but applicable on instances of an object
    fn get_type_id(&self) -> uuid::Uuid;
}

// a trait that can be implemented on a component
// it allows the component to be downcasted 
// which is important for allowing other component
// to interact with it
// ex:
//      Rect impl DowncastableComponent
//      RectSpriteComponent<> can then interact with Rect
//      And access its fields or methods
//      But only as readonly
pub trait DowncastableComponent: Component + Any{}