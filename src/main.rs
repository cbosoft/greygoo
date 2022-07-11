mod trial;
mod state;
mod modifier;
mod read_file_contents;
mod write_file_contents;
mod game;

fn main() {
    let mut w = state::State::load("test.json");
    w.list_potential_modifiers();
    w.check_research_progress(true);
    w.check_trial_progress(true);
    // w.try_research_modifier("miniaturisation I");
    w.save("test.json");
}
