use crate::{
    controllers::{
        commands::{CommandResult, RuntimeCommand, StrEnum},
        Controller,
    },
    models::{
        github_gist_topic_writer::GithubGistTopicWriter, local_topic_writer::LocalTopicWriter,
        model::Model, network_topic_writer::NetworkTopicWriter, topic_handler::TopicHandler,
        TopicWriter,
    },
    settings::{List, ListType, Settings},
    views::{parsed_command::ParsedCommand, View},
};

pub struct RuntimeController {
    model: Model,
    view: Box<dyn View>,
}

impl Controller for RuntimeController {
    fn run(&mut self, settings: &mut Settings) {
        loop {
            if self.model.topic_handler.should_rerender() {
                self.view.render(
                    self.model.topic_handler.get_topics(),
                    self.model.topic_writer.get_banner(),
                    self.model.topic_writer.get_banner_color(),
                );
                self.model
                    .topic_writer
                    .try_write(self.model.topic_handler.get_topics());
            }
            let Some(command) = self.view.get_input() else {
                break;
            };
            if command.is_empty() {
                continue;
            }

            if let CommandResult::Fail(error_message) = self.pass_command(&command, settings) {
                self.view.print_error(&error_message);
            }
            if !self.model.topic_handler.can_continue() {
                break;
            }
        }
    }

    fn close(&mut self) -> anyhow::Result<()> {
        self.model
            .topic_writer
            .write(self.model.topic_handler.get_topics())?;
        self.model.topic_writer.close()?;
        Ok(())
    }
}

impl RuntimeController {
    pub fn new(model: Model, view: Box<dyn View>) -> Self {
        Self { model, view }
    }

    fn pass_command(
        &mut self,
        parsed_command: &ParsedCommand,
        settings: &mut Settings,
    ) -> CommandResult {
        match RuntimeCommand::from_str(parsed_command.command()) {
            Some(command) => match command {
                RuntimeCommand::Add => self.model.topic_handler.add_topics(parsed_command.args()),
                RuntimeCommand::Pick => self.pick_entry(),
                RuntimeCommand::Remove => self
                    .model
                    .topic_handler
                    .remove_topics(parsed_command.args()),
                RuntimeCommand::Undo => self.model.topic_handler.undo(),
                RuntimeCommand::Redo => self.model.topic_handler.redo(),
                RuntimeCommand::Switch => self.switch_list(settings),
                RuntimeCommand::Refresh => {
                    self.set_app_state(&settings.get_list(&self.model.list_name).unwrap(), settings)
                }
                RuntimeCommand::Exit => self.model.topic_handler.exit(),
            },
            None => CommandResult::Fail(format!("Unknown command: {}", parsed_command.command())),
        }
    }

    fn pick_entry(&mut self) -> CommandResult {
        match self.model.topic_handler.pick_random() {
            CommandResult::Success => (),
            fail => return fail,
        }
        if let Some(topic) = self.model.topic_handler.get_chosen_topic() {
            self.view.display_chosen_topic(topic);
            if self.view.get_input().is_some_and(|p| p.command() == "y") {
                return self.model.topic_handler.remove_chosen_topic();
            }
        }
        CommandResult::Success
    }

    fn switch_list(&mut self, settings: &mut Settings) -> CommandResult {
        self.view.print_lists(settings.lists());
        match self.view.get_input() {
            Some(parsed_command) => {
                if parsed_command.is_empty() {
                    CommandResult::Success
                } else if let Some(list) = settings.get_list(parsed_command.command()) {
                    self.set_app_state(&list, settings);
                    CommandResult::Success
                } else {
                    CommandResult::Fail(format!(
                        "Failed to read list: {}",
                        parsed_command.command()
                    ))
                }
            }
            None => CommandResult::Fail("Failed to read line".to_string()),
        }
    }

    fn set_app_state(&mut self, list: &List, settings: &mut Settings) -> CommandResult {
        let mut new_topic_writer: Box<dyn TopicWriter> = match list.list_type() {
            ListType::Local => Box::new(LocalTopicWriter::new(list, settings.documents_path())),
            ListType::Network => Box::new(NetworkTopicWriter::new(list)),
            ListType::GithubGist => Box::new(GithubGistTopicWriter::new(list)),
        };

        match new_topic_writer.read_list() {
            Ok(topics) => {
                settings.set_open_in_list(list);
                _ = self.model.topic_writer.close();

                self.model = Model::new(
                    new_topic_writer,
                    TopicHandler::new(topics.as_slice()),
                    list.name(),
                );
                CommandResult::Success
            }
            Err(error) => CommandResult::Fail(error.to_string()),
        }
    }
}
