use serde::{Deserialize, Serialize};
use std::fmt::{self, Formatter};
use std::str::FromStr;

#[derive(Debug, PartialEq, Eq, Clone, Copy, Serialize, Deserialize)]
pub enum Abilities {
    Strength,
    Dexterity,
    Constitution,
    Intelligence,
    Wisdom,
    Charisma,
}

impl fmt::Display for Abilities {
    fn fmt(&self, f: &mut Formatter) -> fmt::Result {
        let ability = match self {
            Abilities::Strength => "STR",
            Abilities::Dexterity => "DEX",
            Abilities::Constitution => "CON",
            Abilities::Intelligence => "INT",
            Abilities::Wisdom => "WIS",
            Abilities::Charisma => "CHA",
        };
        write!(f, "{:3}", ability)
    }
}

impl Abilities {
    pub fn abbr_to_ability(abbr: &str) -> Option<Self> {
        match abbr.to_uppercase().as_str() {
            "STR" => Some(Abilities::Strength),
            "DEX" => Some(Abilities::Dexterity),
            "CON" => Some(Abilities::Constitution),
            "INT" => Some(Abilities::Intelligence),
            "WIS" => Some(Abilities::Wisdom),
            "CHA" => Some(Abilities::Charisma),
            _ => None,
        }
    }
}

impl FromStr for Abilities {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        Abilities::abbr_to_ability(s).ok_or(())
    }
}

use crate::dice::Dice;

#[derive(Debug, Clone, Copy, Serialize, Deserialize)]
pub struct AbilityScores {
    strength: u8,
    dexterity: u8,
    constitution: u8,
    intelligence: u8,
    wisdom: u8,
    charisma: u8,
}

impl AbilityScores {
    pub fn new() -> Self {
        Self {
            strength: Dice::roll_for_ability(),
            dexterity: Dice::roll_for_ability(),
            constitution: Dice::roll_for_ability(),
            intelligence: Dice::roll_for_ability(),
            wisdom: Dice::roll_for_ability(),
            charisma: Dice::roll_for_ability(),
        }
    }

    pub fn get(&self, ability: Abilities) -> u8 {
        match ability {
            Abilities::Strength => self.strength,
            Abilities::Dexterity => self.dexterity,
            Abilities::Constitution => self.constitution,
            Abilities::Intelligence => self.intelligence,
            Abilities::Wisdom => self.wisdom,
            Abilities::Charisma => self.charisma,
        }
    }

    pub fn set(&mut self, ability: Abilities, value: u8) {
        match ability {
            Abilities::Strength => self.strength = value,
            Abilities::Dexterity => self.dexterity = value,
            Abilities::Constitution => self.constitution = value,
            Abilities::Intelligence => self.intelligence = value,
            Abilities::Wisdom => self.wisdom = value,
            Abilities::Charisma => self.charisma = value,
        }
    }

    pub fn get_ability_modifier(&self, ability: Abilities) -> i8 {
        ((self.get(ability) as i8) - 10) / 2
    }

    pub fn change_ability_value(&mut self, ability: Abilities, value: i8) {
        let old_value = self.get(ability) as i8;
        let new_value = old_value.saturating_add(value);
        self.set(ability, new_value as u8);
    }
}

impl fmt::Display for AbilityScores {
    fn fmt(&self, f: &mut Formatter) -> fmt::Result {
        writeln!(f, "🤹 ABILITIES")?;
        writeln!(f, "────────────────────")?;
        writeln!(
            f,
            "💪 STR: {:>3} | ({:+2})",
            self.get(Abilities::Strength),
            self.get_ability_modifier(Abilities::Strength)
        );
        writeln!(
            f,
            "🏃 DEX: {:>3} | ({:+2})",
            self.get(Abilities::Dexterity),
            self.get_ability_modifier(Abilities::Dexterity)
        );
        writeln!(
            f,
            "❤️  CON: {:>3} | ({:+2})",
            self.get(Abilities::Constitution),
            self.get_ability_modifier(Abilities::Constitution)
        );
        writeln!(
            f,
            "🧠 INT: {:>3} | ({:+2})",
            self.get(Abilities::Intelligence),
            self.get_ability_modifier(Abilities::Intelligence)
        );
        writeln!(
            f,
            "🦉 WIS: {:>3} | ({:+2})",
            self.get(Abilities::Wisdom),
            self.get_ability_modifier(Abilities::Wisdom)
        );
        writeln!(
            f,
            "✨ CHA: {:>3} | ({:+2})",
            self.get(Abilities::Charisma),
            self.get_ability_modifier(Abilities::Charisma)
        );
        writeln!(f, "")
    }
}
