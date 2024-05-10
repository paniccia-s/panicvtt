use std::collections::HashMap;

use crate::entities::entity::Entity;

/// The token by which to uniquely identify Entities within the engine.
type EntityID = u128;

pub struct Engine {
    entities: HashMap<EntityID, Entity>,    
}

impl Engine {
    pub fn new() -> Self {
        Self {
            entities: HashMap::new(),
        }
    }

    pub fn new_entity(&mut self, name: &str) -> &Entity {
        let entity = Entity::new(String::from(name));
        let uuid = entity.get_uuid();
        self.entities.insert(uuid, entity);

        // We just put this entity in, so this cannot fail 
        self.entities.get(&uuid).expect("")
    }

    pub fn delete_entity(&mut self, uuid: EntityID) -> Result<Entity, ()> {
        match self.entities.remove(&uuid) {
            Some(e) => {
                Ok(e)       
            }, 
            None => {
                Err(())
            }
        }
    }

    pub fn list_entities(&self) -> Vec<&Entity> {
        self.entities.values().collect()
    }

}