use crate::ecs::{
    Component, 
    Entity,
    MetaDataComponent
};

pub struct Scene<'a>{
    entities: Vec<Entity<'a>>,
    renderable_entities: Vec<usize> // stores index of entities
}

impl<'a> Scene<'a>{

    pub fn add_entity(&mut self, entity: Entity<'a>){
        self.entities.push(entity)
    }

    pub fn empty() -> Self{
        // a scene with nothing in it
        Self { 
            entities: Vec::new(),
            renderable_entities: Vec::new()
        }
    }

    pub fn dump_all(&mut self) -> &mut Self{
        self.entities = Vec::new();
        self.renderable_entities = Vec::new();
        self
    }

    // create an entity and return 
    pub fn spawn_entity<'b: 'a>(&mut self){
        let ent = Entity::new();
        self.entities.push(ent);
    }

    // create an entity with certain components
    pub fn spawn_entity_with<'b: 'a, C: Component + 'a, I: Iterator<Item=C> >(&mut self, comps: I){
        let mut ent = Entity::new();
        for comp in comps{
            ent.add_component::<C>(comp);
        }
        self.entities.push(ent);
    }

    //
    // Entity getters
    //

    pub fn get_renderable_entities(&self) -> &Vec<usize>{
        &self.renderable_entities
    }

    // get a vec with all entities in it
    pub fn get_all_entities(&'a self) -> &Vec<Entity<'a>> {
        &self.entities
    }

    pub fn get_all_entities_mut(&'a mut self) -> &mut Vec<Entity<'a>> {
        &mut self.entities
    }

    pub fn get_entity_by_index(&self, index: usize) -> &Entity<'a>{
        &self.entities[index]
    }

    //
    //  The big important part
    //

    pub fn update(&mut self){
        // go through each entity and do whatever is necessary
        // also update list of renderable entities
        for e in &self.entities{
            // can unwrap sicne all entities will have a MetaDataComponent
            let entitity_meta_data = e.get_meta_data();
            // update the component
            // check if it is renderable
        }
    }
}