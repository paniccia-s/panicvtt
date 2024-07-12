use std::{error::Error, fmt::Display};

use serde::{Deserialize, Serialize};
use uuid::Uuid;

use crate::{assets::{asset::Asset, asset_manager::AssetManager}, mechanics::dice::Rng, util::enum_map::EnumMap};

use super::{abilities::{Ability, AbilityScoreIntType, AbilityScores, SaveAttributes, SaveIntType}, class::Class, race::Race, skills::{Skill, SkillAttributes, SkillModifierIntType}};

/// An Entity is an agent within the engine that is able to be unique identified and interacted with. 
#[derive(Serialize, Deserialize, Debug)]
pub struct Entity {
    #[serde(with = "uuid::serde::simple")]
    uuid: Uuid,
    name: String,

    hp: u16,
    hp_max: u16, 
    hp_temp: u16,

    level: u8,

    class: u128,
    race: u128,

    abilities: AbilityScores,
    skills: EnumMap<Skill, SkillAttributes>,
    saves: EnumMap<Ability, SaveAttributes>,
}

impl Entity {

    pub fn new(name: String, class: u128, race: u128, abilities: AbilityScores, assets: &AssetManager, rng: &mut Rng) -> Self {
        // Start with HP and level at 0, then level up once to not repeat leveling code 
        let mut s = Self {
            uuid: Uuid::now_v7(),
            name,
            hp: 0, 
            hp_max: 0, 
            hp_temp: 0,
            level: 0, 
            race, 
            class, 
            abilities, 
            skills: EnumMap::from_fn(|_| SkillAttributes::Normal),
            saves: EnumMap::from_fn(|_| SaveAttributes::Normal)
        }; 

        // Leveling up the first time cannot fail - if it does, we have an engine problem
        s.level_up(assets, rng).unwrap(); 
        s
    }


    pub fn get_name(&self) -> &str {
        &self.name
    }

    pub fn get_uuid(&self) -> u128 {
        self.uuid.as_u128()
    }

    pub fn get_hp(&self) -> u16 {
        self.hp
    }

    pub fn get_hp_max(&self) -> u16 {
        self.hp_max
    }

    pub fn get_hp_temp(&self) -> u16 {
        self.hp_temp
    }

    pub fn get_level(&self) -> u8 {
        self.level
    }

    pub fn get_class<'e>(&'e self, assets: &'e AssetManager) -> Option<&'e Class> {
        assets.get_class(self.class)
    }

    pub fn get_race<'e>(&'e self, assets: &'e AssetManager) -> Option<&'e Race> {
        assets.get_race(self.race)
    }

    pub fn get_speed(&self, assets: &AssetManager) -> Option<u8> {
        Some(assets.get_race(self.race)?.get_speed())
    }

    pub fn get_class_name<'a>(&'a self, assets: &'a AssetManager) -> Option<&'a str> {
        Some(assets.get_class(self.class)?.get_name())
    }

    pub fn get_race_name<'a>(&'a self, assets: &'a AssetManager) -> Option<&'a str> {
        Some(assets.get_race(self.race)?.get_name())
    }

    pub fn get_ability_score(&self, ability: Ability) -> AbilityScoreIntType {
        self.abilities.get_ability_score(ability)
    }

    pub fn get_ability_scores(&self) -> &AbilityScores {
        &self.abilities
    }

    pub fn get_ability_modifier(&self, ability: Ability) -> SkillModifierIntType {
        self.abilities.get_ability_modifier(ability)
    }

    pub fn get_skill_score(&self, skill: Skill) -> SkillModifierIntType {
        // Skill = ability[skill.ability] + (attribute.offset * proficiency)
        let attr = &self.skills[skill];
        let prof_multiplier = attr.get_proficiency_modifier();
        let prof_offset = (prof_multiplier * (self.get_proficiency_bonus() as f64)).floor() as u8; 
        
        let ability_modifier = self.get_ability_modifier(skill.get_ability());

        ability_modifier.checked_add(prof_offset as i8).unwrap()
    }

    pub fn get_save_score(&self, ability: Ability) -> SaveIntType {
        // Save = ability[skill.ability] + proficiency
        let ab_offset = self.get_ability_modifier(ability);
        let proficiency = self.saves[ability] as SaveIntType;
        let prof_offset = proficiency * self.get_proficiency_bonus() as SaveIntType;

        ab_offset + prof_offset
    }

    pub fn get_skill_scores(&self) -> EnumMap<Skill, SkillModifierIntType> {
        EnumMap::from_fn(|s| self.get_skill_score(s))
    }

    pub fn get_proficiency_bonus(&self) -> u8 {
        ((self.level - 1) / 4) + 2
    }


    pub fn set_skill_attribute(&mut self, skill: Skill, attribute: SkillAttributes) -> SkillAttributes {
        // Change the attribute for this skill and return the old one 
        let old_attribute = self.skills[skill];
        self.skills[skill] = attribute;
        old_attribute
    }

    pub fn set_save_attribute(&mut self, ability: Ability, attribute: SaveAttributes) -> SaveAttributes {
        // Change the attribute for this ability and return the old one 
        let old_attribute = self.saves[ability];
        self.saves[ability] = attribute;
        old_attribute
    }

    pub fn level_up(&mut self, assets: &AssetManager, rng: &mut Rng) -> Result<u16, EntityError> {
        // !TODO this will eventually be much more involved 
        self.level += 1; 

        // HP can reduce! Mind the signed bounds here 
        let Some(class) = assets.get_class(self.class) else { return Err(EntityError::new()); };
        let roll = class.get_hit_die().roll(rng) as u16;
        let con = self.abilities.get_ability_modifier(Ability::Constitution); 

        self.hp += roll; 
        self.hp = self.hp.saturating_add_signed(con as i16);

        self.hp_max = self.hp;

        Ok(roll)
    }
}

impl Asset for Entity {
    fn get_uuid(&self) -> u128 {
        self.get_uuid()
    }
    
    fn get_owning_campaign(&self) -> Option<u128> {
        todo!()
    }
}

impl Display for Entity {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let uuid_str = self.uuid.as_u128().to_string();
        write!(f, "Entity {} (uuid ...{})", self.name, &uuid_str[uuid_str.len() - 6..])
    }
}

#[derive(Debug)]
pub struct EntityError {}
impl EntityError {
    pub fn new() -> Self {
        EntityError {}
    }
}

impl Default for EntityError {
    fn default() -> Self {
        Self::new()
    }
}
impl Error for EntityError {
    fn source(&self) -> Option<&(dyn Error + 'static)> {
        None
    } 
}

impl Display for EntityError {
    fn fmt(&self, _f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use rand::rngs::mock::StepRng;
    use strum::IntoEnumIterator;

    use crate::{assets::asset::Asset, mechanics::dice::Dice};

    use super::*;

    #[test]
    fn entity_new() {
        let mut rng = Rng::new(0, 0);

        let name_raw = "David Gilmour";
        let name = String::from(name_raw);
        let assets = AssetManager::from_test_config();
        let entity = Entity::new(name, assets.get_testing_class().get_uuid(), assets.get_testing_race().get_uuid(), 
            AbilityScores::from_defaults(), &assets, &mut rng);

        assert_eq!(entity.name, name_raw);
        assert_eq!(entity.abilities, AbilityScores::from_defaults());
    }

    #[test]
    fn entity_getters() {
        let name_raw = "Rick Wright";
        let name = String::from(name_raw);
        let abilities = AbilityScores::new(20, 19, 18, 17, 16, 15);
        let assets = AssetManager::from_test_config();
        let class = assets.get_testing_class();
        let race = assets.get_testing_race();
        let mut rng = StepRng::new(5, 1);

        let entity = Entity::new(name, class.get_uuid(), race.get_uuid(), abilities.clone(), &assets, &mut rng);
        
        assert_eq!(entity.get_name(), entity.name);
        assert_eq!(entity.get_uuid(), entity.uuid.as_u128());
        assert_eq!(entity.get_class_name(&assets).unwrap(), class.get_name());
        assert_eq!(entity.get_race_name(&assets).unwrap(), race.get_name());

        // rng() = 5 + 1 = 6; con = 4; initial HP = 10
        assert_eq!(entity.get_hp(), 10);
        assert_eq!(entity.get_hp_max(), 10);
        assert_eq!(entity.get_hp_temp(), 0);

        assert_eq!(entity.get_ability_score(Ability::Strength), abilities.get_ability_score(Ability::Strength));
        assert_eq!(entity.get_ability_score(Ability::Dexterity), abilities.get_ability_score(Ability::Dexterity));
        assert_eq!(entity.get_ability_score(Ability::Constitution), abilities.get_ability_score(Ability::Constitution));
        assert_eq!(entity.get_ability_score(Ability::Intelligence), abilities.get_ability_score(Ability::Intelligence));
        assert_eq!(entity.get_ability_score(Ability::Wisdom), abilities.get_ability_score(Ability::Wisdom));
        assert_eq!(entity.get_ability_score(Ability::Charisma), abilities.get_ability_score(Ability::Charisma));
       
        assert_eq!(*entity.get_ability_scores(), abilities);

        assert_eq!(entity.get_speed(&assets).unwrap(), 123);
        assert_eq!(entity.get_level(), 1);

        let s = format!("{}", entity);
        let uuid_str = entity.get_uuid().to_string();
        assert_eq!(s, format!("Entity Rick Wright (uuid ...{})",  &uuid_str[uuid_str.len() - 6..]));
    }

    #[test]
    pub fn test_get_skill_score() {
        let assets = AssetManager::from_test_config();
        let class = assets.get_testing_class();
        let race = assets.get_testing_race();
        let mut rng = StepRng::new(5, 1);
        let entity = Entity::new(String::new(), class.get_uuid(), race.get_uuid(), AbilityScores::from_defaults(), &assets, &mut rng);
        let map = entity.get_skill_scores();

        for (skill, score) in map {
            assert_eq!(entity.get_skill_score(skill), score);
        }
    }

    #[test]
    pub fn skill_scores_default() {
        let assets = AssetManager::from_test_config();
        let class = assets.get_testing_class();
        let race = assets.get_testing_race();
        let mut rng = StepRng::new(5, 1);
        let entity = Entity::new(String::new(), class.get_uuid(), race.get_uuid(), AbilityScores::from_defaults(), &assets, &mut rng);
        
        // Each score should be 0 - no proficiency or skill bonus
        for skill in Skill::iter() {
            assert_eq!(entity.get_skill_score(skill), 0);
        }
    }

    #[test]
    pub fn skill_scores_nondefault() {
        // Test each skill within reasonable range 
        let expected_modifiers = [
            -5, -5, -4, -4, -3, -3, 
            -2, -2, -1, -1,  0,  0, 
             1,  1,  2,  2,  3,  3,
             4,  4,  5,  5,  6,  6, 
             7,  7,  8,  8,  9,  9,
            10,
        ];
        
        for i in 0..31 {
            let assets = AssetManager::from_test_config();
            let class = assets.get_testing_class();
            let race = assets.get_testing_race();
            let mut rng = StepRng::new(5, 1);
            let entity = Entity::new(String::new(), class.get_uuid(), race.get_uuid(),
                AbilityScores::new(i, i, i, i, i, i), &assets, &mut rng);
                        
            for skill in Skill::iter() {
                assert_eq!(entity.get_skill_score(skill), expected_modifiers[i as usize]);
            }
        }
    }

    #[test]
    pub fn skill_scores_nondefault_attributes() {
        let expected_modifiers = [
            -5, -5, -4, -4, -3, -3, 
            -2, -2, -1, -1,  0,  0, 
            1,  1,  2,  2,  3,  3,
            4,  4,  5,  5,  6,  6, 
            7,  7,  8,  8,  9,  9,
            10,
        ];

        for i in 0..31 {
            let assets = AssetManager::from_test_config();
            let class = assets.get_testing_class();
            let race = assets.get_testing_race();
            let mut rng = StepRng::new(5, 1);
            let mut entity = Entity::new(String::new(), class.get_uuid(), race.get_uuid(),
                AbilityScores::new(i, i, i, i, i, i), &assets, &mut rng);
            
            let bonus_normal = 0; 
            let bonus_halfprof = entity.get_proficiency_bonus() / 2;
            let bonus_prof = entity.get_proficiency_bonus();
            let bonus_exp = entity.get_proficiency_bonus() * 2;
            let bonuses = [bonus_normal, bonus_halfprof, bonus_prof, bonus_exp]; 

            // Order: Normal, HalfProficient, Proficient, Expertise 
            for (j, attr) in SkillAttributes::iter().enumerate() {
                for skill in Skill::iter() {
                    entity.set_skill_attribute(skill, attr);

                    let bonus = bonuses.get(j).unwrap();
                    let expected = expected_modifiers[i as usize] + *bonus as i8;
                    assert_eq!(entity.get_skill_score(skill), expected);
                }
            }
        }
    }

    #[test]
    pub fn proficiency_bonus() {
        let expected = [
            2, 2, 2, 2, 
            3, 3, 3, 3, 
            4, 4, 4, 4, 
            5, 5, 5, 5, 
            6, 6, 6, 6, 
        ]; 

        for i in 1..21 {
            let assets = AssetManager::from_test_config();
            let class = assets.get_testing_class();
            let race = assets.get_testing_race();
            let mut rng = StepRng::new(5, 1);
            let mut entity = Entity::new(String::new(), class.get_uuid(), race.get_uuid(), AbilityScores::from_defaults(), &assets, &mut rng);
             
            // Level up i - 1 times 
            for _ in 0..i-1 {
                entity.level_up(&assets, &mut rng).unwrap();
            }
             
            assert_eq!(entity.get_proficiency_bonus(), *expected.get((i - 1) as usize).unwrap());
        }
    }

    #[test]
    pub fn saves_default() {
        let assets = AssetManager::from_test_config();
        let class = assets.get_testing_class();
        let race = assets.get_testing_race();
        let mut rng = StepRng::new(5, 1);
        let entity = Entity::new(String::new(), class.get_uuid(), race.get_uuid(), AbilityScores::from_defaults(), &assets, &mut rng);
         

        // Each score should be 0 - no proficiency bonus
        for ability in Ability::iter() {
            assert_eq!(entity.get_save_score(ability), 0);
        }
    }

    #[test]
    pub fn saves_nondefault() {
        // Test each skill within reasonable range 
        let expected_modifiers = [
            -5, -5, -4, -4, -3, -3, 
            -2, -2, -1, -1,  0,  0, 
             1,  1,  2,  2,  3,  3,
             4,  4,  5,  5,  6,  6, 
             7,  7,  8,  8,  9,  9,
            10,
        ];
        
        for i in 0..31 {
            let assets = AssetManager::from_test_config();
            let class = assets.get_testing_class();
            let race = assets.get_testing_race();
            let mut rng = StepRng::new(5, 1);

            let entity = Entity::new(String::new(), class.get_uuid(), race.get_uuid(),
                AbilityScores::new(i, i, i, i, i, i), &assets, &mut rng);
            
            for ability in Ability::iter() {
                assert_eq!(entity.get_save_score(ability), expected_modifiers[i as usize]);
            }
        }
    }

    #[test]
    pub fn saves_nondefault_attributes() {
        // Test each skill within reasonable range 
        let expected_modifiers = [
            -5, -5, -4, -4, -3, -3, 
            -2, -2, -1, -1,  0,  0, 
             1,  1,  2,  2,  3,  3,
             4,  4,  5,  5,  6,  6, 
             7,  7,  8,  8,  9,  9,
            10,
        ];
        
        for i in 0..31 {
            let assets = AssetManager::from_test_config();
            let class = assets.get_testing_class();
            let race = assets.get_testing_race();
            let mut rng = StepRng::new(5, 1);
            let mut entity = Entity::new(String::new(), class.get_uuid(), race.get_uuid(),
                AbilityScores::new(i, i, i, i, i, i), &assets, &mut rng);
            
            for ability in Ability::iter() {
                entity.set_save_attribute(ability, SaveAttributes::Proficient);
            }

            let expected = expected_modifiers[i as usize] + entity.get_proficiency_bonus() as SaveIntType;
            for ability in Ability::iter() {
                assert_eq!(entity.get_save_score(ability), expected);
            }
        }
    }

    #[test]
    pub fn level_up() { 
        let assets = AssetManager::from_test_config();
        let class = assets.get_testing_class();
        let race = assets.get_testing_race();
        let mut rng = StepRng::new(1, 1);
        let mut entity = Entity::new(String::new(), class.get_uuid(), race.get_uuid(), 
            AbilityScores::new(10, 10, 14, 10, 10, 10), &assets, &mut rng);

        // Check initial condition 
        assert_eq!(entity.get_level(), 1);
        assert_eq!(entity.get_hp(), (entity.abilities.get_ability_modifier(Ability::Constitution) + 1 + 1) as u16); 
        assert_eq!(entity.get_hp_max(), entity.get_hp());
        assert_eq!(entity.get_hp_temp(), 0);

        // Level up several times
        let mut hp = entity.get_hp();
        for i in 2..=20 {
            // Roll should increment per steprng 
            let roll = entity.level_up(&assets, &mut rng).unwrap();
            let die = assets.get_class(entity.class).unwrap().get_hit_die();
            assert_eq!(roll, ((i % die.max()) + 1).into());    // Add 1 for 1-indexing the roll
            
            hp += roll + (entity.abilities.get_ability_modifier(Ability::Constitution) as u16); 
            assert_eq!(entity.get_hp(), hp);
        }
    }

    #[test]
    pub fn serde() {
        let assets = AssetManager::from_test_config();
        let class = assets.get_testing_class();
        let race = assets.get_testing_race();

        let mut rng = StepRng::new(0, 0);
        let abilities = AbilityScores::new(0, 5, 10, 15, 20, 25);
        let entity = Entity::new(String::from("Entity Named Finger:"), class.get_uuid(), race.get_uuid(), abilities, &assets, &mut rng);

        let expected_uuid = entity.uuid;
        let expected_hp = entity.hp;

        // Serialize, then deserialize, and check the data 
        let data = serde_yaml::to_string(&entity).unwrap();
        let de: Entity = serde_yaml::from_str(&data).unwrap();
        
        assert_eq!(de.uuid, expected_uuid);
        assert_eq!(de.name, String::from("Entity Named Finger:"));

        assert_eq!(de.hp, expected_hp);
        assert_eq!(de.hp_max, expected_hp);
        assert_eq!(de.hp_temp, 0);

        assert_eq!(de.level, 1);

        assert_eq!(assets.get_class(de.class).unwrap().get_name(), "Testing Class");
        assert_eq!(assets.get_class(de.class).unwrap().get_hit_die(), Dice::D12);

        assert_eq!(assets.get_race(de.race).unwrap().get_name(), "Testing Race");
        assert_eq!(assets.get_race(de.race).unwrap().get_speed(), 123);

        assert_eq!(de.abilities, AbilityScores::new(0, 5, 10, 15, 20, 25));
        
        assert_eq!(de.skills, EnumMap::from_value(SkillAttributes::Normal));
        assert_eq!(de.saves, EnumMap::from_value(SaveAttributes::Normal));
    }

    #[test]
    pub fn entity_error() {
        let e = EntityError::new();
        let e2: EntityError = Default::default();
        assert!(e.source().is_none());

        let f = format!("{}", e2);
        assert_eq!(f, String::new()); 
    }
}
