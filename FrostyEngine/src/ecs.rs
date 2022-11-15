use core::option::Option;

// an id able to identify what type of component a component is
// essentially an easy form of reflection
pub(crate) struct ComponentId(u32);

pub trait Component: core::fmt::Debug{ // debug is required for Vec<Box<dyn Component>>
    // a way for components to depend on other components
    // if no dependancies exist then don't implement any logic
    // otherwise check if a component exists. If it does, then use it
    // if it doesn't, decide what to do
    // an example of it not being used is the Rect component
    // an example of it being used is the sprite component
    fn check_required_components(&self, parent: &Entity);
    
    // this should not be used by the end user,
    // so some macro for deriving components should be created
    // the id should 
    //fn get_type_id(&self) -> ComponentId;
} 

// a representation of a thing in a scene
// some basic components will be added as default unless otherwise specified
// when an entity is constructed
#[derive(Debug)]
pub struct Entity<'a>{
    // using dyn allows any struct with the comopnent trait to be accepted in this vec
    components: Vec<Box<dyn Component + 'a>>
}

impl <'a> Entity<'a>{
    pub fn new() -> Self{
        Self{
            components: Vec::new()
        }
    }

    pub fn apply_updates(&mut self){
        for comp in &self.components{
            println!("{:?}", comp);
        }
    }

    pub fn add_component(&mut self, component: Box<(dyn Component + 'a)>) -> &mut Self{
        self.components.push(component);
        self
    }

    pub fn get_components(&self) -> &Vec<Box<dyn Component + 'a>>{
        &self.components
    }

    pub fn get_component<T: Component>(&self) -> Option<T>{
        for comp in &self.components{

        }
        todo!();
    }
}