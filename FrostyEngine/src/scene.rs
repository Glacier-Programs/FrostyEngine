use hashbrown::HashMap;
use std::rc::Rc;

use crate::ecs::{Component, Entity};

pub struct Scene<'a>{
    entities: Vec<Entity<'a>>
}

impl<'a> Scene<'a>{
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
}