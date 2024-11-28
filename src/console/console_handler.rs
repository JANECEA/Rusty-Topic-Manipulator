use crate::{commands::CommandResult, settings::BannerColor, topic_handler::TopicHandler};

pub trait ConsoleHandler {
    fn pick_topic(&mut self, topics: &mut TopicHandler) -> CommandResult;

    fn render(&self, list: &[String], banner: &str, color: BannerColor);

    fn print_error(&self, message: &str);
}
