use crate::{
    controllers::{controller_factory::ControllerFactory, Controller},
    models::{
        model::Model,
        topic_handler::TopicHandler,
        topic_writer::{local_file_handler::LocalFileHandler, TopicWriter},
    },
    settings::Settings,
    views::View,
};

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
        let documents_path = settings.documents_path().to_owned();
        let list_name = settings.open_in().to_owned();

        let topic_writer =
            LocalFileHandler::new(&settings.get_list(&list_name).unwrap(), &documents_path);
        let topic_handler = TopicHandler::new(&topic_writer.read_list().unwrap());
        let model = Model::new(Box::new(topic_writer), topic_handler);
        Self {
            settings,
            sub_controller: controller_factory.get_controller(model, view),
        }
    }

    pub fn close(&mut self) {
        self.sub_controller.close();
        _ = self.settings.save_settings();
    }

    pub fn run(&mut self) {
        self.sub_controller.run(&mut self.settings);
    }
}
