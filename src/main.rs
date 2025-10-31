use rand::Rng;
use std::fmt;

struct Character {
    strength: u8,
    dexterity: u8,
    constitution: u8,
    intelligence: u8,
    wisdom: u8,
    charisma: u8,
    race: Race,
    class: Class,
    hit_points: u32,
    proficient_skills: Vec<Skill>,
    equipment: Vec<String>,
}

impl Character {
    fn ability_modifier(score: u8) -> i8 {
        ((score as i8) - 10) / 2
    }
    
    fn strength_modifier(&self) -> i8 {
        Self::ability_modifier(self.strength)
    }
    
    fn dexterity_modifier(&self) -> i8 {
        Self::ability_modifier(self.dexterity)
    }
    
    fn constitution_modifier(&self) -> i8 {
        Self::ability_modifier(self.constitution)
    }
    
    fn intelligence_modifier(&self) -> i8 {
        Self::ability_modifier(self.intelligence)
    }
    
    fn wisdom_modifier(&self) -> i8 {
        Self::ability_modifier(self.wisdom)
    }
    
    fn charisma_modifier(&self) -> i8 {
        Self::ability_modifier(self.charisma)
    }

    fn armor_class(&self) -> i8 {
        10 + self.dexterity_modifier()
    }

    fn initiative(&self) -> i8 {
        self.dexterity_modifier()
    }

    fn proficiency_bonus(&self) -> i8 {
        2
    }

    fn skill_bonus(&self, skill: Skill) -> i8 {
        let ability_mod = skill.get_ability_modifier(self);
        if self.proficient_skills.contains(&skill){
            ability_mod + self.proficiency_bonus()
        }
        else {
            ability_mod
        }
    }

    fn display(&self) {
        println!("╔════════════════════════════════════╗");
        println!("║     CHARACTER SHEET                ║");
        println!("╠════════════════════════════════════╣");
        println!("║ Race: {:?}", self.race);
        println!("║ Class: {:?}", self.class);
        println!("║ Hit Points: {}", self.hit_points);
        println!("║ Armor Class: {}", self.armor_class());
        println!("║ Initiative: {:+}", self.initiative());
        println!("╠════════════════════════════════════╣");
        println!("║ ABILITY SCORES                     ║");
        println!("╠════════════════════════════════════╣");
        println!("║ STR: {:2} ({:+2})                     ", self.strength, self.strength_modifier());
        println!("║ DEX: {:2} ({:+2})                     ", self.dexterity, self.dexterity_modifier());
        println!("║ CON: {:2} ({:+2})                     ", self.constitution, self.constitution_modifier());
        println!("║ INT: {:2} ({:+2})                     ", self.intelligence, self.intelligence_modifier());
        println!("║ WIS: {:2} ({:+2})                     ", self.wisdom, self.wisdom_modifier());
        println!("║ CHA: {:2} ({:+2})                     ", self.charisma, self.charisma_modifier());
        println!("╚════════════════════════════════════╝");

        self.display_skills();

        self.display_equipment();
    }

    fn display_skills(&self) {
        println!("\n╔════════════════════════════════════╗");
        println!("║ SKILLS                             ║");
        println!("╠════════════════════════════════════╣");
        
        // We'll iterate through all possible skills
        let all_skills = [
            Skill::Acrobatics, Skill::AnimalHandling, Skill::Arcana,
            Skill::Athletics, Skill::Deception, Skill::History,
            Skill::Insight, Skill::Intimidation, Skill::Investigation,
            Skill::Medicine, Skill::Nature, Skill::Perception,
            Skill::Performance, Skill::Persuasion, Skill::Religion,
            Skill::SleightOfHand, Skill::Stealth, Skill::Survival,
        ];
        
        for skill in all_skills.iter() {
            let bonus = self.skill_bonus(*skill);
            let proficient = if self.proficient_skills.contains(skill) {
                "*"
            } else {
                " "
            };
            println!("║ {}{:?}: {:+}", proficient, skill, bonus);
        }
        
        println!("╠════════════════════════════════════╣");
        println!("║ * = Proficient                     ║");
        println!("╚════════════════════════════════════╝");
    }

        fn display_equipment(&self) {
        println!("\n╔════════════════════════════════════╗");
        println!("║ EQUIPMENT                          ║");
        println!("╠════════════════════════════════════╣");
        
        for item in &self.equipment {
            println!("║ • {}", item);
        }
        
        println!("╚════════════════════════════════════╝");
    }

    fn count_str_skills(&self) -> usize {
        self.proficient_skills.iter().filter(|skill| skill.ability_type()== "STR").count()
    }
}

#[derive(Debug)]
enum Race {
    Human,
    Elf,
    Dwarf,
    Halfling,
    Dragonborn,
    Gnome,
    HalfElf,
    HalfOrc,
    Tiefling,
}

#[derive(Debug, Clone, Copy)]
enum Class {
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

#[derive(Debug)]
enum CharacterError {
    InvalidAbilityScore {score: u8, ability: String},
    InvalidRace,
    InvalidClass,
    MissingRequiredField(String),
}

impl fmt::Display for CharacterError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            CharacterError::InvalidAbilityScore { score, ability } => {
                write!(f, "Invalid {} score: {}. Must be between 3 and 18.", ability, score)
            }
            CharacterError::InvalidRace => {
                write!(f, "Invalid race selection")
            }
            CharacterError::InvalidClass => {
                write!(f, "Invalid class selection")
            }
            CharacterError::MissingRequiredField(field) => {
                write!(f, "Missing required field: {}", field)
            }
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
enum Skill {
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

impl Skill{
    fn ability_type(&self) -> &str {
        match self {
            Skill::Athletics => "STR",
            Skill::Acrobatics | Skill::SleightOfHand | Skill::Stealth => "DEX",
            Skill::Arcana | Skill::History | Skill::Investigation | Skill::Nature | Skill::Religion => "INT",
            Skill::AnimalHandling | Skill::Insight | Skill::Medicine | Skill::Perception | Skill::Survival => "WIS",
            Skill::Deception | Skill::Intimidation | Skill::Performance | Skill::Persuasion => "CHA",
        }
    }

    fn get_ability_modifier(&self, character: &Character) -> i8 {
        match self {
            Skill::Athletics => character.strength_modifier(),
            Skill::Acrobatics | Skill::SleightOfHand | Skill::Stealth => character.dexterity_modifier(),
            Skill::Arcana | Skill::History | Skill::Investigation | Skill::Nature | Skill::Religion => character.intelligence_modifier(),
            Skill::AnimalHandling | Skill::Insight | Skill::Medicine | Skill::Perception | Skill::Survival => character.wisdom_modifier(),
            Skill::Deception | Skill::Intimidation | Skill::Performance | Skill::Persuasion => character.charisma_modifier(),
        }
    }
}

fn roll_3d6() -> u8 {
    let mut rng = rand::thread_rng();
    let mut total = 0;

    for _ in 0..3{
        total += rng.gen_range(1..=6);
    }

    total
}

fn apply_racial_bonuses(character: &mut Character){
    match &character.race{
        Race::Human => {
            character.strength += 1;
            character.dexterity += 1;
            character.constitution += 1;
            character.intelligence += 1;
            character.wisdom += 1;
            character.charisma += 1;
        }
        Race::Elf => {
            character.dexterity += 2;
        }
        Race::Dwarf => {
            character.constitution += 2;
        }
        Race::Halfling => {
            character.dexterity += 2;
        }
        Race::Dragonborn => {
            character.strength += 2;
            character.charisma += 1;
        }
        Race::HalfElf => {
            character.charisma += 2;
            character.dexterity += 1;
            character.constitution += 1;
        }
        Race::HalfOrc => {
            character.constitution += 1;
            character.strength += 2;
        }
        Race::Tiefling => {
            character.charisma += 2;
            character.intelligence += 1;
        }
        Race::Gnome => {
            character.intelligence += 2;
        }
    }
}

fn summarize_character(character: &Character) -> String {
        format!(
            "A {} {} with {} strength and {} dexterity",
            // We can use the Debug trait with {:?} to print enums
            format!("{:?}", character.race).to_lowercase(),
            format!("{:?}", character.class).to_lowercase(),
            character.strength,
            character.dexterity
        )
}

fn calculate_starting_hp(character: &Character) -> u32 {
    let base_hp = match &character.class {
        Class::Barbarian => 12,
        Class::Fighter | Class::Paladin | Class::Ranger => 10,
        Class::Bard | Class::Cleric | Class::Druid | Class::Monk | Class::Rogue | Class::Warlock => 8,
        Class::Sorcerer | Class::Wizard => 6,
    };
    
    let con_mod = character.constitution_modifier();
    (base_hp as i8 + con_mod).max(1) as u32
}

fn create_character (race: Race, class: Class) -> Character{
    let mut character = Character{
        strength: roll_3d6(),
        dexterity: roll_3d6(),
        constitution: roll_3d6(),
        intelligence: roll_3d6(),
        wisdom: roll_3d6(),
        charisma: roll_3d6(),
        race,
        class,
        hit_points: 0,
        proficient_skills: get_class_skills(&class),
        equipment: get_starting_equipment(&class),
    };

    apply_racial_bonuses(&mut character);

    character.hit_points = calculate_starting_hp(&character);

    character
}

fn create_character_with_scores(
    race: Race,
    class: Class,
    strength: u8,
    dexterity: u8,
    constitution: u8,
    intelligence: u8,
    wisdom: u8,
    charisma: u8,
) -> Result<Character, String> {
    validate_ability_score(strength)?;
    validate_ability_score(dexterity)?;
    validate_ability_score(constitution)?;
    validate_ability_score(intelligence)?;
    validate_ability_score(wisdom)?;
    validate_ability_score(charisma)?;

    let mut character = Character {
        strength,
        dexterity,
        constitution,
        intelligence,
        wisdom,
        charisma,
        race,
        class,
        hit_points: 0,
        proficient_skills: get_class_skills(&class),
        equipment: get_starting_equipment(&class),
    };

    apply_racial_bonuses(&mut character);
    character.hit_points = calculate_starting_hp(&character);
    Ok(character)
}

fn create_character_with_scores_improved(
    race: Race,
    class: Class,
    strength: u8,
    dexterity: u8,
    constitution: u8,
    intelligence: u8,
    wisdom: u8,
    charisma: u8,
) -> Result<Character, CharacterError> {
    // Validate each ability score with descriptive names
    validate_ability_score_improved(strength, "Strength")?;
    validate_ability_score_improved(dexterity, "Dexterity")?;
    validate_ability_score_improved(constitution, "Constitution")?;
    validate_ability_score_improved(intelligence, "Intelligence")?;
    validate_ability_score_improved(wisdom, "Wisdom")?;
    validate_ability_score_improved(charisma, "Charisma")?;
    
    let mut character = Character {
        strength,
        dexterity,
        constitution,
        intelligence,
        wisdom,
        charisma,
        race,
        class,
        hit_points: 0,
        proficient_skills: get_class_skills(&class),
        equipment: get_starting_equipment(&class),
    };
    
    apply_racial_bonuses(&mut character);
    character.hit_points = calculate_starting_hp(&character);
    
    Ok(character)
}

fn get_class_skills(class: &Class) -> Vec<Skill> {
    match class {
        Class::Barbarian => vec![Skill::Athletics, Skill::Survival],
        Class::Bard => vec![Skill::Performance, Skill::Persuasion, Skill::Deception],
        Class::Cleric => vec![Skill::Insight, Skill::Religion],
        Class::Druid => vec![Skill::Nature, Skill::Survival],
        Class::Fighter => vec![Skill::Athletics, Skill::Intimidation],
        Class::Monk => vec![Skill::Acrobatics, Skill::Stealth],
        Class::Paladin => vec![Skill::Intimidation, Skill::Religion],
        Class::Ranger => vec![Skill::Survival, Skill::Perception],
        Class::Rogue => vec![Skill::Stealth, Skill::SleightOfHand, Skill::Deception, Skill::Perception],
        Class::Sorcerer => vec![Skill::Persuasion, Skill::Intimidation],
        Class::Warlock => vec![Skill::Deception, Skill::Intimidation],
        Class::Wizard => vec![Skill::Arcana, Skill::History],
    }
}

fn get_starting_equipment(class: &Class) -> Vec<String> {
    match class {
        Class::Barbarian => vec![
            "Greataxe".to_string(),
            "Javelin (4)".to_string(),
            "Explorer's Pack".to_string(),
        ],
        Class::Fighter => vec![
            "Chain Mail".to_string(),
            "Longsword".to_string(),
            "Shield".to_string(),
            "Dungeoneer's Pack".to_string(),
        ],
        Class::Wizard => vec![
            "Quarterstaff".to_string(),
            "Spellbook".to_string(),
            "Component Pouch".to_string(),
            "Scholar's Pack".to_string(),
        ],
        Class::Rogue => vec![
            "Rapier".to_string(),
            "Shortbow".to_string(),
            "Arrows (20)".to_string(),
            "Thieves' Tools".to_string(),
            "Burglar's Pack".to_string(),
        ],
        Class::Cleric => vec![
            "Mace".to_string(),
            "Scale Mail".to_string(),
            "Shield".to_string(),
            "Holy Symbol".to_string(),
            "Priest's Pack".to_string(),
        ],
        // Add a default for other classes
        _ => vec![
            "Simple Weapon".to_string(),
            "Common Clothes".to_string(),
            "Belt Pouch".to_string(),
        ],
    }
}

fn validate_ability_score(score: u8) -> Result<u8, String> {
    if score >= 3 && score <= 18 {
        Ok(score)
    } else {
        Err(format!("Ability score {} is out of range (3-18)", score))
    }
}

fn validate_ability_score_improved(score: u8, ability_name: &str) -> Result<u8, CharacterError> {
    if score >= 3 && score <= 18 {
        Ok(score)
    } else {
        Err(CharacterError::InvalidAbilityScore {
            score,
            ability: ability_name.to_string(),
        })
    }
}

fn main() {
    println!("=== Valid Character ===");
    // This should succeed
    match create_character_with_scores_improved(
        Race::Elf,
        Class::Wizard,
        8, 16, 12, 15, 13, 10  // All valid scores
    ) {
        Ok(character) => {
            println!("Successfully created character!");
            character.display();
        }
        Err(error) => {
            println!("Failed to create character: {}", error);
        }
    }
    
    println!("\n=== Invalid Character ===");
    // This should fail - strength is too low
    match create_character_with_scores_improved(
        Race::Dwarf,
        Class::Fighter,
        2, 14, 16, 10, 12, 8  // 2 is invalid!
    ) {
        Ok(character) => {
            println!("Successfully created character!");
            character.display();
        }
        Err(error) => {
            println!("Failed to create character: {}", error);
        }
    }
}