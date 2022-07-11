use std::collections::HashMap;

use serde::Deserialize;

use crate::modifier::Modifier;
use crate::read_file_contents::get_contents;

#[derive(Deserialize)]
pub struct Game {
    pub world_mass: f32,
    pub modifiers: HashMap<String, Modifier>
}

impl Default for Game {
    fn default() -> Self {
        let game_source = get_contents("game.json").unwrap();
        serde_json::from_str(&game_source).unwrap()
    }
}