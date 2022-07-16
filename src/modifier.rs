use std::str::FromStr;

use serde::Deserialize;
use regex::Regex;

fn one() -> f64 { 1f64 }
fn zero() -> f64 { 0f64 }

#[derive(Deserialize)]
pub struct Modifier {
    pub description: String,

    // Stats
    #[serde(default="one")]
    pub initial_mass_mult: f64,

    #[serde(default="one")]
    pub growth_rate_mult: f64,

    #[serde(default="one")]
    pub death_rate_mult: f64,

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
}