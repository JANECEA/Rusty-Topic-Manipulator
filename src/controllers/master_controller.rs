use crate::{
    controllers::{
        commands::{ArgCommand, CommandResult, RuntimeCommand},
        Controller,
    },
    models::{
        model::Model,
        topic_handler::TopicHandler,
        topic_writer::{local_file_handler::LocalFileHandler, TopicWriter},
    },
    settings::{BannerColor, List, Settings},
    views::{parsed_command::ParsedCommand, View},
};

pub struct MasterController<V: View, C: Controller> {
    settings: Settings,
    view: V,
    sub_controller: C,
    model: Model,
}

impl<V: View, C: Controller> MasterController<V, C> {
    pub fn new(mut settings: Settings, view: V, sub_controller: C) -> Self {
        let documents_path = settings.documents_path().to_owned();
        let list_name = settings.open_in().to_owned();

        let topic_writer =
            LocalFileHandler::new(&settings.get_list(&list_name).unwrap(), &documents_path);
        let topic_handler = TopicHandler::new(&topic_writer.read_list().unwrap());
        Self {
            settings,
            view,
            sub_controller,
            model: Model::new(Box::new(topic_writer), topic_handler),
        }
    }

    pub fn set_app_state(&mut self, list: &List) -> CommandResult {
        let new_topic_writer =
            Box::new(LocalFileHandler::new(list, self.settings.documents_path()));

        match new_topic_writer.read_list() {
            Ok(topics) => {
                self.settings.set_open_in_list(list);
                _ = self.model.topic_writer.overwrite_old();

                self.model = Model::new(new_topic_writer, TopicHandler::new(topics.as_slice()));
                CommandResult::Success
            }
            Err(error) => CommandResult::Fail(error.to_string()),
        }
    }

    pub fn close(&mut self) {
        _ = self
            .model
            .topic_writer
            .write(self.model.topic_handler.get_topics());
        _ = self.model.topic_writer.overwrite_old();
        _ = self.settings.save_settings();
    }

    fn pass_runtime_command(&mut self, parsed_command: &ParsedCommand) -> CommandResult {
        let topic_handler = &mut self.model.topic_handler;
        match RuntimeCommand::from_str(parsed_command.command()) {
            Some(command) => match command {
                RuntimeCommand::Add => topic_handler.add_topics(parsed_command.args()),
                RuntimeCommand::Pick => self.pick_entry_runtime(),
                RuntimeCommand::Remove => topic_handler.remove_topics(parsed_command.args()),
                RuntimeCommand::Undo => topic_handler.undo(),
                RuntimeCommand::Redo => topic_handler.redo(),
                RuntimeCommand::Switch => self.switch_list_runtime(),
                RuntimeCommand::Exit => topic_handler.exit(),
            },
            None => CommandResult::Fail(format!("Unknown command: {}", parsed_command.command())),
        }
    }

    fn pick_entry_runtime(&mut self) -> CommandResult {
        match self.model.topic_handler.pick_random() {
            CommandResult::Success => (),
            fail => return fail,
        }
        if let Some(topic) = self.model.topic_handler.get_chosen_topic() {
            self.view.display_chosen_topic(topic);
            if self.view.confirm() {
                return self.model.topic_handler.remove_chosen_topic();
            }
        }
        CommandResult::Success
    }

    fn switch_list_runtime(&mut self) -> CommandResult {
        self.view.print_lists(self.settings.lists());
        match self.view.get_input() {
            Some(parsed_command) => {
                if parsed_command.is_empty() {
                    CommandResult::Success
                } else if let Some(list) = self.settings.get_list(parsed_command.command()) {
                    self.set_app_state(&list);
                    CommandResult::Success
                } else {
                    CommandResult::Fail(format!("Failed to read list: {trimmed_line}"))
                }
            }
            None => CommandResult::Fail("Failed to read line".to_string()),
        }
    }

    fn pass_arg_command(&mut self, parsed_command: &ParsedCommand) -> CommandResult {
        let topics_handler = &mut self.model.topic_handler;
        match ArgCommand::from_str(parsed_command.command()) {
            Some(command) => match command {
                ArgCommand::Add => topics_handler.add_topics(parsed_command.args()),
                ArgCommand::Pick => self.pick_entry_arg(parsed_command.args()),
                ArgCommand::Remove => topics_handler.remove_topics(parsed_command.args()),
                ArgCommand::Entries => {
                    self.view
                        .render(topics_handler.get_topics(), "", &BannerColor::White);
                    CommandResult::Success
                }
                ArgCommand::List => {
                    self.view.print_lists(self.settings.lists());
                    CommandResult::Success
                }
                ArgCommand::Switch => self.settings.set_open_in(&parsed_command.args()[0]),
            },
            None => CommandResult::Fail(format!("Unknown command: {}", parsed_command.command())),
        }
    }

    fn pick_entry_arg(&mut self, args: &[String]) -> CommandResult {
        self.model.topic_handler.pick_random();
        if let Some(entry) = self.model.topic_handler.get_chosen_topic() {
            self.view.display_chosen_topic(entry);
        }
        match args.len() {
            0 => CommandResult::Success,
            1 if args[0] == "-y" => self.model.topic_handler.remove_chosen_topic(),
            1 => CommandResult::Fail(format!("Incorrect argument: {}", args[0])),
            _ => CommandResult::Fail("Incorrect number of arguments".to_string()),
        }
    }

    pub fn run_program(&mut self) {
        loop {
            if self.model.topic_handler.should_rerender() {
                self.model
                    .topic_writer
                    .try_write(self.model.topic_handler.get_topics());
                self.view.render(
                    self.model.topic_handler.get_topics(),
                    self.model.topic_writer.get_banner(),
                    self.model.topic_writer.get_banner_color(),
                );
            }
            let Some(command) = self.view.get_input() else {
                break;
            };

            if let CommandResult::Fail(error_message) = self.pass_runtime_command(&command) {
                self.view.print_error(&error_message);
            }
            if !self.model.topic_handler.can_continue() {
                break;
            }
        }
    }

    pub fn run_with_args(&mut self) {
        if let CommandResult::Fail(error_message) =
            self.pass_arg_command(&self.view.get_input().unwrap())
        {
            self.view.print_error(&error_message);
        }
    }

    pub fn run() {
        todo!()
    }
}
