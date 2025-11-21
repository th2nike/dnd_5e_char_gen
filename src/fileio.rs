use crate::{character::Character, class::Class, race::Race};
use std::path::Path;
use std::{fs, io};

pub fn ensure_characters_directory() -> Result<(), io::Error> {
    let dir = "characters";
    if !Path::new(dir).exists() {
        fs::create_dir(dir)?;
    }
    Ok(())
}

pub fn generate_filename(race: &Race, class: &Class) -> String {
    use std::time::{SystemTime, UNIX_EPOCH};

    let timestamp = SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .unwrap()
        .as_secs();

    format!("characters/{:?}_{:?}_{}.json", race, class, timestamp)
}

pub fn list_character_files() -> Result<Vec<String>, io::Error> {
    ensure_characters_directory()?;

    let mut files = Vec::new();

    for entry in fs::read_dir("characters")? {
        let entry = entry?;
        let path = entry.path();

        // Only include .json files
        if path.extension().and_then(|s| s.to_str()) == Some("json") {
            if let Some(filename) = path.to_str() {
                files.push(filename.to_string());
            }
        }
    }

    Ok(files)
}

pub fn save_character_auto(character: &Character) -> Result<String, io::Error> {
    ensure_characters_directory()?;

    let filename = generate_filename(&character.race, &character.class);

    let json = serde_json::to_string_pretty(character)
        .map_err(|e| io::Error::new(io::ErrorKind::Other, e))?;

    fs::write(&filename, json)?;

    Ok(filename)
}
