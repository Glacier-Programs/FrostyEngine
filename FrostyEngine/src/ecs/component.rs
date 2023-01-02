use super::Entity;

// an id able to identify what type of component a component is
// essentially an easy form of reflection
pub struct ComponentId(u32);

// Flags used to help specify the use of a component
pub enum ComponentFlags{
    Input, // Component impls input::InputComponent 
    Renderable, // Component impls render::RenderableComponent<T>
    Unflagged // Component has no special features
}

pub trait Component: core::fmt::Debug{ // debug is required for Vec<Box<dyn Component>>
    // a way for components to depend on other components
    // if no dependancies exist then don't implement any logic
    // otherwise check if a component exists. If it does, then use it
    // if it doesn't, decide what to do
    // an example of it not being used is the Rect component
    // an example of it being used is the sprite component
    fn check_required_components(&self, parent: &Entity);
    
    // a way to check the flags of a component
    fn get_flags(&self) -> Vec<ComponentFlags>;

    // this should not be used by the end user,
    // so some macro for deriving components should be created
    // the id should be unique for each component type
    // ex: all rects should return 3 while all sprites should return 4
    fn id() -> uuid::Uuid where Self: Sized;
    fn get_type_id(&self) -> uuid::Uuid;
} 