use std::str::FromStr;

use serde::{Serialize, Deserialize};
use regex::Regex;
use crate::state::State;

use crate::serde_default_funcs::one;

const COND_HAS_MOD: &str = r"^has modifier (.*)$";
const COND_TRIAL_BOT_MASS: &str = r"trial bot mass (less|greater) than (\d+(?:\.\d+)?(?:e\d+)?)";
const COND_POP_UNEASE: &str = r"population unease (less|greater) than (\d+(?:\.\d+)?(?:e\d+)?)";

#[derive(Serialize, Deserialize)]
pub struct Effect {
    // Stats
    #[serde(default="one")]
    pub initial_mass_mult: f64,

    #[serde(default="one")]
    pub growth_rate_mult: f64,

    #[serde(default="one")]
    pub death_rate_mult: f64,

    #[serde(default="one")]
    pub unease_gain_mult: f64,

    #[serde(default="one")]
    pub inspiration_gain_mult: f64,

    #[serde(default)]
    pub condition: String
}

impl Effect {
    pub fn is_triggered(&self, state: &State) -> bool {
        if self.condition.is_empty() {
            true
        }
        else {
            if let Some(c) = Regex::new(COND_HAS_MOD).expect("COND_HAS_MOD").captures(&self.condition) {
                state.active_modifiers.contains(&c[1].to_string())
            }
            else if let Some(c) = Regex::new(COND_TRIAL_BOT_MASS).expect("COND_TRIAL_BOT_MASS").captures(&self.condition) {
                if let Some(trial) = &state.trial_in_progress {
                    let cond_op_is_greater = c[1].eq("greater");
                    let cond_bot_mass = f64::from_str(&c[2]).expect("COND_TRIAL_BOT_MASS float parse fail");

                    if cond_op_is_greater {
                        trial.bot_mass > cond_bot_mass
                    }
                    else {
                        trial.bot_mass < cond_bot_mass
                    }
                }
                else {
                    false
                }
            }
            else if let Some(c) = Regex::new(COND_POP_UNEASE).expect("COND_POP_UNEASE").captures(&self.condition) {
                let cond_op_is_greater = c[1].eq("greater");
                let cond_unease = f64::from_str(&c[2]).expect("COND_POP_UNEASE float parse fail");

                if cond_op_is_greater {
                    state.population_unease > cond_unease
                }
                else {
                    state.population_unease < cond_unease
                }

            }
            else {
                false
            }
        }
    }
}