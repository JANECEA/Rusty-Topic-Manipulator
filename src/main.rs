mod controllers;
mod models;
mod settings;
mod views;

use controllers::{
    controller_factory::{ArgControllerFactory, RuntimeControllerFactory},
    master_controller::MasterController,
};
use settings::Settings;
use std::io;
use views::{
    arg_console_handler::ArgsConsoleHandler, runtime_console_handler::RuntimeConsoleHandler,
};

fn main() {
    let args: Vec<String> = std::env::args().skip(1).collect();
    let settings_result = Settings::get_settings();
    let Ok(settings) = settings_result else {
        eprintln!("{}", settings_result.unwrap_err());
        return;
    };

    let mut master_controller = if args.is_empty() {
        MasterController::new(
            settings,
            Box::new(RuntimeConsoleHandler::new(io::BufReader::new(io::stdin()))),
            RuntimeControllerFactory::new(),
        )
    } else {
        MasterController::new(
            settings,
            Box::new(ArgsConsoleHandler::new(args, io::stdout(), io::stderr())),
            ArgControllerFactory::new(),
        )
    };
    master_controller.run();
    master_controller.close();
}
