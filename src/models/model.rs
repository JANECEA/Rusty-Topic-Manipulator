use crate::models::{topic_handler::TopicHandler, TopicWriter};

pub struct Model {
    pub topic_writer: Box<dyn TopicWriter>,
    pub topic_handler: TopicHandler,
}

impl Model {
    pub fn new(topic_writer: Box<dyn TopicWriter>, topic_handler: TopicHandler) -> Self {
        Self {
            topic_writer,
            topic_handler,
        }
    }
}
