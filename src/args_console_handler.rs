use crate::{
    commands::CommandResult, console_handler::ConsoleHandler, topic_handler::TopicHandler,
};

pub struct ArgsConsoleHandler {}

impl ConsoleHandler for ArgsConsoleHandler {
    fn pick_topic(&mut self, topics: &mut TopicHandler) -> CommandResult {
        let mut result: CommandResult = topics.pick_random();
        if let Some(topic) = topics.get_chosen_topic() {
            println!("{}", topic);
            result = topics.remove_chosen_topic();
        }
        result
    }

    fn render(&self, list: &[String]) -> CommandResult {
        for topic in list {
            println!("{}", topic);
        }
        CommandResult::Success
    }

    fn print_error(&self, message: &str) {
        eprintln!("{}", message)
    }
}

impl ArgsConsoleHandler {
    pub fn new() -> Self {
        Self {}
    }
}
