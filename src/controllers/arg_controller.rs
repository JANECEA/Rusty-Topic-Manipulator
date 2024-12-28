use crate::{
    controllers::{
        commands::{ArgCommand, CommandResult},
        Controller,
    },
    models::model::Model,
    settings::{BannerColor, Settings},
    views::{parsed_command::ParsedCommand, View},
};

use super::commands::StrEnum;

pub struct ArgController {
    model: Model,
    view: Box<dyn View>,
}

impl Controller for ArgController {
    fn run(&mut self, settings: &mut Settings) {
        let parsed_command = self.view.get_input().unwrap();

        if let CommandResult::Fail(error_message) = self.pass_command(&parsed_command, settings) {
            self.view.print_error(&error_message);
        }
    }

    fn close(&mut self) -> std::io::Result<()> {
        self.model
            .topic_writer
            .write(self.model.topic_handler.get_topics())?;
        self.model.topic_writer.overwrite_old()?;
        Ok(())
    }
}

impl ArgController {
    pub fn new(model: Model, view: Box<dyn View>) -> Self {
        Self { model, view }
    }

    fn pass_command(
        &mut self,
        parsed_command: &ParsedCommand,
        settings: &mut Settings,
    ) -> CommandResult {
        match ArgCommand::from_str(parsed_command.command()) {
            Some(command) => match command {
                ArgCommand::Add => self.model.topic_handler.add_topics(parsed_command.args()),
                ArgCommand::Pick => self.pick_entry(parsed_command.args()),
                ArgCommand::Remove => self
                    .model
                    .topic_handler
                    .remove_topics(parsed_command.args()),
                ArgCommand::Entries => {
                    self.view.render(
                        self.model.topic_handler.get_topics(),
                        "",
                        &BannerColor::White,
                    );
                    CommandResult::Success
                }
                ArgCommand::List => {
                    self.view.print_lists(settings.lists());
                    CommandResult::Success
                }
                ArgCommand::Switch => settings.set_open_in(&parsed_command.args()[0]),
            },
            None => CommandResult::Fail(format!("Unknown command: {}", parsed_command.command())),
        }
    }

    fn pick_entry(&mut self, args: &[String]) -> CommandResult {
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
}
