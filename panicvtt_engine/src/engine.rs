use std::collections::HashMap;

use crate::entities::{entity::Entity, entityview::EntityView};



pub struct Engine {
    entities: HashMap<u128, Entity>,    
}

impl Engine {
    pub fn new() -> Self {
        Self {
            entities: HashMap::new(),
        }
    }

    pub fn new_entity(&mut self, name: &str) -> EntityView {
        let entity = Entity::new(String::from(name));
        let view = entity.to_view();
        self.entities.insert(entity.get_uuid(), entity);

        view
    }

    pub fn delete_entity(&mut self, entity: &EntityView) -> Result<EntityView, ()> {
        match self.entities.remove(&entity.get_uuid()) {
            Some(e) => {
                Ok(e.to_view())       
            }, 
            None => {
                Err(())
            }
        }
    }
}