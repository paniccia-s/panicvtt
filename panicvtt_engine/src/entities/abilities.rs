use std::{fmt::Display, str::FromStr};

use enum_map::Enum;
use strum::EnumIter;

use super::skills::SkillModifierIntType;

pub type AbilityScoreIntType = u8;

#[derive(Debug, PartialEq, Clone)]
pub struct AbilityScores {
    strength: AbilityScoreIntType,
    dexterity: AbilityScoreIntType,
    constitution: AbilityScoreIntType,
    intelligence: AbilityScoreIntType,
    wisdom: AbilityScoreIntType,
    charisma: AbilityScoreIntType,
}

impl AbilityScores {
    pub fn new(strength: AbilityScoreIntType, dexterity: AbilityScoreIntType, constitution: AbilityScoreIntType, intelligence: AbilityScoreIntType, wisdom: AbilityScoreIntType, charisma: AbilityScoreIntType) -> Self {
        Self {
            strength, dexterity, constitution, 
            intelligence, wisdom, charisma
        }
    }

    pub fn from_defaults() -> Self {
        Self::new(10, 10, 10, 10, 10, 10)
    }

    pub fn get_ability_score(&self, ability: Ability) -> AbilityScoreIntType {
        match ability {
            Ability::Strength => self.strength,
            Ability::Dexterity => self.dexterity,
            Ability::Constitution => self.constitution,
            Ability::Intelligence => self.intelligence,
            Ability::Wisdom => self.wisdom,
            Ability::Charisma => self.charisma,
        }
    }

    pub fn get_ability_modifier(&self, ability: Ability) -> SkillModifierIntType {
        let score = match ability {
            Ability::Strength => self.strength,
            Ability::Dexterity => self.dexterity,
            Ability::Constitution => self.constitution,
            Ability::Intelligence => self.intelligence,
            Ability::Wisdom => self.wisdom,
            Ability::Charisma => self.charisma,
        };

        // Max ability score possible is 30, so this won't panic
        ((score as f64 - 10f64) / 2f64).floor() as SkillModifierIntType
    }
}

impl Display for AbilityScores {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{{ {}{}, {}{}, {}{}, {}{}, {}{}, {}{} }}", 
            self.strength, Ability::Strength, 
            self.dexterity, Ability::Dexterity, 
            self.constitution, Ability::Constitution, 
            self.intelligence, Ability::Intelligence, 
            self.wisdom, Ability::Wisdom, 
            self.charisma, Ability::Charisma, 
        )
    }
}

#[derive(Clone, Copy, Enum, EnumIter)]
pub enum Ability {
    Strength, 
    Dexterity,
    Constitution,
    Intelligence,
    Wisdom,
    Charisma,
}

impl FromStr for Ability {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "STR" => Ok(Ability::Strength), 
            "DEX" => Ok(Ability::Dexterity), 
            "CON" => Ok(Ability::Constitution), 
            "INT" => Ok(Ability::Intelligence), 
            "WIS" => Ok(Ability::Wisdom), 
            "CHA" => Ok(Ability::Charisma), 
            _ => Err(())
        }
    }
}

impl Display for Ability {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", match self {
            Ability::Strength => "STR",
            Ability::Dexterity => "DEX",
            Ability::Constitution => "CON",
            Ability::Intelligence => "INT",
            Ability::Wisdom => "WIS",
            Ability::Charisma => "CHA",
        })
    }
}

pub type SaveIntType = i8; 

#[derive(Debug, Clone, Copy, EnumIter)]
pub enum SaveAttributes {
    Normal          = 0,
    Proficient      = 1, 
}



#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn ability_scores_new() {
        let scores = AbilityScores::new(1, 2, 3, 4, 5, 6);
        assert_eq!(scores.strength, 1);
        assert_eq!(scores.dexterity, 2);
        assert_eq!(scores.constitution, 3);
        assert_eq!(scores.intelligence, 4);
        assert_eq!(scores.wisdom, 5);
        assert_eq!(scores.charisma, 6);
    }
    
    #[test]
    fn ability_scores_from_default() {
        let scores = AbilityScores::from_defaults();
        assert_eq!(scores.strength, 10);
        assert_eq!(scores.dexterity, 10);
        assert_eq!(scores.constitution, 10);
        assert_eq!(scores.intelligence, 10);
        assert_eq!(scores.wisdom, 10);
        assert_eq!(scores.charisma, 10);
    }
    #[test]
    fn ability_scores_get() {
        let scores = AbilityScores::new(1, 2, 3, 4, 5, 6);

        assert_eq!(scores.get_ability_score(Ability::Strength), scores.strength);
        assert_eq!(scores.get_ability_score(Ability::Dexterity), scores.dexterity);
        assert_eq!(scores.get_ability_score(Ability::Constitution), scores.constitution);
        assert_eq!(scores.get_ability_score(Ability::Intelligence), scores.intelligence);
        assert_eq!(scores.get_ability_score(Ability::Wisdom), scores.wisdom);
        assert_eq!(scores.get_ability_score(Ability::Charisma), scores.charisma);
    }

}
