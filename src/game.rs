use std::collections::HashMap;

use serde::Deserialize;

use crate::modifier::Modifier;
use crate::read_file_contents::get_contents;

pub struct Stats {
    pub initial_bot_mass: f64,
    pub growth_rate: f64,
    pub death_rate: f64
}

#[derive(Deserialize)]
pub struct Game {
    pub world_mass: f64,
    pub modifiers: HashMap<String, Modifier>
}

impl Default for Game {
    fn default() -> Self {
        let game_source = get_contents("game.json").unwrap();
        serde_json::from_str(&game_source).unwrap()
    }
}

impl Game {
    pub fn get_stats(&self, modifiers: &Vec<String>) -> Result<Stats, ()> {
        let mut stats = Stats{
            initial_bot_mass: 1f64,
            growth_rate: 1f64,
            death_rate: 1f64
        };
        
        for mod_name in modifiers {
            if let Some(modifier) = self.modifiers.get(mod_name.as_str()) {
                stats.initial_bot_mass *= modifier.initial_mass_mult;
                stats.growth_rate *= modifier.growth_rate_mult;
                stats.death_rate *= modifier.death_rate_mult;
            }
            else {
                return Err(())
            }
        }
        
        Ok(stats)
    }
}