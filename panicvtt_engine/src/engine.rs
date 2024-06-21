use std::{collections::HashMap, path::Path};

use crate::{assets::asset_manager::AssetManager, entities::{abilities::{Ability, AbilityScoreIntType, AbilityScores}, class::Class, entity::Entity, race::Race}, mechanics::dice::{Dice, Rng}};

/// The token by which to uniquely identify Entities within the engine.
type EntityID = u128;

pub struct Engine<'e> {
    entities: HashMap<EntityID, Entity<'e>>,    
    asset_manager: AssetManager,
    rng: Rng,
}

impl<'e> Engine<'e> {
    pub fn new(rng: Rng, asset_root: &Path) -> Self {
        Self {
            entities: HashMap::new(),
            asset_manager: AssetManager::new(asset_root).unwrap(),  // For now, panic if something goes wrong
            rng
        }
    }

    // pub fn new_entity(&'e mut self, name: &str) -> &Entity {  
    //     let entity = Entity::new(String::from(name), 
    //         Class::new(String::from("Class Name"), Dice::D12), 
    //         Race::new(String::from("Race Name"), 30), 
    //         AbilityScores::from_defaults(), 
    //         &self.asset_manager, 
    //         &mut self.rng);
        
    //     let uuid = entity.get_uuid();
    //     self.entities.insert(uuid, entity);

    //     // We just put this entity in, so this cannot fail 
    //     self.entities.get(&uuid).unwrap()
    // }

    // pub fn new_entity_with_abilities(&'e mut self, name: &str, abilities: AbilityScores) -> &Entity {
    //     let entity = Entity::new(String::from(name), 
    //         Class::new(String::from("Class Name"), Dice::D12),
    //         Race::new(String::from("Race Name"), 30), 
    //         abilities, 
    //         &self.asset_manager,
    //         &mut self.rng);
        
    //     let uuid = entity.get_uuid();
    //     self.entities.insert(uuid, entity);
        
    //     // We just put this entity in, so this cannot fail 
    //     self.entities.get(&uuid).expect("")
    // }

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
