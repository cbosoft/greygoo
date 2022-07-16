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

#[derive(Serialize, Deserialize)]
pub struct State {
    active_modifiers: Vec<String>,
    modifiers_in_progress: Vec<(String, i64)>,
    trial_in_progress: Option<Trial>,

    #[serde(skip)]
    game: Game
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
            self.trial_in_progress.as_mut().unwrap().update(&self.game, &mut self.active_modifiers);
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
        Ok(Trial::new(&self.game, &self.active_modifiers))
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
        // if self.trial_in_progress.is_some() {
        //     let bc = self.trial_in_progress.as_ref().unwrap().get_current_number_bots();
        //     let t_wasted = self.trial_in_progress.as_ref().unwrap().get_current_time_progress() as i64;
        //     let fmt_t_wasted = fmt_t(t_wasted);
        //     self.trial_in_progress = None;
        //     println!("Trial cancelled. {:.0} bots were silenced. {} of research time, wasted.", bc, fmt_t_wasted);
        // }
        // else {
        //     println!("No trial to cancel.")
        // }
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
                            // let pc = 100f32*bot_count / trial.target_count;
                            // let t_elapsed = trial.get_current_time_progress();
                            // let fmt_t_elapsed = fmt_t(t_elapsed as i64);
                            // let rising_ind = if trial.is_rising() {
                            //     "📈"
                            // }
                            // else {
                            //     "📉"
                            // };
                            // trial.plot();
                            // println!("Trial running: {} {:.0} bots currently active (~{:.1}% domination). {} elapsed", rising_ind, bot_count, pc, fmt_t_elapsed);
                            println!("Trial is in progress. {:.2e}", bot_mass)
                        }
                    }
                }
            },
            None => {
                println!("No trial in progress.")
            }
        }
    }
}