use crate::{
    controllers::{controller_factory::ControllerFactory, Controller},
    models::{
        github_gist_topic_writer::GithubGistTopicWriter, local_topic_writer::LocalTopicWriter,
        model::Model, network_topic_writer::NetworkTopicWriter, topic_handler::TopicHandler,
        TopicWriter,
    },
    settings::{ListType, Settings},
    views::View,
};
use anyhow::Result;

pub struct MasterController {
    settings: Settings,
    sub_controller: Box<dyn Controller>,
}

impl MasterController {
    pub fn new(
        mut settings: Settings,
        view: Box<dyn View>,
        controller_factory: impl ControllerFactory,
    ) -> Self {
        let documents_path = settings.documents_path().clone();
        let list_name = settings.open_in().to_string();
        let list = &settings
            .get_list(&list_name)
            .unwrap_or_else(|| settings.get_list_by_index(0));

        let mut topic_writer: Box<dyn TopicWriter> = match list.list_type() {
            ListType::Local => Box::new(LocalTopicWriter::new(list, &documents_path)),
            ListType::Network => Box::new(NetworkTopicWriter::new(list)),
            ListType::GithubGist => Box::new(GithubGistTopicWriter::new(list)),
        };

        let topic_handler =
            TopicHandler::new(&topic_writer.read_list().expect("Could not read topics."));

        Self {
            settings,
            sub_controller: controller_factory
                .get_controller(Model::new(topic_writer, topic_handler), view),
        }
    }

    pub fn close(&mut self) -> Result<()> {
        self.sub_controller.close()?;
        self.settings.save_settings()?;
        Ok(())
    }

    pub fn run(&mut self) {
        self.sub_controller.run(&mut self.settings);
    }
}
