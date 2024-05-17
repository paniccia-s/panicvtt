use std::fmt::Display;

use enum_map::{enum_map, EnumMap};
use uuid::Uuid;

use super::{abilities::{Ability, AbilityScoreIntType, AbilityScores}, skills::{Skill, SkillAttributes, SkillModifierIntType}};

/// An Entity is an agent within the engine that is able to be unique identified and interacted with. 
pub struct Entity {
    uuid: Uuid,
    name: String, 
    level: u8,
    abilities: AbilityScores,
    skills: EnumMap<Skill, SkillAttributes>,
}

impl Entity {
    pub fn new(name: String) -> Entity {
        Self::from_ability_scores(name, AbilityScores::from_defaults())
    }
    
    pub fn from_ability_scores(name: String, abilities: AbilityScores) -> Self {
        Self {
            uuid: Uuid::now_v7(),
            name, 
            level: 1,
            abilities, 
            skills: enum_map! {
                Skill::Acrobatics => SkillAttributes::Normal,
                Skill::AnimalHandling => SkillAttributes::Normal,
                Skill::Arcana => SkillAttributes::Normal,
                Skill::Athletics => SkillAttributes::Normal,
                Skill::Deception => SkillAttributes::Normal,
                Skill::History => SkillAttributes::Normal,
                Skill::Insight => SkillAttributes::Normal,
                Skill::Intimidation => SkillAttributes::Normal,
                Skill::Investigation => SkillAttributes::Normal,
                Skill::Medicine => SkillAttributes::Normal,
                Skill::Nature => SkillAttributes::Normal,
                Skill::Perception => SkillAttributes::Normal,
                Skill::Performance => SkillAttributes::Normal,
                Skill::Persuasion => SkillAttributes::Normal,
                Skill::Religion => SkillAttributes::Normal,
                Skill::SlightOfHand => SkillAttributes::Normal,
                Skill::Stealth => SkillAttributes::Normal,
                Skill::Survival => SkillAttributes::Normal,
            }
        }
    }

    // !TODO this function will probably not exist for long 
    pub fn from_level(name: String, level: u8) -> Self {
        let mut s = Self::new(name);
        s.level = level; 
        s
    }

    pub fn get_name(&self) -> &str {
        &self.name
    }

    pub fn get_uuid(&self) -> u128 {
        self.uuid.as_u128()
    }

    pub fn get_level(&self) -> u8 {
        self.level
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

    pub fn get_skill_scores(&self) -> EnumMap<Skill, SkillModifierIntType> {
        EnumMap::from_fn(|s| self.get_skill_score(s))
    }

    // !TODO this will change 
    pub fn get_proficiency_bonus(&self) -> u8 {
        // prof = ((level - 1) / 4) + 2 
        ((self.level - 1) / 4) + 2
    }


    pub fn set_skill_attribute(&mut self, skill: Skill, attribute: SkillAttributes) -> SkillAttributes {
        // Change the attribute for this skill and return the old one 
        let old_attribute = self.skills[skill];
        self.skills[skill] = attribute;
        old_attribute
    }

}

impl Display for Entity {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let uuid_str = self.uuid.as_u128().to_string();
        write!(f, "Entity {} (uuid ...{})", self.name, &uuid_str[uuid_str.len() - 6..])
    }
}


#[cfg(test)]
mod tests {
    use strum::IntoEnumIterator;

    use super::*;

    #[test]
    fn entity_new() {
        let name_raw = "David Gilmour";
        let name = String::from(name_raw);
        let entity = Entity::new(name);

        assert_eq!(entity.name, name_raw);
        assert_eq!(entity.abilities, AbilityScores::from_defaults());
    }

    #[test]
    fn entity_from_ability_scores() {
        let name_raw = "Rick Wright";
        let name = String::from(name_raw);
        let abilities = AbilityScores::new(1, 2, 3, 4, 5, 6);
        let entity = Entity::from_ability_scores(name, abilities.clone());

        assert_eq!(entity.name, name_raw);
        assert_eq!(entity.abilities, abilities);
    }

    #[test]
    fn entity_getters() {
        let name_raw = "Nick Mason";
        let name = String::from(name_raw);
        let abilities = AbilityScores::new(20, 19, 18, 17, 16, 15);
        let entity = Entity::from_ability_scores(name, abilities.clone());

        assert_eq!(entity.get_name(), entity.name);
        assert_eq!(entity.get_uuid(), entity.uuid.as_u128());

        assert_eq!(entity.get_ability_score(Ability::Strength), abilities.get_ability_score(Ability::Strength));
        assert_eq!(entity.get_ability_score(Ability::Dexterity), abilities.get_ability_score(Ability::Dexterity));
        assert_eq!(entity.get_ability_score(Ability::Constitution), abilities.get_ability_score(Ability::Constitution));
        assert_eq!(entity.get_ability_score(Ability::Intelligence), abilities.get_ability_score(Ability::Intelligence));
        assert_eq!(entity.get_ability_score(Ability::Wisdom), abilities.get_ability_score(Ability::Wisdom));
        assert_eq!(entity.get_ability_score(Ability::Charisma), abilities.get_ability_score(Ability::Charisma));
       
        assert_eq!(*entity.get_ability_scores(), abilities);

        let entity = Entity::from_level(String::from(name_raw), 15);
        assert_eq!(entity.get_level(), 15);
    }

    #[test]
    pub fn test_get_skill_score() {
        let entity = Entity::new(String::new());
        let map = entity.get_skill_scores();

        for (skill, score) in map {
            assert_eq!(entity.get_skill_score(skill), score);
        }
    }

    #[test]
    pub fn skill_scores_default() {
        let entity = Entity::new(String::from("John Bonham"));

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
            let entity = Entity::from_ability_scores(String::from("Jimmy Page"), 
                AbilityScores::new(i, i, i, i, i, i)
            );
            
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
            let mut entity = Entity::from_ability_scores(String::from(""), 
                AbilityScores::new(i, i, i, i, i, i)
            );
            
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
            let entity = Entity::from_level(String::new(), i);
            assert_eq!(entity.get_proficiency_bonus(), *expected.get((i - 1) as usize).unwrap());
        }
    }

}
