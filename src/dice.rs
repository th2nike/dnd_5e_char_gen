use rand::{Rng, thread_rng};
use regex::Regex;

#[derive(Debug)]
pub struct Dice {
    pub dice_count: u8,
    pub dice_sides: u8,
    pub modifier: i8,
}

impl Dice {
    pub fn new(count: u8, sides: u8, modifier: i8) -> Self {
        Dice {
            dice_count: count,
            dice_sides: sides,
            modifier,
        }
    }

    pub fn roll(&self) -> u8 {
        let mut rnd = thread_rng();
        let mut total: i8 = 0;

        for _ in 0..self.dice_count {
            total += rnd.gen_range(1..=self.dice_sides) as i8;
        }

        total += self.modifier as i8;

        total.max(1) as u8
    }

    pub fn roll_for_ability() -> u8{
        let mut rolls: Vec<u8> = vec![];
        for i in 0..=3 {
            let dice = Dice::new(1,6, 0);
            let dice = dice.roll();
            rolls.push(dice);
        }
        rolls.sort();
        let result = rolls[1] + rolls[2] + rolls[3];
        result
    }

    pub fn parse_dice_roll(dice_roll: String) -> u8 {
        let dice_roll = dice_roll.to_lowercase();
        let mut modifier: i8 = 0;

        let parts: Vec<&str> = dice_roll.split('d').collect();
        let dice_count: u8 = parts[0].parse().unwrap_or(1);

        let right = parts[1];

        let (dice_sides, modifier_str) = if right.contains('+') {
            let sub: Vec<&str> = right.split('+').collect();
            (sub[0], Some(sub[1]))
        } else if right.contains('-') {
            let sub: Vec<&str> = right.split('-').collect();
            modifier = -(sub[1].parse::<i8>().unwrap_or(0));
            (sub[0], None)
        } else {
            (right, None)
        };

        let dice_sides: u8 = dice_sides.parse().unwrap_or(6);

        if let Some(m) = modifier_str {
            modifier = m.parse::<i8>().unwrap_or(0);
        }

        Dice::new(dice_count, dice_sides, modifier).roll()
    }

    //not mine - just wanted to see how it can be done uisng regexes.
    pub fn parse_dice_roll_with_regex(input: &str) {
        let re = Regex::new(r"^(\d*)d(\d+)(?:([+-])(\d+))?$").unwrap();
        if let Some(caps) = re.captures(input) {
            // Extract values and handle potential errors (e.g., empty dice count)
            let num_dice: u32 = caps.get(1).map_or(1, |m| m.as_str().parse().unwrap());
            let num_sides: u32 = caps.get(2).map_or(0, |m| m.as_str().parse().unwrap());

            let modifier_sign = caps.get(3).map_or("", |m| m.as_str());
            let modifier_value: i32 = caps.get(4).map_or(0, |m| m.as_str().parse().unwrap());

            let modifier = if modifier_sign == "-" {
                -modifier_value
            } else {
                modifier_value
            };

            println!(
                "Roll {} dice with {} sides, modifier: {}",
                num_dice, num_sides, modifier
            );
        } else {
            println!("Invalid dice notation: {}", input);
        }
    }
}

impl Default for Dice{
    fn default() -> Self{
        Dice::new(1, 6, 0)
    }
}
