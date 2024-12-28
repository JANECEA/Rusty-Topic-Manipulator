use crate::settings::Settings;

pub mod arg_controller;
pub mod commands;
pub mod controller_factory;
pub mod master_controller;
pub mod runtime_controller;

pub trait Controller {
    fn run(&mut self, settings: &mut Settings);

    fn close(&mut self) -> std::io::Result<()>;
}
