use hashbrown::HashMap;

use super::{MetaDataComponent, Component};

// a representation of a thing in a scene
// some basic components will be added as default unless otherwise specified
// when an entity is constructed
#[derive(Debug)]
pub struct Entity<'a>{
    // using dyn allows any struct with the comopnent trait to be accepted in this vec
    meta_data: MetaDataComponent,
    // for discussion
    // is components necessary when a hashmap is able to store
    // all its indices and thus also its contents?
    components: Vec<&'a mut dyn Component>
}

impl <'a> Entity<'a>{
    pub fn new() -> Self{
        Self{
            meta_data: MetaDataComponent{ component_indices: HashMap::new(), updating_component_indice: HashMap::new(), is_renderable: false },
            components: Vec::new()
        }
    }

    pub fn add_component<C: Component + 'a>(&mut self, component: &mut C) -> &mut Self{
        // check the flags and update meta data accordingly
        let flags = component.get_flags();
        if flags.contains(&super::ComponentFlags::Renderable){
            self.meta_data.is_renderable = true;
        }
        // Add the component to list of components 
        self.components.push( &component as (&mut dyn Component) );
        self
    }

    pub fn get_components(&self) -> &Vec<&'a mut dyn Component >{
        &self.components
    }

    pub fn get_component<C: Component>(&self) -> Option<&dyn Component >{
        //let component_id = T.get_type_id();
        let id = C::id();
        let index = self.meta_data.component_indices.get(&id);
        match index{
            None => None,
            Some(i) => Some( self.components[*i] )
        }
    }

    pub(crate) fn get_meta_data(&self) -> &MetaDataComponent{
        &self.meta_data
    }
}