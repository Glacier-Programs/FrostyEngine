use crate::ecs::{
    Component, 
    Entity,
    MetaDataComponent
};

pub struct Scene{
    entities: Vec<Entity>,
    renderable_entities: Vec<usize> // stores index of entities
}

impl Scene{

    pub fn add_entity(&mut self, entity: Entity){
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
    pub fn spawn_entity(&mut self){
        let ent = Entity::new();
        self.entities.push(ent);
    }

    // create an entity with certain components
    pub fn spawn_entity_with<C: Component, I: Iterator<Item=C> >(&mut self, comps: &mut I){
        let mut ent = Entity::new();
        for comp in comps{
            //ent.add_component::<C>(comp);
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
    pub fn get_all_entities(&self) -> &Vec<Entity> {
        &self.entities
    }

    pub fn get_all_entities_mut(&mut self) -> &mut Vec<Entity> {
        &mut self.entities
    }

    pub fn get_entity_by_index(&self, index: usize) -> &Entity{
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