use std::{collections::HashMap, path::Path};

use crate::{assets::{asset::Asset, asset_manager::AssetManager}, entities::{abilities::{Ability, AbilityScoreIntType, AbilityScores}, entity::Entity}, mechanics::dice::Rng};

/// The token by which to uniquely identify Entities within the engine.
type EntityID = u128;

pub struct Engine {
    entities: HashMap<EntityID, Entity>,    
    asset_manager: AssetManager,
    rng: Rng,
}

impl Engine {
    pub fn new(rng: Rng, asset_root: &Path) -> Self {
        Self {
            entities: HashMap::new(),
            asset_manager: AssetManager::new(asset_root).unwrap(),  // For now, panic if something goes wrong
            rng
        }
    }

    pub fn new_entity(&mut self, name: &str) -> &Entity {
        Self::new_entity_with_abilities(self, name, AbilityScores::from_defaults())
    }

    pub fn new_entity_with_abilities(&mut self, name: &str, abilities: AbilityScores) -> &Entity {
        // Use default class/race 
        let entity = Entity::new(String::from(name), 
            self.asset_manager.get_default_class().get_uuid(), 
            self.asset_manager.get_default_race().get_uuid(), 
            abilities, 
            &self.asset_manager, &mut self.rng);

        // Register this entity 
        let uuid = entity.get_uuid();
        self.entities.insert(uuid, entity);

        self.entities.get(&uuid).unwrap()
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
