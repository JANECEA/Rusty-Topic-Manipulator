use crate::controllers::commands::{ArgCommand, CommandResult};
use crate::controllers::Controller;
use crate::models::model::Model;
use crate::settings::{BannerColor, Settings};
use crate::views::parsed_command::ParsedCommand;
use crate::views::View;

pub struct ArgsController {
    model: Model,
    view: Box<dyn View>,
}

impl Controller for ArgsController {
    fn pass_command(
        &mut self,
        parsed_command: ParsedCommand,
        settings: &mut Settings,
    ) -> CommandResult {
        match ArgCommand::from_str(parsed_command.command()) {
            Some(command) => match command {
                ArgCommand::Add => self.model.topic_handler.add_topics(parsed_command.args()),
                ArgCommand::Pick => self.pick_entry_arg(parsed_command.args()),
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
}

impl ArgsController {
    pub fn new(model: Model, view: impl View) -> Self {
        Self { model, view }
    }
}
