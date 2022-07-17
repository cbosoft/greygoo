use std::str::FromStr;
use std::collections::HashMap;

use serde::Deserialize;
use regex::Regex;

use crate::serde_default_funcs::zero;
use crate::effect::Effect;
use crate::game::Game;
use crate::state::State;
use crate::trial::Trial;

#[derive(Deserialize)]
pub struct Modifier {
    pub description: String,

    pub effects: HashMap<String, Effect>,

    // Costs
    time_cost: String,

    #[serde(default="zero")]
    pub mass_cost: f64,

    // Prerequisites
    #[serde(default)]
    pub locked_by: Vec<String>,
}

impl Modifier {
    pub fn get_time_cost(&self) -> i64 {
        let re = Regex::new(r"(\d+)([wdhms])").unwrap();
        if let Some(c) = re.captures(self.time_cost.as_str()) {
            let t = i64::from_str(&c[1]).unwrap();
            let unit: i64 = match &c[2] {
                "w" => 604800,
                "d" => 86400,
                "h" => 3600,
                "m" => 60,
                _ => 1
            };
            t*unit
        }
        else {
            panic!("time cost not in expected format! Should be \"\\d+[wdhms]\", but got \"{}\".", self.time_cost);
        }
    }

    pub fn get_effect(&self, state: &State) -> Option<&Effect> {
        let mut rv: Option<&Effect> = self.effects.get("default");

        let mut effects: Vec<&String> = (&self.effects).into_iter().map(|(s, _)| s).collect();
        effects.retain(|s| s.ne(&"default"));
        effects.sort();

        for (name, effect) in &self.effects {
            if effect.is_triggered(state) {
                rv = Some(effect);
            }
        }

        rv
    }
}