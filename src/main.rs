mod console_manager;
mod file_handler;
mod topic_handler;
mod undo_redo_handler;

use console_manager::ConsoleHandler;
use console_manager::ParsedCommand;
use file_handler::FileHandler;
use topic_handler::{Command, CommandResult, TopicHandler};

fn pass_command(
    parsed_command: &ParsedCommand,
    topics: &mut TopicHandler,
    console_handler: &mut ConsoleHandler,
) -> CommandResult {
    if let Some(command) = Command::from_str(parsed_command.get_command()) {
        match command {
            Command::Add => topics.add_topics(parsed_command.get_args()),
            Command::Pick => console_handler.pick_prompt(topics),
            Command::Remove => topics.remove_topics(parsed_command.get_args()),
            Command::Undo => topics.undo(),
            Command::Redo => topics.redo(),
            Command::Exit => topics.exit(),
        }
    } else {
        CommandResult::Fail(format!("Unknown command: {}", parsed_command.get_command()))
    }
}

fn run_program(
    topics: &mut TopicHandler,
    console_handler: &mut ConsoleHandler,
    file_handler: &FileHandler,
) {
    loop {
        if topics.should_rerender() {
            file_handler.try_write(topics);
            console_handler.render(topics);
        }
        let line: String = ConsoleHandler::read_line().unwrap_or_default();
        let trimmed_line: &str = line.trim();
        if trimmed_line.is_empty() {
            continue;
        }
        if let CommandResult::Fail(error_message) = pass_command(
            &ParsedCommand::parse_from_line(trimmed_line),
            topics,
            console_handler,
        ) {
            console_handler.print_error(&error_message);
        }
        if !topics.can_continue() {
            break;
        }
    }
}

fn main() {
    let file_handler = FileHandler::new();
    let mut console_handler = ConsoleHandler::new();
    let mut topics: TopicHandler = TopicHandler::new(&file_handler.read_list().unwrap());

    run_program(&mut topics, &mut console_handler, &file_handler);
    file_handler.write(&topics).unwrap();
    file_handler.overwrite_old().unwrap();
}
