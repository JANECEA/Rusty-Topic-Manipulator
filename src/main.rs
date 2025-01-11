mod controllers;
mod models;
mod settings;
mod views;

use controllers::{
    controller_factory::{ArgControllerFactory, RuntimeControllerFactory},
    master_controller::MasterController,
};
use settings::Settings;
use std::io::{self, BufReader};
use views::{arg_view::ArgConsoleView, runtime_view::RuntimeConsoleView};

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
            Box::new(RuntimeConsoleView::new(BufReader::new(io::stdin()))),
            RuntimeControllerFactory::new(),
        )
    } else {
        MasterController::new(
            settings,
            Box::new(ArgConsoleView::new(args, io::stdout(), io::stderr())),
            ArgControllerFactory::new(),
        )
    };
    master_controller.run();

    if let Err(error) = master_controller.close() {
        eprintln!("{error}");
    }
}
