use crate::{
    ability::{self, Abilities, AbilityScores},
    class::Class,
    dice::Dice,
    equipment::{Armor, Weapon},
    race::Race,
    skill::Skill,
    experience::XP_TABLE,
};
use std::fmt::{self, Formatter};
use serde::{Serialize, Deserialize};

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
    // pub armor: Vec<Armor>,
    pub weapons: Vec<Weapon>,
    pub current_load: u8,
}

impl Character {
    pub fn apply_racial_bonuses(&mut self) {
        match self.race{
            Race::Dragonborn => {
                self.stats.change_ability_value(Abilities::Strength, 2);
                self.stats.change_ability_value(Abilities::Charisma, 1);
            },
            Race::Dwarf => {
                self.stats.change_ability_value(Abilities::Constitution, 2);
            },
            Race::Elf => {
                self.stats.change_ability_value(Abilities::Dexterity, 2);
            },
            Race::Gnome => {
                self.stats.change_ability_value(Abilities::Intelligence, 2);
            },
            Race::HalfElf => {  //half-elves choose where to put their 1 and 1, but for simplicity - I am hardcoding it for now
                self.stats.change_ability_value(Abilities::Charisma, 2);
                self.stats.change_ability_value(Abilities::Dexterity, 1);
                self.stats.change_ability_value(Abilities::Intelligence, 2);
            },
            Race::HalfOrc => {
                self.stats.change_ability_value(Abilities::Strength, 2);
                self.stats.change_ability_value(Abilities::Constitution, 1);
            },
            Race::Halfling => {
                self.stats.change_ability_value(Abilities::Dexterity, 2);
            },
            Race::Human => {
                self.stats.change_ability_value(Abilities::Strength, 1);
                self.stats.change_ability_value(Abilities::Dexterity, 1);
                self.stats.change_ability_value(Abilities::Constitution, 1);
                self.stats.change_ability_value(Abilities::Intelligence, 1);
                self.stats.change_ability_value(Abilities::Wisdom, 1);
                self.stats.change_ability_value(Abilities::Charisma, 1);
            },
            Race::Tiefling => {},
        }
    }

    pub fn calculate_max_hp(&mut self) {
        let base_hp = match &self.class {
            Class::Barbarian => 12,
            Class::Bard | Class::Cleric |Class::Druid | Class::Monk | Class::Rogue | Class::Warlock => 8,
            Class::Fighter | Class::Paladin | Class::Ranger=> 10,
            Class::Sorcerer | Class::Wizard=> 6,
        };

        let modifier = self.stats.get_ability_modifier(Abilities::Constitution);

        self.max_hp = (base_hp + modifier).max(1) as u16
    }

    fn calculate_carying_capacity(&self) -> u16 {
        self.stats.get(Abilities::Strength) as u16 * 15
    }

    pub fn calculate_armor_class(&self) ->i16 {
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

    // get next level xp cap based on current xp
    pub fn show_needed_xp(&self) -> u32{
        XP_TABLE.iter().find(|x| **x > self.current_xp).copied().unwrap()
    }

    pub fn calculate_current_level(&self) -> u8{
        XP_TABLE.iter().position(|x| *x > self.current_xp).unwrap() as u8 - 1
    }

    //get xp needed to level up
    pub fn xp_needed_for_level_up(&self) -> u32{
        self.show_needed_xp() - &self.current_xp
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
            current_load: 0,
        };

        char.apply_racial_bonuses();

        char.calculate_max_hp();
        char.current_hp = char.max_hp;

        char.skills = char.class.get_class_skills();

        char
    }
}

impl fmt::Display for Character {
    fn fmt(&self, f: &mut Formatter) -> fmt::Result {
        writeln!(f, "")?;
        writeln!(f, "  ═══════════════════════════════════════════════════")?;
        writeln!(
            f,
            "    {}  •  {}  •  {}",
            self.name.to_uppercase(),
            format!("{}", self.race).to_uppercase(),
            format!("{}", self.class).to_uppercase()
        )?;
        writeln!(f, "  ═══════════════════════════════════════════════════")?;
        writeln!(f, "")?;

        // Quick stats bar with emoji
        writeln!(
            f,
            "    ❤  {}/{}    🛡 {}    ⚡ {:+}    ⭐ +{}    💪 {}/{}",
            self.current_hp,
            self.max_hp,
            self.calculate_armor_class(),
            self.calculate_initiative(),
            self.proficiency_bonus(),
            self.current_load,
            self.calculate_carying_capacity()
        )?;
        writeln!(
            f,
            "    XP  {}     🦸 {}",
            self.current_xp,
            self.level,
        )?;

        writeln!(f, "")?;

        // Abilities section
        writeln!(f, "{}", self.stats)?;

        // Skills section
        writeln!(f, "  🎯 PROFICIENT SKILLS")?;
        writeln!(f, "  ────────────────────")?;
        if self.skills.is_empty() {
            writeln!(f, "    No proficient skills")?;
        } else {
            for skill in &self.skills {
                let bonus = self.skill_bonus(*skill);
                writeln!(f, "    • {:?} {:+}", skill, bonus)?;
            }
        }
        writeln!(f, "")?;

        // Equipment section
        // writeln!(f, "  ⚔️  EQUIPMENT")?;
        // writeln!(f, "  ──────────────")?;
        // if self.weapons.is_empty() {
        //     writeln!(f, "    No weapons equipped")?;
        // } else {
        //     for weapon in &self.weapons {
        //         writeln!(
        //             f,
        //             "    • {} — {} {} damage, {:.2} gp",
        //             weapon.weapon_type,
        //             weapon.damage,
        //             format!("{}", weapon.damage_type).to_lowercase(),
        //             weapon.price
        //         )?;
        //     }
        // }
        writeln!(f, "")?;

        Ok(())
    }
}
