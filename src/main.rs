#![allow(dead_code)]
mod application_state;
mod commands;
mod console_handler;
mod settings;
mod topic_handler;
mod topic_writer;
mod undo_redo_handler;

use application_state::ApplicationState;
use settings::Settings;

fn main() {
    let args: Vec<String> = std::env::args().skip(1).collect();
    let result = Settings::get_settings();
    let Ok(settings) = result else {
        eprintln!("{}", result.unwrap_err());
        return;
    };

    let mut application_state = ApplicationState::new(settings);

    if args.is_empty() {
        application_state.run_program();
    } else {
        application_state.run_with_args(&args);
    }
    application_state.close();
}
