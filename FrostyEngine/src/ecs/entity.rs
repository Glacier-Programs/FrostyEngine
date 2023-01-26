use hashbrown::HashMap;

use std::{
    rc::Rc,
    cell::RefCell, 
    any::TypeId
};

use super::{MetaDataComponent, Component, ComponentFlags, component_builder::ComponentBuilder, component};

type COMPONENTPOINTER = Rc<RefCell<dyn Component>>;

// a representation of a thing in a scene
// some basic components will be added as default unless otherwise specified
// when an entity is constructed
#[derive(Debug)]
pub struct Entity{
    // using dyn allows any struct with the comopnent trait to be accepted in this vec
    meta_data: MetaDataComponent,
    // for discussion
    // is components necessary when a hashmap is able to store
    // all its indices and thus also its contents?
    components: Vec<COMPONENTPOINTER>
}

impl Entity{
    pub fn new() -> Self{
        let mut component_indices: HashMap<TypeId, usize> = HashMap::new();
        component_indices.insert(MetaDataComponent::id(), 0usize);
        Self{
            meta_data: MetaDataComponent{ component_indices: HashMap::new(), updating_component_indice: HashMap::new(), renderable_index: 0usize },
            components: Vec::new()
        }
    }

    /* 
    pub fn add_component<'a, C: Component>(&'a mut self, component: C) -> &mut Self{
        // check the flags and update meta data accordingly
        let flags = component.get_flags();
        if flags.contains(&super::ComponentFlags::Renderable){
            self.meta_data.is_renderable = true;
        }
        // Add the component to list of components 
        self.components.push(
            Rc::new(
                RefCell::new(
                    component 
                )
            )
        );
        self
    }
    */

    // builder is taken by reference so that the same builder
    // can be used multiple times. 
    // Build component will insantiate the component in the scope of the entity
    // this is done so that the lifetime of a component will always be at most
    // equal to the entity
    pub fn build_component<'a, B: ComponentBuilder>(&mut self, builder: &B) -> &mut Self{
        let built_component = builder.build();
        let flags = built_component.get_flags();
        // check the components that builder::output depends on
        built_component.check_required_components(self);

        // check component flags
        if flags.contains(&ComponentFlags::Renderable){ self.meta_data.renderable_index = self.meta_data.component_indices.len(); }
        if flags.contains(&ComponentFlags::Input){  }

        // update meta data
        self.meta_data.component_indices.insert( B::Output::id() , self.meta_data.component_indices.len());

        self.components.push(
            Rc::new(
                RefCell::new(
                    built_component
                )
            )
        );
        self
    } 

    pub fn get_components(&self) -> &Vec<COMPONENTPOINTER>{
        &self.components
    }

    pub fn get_component<C: Component>(&self) -> Option<COMPONENTPOINTER>{
        // get the id of the wanted type
        // find if the location of the type is stored in the entities metadata
        // if it is, return the component
        // otherwise, return None
        let id = C::id();
        let index = self.meta_data.component_indices.get(&id);
        match index{
            None => None,
            Some(i) => Some( self.components[*i].clone() )
        }
    }

    pub fn get_component_at(&self, index: usize) -> Option<COMPONENTPOINTER>{
        // get a component at a specific index in self.components
        // should only be used if the location of a specific component
        // can be guarenteed
        if index > self.components.len(){
            None
        } else{
            Some(self.components[index].clone())
        }
    }
    
    pub(crate) fn get_meta_data(&self) -> &MetaDataComponent{
        &self.meta_data
    }
}