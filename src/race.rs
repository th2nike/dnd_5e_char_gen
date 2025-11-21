use std::fmt::{self, Formatter};
use serde::{Serialize, Deserialize};

#[derive(Debug, Serialize, Deserialize)]
pub enum Race {
    Human,
    Elf,
    HalfOrc,
    HalfElf,
    Halfling,
    Gnome,
    Dwarf,
    Dragonborn,
    Tiefling,
}

impl fmt::Display for Race {
    fn fmt(&self, f: &mut Formatter) -> fmt::Result {
        let race = match self {
            Race::Dragonborn => "Dragonborn",
            Race::Dwarf => "Dwarf",
            Race::Elf => "Elf",
            Race::Gnome => "Gnome",
            Race::Human => "Human",
            Race::HalfOrc => "Half Orc",
            Race::HalfElf => "Half Elf",
            Race::Halfling => "Halfling",
            Race::Tiefling => "Tiefling",
        };
        write!(f, "{}", race)
    }
}