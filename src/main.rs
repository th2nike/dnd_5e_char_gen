mod ability;
mod character;
mod class;
mod dice;
mod equipment;
mod race;
mod skill;
mod fileio;
mod experience;

use crate::{character::Character, class::Class, race::Race};
use std::{fs, io};


fn main() -> Result<(), io::Error> {
    println!("=== D&D Character Generator ===\n");
    
    // Create a few characters
    let mut wizard = Character::new("Anja".to_string(), Race::Elf, Class::Sorcerer);
    let fighter = Character::new("Thor".to_string(), Race::Dwarf, Class::Fighter);
    let rogue = Character::new("Glog".to_string(), Race::HalfElf, Class::Rogue);
    
    // Save them all
    // println!("Saving characters...");
    // let wizard_file = fileio::save_character_auto(&wizard)?;
    // println!("  Wizard saved to: {}", wizard_file);
    
    // let fighter_file = fileio::save_character_auto(&fighter)?;
    // println!("  Fighter saved to: {}", fighter_file);
    
    // let rogue_file = fileio::save_character_auto(&rogue)?;
    // println!("  Rogue saved to: {}", rogue_file);
    
    // // List all saved characters
    // println!("\nSaved characters:");
    // let files = fileio::list_character_files()?;
    // for file in &files {
    //     println!("  - {}", file);
    // }
    

    println!("{}", wizard);
    println!("{}", fighter);
    println!("{}", rogue);

    Ok(())
}