use std::fmt::Display;

use enum_map::EnumMap;
use uuid::Uuid;

use crate::mechanics::dice::{Dice, Rng};

use super::{abilities::{Ability, AbilityScoreIntType, AbilityScores, SaveAttributes, SaveIntType}, class::Class, skills::{Skill, SkillAttributes, SkillModifierIntType}};

/// An Entity is an agent within the engine that is able to be unique identified and interacted with. 
//#[derive(Builder)]
pub struct Entity {
    //#[builder(default = "Uuid::now_v7()")]
    uuid: Uuid,
    name: String, 

   // #[builder(default = "self.hp_max.unwrap_or(0)")]
    hp: u16,
    //#[builder(default = "0")]
    hp_max: u16, 
    //#[builder(default = "0")]
    hp_temp: u16,

    //#[builder(default = "1")]
    level: u8,
    //#[builder(default = "30")]
    speed: u8,

    class: Class,

    //#[builder(default = "AbilityScores::from_defaults()")]
    abilities: AbilityScores,
    //#[builder(default = "EnumMap::from_fn(|_| SkillAttributes::Normal)")]
    skills: EnumMap<Skill, SkillAttributes>,
    //#[builder(default = "EnumMap::from_fn(|_| SaveAttributes::Normal)")]
    saves: EnumMap<Ability, SaveAttributes>,
}

impl Entity {

    /// !TODO speed will eventually come from Race 
    pub fn new(name: String, class: Class, abilities: AbilityScores, rng: &mut Rng, speed: u8) -> Self {
        let hp = (class.get_hit_die().roll(rng) + abilities.get_ability_modifier(Ability::Constitution) as u8).into();

        Self {
            uuid: Uuid::now_v7(),
            name, 
            hp, 
            hp_max: hp, 
            hp_temp: 0,
            level: 1, 
            speed, 
            class, 
            abilities, 
            skills: EnumMap::from_fn(|_| SkillAttributes::Normal),
            saves: EnumMap::from_fn(|_| SaveAttributes::Normal)
        }
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

    pub fn get_speed(&self) -> u8 {
        self.speed
    }

    pub fn get_class_name(&self) -> &str {
        self.class.get_name()
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
}

impl Display for Entity {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let uuid_str = self.uuid.as_u128().to_string();
        write!(f, "Entity {} (uuid ...{})", self.name, &uuid_str[uuid_str.len() - 6..])
    }
}

// Hello future Sam: 
// TODO: fix up all these tests. No longer using Builder here because there aren't any optional params anymore
// (speed will come from Race and level will need to wait until we can level up (maybe add a #[cfg(test)] level setter?))
// Once this is all fixed, need to add Race, then ways to serde Class, Race, and Entity. 
// Hope the roadtrip was fun! 

#[cfg(test)]
mod tests {
    use strum::IntoEnumIterator;

    use super::*;

    #[test]
    fn entity_new() {
        let mut rng = Rng::new(0, 0);
        let class = Class::new(String::new(), Dice::D8);

        let name_raw = "David Gilmour";
        let name = String::from(name_raw);
        let entity = Entity::new(name, class, AbilityScores::from_defaults(), &mut rng, 30);

        assert_eq!(entity.name, name_raw);
        assert_eq!(entity.abilities, AbilityScores::from_defaults());
    }

    #[test]
    fn entity_getters() {
        let name_raw = "Rick Wright";
        let name = String::from(name_raw);
        let abilities = AbilityScores::new(20, 19, 18, 17, 16, 15);
        let entity = EntityBuilder::default()
            .name(name)
            .hp(50)
            .hp_max(75)
            .hp_temp(3)
            .level(15)
            .speed(45)
            .abilities(abilities.clone())
            .build().unwrap();

        assert_eq!(entity.get_name(), entity.name);
        assert_eq!(entity.get_uuid(), entity.uuid.as_u128());

        assert_eq!(entity.get_hp(), 50);
        assert_eq!(entity.get_hp_max(), 75);
        assert_eq!(entity.get_hp_temp(), 3);

        assert_eq!(entity.get_ability_score(Ability::Strength), abilities.get_ability_score(Ability::Strength));
        assert_eq!(entity.get_ability_score(Ability::Dexterity), abilities.get_ability_score(Ability::Dexterity));
        assert_eq!(entity.get_ability_score(Ability::Constitution), abilities.get_ability_score(Ability::Constitution));
        assert_eq!(entity.get_ability_score(Ability::Intelligence), abilities.get_ability_score(Ability::Intelligence));
        assert_eq!(entity.get_ability_score(Ability::Wisdom), abilities.get_ability_score(Ability::Wisdom));
        assert_eq!(entity.get_ability_score(Ability::Charisma), abilities.get_ability_score(Ability::Charisma));
       
        assert_eq!(*entity.get_ability_scores(), abilities);

        assert_eq!(entity.get_speed(), 45);
        assert_eq!(entity.get_level(), 15);
    }

    #[test]
    pub fn test_get_skill_score() {
        let entity = EntityBuilder::default()
        .name(String::new())
        .build().unwrap();
        let map = entity.get_skill_scores();

        for (skill, score) in map {
            assert_eq!(entity.get_skill_score(skill), score);
        }
    }

    #[test]
    pub fn skill_scores_default() {
        let entity = EntityBuilder::default()
        .name(String::new())
        .build().unwrap();

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
            let entity = EntityBuilder::default()
                .name(String::new())
                .abilities(AbilityScores::new(i, i, i, i, i, i))
                .build().unwrap();
            
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
            let mut entity = EntityBuilder::default()
                .name(String::new())
                .abilities(AbilityScores::new(i, i, i, i, i, i))
                .build().unwrap();
            
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
            let entity = EntityBuilder::default()
            .name(String::new())
            .level(i)
            .build().unwrap();

            assert_eq!(entity.get_proficiency_bonus(), *expected.get((i - 1) as usize).unwrap());
        }
    }

    #[test]
    pub fn saves_default() {
        let entity = EntityBuilder::default()
        .name(String::new())
        .build().unwrap();

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
            let entity = EntityBuilder::default()
                .name(String::new())
                .abilities(AbilityScores::new(i, i, i, i, i, i))
                .build().unwrap();
            
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
            let mut entity = EntityBuilder::default()
                .name(String::new())
                .abilities(AbilityScores::new(i, i, i, i, i, i))
                .build().unwrap();
            
            for ability in Ability::iter() {
                entity.set_save_attribute(ability, SaveAttributes::Proficient);
            }

            let expected = expected_modifiers[i as usize] + entity.get_proficiency_bonus() as SaveIntType;
            for ability in Ability::iter() {
                assert_eq!(entity.get_save_score(ability), expected);
            }
        }
    }
}
