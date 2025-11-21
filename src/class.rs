use crate::{
    dice::Dice,
    equipment::{ArmorType, WeaponType},
    skill::Skill,
};
use serde::{Deserialize, Serialize};
use std::fmt::{self, Formatter};

#[derive(Debug, Serialize, Deserialize)]
pub enum Class {
    Barbarian,
    Bard,
    Cleric,
    Druid,
    Fighter,
    Monk,
    Paladin,
    Ranger,
    Rogue,
    Sorcerer,
    Warlock,
    Wizard,
}

impl Class {
    pub fn get_class_armor_proficiency(&self) -> Vec<ArmorType> {
        match self {
            Class::Barbarian => {
                vec![ArmorType::Light, ArmorType::Medium]
            }
            Class::Bard => {
                vec![ArmorType::Light]
            }
            Class::Cleric => {
                vec![ArmorType::Light, ArmorType::Medium]
            }
            Class::Druid => {
                vec![ArmorType::Light, ArmorType::Medium]
            }
            Class::Fighter => {
                vec![ArmorType::Light, ArmorType::Medium, ArmorType::Heavy]
            }
            Class::Monk => {
                vec![]
            }
            Class::Paladin => {
                vec![ArmorType::Light, ArmorType::Medium, ArmorType::Heavy]
            }
            Class::Ranger => {
                vec![ArmorType::Light, ArmorType::Medium]
            }
            Class::Rogue => {
                vec![ArmorType::Light]
            }
            Class::Sorcerer => {
                vec![]
            }
            Class::Warlock => {
                vec![ArmorType::Light]
            }
            Class::Wizard => {
                vec![]
            }
        }
    }

    pub fn get_weapon_proficieny(&self) -> Vec<WeaponType> {
        match self {
            //for some classes the granularity of weapon proficiency is much detailed, but ...
            Class::Barbarian => {
                vec![WeaponType::Simple, WeaponType::Martial]
            }
            Class::Bard => {
                vec![WeaponType::Simple]
            }
            Class::Cleric => {
                vec![WeaponType::Simple]
            }
            Class::Druid => {
                vec![WeaponType::Simple]
            }
            Class::Fighter => {
                vec![WeaponType::Simple, WeaponType::Martial]
            }
            Class::Monk => {
                vec![WeaponType::Simple]
            }
            Class::Paladin => {
                vec![WeaponType::Simple, WeaponType::Martial]
            }
            Class::Ranger => {
                vec![WeaponType::Simple, WeaponType::Martial]
            }
            Class::Rogue => {
                vec![WeaponType::Simple]
            }
            Class::Sorcerer => {
                vec![WeaponType::Simple]
            }
            Class::Warlock => {
                vec![WeaponType::Simple]
            }
            Class::Wizard => {
                vec![WeaponType::Simple]
            }
        }
    }

    pub fn get_class_skills(&self) -> Vec<Skill> {
        match self {
            //In most cases user is free to choose skills (a couple or more)
            //here for simplicity we are hard coding the values
            Class::Barbarian => vec![Skill::Athletics, Skill::Survival],
            Class::Bard => vec![Skill::Acrobatics, Skill::Performance, Skill::History],
            Class::Cleric => vec![Skill::Medicine, Skill::Religion],
            Class::Druid => vec![Skill::AnimalHandling, Skill::Nature],
            Class::Fighter => vec![Skill::Athletics, Skill::Intimidation],
            Class::Monk => vec![Skill::Religion, Skill::History],
            Class::Paladin => vec![Skill::Intimidation, Skill::Religion],
            Class::Ranger => vec![Skill::Survival, Skill::Perception],
            Class::Rogue => vec![Skill::Stealth, Skill::Deception],
            Class::Sorcerer => vec![Skill::Persuasion, Skill::Intimidation],
            Class::Warlock => vec![Skill::Nature, Skill::Arcana],
            Class::Wizard => vec![Skill::Arcana, Skill::Religion],
        }
    }

    pub fn get_class_hit_dice(&self) -> Dice {
        match self {
            Class::Barbarian => Dice::new(1, 12, 0),
            Class::Bard => Dice::new(1, 8, 0),
            Class::Cleric => Dice::new(1, 8, 0),
            Class::Druid => Dice::new(1, 8, 0),
            Class::Fighter => Dice::new(1, 10, 0),
            Class::Monk => Dice::new(1, 8, 0),
            Class::Paladin => Dice::new(1, 10, 0),
            Class::Ranger => Dice::new(1, 10, 0),
            Class::Rogue => Dice::new(1, 8, 0),
            Class::Sorcerer => Dice::new(1, 6, 0),
            Class::Warlock => Dice::new(1, 8, 0),
            Class::Wizard => Dice::new(1, 6, 0),
        }
    }
}

impl fmt::Display for Class {
    fn fmt(&self, f: &mut Formatter) -> fmt::Result {
        let class = match self {
            Class::Barbarian => "Barbarian",
            Class::Bard => "Bard",
            Class::Cleric => "Cleric",
            Class::Druid => "Druid",
            Class::Fighter => "Fighter",
            Class::Monk => "Monk",
            Class::Paladin => "Paladin",
            Class::Ranger => "Ranger",
            Class::Rogue => "Rogue",
            Class::Sorcerer => "Sorcerer",
            Class::Warlock => "Warlock",
            Class::Wizard => "Wizard",
        };
        write!(f, "{}", class)
    }
}
