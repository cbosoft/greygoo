use argparse::{Store, StoreTrue};

mod trial;
mod state;
mod modifier;
mod read_file_contents;
mod write_file_contents;
mod game;
mod fmt_t;
mod effect;
mod serde_default_funcs;
mod fmt_mass;

fn main() {
    // options
    let mut should_check = false;
    let mut should_list = false;
    let mut what_to_research = String::new();
    let mut should_do_trial = false;
    let mut should_cancel_trial = false;
    {
        let mut parser = argparse::ArgumentParser::new();
        parser.refer(&mut should_check)
            .add_option(&["-c", "--check-progress"], StoreTrue,
                        "Check progress of trials and research.");
        parser.refer(&mut should_list)
            .add_option(&["-l", "--list-research"], StoreTrue,
                        "List any available research.");
        parser.refer(&mut what_to_research)
            .add_option(&["-r", "--do-research"], Store,
                        "Conduct research");
        parser.refer(&mut should_do_trial)
            .add_option(&["-t", "--do-trial"], StoreTrue,
                        "Conduct trial with current state-of-the-art robots.");
        parser.refer(&mut should_cancel_trial)
            .add_option(&["-x", "--stop-trial"], StoreTrue,
                        "Cancel a currently running trial, activating the self-destruct of any active bots. Can be combined with --do-trial to effectively restart a trial.");
        parser.parse_args_or_exit();
    }
    let should_research = !what_to_research.is_empty();

    if !(should_research || should_check || should_list || should_do_trial) {
        should_check = true;
    }

    let mut w = state::State::load("test.json");

    if should_list {
        w.list_potential_modifiers();
    }

    if should_check {
        w.check_research_progress(true);
        w.check_trial_progress(true);
    }

    if should_research {
        w.try_research_modifier(what_to_research.as_str());
    }

    if should_cancel_trial {
        w.stop_trial()
    }

    if should_do_trial {
        w.start_trial();
    }

    w.save("test.json");
}
