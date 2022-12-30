use hashbrown::HashMap;
use std::rc::Rc;

use crate::ecs::{Component, Entity};

pub struct Scene<'a>{
    entities: Vec<Entity<'a>>
}

impl<'a> Scene<'a>{

    pub fn add_entity(&mut self, entity: Entity<'a>){
        self.entities.push(entity)
    }

    pub fn empty() -> Self{
        // a scene with nothing in it
        Self { 
            entities: Vec::new()
        }
    }

    pub fn dump_all(&mut self) -> &mut Self{
        self.entities = Vec::new();
        self
    }

    // create an entity and return 
    pub fn spawn_entity<'b: 'a>(&mut self){
        let ent = Entity::new();
        self.entities.push(ent);
    }

    // create an entity with certain components
    pub fn spawn_entity_with<'b: 'a, I: Iterator<Item=Box<dyn Component>> >(&mut self, comps: I){
        let mut ent = Entity::new();
        for comp in comps{
            ent.add_component(comp);
        }
        self.entities.push(ent);
    }
}