use chrono::Utc;

use serde::{Deserialize, Serialize};
use textplots::{Chart, Plot, Shape};

const TAU: f32 = 300f32;


pub enum TrialStatus {
    InProgress(f32),
    Success,
    Failure
}

#[derive(Deserialize, Serialize)]
pub struct Trial {
    initial_bot_count: f32,
    pub target_count: f32,
    death_chance: f32,
    birth_chance: f32,
    start_ts: i64,
    end_ts: i64
}

impl Trial {
    pub fn new(initial_bot_count: f32, bot_mass: f32, world_mass: f32, death_chance: f32,
               birth_chance: f32) -> Trial {
        let target_count = world_mass / bot_mass;
        let a = 1f32 + birth_chance - death_chance;
        let final_number_bots = if a < 1f32 { 0.99f32 } else { target_count };
        let dt_end = (TAU * (final_number_bots / initial_bot_count).ln() / a.ln()) as i64;
        let start_ts = Utc::now().naive_utc().timestamp();
        let end_ts = start_ts + dt_end;

        Trial {
            initial_bot_count, target_count, death_chance, birth_chance, start_ts, end_ts
        }
    }

    pub fn get_current_number_bots(&self) -> f32 {
        let a = 1f32 + self.birth_chance - self.death_chance;
        let now_ts = Utc::now().naive_utc().timestamp();
        let dt = (now_ts - self.start_ts) as f32;
        self.initial_bot_count * a.powf(dt / TAU)
    }

    pub fn get_current_time_progress(&self) -> f32 {
        let now_ts = Utc::now().naive_utc().timestamp();
        let now_dt = (now_ts - self.start_ts) as f32;
        now_dt
    }

    pub fn get_current_time_progress_frac(&self) -> f32 {
        let now_dt = self.get_current_time_progress();
        let all_dt = (self.end_ts - self.start_ts) as f32;
        now_dt / all_dt
    }

    pub fn get_status(&self) -> TrialStatus {
        let bot_count = self.get_current_number_bots();
        let current_time_progress = self.get_current_time_progress_frac();

        if bot_count >= self.target_count {
            TrialStatus::Success
        }
        else {
            if current_time_progress >= 1f32 {
                TrialStatus::Failure
            }
            else {
                TrialStatus::InProgress(bot_count)
            }
        }
    }

    pub fn is_rising(&self) -> bool {
        (self.birth_chance - self.death_chance) > 1f32
    }

    pub fn plot(&self) {
        let now_ts = Utc::now().naive_utc().timestamp();
        let dt_ts = now_ts - self.start_ts;
        let plot_str = Chart::new(64, 32, 0f32, dt_ts as f32)
            .lineplot(&Shape::Continuous(Box::new(|t| self.initial_bot_count * (1f32 + self.birth_chance - self.death_chance).powf(t / TAU))))
            .to_string();
        let plot_lines: Vec<&str> = plot_str.split("\n").collect();

        let ylabel = "  count                        ";
        let xlabel = "                 time  ";
        let ychars: Vec<char> = ylabel.chars().collect();
        for i in 0..plot_lines.len()-1 {
            println!(" {}  {}", ychars[i], plot_lines[i]);
        }
        println!("{}", xlabel);

    }
}