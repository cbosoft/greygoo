use std::collections::{HashMap, HashSet};

use chrono::Utc;
use serde::{Serialize, Deserialize};

use crate::modifier::Modifier;
use crate::trial::{
    Trial,
    TrialStatus
};
use crate::game::Game;
use crate::read_file_contents::get_contents;
use crate::write_file_contents::write_contents;
use crate::fmt_t::fmt_t;
use crate::fmt_mass::fmt_mass;
use crate::serde_default_funcs::zero;

#[derive(Serialize, Deserialize)]
pub struct State {
    pub active_modifiers: Vec<String>,
    pub modifiers_in_progress: Vec<(String, i64)>,
    pub trial_in_progress: Option<Trial>,
    
    // Permanent stats
    #[serde(default="zero")]
    pub population_unease: f64,
    
    #[serde(default="zero")]
    pub scientific_inspiration: f64,

    #[serde(skip)]
    pub game: Game
}

#[derive(Debug)]
pub struct Stats {
    pub initial_bot_mass: f64,
    pub growth_rate: f64,
    pub death_rate: f64,
    pub unease_gain: f64,
    pub inspiration_gain: f64
}


impl State {

    pub fn load(filename: &str) -> State {
        let contents = get_contents(filename).unwrap();
        let mut w: State = serde_json::from_str(&contents).unwrap();
        w.update_modifiers_in_progress();
        w.update_trial_in_progress();
        w
    }

    pub fn save(&self, filename: &str) {
        let fc = serde_json::to_string(self).expect("could not serialise");
        write_contents(filename, fc.as_str()).unwrap();
    }

    fn update_modifiers_in_progress(&mut self) {
        let now = Utc::now().naive_utc().timestamp();
        let mut newly_complete_modifiers: HashSet<String> = HashSet::new();
        for (mod_name, ts_complete) in &self.modifiers_in_progress {
            if *ts_complete <= now {
                newly_complete_modifiers.insert(mod_name.to_string());
                self.active_modifiers.push(mod_name.to_string());
            }
        }

        self.modifiers_in_progress.retain(|(n, _tsc)| !newly_complete_modifiers.contains(n));
    }

    fn update_trial_in_progress(&mut self) {
        if self.trial_in_progress.is_some() {
            self.update_trial();
        }
    }

    fn get_potential_modifiers(&self) -> Option<HashMap<&String, &Modifier>> {
        let game = &self.game;
        if game.modifiers.len() > 0 {
            let mut potential_modifiers: HashMap<&String, &Modifier> = HashMap::new();
            for (mod_name, modifier) in &game.modifiers {

                let mut ok = true;
                if !modifier.locked_by.is_empty() {
                    for locking_mod_name in &modifier.locked_by {
                        if !self.active_modifiers.contains(locking_mod_name) {
                            ok = false;
                            break;
                        }
                    }
                }

                if ok {
                    potential_modifiers.insert(mod_name, modifier);
                }
            }

            if potential_modifiers.len() > 0 {
                Some(potential_modifiers)
            }
            else {
                None
            }
        }
        else {
            None
        }
    }

    pub fn list_potential_modifiers(&self) {
        match self.get_potential_modifiers() {
            Some(mods) => {
                println!("Potential modifiers:");
                for (mod_name, modifier) in mods {
                    println!(" - {}: {}", mod_name, modifier.description);
                }
            },
            None => {
                println!("No potential modifiers.")
            }
        }
    }

    fn start_research_into(&mut self, mod_name: &String) {
        let modifier = self.game.modifiers.get(&mod_name.to_string()).unwrap();
        let now_ts = Utc::now().naive_utc().timestamp();
        let dt_ts = modifier.get_time_cost();
        let end_ts = now_ts + dt_ts;
        self.modifiers_in_progress.push((mod_name.to_string(), end_ts));
    }

    pub fn try_research_modifier(&mut self, mod_name: &str) {
        match self.get_potential_modifiers() {
            Some(mods) => {
                let sr_mod_name = &mod_name.to_string();
                if mods.contains_key(sr_mod_name) {
                    self.start_research_into(sr_mod_name);
                    println!("Starting research into \"{}\"", sr_mod_name);
                }
                else {
                    if self.game.modifiers.contains_key(sr_mod_name) {
                        let modif = self.game.modifiers.get(sr_mod_name).unwrap();
                        println!("Cannot research \"{}\", as it is locked", sr_mod_name);
                    }
                    else {
                        println!("Cannot research \"{}\", no such modifier!", sr_mod_name);
                    }
                }
            },
            None => {
                println!("No modifiers are available to research at the moment.");
            }
        }
    }

    fn get_trial(&self) -> Result<Trial, ()> {
        let stats = self.get_stats();
        Ok(Trial::new(stats))
    }

    pub fn start_trial(&mut self) {
        if self.trial_in_progress.is_some() {
            println!("Cannot start a new trial while another is in progress.");
        }
        else if let Ok(trial) = self.get_trial() {
            let _ = self.trial_in_progress.insert(trial);
            println!("New trial begun!")
        }
        else {
            println!("Could not start trial - something went wrong!");
        }
    }

    pub fn stop_trial(&mut self) {
        if self.trial_in_progress.is_some() {
            let bot_mass = self.trial_in_progress.as_ref().unwrap().bot_mass;
            let fmt_bot_mass = fmt_mass(bot_mass);
            let t_wasted = self.trial_in_progress.as_ref().unwrap().get_current_time_progress() as i64;
            let fmt_t_wasted = fmt_t(t_wasted);
            self.trial_in_progress = None;
            println!("Trial cancelled. {} of bots were silenced. {} of research time, wasted.", fmt_bot_mass, fmt_t_wasted);
        }
        else {
            println!("No trial to cancel.")
        }
    }

    pub fn check_research_progress(&self, loud: bool) {
        if self.modifiers_in_progress.len() > 0 {
            let now_ts = Utc::now().naive_utc().timestamp();
            println!("Research in progress:");
            for (n, e_ts) in &self.modifiers_in_progress {
                let fmt_ttgo = fmt_t(e_ts - now_ts);
                println!(" - {} ({} to go)", n, fmt_ttgo);
            }
        }
        else {
            if loud {
                println!("No research in progress.");
            }
        }
    }

    pub fn check_trial_progress(&self, loud: bool) {
        match &self.trial_in_progress {
            Some(trial) => {
                match trial.get_status(&self.game) {
                    TrialStatus::Failure => {
                        println!("Trial failed!")
                    },
                    TrialStatus::Success => {
                        println!("Trial success! You win!")
                    },
                    TrialStatus::InProgress(bot_mass) => {
                        if loud {
                            let fmt_bot_mass = fmt_mass(bot_mass);
                            let pc = 100f64 * bot_mass / self.game.world_mass;
                            let t_elapsed = trial.get_current_time_progress();
                            let fmt_t_elapsed = fmt_t(t_elapsed as i64);
                            // let rising_ind = if trial.is_rising() {
                            //     "📈"
                            // }
                            // else {
                            //     "📉"
                            // };
                            // trial.plot();
                            println!("Trial running // Bots totalling {} (~{:.1}% domination) // {} elapsed", fmt_bot_mass, pc, fmt_t_elapsed);
                        }
                    }
                }
            },
            None => {
                println!("No trial in progress.")
            }
        }
    }

    pub fn get_stats(&self) -> Stats {
        let mut stats = Stats{
            initial_bot_mass: 1f64,
            growth_rate: 1f64,
            death_rate: 1f64,

            unease_gain: 0.01f64,
            inspiration_gain: 0.01f64
        };

        for mod_name in &self.active_modifiers {
            if let Some(modifier) = self.game.modifiers.get(mod_name.as_str()) {
                if let Some(effect) = modifier.get_effect(self) {
                    stats.initial_bot_mass *= effect.initial_mass_mult;
                    stats.growth_rate *= effect.growth_rate_mult;
                    stats.death_rate *= effect.death_rate_mult;
                    stats.unease_gain *= effect.unease_gain_mult;
                    stats.inspiration_gain *= effect.inspiration_gain_mult;
                }
            }
        }

        stats
    }


    pub fn update_trial(&mut self) -> Vec<String> {
        let mut events: Vec<String> = Vec::new();

        if self.trial_in_progress.is_some() {
            let current_ts = Utc::now().timestamp();

            // find out when the next event will run and what it will be
            let (mut next_event_dt, mut next_event) = self.get_next_event();
            let mut next_event_ts = self.trial_in_progress.as_ref().unwrap().last_update_ts + next_event_dt;

            while next_event_ts < current_ts {
                // run up to when the event should fire
                self.update_trial_until(next_event_ts);

                // run event, note it down too.
                events.push(next_event.clone());
                // self.run_event(&next_event);

                // find out when the next event will run and what it will be
                (next_event_dt, next_event) = self.get_next_event();
                next_event_ts = self.trial_in_progress.as_ref().unwrap().last_update_ts + next_event_dt;
            };

            let trial = self.trial_in_progress.as_mut().unwrap();
            if trial.last_update_ts < current_ts {
                self.update_trial_until(current_ts);
            }
        }

        events
    }

    fn get_next_event(&self) -> (i64, String) {
        (1_000_000i64, "foo".to_string())
    }

    fn update_trial_until(&mut self, until_ts: i64) {
        let stats = self.get_stats();
        if let Some(trial) = self.trial_in_progress.as_mut() {
            let dt = (until_ts - trial.last_update_ts) as f64;

            // grow robots (exponential)
            trial.bot_mass *= (1f64 + stats.growth_rate - stats.death_rate).powf(dt / self.game.tau);

            // Grow unease (linear)
            self.population_unease += stats.unease_gain * (dt / self.game.tau);
            if self.population_unease > 100f64 {
                self.population_unease = 100f64;
            }

            // Gain inspiration (also linear)
            self.scientific_inspiration += stats.inspiration_gain * (dt / self.game.tau);

            // mark update time
            trial.last_update_ts = until_ts;
        }
    }
}