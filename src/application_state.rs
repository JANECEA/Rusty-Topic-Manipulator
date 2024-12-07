use crate::{
    commands::{ArgCommand, CommandResult, RuntimeCommand},
    console_handler::{
        args_console_handler::ArgsConsoleHandler, parsed_command::ParsedCommand,
        runtime_console_handler::RuntimeConsoleHandler, ConsoleHandler,
    },
    settings::{BannerColor, List, Settings},
    topic_handler::TopicHandler,
    topic_writer::{file_handler::LocalFileHandler, TopicWriter},
};

struct AppState {
    topic_writer: Box<dyn TopicWriter>,
    topic_handler: TopicHandler,
}

pub struct ApplicationState {
    settings: Settings,
    app_state: AppState,
}

impl ApplicationState {
    pub fn new(mut settings: Settings) -> Self {
        let documents_path = settings.documents_path().to_owned();
        let list_name = settings.open_in().to_owned();

        let topic_writer =
            LocalFileHandler::new(&settings.get_list(&list_name).unwrap(), &documents_path);
        let topic_handler = TopicHandler::new(&topic_writer.read_list().unwrap());
        Self {
            settings,
            app_state: AppState {
                topic_writer: Box::new(topic_writer),
                topic_handler,
            },
        }
    }

    pub fn set_app_state(&mut self, list: &List) -> CommandResult {
        let new_topic_writer =
            Box::new(LocalFileHandler::new(list, self.settings.documents_path()));

        match new_topic_writer.read_list() {
            Ok(topics) => {
                self.settings.set_open_in_list(list);
                self.app_state.topic_handler = TopicHandler::new(topics.as_slice());
                _ = self.app_state.topic_writer.overwrite_old();
                self.app_state.topic_writer = new_topic_writer;
                CommandResult::Success
            }
            Err(error) => CommandResult::Fail(error.to_string()),
        }
    }

    pub fn close(&mut self) {
        _ = self
            .app_state
            .topic_writer
            .write(self.app_state.topic_handler.get_topics());
        _ = self.app_state.topic_writer.overwrite_old();
        _ = self.settings.save_settings();
    }
}

impl ApplicationState {
    fn pass_runtime_command(
        &mut self,
        console_handler: &mut RuntimeConsoleHandler,
        parsed_command: &ParsedCommand,
    ) -> CommandResult {
        let topic_handler = &mut self.app_state.topic_handler;
        match RuntimeCommand::from_str(parsed_command.command()) {
            Some(command) => match command {
                RuntimeCommand::Add => topic_handler.add_topics(parsed_command.args()),
                RuntimeCommand::Pick => self.pick_entry_runtime(console_handler),
                RuntimeCommand::Remove => topic_handler.remove_topics(parsed_command.args()),
                RuntimeCommand::Undo => topic_handler.undo(),
                RuntimeCommand::Redo => topic_handler.redo(),
                RuntimeCommand::Switch => self.switch_list_runtime(console_handler),
                RuntimeCommand::Exit => topic_handler.exit(),
            },
            None => CommandResult::Fail(format!("Unknown command: {}", parsed_command.command())),
        }
    }

    fn pick_entry_runtime(&mut self, console_handler: &mut RuntimeConsoleHandler) -> CommandResult {
        let mut result: CommandResult = self.app_state.topic_handler.pick_random();
        if let Some(topic) = self.app_state.topic_handler.get_chosen_topic() {
            console_handler.display_chosen_topic(topic);
            if console_handler.confirm() {
                result = self.app_state.topic_handler.remove_chosen_topic();
            }
        }
        result
    }

    fn switch_list_runtime(
        &mut self,
        console_handler: &mut RuntimeConsoleHandler,
    ) -> CommandResult {
        console_handler.print_lists(self.settings.lists());
        if let Some(line) = RuntimeConsoleHandler::read_line() {
            let trimmed_line = line.trim();
            if let Some(list) = self.settings.get_list(trimmed_line) {
                self.set_app_state(&list);
                CommandResult::Success
            } else {
                CommandResult::Fail(format!("Failed to read list: {trimmed_line}"))
            }
        } else {
            CommandResult::Fail("Failed to read line".to_string())
        }
    }

    fn pass_arg_command(
        &mut self,
        console_handler: &mut ArgsConsoleHandler,
        parsed_command: &ParsedCommand,
    ) -> CommandResult {
        let topics_handler = &mut self.app_state.topic_handler;
        match ArgCommand::from_str(parsed_command.command()) {
            Some(command) => match command {
                ArgCommand::Add => topics_handler.add_topics(parsed_command.args()),
                ArgCommand::Pick => self.pick_entry_arg(console_handler, parsed_command.args()),
                ArgCommand::Remove => topics_handler.remove_topics(parsed_command.args()),
                ArgCommand::Entries => {
                    console_handler.render(topics_handler.get_topics(), "", &BannerColor::White);
                    CommandResult::Success
                }
                ArgCommand::List => {
                    console_handler.print_lists(self.settings.lists());
                    CommandResult::Success
                }
                ArgCommand::Switch => self.settings.set_open_in(&parsed_command.args()[0]),
            },
            None => CommandResult::Fail(format!("Unknown command: {}", parsed_command.command())),
        }
    }

    fn pick_entry_arg(
        &mut self,
        console_handler: &mut ArgsConsoleHandler,
        args: &[String],
    ) -> CommandResult {
        self.app_state.topic_handler.pick_random();
        if let Some(entry) = self.app_state.topic_handler.get_chosen_topic() {
            console_handler.display_chosen_topic(entry);
        }
        match args.len() {
            0 => CommandResult::Success,
            1 if args[0] == "-y" => self.app_state.topic_handler.remove_chosen_topic(),
            1 => CommandResult::Fail(format!("Incorrect argument: {}", args[0])),
            _ => CommandResult::Fail("Incorrect number of arguments".to_string()),
        }
    }

    pub fn run_program(&mut self) {
        let mut console_handler = RuntimeConsoleHandler::new();

        loop {
            if self.app_state.topic_handler.should_rerender() {
                self.app_state
                    .topic_writer
                    .try_write(self.app_state.topic_handler.get_topics());
                console_handler.render(
                    self.app_state.topic_handler.get_topics(),
                    self.app_state.topic_writer.get_banner(),
                    self.app_state.topic_writer.get_banner_color(),
                );
            }
            let line: String = RuntimeConsoleHandler::read_line().unwrap_or_default();
            let trimmed_line: &str = line.trim();
            if trimmed_line.is_empty() {
                continue;
            }
            if let CommandResult::Fail(error_message) = self.pass_runtime_command(
                &mut console_handler,
                &ParsedCommand::parse_from_line(trimmed_line),
            ) {
                crate::console_handler::ConsoleHandler::print_error(
                    &console_handler,
                    &error_message,
                );
            }
            if !self.app_state.topic_handler.can_continue() {
                break;
            }
        }
    }

    pub fn run_with_args(&mut self, args: &[String]) {
        let mut console_handler = ArgsConsoleHandler::new();

        if let CommandResult::Fail(error_message) =
            self.pass_arg_command(&mut console_handler, &ParsedCommand::parse_from_args(args))
        {
            console_handler.print_error(&error_message);
        }
    }
}
