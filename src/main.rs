#![allow(dead_code)]
mod controllers;
mod models;
mod settings;
mod views;

use controllers::{
    args_controller::ArgsController, master_controller::MasterController,
    runtime_controller::RuntimeController, Controller,
};
use settings::Settings;
use std::io;
use views::{
    args_console_handler::ArgsConsoleHandler, runtime_console_handler::RuntimeConsoleHandler, View,
};

fn main() {
    let args: Vec<String> = std::env::args().skip(1).collect();
    let settings_result = Settings::get_settings();
    let Ok(settings) = settings_result else {
        eprintln!("{}", settings_result.unwrap_err());
        return;
    };

    let (view, sub_controller): (impl View, impl Controller) = if args.is_empty() {
        (
            RuntimeConsoleHandler::new(io::BufReader::new(io::stdin())),
            RuntimeController::new(),
        )
    } else {
        (
            ArgsConsoleHandler::new(args, io::stdout(), io::stderr()),
            ArgsController::new(),
        )
    };

    let mut master_controller = MasterController::new(settings, view, sub_controller);
    master_controller.run();
    master_controller.close();
}
