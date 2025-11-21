use std::fmt::{self, Formatter};

use crate::{character::Character, class::Class};
use serde::{Serialize, Deserialize};

use std::{io, fs};
use std::collections::HashMap;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Weapon {
    pub name: String,
    pub weapon_type: WeaponType,
    pub weapon_range: WeaponRange, 
    pub damage: String,
    pub damage_type: DamageType,
    pub weight: f32,
    pub price: f32,
    pub properties: Vec<String>,
}

#[derive(Debug, Clone, Copy, Serialize, Deserialize)]
pub enum WeaponType {
    Simple,
    Martial,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum WeaponRange{
    Melee,
    Ranged,
}

impl fmt::Display for WeaponType{
    fn fmt(&self, f: &mut Formatter) -> fmt::Result{
        match self{
            WeaponType::Simple => write!(f, "Simple"),
            WeaponType::Martial => write!(f, "Martial"),
        }
    }
}

#[derive(Debug, Clone, Copy, Serialize, Deserialize)]
pub enum DamageType {
    Piercing,
    Slashing,
    Bludgeoning,
}

#[derive(Debug, Deserialize)]
struct WeaponDatabase{
    weapon: Vec<Weapon>,
}

impl fmt::Display for DamageType {
    fn fmt(&self, f: &mut Formatter) -> fmt::Result {
        let damage_type = match self {
            DamageType::Bludgeoning => "Bludgeoning",
            DamageType::Piercing => "Piercing",
            DamageType::Slashing => "Slashing",
        };
        write!(f, "{}", damage_type)
    }
}

#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct Armor {
    pub name: String,
    pub armor_type: ArmorType,
    pub base_ac: u8,
    pub dex_bonus_max: Option<i8>,
    pub weight: f32,
    pub price: f32,
    pub stealth_disadvantage: bool,
}

#[derive(Debug, Clone, Deserialize, Serialize)]
pub enum ArmorType {
    Light,
    Medium,
    Heavy,
}

impl fmt::Display for ArmorType{
    fn fmt(&self, f: &mut Formatter) -> fmt::Result{
        match self{
            ArmorType::Light => write!(f, "Light"),
            ArmorType::Medium => write!(f, "Medium"),
            ArmorType::Heavy => write!(f, "Heavy"),
        }
    }
}

#[derive(Debug, Deserialize)]
struct ArmorDatabase{
    armor: Vec<Armor>,
}


///IMPL Section
impl Armor{
    pub fn load_armor_database() -> Result<HashMap<String, Armor>, io::Error> {
        let toml_content = fs::read_to_string("data/armor.toml")?;
        
        let database: ArmorDatabase = toml::from_str(&toml_content)
            .map_err(|e| io::Error::new(io::ErrorKind::InvalidData, e))?;

        let armor_map: HashMap<String, Armor> = database
            .armor
            .into_iter()
            .map(|armor| (armor.name.clone(), armor))
            .collect();
        
        Ok(armor_map)
    }

    pub fn get_armor(key: &str) -> Option<Armor> {
        Armor::load_armor_database()
            .ok()?
            .get(key)
            .cloned()
    }
}


impl Weapon{
    pub fn load_weapon_database() -> Result<HashMap<String, Weapon>, io::Error> {
        let toml_content = fs::read_to_string("data/weapon.toml")?;
        
        let database: WeaponDatabase = toml::from_str(&toml_content)
            .map_err(|e| io::Error::new(io::ErrorKind::InvalidData, e))?;

        let weapon_map: HashMap<String, Weapon> = database
            .weapon
            .into_iter()
            .map(|weapon| (weapon.name.clone(), weapon))
            .collect();
        
        Ok(weapon_map)
    }

    pub fn get_weapon(key: &str) -> Option<Weapon> {
        Weapon::load_weapon_database()
            .ok()?
            .get(key)
            .cloned()
    }
}