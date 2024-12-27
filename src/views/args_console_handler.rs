use crate::{
    settings::{BannerColor, List},
    views::parsed_command::ParsedCommand,
    views::View,
};
use std::io::Write;

pub struct ArgsConsoleHandler<S: Write, E: Write> {
    input: Option<Vec<String>>,
    std_writer: S,
    err_writer: E,
}

impl<S: Write, E: Write> View for ArgsConsoleHandler<S, E> {
    fn display_chosen_topic(&mut self, topic: &str) {
        _ = writeln!(&self.std_writer, "{topic}");
    }

    fn print_lists(&self, lists: &[List]) {
        for list in lists {
            _ = writeln!(&self.std_writer, "{}", list.name());
        }
    }

    fn render(&self, list: &[String], _banner: &str, _color: &BannerColor) {
        for topic in list {
            _ = writeln!(&self.std_writer, "{topic}");
        }
    }

    fn print_error(&self, message: &str) {
        _ = writeln!(&self.err_writer, "{message}")
    }

    fn get_input(&mut self) -> Option<ParsedCommand> {
        match self.input.as_mut() {
            Some(input) => {
                let command = ParsedCommand::parse_from_args(input.as_slice());
                self.input = None;
                Some(command)
            }
            None => None,
        }
    }
}

impl<S: Write, E: Write> ArgsConsoleHandler<S, E> {
    pub fn new(args: Vec<String>, std_writer: S, err_writer: E) -> Self {
        Self {
            input: Some(args),
            std_writer,
            err_writer,
        }
    }
}
