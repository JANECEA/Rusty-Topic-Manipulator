use crate::{commands::CommandResult, topic_handler::TopicHandler};

pub trait ConsoleHandler {
    fn pick_topic(&mut self, topics: &mut TopicHandler) -> CommandResult;

    fn render(&self, list: &[String]) -> CommandResult;

    fn print_error(&self, message: &str);
}