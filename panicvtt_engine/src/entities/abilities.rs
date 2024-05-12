use std::fmt::Display;

type AbilityScoreIntType = u8;
pub(super) struct AbilityScores {
    strength: AbilityScoreIntType,
    dexterity: AbilityScoreIntType,
    constitution: AbilityScoreIntType,
    intelligence: AbilityScoreIntType,
    wisdom: AbilityScoreIntType,
    charisma: AbilityScoreIntType,
}

impl AbilityScores {
    pub(super) fn new(strength: AbilityScoreIntType, dexterity: AbilityScoreIntType, constitution: AbilityScoreIntType, intelligence: AbilityScoreIntType, wisdom: AbilityScoreIntType, charisma: AbilityScoreIntType) -> Self {
        Self {
            strength, dexterity, constitution, 
            intelligence, wisdom, charisma
        }
    }

    pub(super) fn from_defaults() -> Self {
        Self::new(10, 10, 10, 10, 10, 10)
    }

    pub(super) fn get_ability_score(&self, ability: Ability) -> AbilityScoreIntType {
        return match ability {
            Ability::Strength => self.strength,
            Ability::Dexterity => self.dexterity,
            Ability::Constitution => self.constitution,
            Ability::Intelligence => self.intelligence,
            Ability::Wisdom => self.wisdom,
            Ability::Charisma => self.charisma,
        };
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

pub(super) enum Ability {
    Strength, 
    Dexterity,
    Constitution,
    Intelligence,
    Wisdom,
    Charisma,
}

impl Ability { 
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

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn stats_new() {
        let scores = AbilityScores::new(1, 2, 3, 4, 5, 6);
        assert_eq!(scores.strength, 1);
        assert_eq!(scores.dexterity, 2);
        assert_eq!(scores.constitution, 3);
        assert_eq!(scores.intelligence, 4);
        assert_eq!(scores.wisdom, 5);
        assert_eq!(scores.charisma, 6);
    }
    
    #[test]
    fn stats_from_default() {
        let scores = AbilityScores::from_defaults();
        assert_eq!(scores.strength, 10);
        assert_eq!(scores.dexterity, 10);
        assert_eq!(scores.constitution, 10);
        assert_eq!(scores.intelligence, 10);
        assert_eq!(scores.wisdom, 10);
        assert_eq!(scores.charisma, 10);
    }
}
