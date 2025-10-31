use std::fmt::{self, Formatter};
use rand::{Rng, thread_rng};
use strum::IntoEnumIterator;
use strum_macros::EnumIter;

#[derive(Debug)]
struct Stat{
    name: Abilities,
    value: u8,
}

impl fmt::Display for Stat{
    fn fmt(&self, f: &mut Formatter) -> fmt::Result{
        write!(f, "{:5}: {:2} | {:+2}", self.name, self.value, stat_modifier(self.value))
    }
}

#[derive(Debug)]
enum Race {
    Elf,
    Human,
    Orc,
    Dwarf,
}

impl fmt::Display for Race{
    fn fmt(&self,f: &mut Formatter) -> fmt::Result{
        let race = match self {
            Race::Dwarf => "Dwarf",
            Race::Elf => "Elf",
            Race::Human => "Human",
            Race::Orc => "Orc",
        };
        write!(f, "{}", race)
    }
}

#[derive(Debug)]
enum Class{
    Fighter,
    Wizard,
    Sorcerer,
    Thief,
}

impl fmt::Display for Class{
    fn fmt(&self,f: &mut Formatter) -> fmt::Result{
        let class = match self {
            Class::Fighter => "Fighter",
            Class::Sorcerer => "Sorcerer",
            Class::Thief => "Thief",
            Class::Wizard => "Wizard",
        };
        write!(f, "{}", class)
    }
}

#[derive(Debug, Clone, Copy, EnumIter, PartialEq, Eq)]
enum Abilities {
    Strength,
    Dexterity,
    Constitution,
    Intelligence,
    Wisdom,
    Charisma,
}

impl fmt::Display for Abilities{
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result{
        let name = match self{
            Abilities::Strength => "STR",
            Abilities::Dexterity => "DEX",
            Abilities::Constitution => "CON",
            Abilities::Intelligence => "INT",
            Abilities::Wisdom => "WIS",
            Abilities::Charisma => "CHA",
        };
        write!(f, "{}", name)
    }
}

#[derive(Debug)]
struct Character {
    race: Race,
    class: Class,
    stats: Vec<Stat>,
}

impl Character {
    fn new(race: Race, class: Class) -> Self {
        Self {
            race,
            class,
            stats: Abilities::iter().map(|a| Stat {name: a, value: roll_3d6()}).collect(),
        }
    }

    fn get_stat(&self, stat_name: Abilities) -> Option<u8> {
        self.stats.iter().find(|x| x.name == stat_name).map(|x| x.value)
    }

    fn train_stat(&mut self, stat_name: Abilities, amount: u8) {
        let stat: u8 = self.get_stat(stat_name).unwrap_or(0);
        println!("{} {}", stat, amount);

        for stat in self.stats.iter_mut(){
            if (stat.name == stat_name){
                if (stat.value + amount) >= 255 {
                    stat.value = 255
                } else {}
                    stat.value += amount
            }
        }
    }

    fn train_stat_functional(&mut self, stat_name: Abilities, amount: u8) {
        self.stats.iter_mut().filter(|x| x.name == stat_name).for_each(|x| x.value += amount);
    }

    fn apply_racial_bonuses(&mut self) {
        match self.race {
            Race::Dwarf => self.train_stat(Abilities::Constitution, 2),
            Race::Elf => self.train_stat(Abilities::Dexterity, 2),
            Race::Orc => {
                self.train_stat(Abilities::Strength, 2);
                self.train_stat(Abilities::Constitution, 1);
            }
            Race::Human => {
                for abiity in Abilities::iter(){
                    self.train_stat(abiity, 1);
                }
            }
        };
    }

    fn calculate_hp(&self) -> u8 {
        let starting_hp = match self.class{
            Class::Fighter => 10,
            Class::Sorcerer => 6,
            Class::Thief => 8,
            Class::Wizard => 6,
        };

        let modifier = stat_modifier(self.get_stat(Abilities::Constitution).unwrap());
        (starting_hp + modifier) as u8
    }

    fn get_modifiers(&self) -> Vec<i8>{
        self.stats.iter().map(|x| stat_modifier(x.value)).collect()
    }

    fn total_stats(&self) -> u16{
        self.stats.iter().map(|x| x.value as u16).sum()
    }

    fn strongest_stat(&self) -> Option<&Stat> {
        self.stats.iter().max_by_key(|x| x.value)
    }

    fn has_weakness(&self) -> bool{
        self.stats.iter().any(|x| x.value < 8)
    }

    fn high_stats(&self) -> Vec<&Stat>{
        self.stats.iter().filter(|x| x.value > 15).collect()
    }

    fn stat_line(&self) -> String {
        self.stats.iter()
                    .map(|x| format!("{} {} ({})", x.name, x.value, stat_modifier(x.value)))
                    .collect::<Vec<_>>()
                    .join(", ")
    }

    fn average_stat(&self) -> f32{
        self.stats.iter().fold(0.0, |acc, x: &Stat| x.value as f32 + acc) / self.stats.len() as f32
    }

    fn total_modifier_sum(&self) -> i8 {
        self.stats.iter().map(|x| stat_modifier(x.value)).sum()
    }

    fn count_strong_modifiers(&self) -> usize {
        self.stats.iter().filter(|x| stat_modifier(x.value) > 2).count()
    }

    fn total_and_average(&self) -> (u16, f32) {
        let (sum, count) = self.stats
            .iter()
            .fold((0u16, 0usize), |(sum, count), s| (sum + s.value as u16, count + 1));
        (sum, sum as f32 / count as f32)
    }

}

impl fmt::Display for Character{
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result{
        writeln!(f, "================ Character ================")?;
        writeln!(f, "Race: {}", self.race)?;
        writeln!(f, "Class: {}", self.class)?;
        writeln!(f, "-------------------------------------------")?;
        for stat in self.stats.iter(){
            writeln!(f, "{}", stat)?;
        };
        writeln!(f, "===========================================")
    }   
}

fn roll_3d6() -> u8 {
    let mut total: u8 = 0;
    let rnd = &mut thread_rng();

    for _ in 0..3 {
        total += rnd.gen_range(1..=6);
    }
    total
}

fn stat_modifier(stat: u8) -> i8{
    ((stat as i8) - 10) / 2
}

fn main(){
    let mut hero = Character::new(Race::Human, Class::Fighter);
    println!("{}", hero);

    println!("{}", hero.get_stat(Abilities::Charisma).unwrap());
    hero.train_stat(Abilities::Strength, 30);
    println!("{}", hero);
    hero.apply_racial_bonuses();
    println!("{}", hero);
    println!("HP: {}", hero.calculate_hp());
}