use std::collections::HashMap;

use crate::{entities::{abilities::{Ability, AbilityScoreIntType, AbilityScores}, class::Class, entity::Entity}, mechanics::dice::{Dice, Rng}};

/// The token by which to uniquely identify Entities within the engine.
type EntityID = u128;

pub struct Engine {
    entities: HashMap<EntityID, Entity>,    
    rng: Rng,
}

impl Engine {
    pub fn new(rng: Rng) -> Self {
        Self {
            entities: HashMap::new(),
            rng
        }
    }

    pub fn new_entity(&mut self, name: &str) -> &Entity { 
        let entity = Entity::new(String::from(name), Class::new(String::from("Class Name"), Dice::D12), AbilityScores::from_defaults(), &mut self.rng, 30);
        let uuid = entity.get_uuid();
        self.entities.insert(uuid, entity);

        // We just put this entity in, so this cannot fail 
        self.entities.get(&uuid).expect("")
    }

    pub fn new_entity_with_abilities(&mut self, name: &str, abilities: AbilityScores) -> &Entity {
        let entity = Entity::new(String::from(name), Class::new(String::from("Class Name"), Dice::D12), abilities, &mut self.rng, 30);
        
        let uuid = entity.get_uuid();
        self.entities.insert(uuid, entity);
        
        // We just put this entity in, so this cannot fail 
        self.entities.get(&uuid).expect("")
    }

    pub fn delete_entity(&mut self, uuid: EntityID) -> Option<Entity> {
        self.entities.remove(&uuid) 
    }

    pub fn list_entities(&self) -> Vec<&Entity> {
        self.entities.values().collect()
    }

    pub fn get_ability_score(&self, uuid: EntityID, ability: Ability) -> Option<AbilityScoreIntType>  {
        Some(self.entities.get(&uuid)?.get_ability_score(ability))
    }

    pub fn get_ability_scores(&self, uuid: EntityID) -> Option<&AbilityScores> {
        Some(self.entities.get(&uuid)?.get_ability_scores())
    }
}
