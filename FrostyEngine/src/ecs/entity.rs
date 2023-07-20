use hashbrown::HashMap;

use std::{
    rc::Rc,
    cell::RefCell, 
    any::TypeId, process::Output
};

use crate::{ecs::component::downcast_component, error::EcsError, render::{sprite_component::ReturnsBuffer, gpu_package::GPUPackage}};

use super::{MetaDataComponent, Component, ComponentFlags, ComponentType, component_builder::{ComponentBuilder, SpriteComponentBuilder}};

pub type COMPONENTPOINTER = Rc<RefCell<dyn Component>>;

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
    components: Vec<ComponentType>
}

impl Entity{
    pub fn new() -> Self{
        let mut component_indices: HashMap<TypeId, usize> = HashMap::new();
        component_indices.insert(MetaDataComponent::id(), 0usize);
        let meta_data_component =  MetaDataComponent{ 
            component_indices: component_indices, 
            updating_component_index: HashMap::new(), 
            ephemeral_components: Vec::new(),
            renderable_index: 0usize 
        };
        Self{
            components: vec![],
            meta_data: meta_data_component,
        }
    }

    // builder is taken by reference so that the same builder
    // can be used multiple times. 
    // Build component will insantiate the component in the scope of the entity
    // this is done so that the lifetime of a component will always be at most
    // equal to the entity
    pub fn build_component<B: ComponentBuilder>(&mut self, builder: &B) -> &mut Self{
        let built_component: <B as ComponentBuilder>::Output = builder.build();
        // check the components that builder::output depends on
        builder.check_required_components(self);

        for flag in built_component.get_flags(){
            match flag{
                ComponentFlags::Ephemeral(_) => self.meta_data.ephemeral_components.push(self.components.len()),
                _ => {}
            }
        }

        // update meta data
        self.meta_data.component_indices.insert( B::Output::id() , self.meta_data.component_indices.len());
        
        self.components.push(
            ComponentType::Base(
                Rc::new(
                    built_component
                )
            )
        );

        self
    } 

    pub fn build_sprite_component<B: SpriteComponentBuilder>(&mut self, builder: &B, gpu_handles: GPUPackage) -> &mut Self{
        let builder = builder.check_required_components(self);
        let built_component: <B as SpriteComponentBuilder>::Output = builder.build(gpu_handles);

        for flag in built_component.get_flags(){
            match flag{
                ComponentFlags::Ephemeral(_) => self.meta_data.ephemeral_components.push(self.components.len()),
                _ => {}
            }
        }

        // update meta data
        self.meta_data.component_indices.insert( B::Output::id() , self.meta_data.component_indices.len());
        if !self.meta_data.get_renderability(){
            self.meta_data.renderable_index = self.components.len()
        }

        self.components.push(
            ComponentType::Render(
                Rc::new(
                    built_component
                )
            )
        );

        self
    } 

    pub fn get_components(&self) -> &Vec<ComponentType>{ &self.components }

    pub fn get_component<C: Component>(&self) -> Result<&C, EcsError>{
        // get the id of the wanted type
        // find if the location of the type is stored in the entities metadata
        // if it is, return the component
        // otherwise, return None
        let id = C::id();
        match self.meta_data.component_indices.get(&id){
            None => { /* Component does not exist */ Err(EcsError::ComponentDoesNotExist) },
            Some(i) => match self.components.get(*i){
                None => Err(EcsError::ComponentNotAtIndex),
                Some(component) => {
                    if let Ok(downcasted_component) = unsafe { 
                        downcast_component::<C>(component.to_dyn_component()) 
                    }{
                        Ok(downcasted_component)
                    } else{
                        Err(EcsError::DowncastFail)
                    }
                }
            }
        }
       
    }

    pub fn get_component_at(&self, index: usize) -> Option<ComponentType>{
        // get a component at a specific index in self.components
        // should only be used if the location of a specific component
        // can be guarenteed
        if index > self.components.len(){
            None
        } else{
            Some( self.components[index].clone() )
        }
    }
    
    pub(crate) fn get_meta_data(&self) -> &MetaDataComponent{
        // metadata shouldn't be stored in entity.components
        &self.meta_data
    }
}