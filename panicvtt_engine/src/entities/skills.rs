use enum_map::Enum; 

use num_derive::FromPrimitive;
use serde::{Deserialize, Serialize};
use strum::{EnumCount, EnumIter};

use super::abilities::Ability;

pub type SkillModifierIntType = i8;

#[repr(u8)]
#[derive(Debug, Enum, EnumIter, PartialEq, Eq, Hash, Clone, Copy, EnumCount, FromPrimitive)]
pub enum Skill {
    Acrobatics      = 0, 
    AnimalHandling  = 1,
    Arcana          = 2,
    Athletics       = 3,
    Deception       = 4,
    History         = 5,
    Insight         = 6, 
    Intimidation    = 7, 
    Investigation   = 8, 
    Medicine        = 9, 
    Nature          = 10,
    Perception      = 11,
    Performance     = 12, 
    Persuasion      = 13, 
    Religion        = 14, 
    SlightOfHand    = 15, 
    Stealth         = 16, 
    Survival        = 17, 
}

impl From<Skill> for usize {
    fn from(val: Skill) -> Self {
        val as usize
    }
}

impl From<usize> for Skill {
    fn from(value: usize) -> Self {
        num::FromPrimitive::from_usize(value).unwrap()
    }
}

pub type SkillIntType = u8;

impl Skill { 
    pub fn get_ability(&self) -> Ability {
        match self {
            Skill::Acrobatics => Ability::Dexterity,
            Skill::AnimalHandling => Ability::Wisdom,
            Skill::Arcana => Ability::Intelligence,
            Skill::Athletics => Ability::Strength,
            Skill::Deception => Ability::Charisma,
            Skill::History => Ability::Intelligence,
            Skill::Insight => Ability::Wisdom,
            Skill::Intimidation => Ability::Charisma,
            Skill::Investigation => Ability::Intelligence,
            Skill::Medicine => Ability::Wisdom,
            Skill::Nature => Ability::Intelligence,
            Skill::Perception => Ability::Wisdom,
            Skill::Performance => Ability::Charisma,
            Skill::Persuasion => Ability::Charisma,
            Skill::Religion => Ability::Intelligence,
            Skill::SlightOfHand => Ability::Dexterity,
            Skill::Stealth => Ability::Dexterity,
            Skill::Survival => Ability::Wisdom,
        }
    }
}


#[derive(Debug, Clone, Copy, EnumIter, Serialize, Deserialize, Eq, PartialEq)]
pub enum SkillAttributes {
    Normal          = 0, 
    HalfProficient  = 1,   
    Proficient      = 2, 
    Expertise       = 3, 
}

impl SkillAttributes {
    pub fn get_proficiency_modifier(&self) -> f64 {
        match self {
            SkillAttributes::Normal => 0f64,
            SkillAttributes::HalfProficient => 0.5f64,
            SkillAttributes::Proficient => 1f64,
            SkillAttributes::Expertise => 2f64,
        }
    }
}
