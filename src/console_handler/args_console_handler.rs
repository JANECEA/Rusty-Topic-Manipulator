use crate::{
    console_handler::ConsoleHandler,
    settings::{BannerColor, List},
};

pub struct ArgsConsoleHandler {}

impl ConsoleHandler for ArgsConsoleHandler {
    fn display_chosen_topic(&mut self, topic: &str) {
        println!("{}", topic);
    }

    fn render(&self, list: &[String], _banner: &str, _color: &BannerColor) {
        for topic in list {
            println!("{}", topic);
        }
    }

    fn print_error(&self, message: &str) {
        eprintln!("{}", message)
    }

    fn print_lists(&self, lists: &[List]) {
        for list in lists {
            println!("{}", list.name())
        }
    }
}

impl ArgsConsoleHandler {
    pub fn new() -> Self {
        Self {}
    }
}
