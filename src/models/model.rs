use crate::models::{topic_handler::TopicHandler, TopicWriter};

pub struct Model {
    pub topic_writer: Box<dyn TopicWriter>,
    pub topic_handler: TopicHandler,
    pub list_name: String,
}

impl Model {
    pub fn new(
        topic_writer: Box<dyn TopicWriter>,
        topic_handler: TopicHandler,
        list_name: &str,
    ) -> Self {
        Self {
            topic_writer,
            topic_handler,
            list_name: list_name.to_string(),
        }
    }
}
