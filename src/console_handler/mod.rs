pub mod args_console_handler;
pub mod parsed_command;
pub mod runtime_console_handler;

use crate::settings::{BannerColor, List};

pub trait ConsoleHandler {
    fn display_chosen_topic(&mut self, topic: &str);

    fn print_lists(&self, lists: &[List]);

    fn render(&self, list: &[String], banner: &str, color: &BannerColor);

    fn print_error(&self, message: &str);
}
