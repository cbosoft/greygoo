use std::os::macos::raw::stat;
use chrono::Utc;

use serde::{Deserialize, Serialize};
use textplots::{Chart, Plot, Shape};

use crate::game::Game;
// use crate::modifier::Modifier;

const TAU: f64 = 300f64;


pub enum TrialStatus {
    InProgress(f64),
    Success,
    Failure
}

#[derive(Deserialize, Serialize)]
pub struct Trial {
    // Progress
    bot_mass: f64,
    population_unease: f64,

    // Timings
    start_ts: i64,
    last_update_ts: i64,
}

impl Trial {
    pub fn new(game: &Game, modifiers: &Vec<String>) -> Trial {
        let start_ts = Utc::now().timestamp();
        let last_update_ts = start_ts;

        let stats = game.get_stats(modifiers).unwrap();

        Trial {
            bot_mass: stats.initial_bot_mass, population_unease: 0f64, start_ts, last_update_ts
        }
    }

    pub fn update(&mut self, game: &Game, modifiers: &mut Vec<String>) -> Vec<String> {
        let mut events: Vec<String> = Vec::new();
        let current_ts = Utc::now().timestamp();
        let mut next_event_ts = 0i64;
        while next_event_ts < current_ts {
            // find out when the next event will run and what it will be
            let (next_event_dt, next_event) = self.get_next_event();
            next_event_ts = self.last_update_ts + next_event_dt;

            // run up to when the event should fire
            self.update_until(next_event_ts, game, modifiers);

            // run event, note it down too.
            events.push(next_event.to_string());
            // game.run_event(&next_event, modifiers);
        };

        if self.last_update_ts < current_ts {
            self.update_until(current_ts, game, modifiers);
        }

        events
    }

    fn get_next_event(&self) -> (i64, String) {
        (1_000_000i64, "foo".to_string())
    }

    fn update_until(&mut self, until_ts: i64, game: &Game, modifiers: &Vec<String>) {
        let stats = game.get_stats(modifiers).unwrap();
        let dt = (until_ts - self.last_update_ts) as f64;
        self.bot_mass *= (1f64 + stats.growth_rate - stats.death_rate).powf(dt / TAU);
    }

    pub fn get_status(&self, game: &Game) -> TrialStatus {
        if self.bot_mass >= game.world_mass {
            TrialStatus::Success
        }
        else {
            TrialStatus::InProgress(self.bot_mass)
        }
    }

    // pub fn get_current_number_bots(&self) -> f32 {
    //     let a = 1f32 + self.birth_chance - self.death_chance;
    //     let now_ts = Utc::now().naive_utc().timestamp();
    //     let dt = (now_ts - self.start_ts) as f32;
    //     self.initial_bot_count * a.powf(dt / TAU)
    // }
    //
    // pub fn get_current_time_progress(&self) -> f32 {
    //     let now_ts = Utc::now().naive_utc().timestamp();
    //     let now_dt = (now_ts - self.start_ts) as f32;
    //     now_dt
    // }
    //
    // pub fn get_current_time_progress_frac(&self) -> f32 {
    //     let now_dt = self.get_current_time_progress();
    //     let all_dt = (self.end_ts - self.start_ts) as f32;
    //     now_dt / all_dt
    // }
    //
    // pub fn get_status(&self) -> TrialStatus {
    //     let bot_count = self.get_current_number_bots();
    //     let current_time_progress = self.get_current_time_progress_frac();
    //
    //     if bot_count >= self.target_count {
    //         TrialStatus::Success
    //     }
    //     else {
    //         if current_time_progress >= 1f32 {
    //             TrialStatus::Failure
    //         }
    //         else {
    //             TrialStatus::InProgress(bot_count)
    //         }
    //     }
    // }
    //
    // pub fn is_rising(&self) -> bool {
    //     (self.birth_chance - self.death_chance) > 1f32
    // }
    //
    // pub fn plot(&self) {
    //     let now_ts = Utc::now().naive_utc().timestamp();
    //     let dt_ts = now_ts - self.start_ts;
    //     let plot_str = Chart::new(64, 32, 0f32, dt_ts as f32)
    //         .lineplot(&Shape::Continuous(Box::new(|t| self.initial_bot_count * (1f32 + self.birth_chance - self.death_chance).powf(t / TAU))))
    //         .to_string();
    //     let plot_lines: Vec<&str> = plot_str.split("\n").collect();
    //
    //     let ylabel = "  count                        ";
    //     let xlabel = "                 time  ";
    //     let ychars: Vec<char> = ylabel.chars().collect();
    //     for i in 0..plot_lines.len()-1 {
    //         println!(" {}  {}", ychars[i], plot_lines[i]);
    //     }
    //     println!("{}", xlabel);
    //
    // }
}