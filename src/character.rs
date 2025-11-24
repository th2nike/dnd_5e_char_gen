use crate::{
    ability::{Abilities, AbilityScores},
    class::Class,
    equipment::{self, Armor, Weapon},
    experience::XP_TABLE,
    money::{Money, MoneyType},
    race::Race,
    skill::Skill,
};
use serde::{Deserialize, Serialize};
use std::fmt::{self, Formatter};
use strum_macros::{self, Display};

#[derive(Debug, Serialize, Deserialize)]
pub struct Character {
    pub name: String,
    pub race: Race,
    pub class: Class,
    pub level: u8,
    pub current_xp: u32,
    pub stats: AbilityScores,
    pub max_hp: u16,
    pub current_hp: u16,
    pub skills: Vec<Skill>,
    pub armor: Vec<Armor>,
    pub weapons: Vec<Weapon>,
    pub current_load: f32,
    pub current_money: Vec<Money>,
}

#[derive(Debug, Clone, Copy, Display)]
pub enum Encumberance {
    Normal,
    Encumbered,
    HeavilyEncumbered,
    OverCapacity,
}



impl Character {
    pub fn apply_racial_bonuses(&mut self) {
        match self.race {
            Race::Dragonborn => {
                self.stats.change_ability_value(Abilities::Strength, 2);
                self.stats.change_ability_value(Abilities::Charisma, 1);
            }
            Race::Dwarf => {
                self.stats.change_ability_value(Abilities::Constitution, 2);
            }
            Race::Elf => {
                self.stats.change_ability_value(Abilities::Dexterity, 2);
            }
            Race::Gnome => {
                self.stats.change_ability_value(Abilities::Intelligence, 2);
            }
            Race::HalfElf => {
                //half-elves choose where to put their 1 and 1, but for simplicity - I am hardcoding it for now
                self.stats.change_ability_value(Abilities::Charisma, 2);
                self.stats.change_ability_value(Abilities::Dexterity, 1);
                self.stats.change_ability_value(Abilities::Intelligence, 2);
            }
            Race::HalfOrc => {
                self.stats.change_ability_value(Abilities::Strength, 2);
                self.stats.change_ability_value(Abilities::Constitution, 1);
            }
            Race::Halfling => {
                self.stats.change_ability_value(Abilities::Dexterity, 2);
            }
            Race::Human => {
                self.stats.change_ability_value(Abilities::Strength, 1);
                self.stats.change_ability_value(Abilities::Dexterity, 1);
                self.stats.change_ability_value(Abilities::Constitution, 1);
                self.stats.change_ability_value(Abilities::Intelligence, 1);
                self.stats.change_ability_value(Abilities::Wisdom, 1);
                self.stats.change_ability_value(Abilities::Charisma, 1);
            }
            Race::Tiefling => {}
        }
    }

    pub fn calculate_max_hp(&mut self) {
        let base_hp = match &self.class {
            Class::Barbarian => 12,
            Class::Bard
            | Class::Cleric
            | Class::Druid
            | Class::Monk
            | Class::Rogue
            | Class::Warlock => 8,
            Class::Fighter | Class::Paladin | Class::Ranger => 10,
            Class::Sorcerer | Class::Wizard => 6,
        };

        let modifier = self.stats.get_ability_modifier(Abilities::Constitution);

        self.max_hp = (base_hp + modifier).max(1) as u16
    }

    fn calculate_carying_capacity(&self) -> u16 {
        self.stats.get(Abilities::Strength) as u16 * 15
    }

    pub fn calculate_armor_class(&self) -> i16 {
        let base_ac = 10;
        let modifier = self.stats.get_ability_modifier(Abilities::Dexterity) as i16;
        (base_ac + modifier).max(1) as i16
    }

    pub fn calculate_initiative(&self) -> i8 {
        self.stats.get_ability_modifier(Abilities::Dexterity) as i8
    }

    pub fn proficiency_bonus(&self) -> i8 {
        // In 5e, proficiency bonus is +2 at level 1, since we are creating initial chars,
        // We'll hardcode this for now
        2
    }

    pub fn skill_bonus(&self, skill: Skill) -> i8 {
        let ability_mod = skill.get_ability_modifier(self);

        // Check if we're proficient in this skill
        // The contains method checks if the Vec contains a value
        if self.skills.contains(&skill) {
            ability_mod + self.proficiency_bonus()
        } else {
            ability_mod
        }
    }

    #[allow(dead_code)]
    // get next level xp cap based on current xp
    pub fn show_needed_xp(&self) -> u32 {
        XP_TABLE
            .iter()
            .find(|x| **x > self.current_xp)
            .copied()
            .unwrap_or(355_000)
    }

    #[allow(dead_code)]
    pub fn calculate_current_level(&self) -> u8 {
        XP_TABLE
            .iter()
            .position(|x| *x > self.current_xp)
            .map(|pos| pos as u8 - 1)
            .unwrap_or(20)  // Max level is 20
    }

    #[allow(dead_code)]
    //get xp needed to level up
    pub fn xp_needed_for_level_up(&self) -> u32 {
        self.show_needed_xp() - &self.current_xp
    }

    pub fn calculate_weight(&self) -> f32 {
        let mut total: f32 = 0.00;
        total += self.armor.iter().fold(0.0, |acc, x| acc + x.weight);
        total += self.weapons.iter().fold(0.0, |acc, x| acc + x.weight);
        total
    }

    fn set_class_default_armor(&mut self) {
        let armor = match self.class {
            Class::Barbarian => {
                if let Some(armor) = equipment::Armor::get_armor("Studded Leather") {
                    vec![armor]
                } else {
                    vec![]
                }
            }
            Class::Bard => vec![],
            Class::Cleric => vec![],
            Class::Druid => vec![],
            Class::Fighter => vec![],
            Class::Monk => vec![],
            Class::Paladin => vec![],
            Class::Ranger => vec![],
            Class::Rogue => vec![],
            Class::Sorcerer => vec![],
            Class::Warlock => vec![],
            Class::Wizard => vec![],
        };

        self.armor = armor;
    }

    fn set_class_default_weapon(&mut self) {
        let weapons = match self.class {
            Class::Barbarian => {
                if let Some(weapon) = equipment::Weapon::get_weapon("Great Club") {
                    vec![weapon]
                } else {
                    vec![]
                }
            }
            Class::Bard => vec![],
            Class::Cleric => vec![],
            Class::Druid => vec![],
            Class::Fighter => vec![],
            Class::Monk => vec![],
            Class::Paladin => vec![],
            Class::Ranger => vec![],
            Class::Rogue => vec![],
            Class::Sorcerer => vec![],
            Class::Warlock => vec![],
            Class::Wizard => vec![],
        };
        self.weapons = weapons;
    }

    fn set_starter_money(&mut self) {
        //basic one before getting normally done
        self.current_money = vec![
            Money {
                coin_type: MoneyType::Copper,
                amount: 10,
            },
            Money {
                coin_type: MoneyType::Silver,
                amount: 5,
            },
            Money {
                coin_type: MoneyType::Gold,
                amount: 0,
            },
        ];
    }

    //some fun
    pub fn calculate_look_cost(&self) -> f32 {
        let mut total: f32 = 0.00;
        total += self.armor.iter().fold(0.0, |acc, x| acc + x.price);
        total += self.weapons.iter().fold(0.0, |acc, x| acc + x.price);
        total
    }

    fn calculate_encumberance(&self) -> (Encumberance, f32) {
        let current_load: f32 = self.calculate_weight();
        if (current_load > self.stats.get(Abilities::Strength) as f32 * 5.0)
            && (current_load < self.stats.get(Abilities::Strength) as f32 * 10.0)
        {
            (Encumberance::Encumbered, current_load)
        } else if (current_load > self.stats.get(Abilities::Strength) as f32 * 10.0)
            && (current_load < self.stats.get(Abilities::Strength) as f32 * 15.0)
        {
            (Encumberance::HeavilyEncumbered, current_load)
        } else if current_load > self.stats.get(Abilities::Strength) as f32 * 15.0 {
            (Encumberance::OverCapacity, current_load)
        } else {
            (Encumberance::Normal, current_load)
        }
    }

    pub fn new(name: String, race: Race, class: Class) -> Self {
        let mut char = Character {
            name,
            race,
            class,
            level: 1,
            current_xp: 0,
            stats: AbilityScores::new(),
            max_hp: 0,
            current_hp: 0,
            skills: vec![],
            weapons: vec![],
            armor: vec![],
            current_load: 0.0,
            current_money: vec![],
        };

        char.apply_racial_bonuses();

        char.calculate_max_hp();
        char.current_hp = char.max_hp;

        char.skills = char.class.get_class_skills();

        char.set_class_default_armor();
        char.set_class_default_weapon();

        char.set_starter_money();

        char
    }
}

impl fmt::Display for Character {
    fn fmt(&self, f: &mut Formatter) -> fmt::Result {
        writeln!(f, "")?;
        writeln!(f, "═══════════════════════════════════════════════════")?;
        writeln!(
            f,
            "    {}  •  {}  •  {}",
            self.name.to_uppercase(),
            format!("{}", self.race).to_uppercase(),
            format!("{}", self.class).to_uppercase()
        )?;
        writeln!(f, "═══════════════════════════════════════════════════")?;
        writeln!(f, "")?;

        // Quick stats bar with emoji
        writeln!(
            f,
            "❤  {}/{}    🛡 {}    ⚡ {:+}    ⭐ +{}",
            self.current_hp,
            self.max_hp,
            self.calculate_armor_class(),
            self.calculate_initiative(),
            self.proficiency_bonus(),
        )?;
        writeln!(f, "")?;
        writeln!(
            f,
            "XP  {}       🦸 {}    💪 {}/{} lbs {}",
            self.current_xp,
            self.level,
            self.calculate_weight(),
            self.calculate_carying_capacity(),
            self.calculate_encumberance().0,
        )?;

        writeln!(f, "\n═══════════════════════════════════════════════════")?;

        // Abilities section
        writeln!(f, "\n{}", self.stats)?;

        // Skills section
        writeln!(f, "🎯 PROFICIENT SKILLS")?;
        writeln!(f, "────────────────────")?;
        if self.skills.is_empty() {
            writeln!(f, "No proficient skills")?;
        } else {
            for skill in &self.skills {
                let bonus = self.skill_bonus(*skill);
                writeln!(f, " • {:?} {:+}", skill, bonus)?;
            }
        }
        writeln!(f, "═══════════════════════════════════════════════════")?;

        // Equipment section
        writeln!(f, "\n🛠️  EQUIPMENT")?;
        writeln!(f, "────────────────────")?;

        if self.calculate_look_cost() > 0.0 {
            writeln!(f, "You look Gorgeous! 😎")?;
            writeln!(
                f,
                "You have spent {} on your look!",
                self.calculate_look_cost()
            )?;
        } else {
            writeln!(f, "Get a job you clown! 🤡")?;
        }
        writeln!(f, "────────────────────")?;

        if self.armor.is_empty() {
            writeln!(f, "🛡️ No armor equipped")?;
        } else {
            for armor in &self.armor {
                writeln!(f, "🛡️  {} | {}", armor.name, armor.armor_type)?;
            }
        }

        if self.weapons.is_empty() {
            writeln!(f, "⚔️  No weapon equipped")?;
        } else {
            for weapon in &self.weapons {
                writeln!(
                    f,
                    "⚔️  {} | {} | Damage: {} {}",
                    weapon.name, weapon.weapon_type, weapon.damage, weapon.damage_type,
                )?;
            }
        }
        writeln!(f, "═══════════════════════════════════════════════════")?;

        // Equipment section
        writeln!(f, "\n💰  Money")?;
        writeln!(f, "────────────────────")?;
        if self.current_money.is_empty() {
            writeln!(f, "No money you poor bastard!")?;
        } else {
            write!(f, "🪙 ")?;
            for money in &self.current_money {
                write!(f, " {} : {} ", money.coin_type, money.amount,)?;
            }
        }

        writeln!(f, "\n═══════════════════════════════════════════════════")?;
        Ok(())
    }
}
