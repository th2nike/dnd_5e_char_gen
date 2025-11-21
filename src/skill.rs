use crate::{ability::Abilities, character::Character};
use std::fmt::{self, Formatter};
use serde::{Serialize, Deserialize};

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum Skill {
    // Strength
    Athletics,
    // Dexterity
    Acrobatics,
    SleightOfHand,
    Stealth,
    // Intelligence
    Arcana,
    History,
    Investigation,
    Nature,
    Religion,
    // Wisdom
    AnimalHandling,
    Insight,
    Medicine,
    Perception,
    Survival,
    // Charisma
    Deception,
    Intimidation,
    Performance,
    Persuasion,
}

impl Skill {
    pub fn ability_type(&self) -> &str {
        match self {
            Skill::Athletics => "STR",
            Skill::Acrobatics | Skill::SleightOfHand | Skill::Stealth => "DEX",
            Skill::Arcana
            | Skill::History
            | Skill::Investigation
            | Skill::Nature
            | Skill::Religion => "INT",
            Skill::AnimalHandling
            | Skill::Insight
            | Skill::Medicine
            | Skill::Perception
            | Skill::Survival => "WIS",
            Skill::Deception | Skill::Intimidation | Skill::Performance | Skill::Persuasion => {
                "CHA"
            }
        }
    }

    pub fn get_ability_modifier(&self, char: &Character) -> i8 {
        match self {
            Skill::Athletics => char.stats.get_ability_modifier(Abilities::Strength),
            Skill::Acrobatics | Skill::SleightOfHand | Skill::Stealth => {
                char.stats.get_ability_modifier(Abilities::Dexterity)
            }
            Skill::Arcana
            | Skill::History
            | Skill::Investigation
            | Skill::Nature
            | Skill::Religion => char.stats.get_ability_modifier(Abilities::Intelligence),
            Skill::AnimalHandling
            | Skill::Insight
            | Skill::Medicine
            | Skill::Perception
            | Skill::Survival => char.stats.get_ability_modifier(Abilities::Wisdom),
            Skill::Deception | Skill::Intimidation | Skill::Performance | Skill::Persuasion => {
                char.stats.get_ability_modifier(Abilities::Charisma)
            }
        }
    }
}

impl fmt::Display for Skill {
    fn fmt(&self, f: &mut Formatter) -> fmt::Result {
        let skill = match self {
            Skill::Acrobatics => "Acrobatics",
            Skill::AnimalHandling => "Animal Handling",
            Skill::Arcana => "Arcana",
            Skill::Athletics => "Athletics",
            Skill::Deception => "Deception",
            Skill::History => "History",
            Skill::Insight => "Insight",
            Skill::Intimidation => "Intimidation",
            Skill::Medicine => "Medicine",
            Skill::Nature => "Nature",
            Skill::Perception => "Perception",
            Skill::Performance => "Performance",
            Skill::Persuasion => "Persuasion",
            Skill::Religion => "Religion",
            Skill::SleightOfHand => "SleightOfHand",
            Skill::Stealth => "Stealth",
            Skill::Survival => "Survival",
            Skill::Investigation => "Investigation",
        };
        write!(f, "{}", skill)
    }
}
