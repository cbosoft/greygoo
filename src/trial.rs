use chrono::Utc;

use serde::{Deserialize, Serialize};
use textplots::{Chart, Plot, Shape};

use crate::game::Game;
use crate::state::Stats;
// use crate::modifier::Modifier;


pub enum TrialStatus {
    InProgress(f64),
    Success,
    Failure
}

#[derive(Deserialize, Serialize)]
pub struct Trial {
    // Progress
    pub bot_mass: f64,

    // Timings
    pub start_ts: i64,
    pub last_update_ts: i64,
}

impl Trial {
    pub fn new(stats: Stats) -> Trial {
        let start_ts = Utc::now().timestamp();
        let last_update_ts = start_ts;

        Trial {
            bot_mass: stats.initial_bot_mass, start_ts, last_update_ts
        }
    }

    pub fn get_status(&self, game: &Game) -> TrialStatus {
        if self.bot_mass >= game.world_mass {
            TrialStatus::Success
        }
        else if self.bot_mass <= 0f64 {
            TrialStatus::Failure
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
    pub fn get_current_time_progress(&self) -> f32 {
        let now_ts = Utc::now().naive_utc().timestamp();
        let now_dt = (now_ts - self.start_ts) as f32;
        now_dt
    }
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