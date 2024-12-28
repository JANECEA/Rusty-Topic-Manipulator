pub mod arg_console_handler;
pub mod parsed_command;
pub mod runtime_console_handler;

use crate::settings::{BannerColor, List};
use crate::views::parsed_command::ParsedCommand;

pub trait View {
    fn display_chosen_topic(&mut self, topic: &str);

    fn print_lists(&mut self, lists: &[List]);

    fn render(&mut self, list: &[String], banner: &str, color: &BannerColor);

    fn print_error(&mut self, message: &str);

    fn get_input(&mut self) -> Option<ParsedCommand>;
}
