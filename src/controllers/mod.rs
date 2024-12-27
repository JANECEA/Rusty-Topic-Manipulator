use crate::controllers::commands::CommandResult;
use crate::settings::Settings;
use crate::views::parsed_command::ParsedCommand;

pub mod args_controller;
pub mod commands;
pub mod master_controller;
pub mod runtime_controller;

pub trait Controller {
    fn pass_command(
        &mut self,
        parsed_command: ParsedCommand,
        settings: &mut Settings,
    ) -> CommandResult;
}
