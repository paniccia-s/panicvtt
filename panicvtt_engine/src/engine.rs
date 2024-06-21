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

    pub fn new_entity(&'e mut self, name: &str) -> &'e Entity {
        Self::new_entity_with_abilities(self, name, AbilityScores::from_defaults())
    }

    pub fn new_entity_with_abilities(&'e mut self, name: &str, abilities: AbilityScores) -> &'e Entity {
        // Use default class/race 
        let entity = Entity::new(String::from(name), 
            self.asset_manager.get_default_class(), 
            self.asset_manager.get_default_race(), 
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
