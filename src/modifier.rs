use std::str::FromStr;

use serde::Deserialize;
use regex::Regex;

fn one() -> f32 { 1f32 }
fn zero() -> f32 { 0f32 }

#[derive(Deserialize)]
pub struct Modifier {
    pub description: String,

    #[serde(default="one")]
    pub mass_mult: f32,

    #[serde(default="one")]
    pub production_mult: f32,

    #[serde(default="one")]
    pub birth_rate_mult: f32,

    #[serde(default="one")]
    pub death_rate_mult: f32,

    time_cost: String,

    #[serde(default="zero")]
    pub mass_cost: f32,

    #[serde(default)]
    pub locked_by: String,
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
}