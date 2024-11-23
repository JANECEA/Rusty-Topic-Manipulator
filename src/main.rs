mod commands;
mod console;
mod topic_handler;
mod undo_redo_handler;
mod writer;

use commands::{Command, CommandResult};
use console::{
    args_console_handler::ArgsConsoleHandler, console_handler::ConsoleHandler,
    parsed_command::ParsedCommand, runtime_console_handler::RuntimeConsoleHandler,
};
use topic_handler::TopicHandler;
use writer::{file_handler::FileHandler, topic_writer::TopicWriter};

fn pass_arg_command(
    parsed_command: &ParsedCommand,
    topics: &mut TopicHandler,
    console_handler: &mut impl ConsoleHandler,
) -> CommandResult {
    let fail = CommandResult::Fail(format!("Unknown command: {}", parsed_command.get_command()));

    if let Some(command) = Command::from_str(parsed_command.get_command()) {
        match command {
            Command::Add => topics.add_topics(parsed_command.get_args()),
            Command::Pick => console_handler.pick_topic(topics),
            Command::Remove => topics.remove_topics(parsed_command.get_args()),
            Command::Topics => console_handler.render(topics.get_topics()),
            _ => fail,
        }
    } else {
        fail
    }
}

fn pass_runtime_command(
    parsed_command: &ParsedCommand,
    topics: &mut TopicHandler,
    console_handler: &mut impl ConsoleHandler,
) -> CommandResult {
    let fail = CommandResult::Fail(format!("Unknown command: {}", parsed_command.get_command()));

    if let Some(command) = Command::from_str(parsed_command.get_command()) {
        match command {
            Command::Add => topics.add_topics(parsed_command.get_args()),
            Command::Pick => console_handler.pick_topic(topics),
            Command::Remove => topics.remove_topics(parsed_command.get_args()),
            Command::Undo => topics.undo(),
            Command::Redo => topics.redo(),
            Command::List => todo!(),
            Command::Switch => todo!(),
            Command::Exit => topics.exit(),
            _ => fail,
        }
    } else {
        fail
    }
}

fn run_program(topics: &mut TopicHandler, topic_writer: &impl TopicWriter) {
    let mut console_handler = RuntimeConsoleHandler::new();

    loop {
        if topics.should_rerender() {
            topic_writer.try_write(topics.get_topics());
            console_handler.render(topics.get_topics());
        }
        let line: String = RuntimeConsoleHandler::read_line().unwrap_or_default();
        let trimmed_line: &str = line.trim();
        if trimmed_line.is_empty() {
            continue;
        }
        if let CommandResult::Fail(error_message) = pass_runtime_command(
            &ParsedCommand::parse_from_line(trimmed_line),
            topics,
            &mut console_handler,
        ) {
            console_handler.print_error(&error_message);
        }
        if !topics.can_continue() {
            break;
        }
    }
}

fn execute_args(topics: &mut TopicHandler, args: &[String]) {
    let mut console_handler = ArgsConsoleHandler::new();

    if let CommandResult::Fail(error_message) = pass_arg_command(
        &ParsedCommand::parse_from_args(args),
        topics,
        &mut console_handler,
    ) {
        console_handler.print_error(&error_message);
    }
}

fn main() {
    let args: Vec<String> = std::env::args().skip(1).collect();
    let topic_writer = FileHandler::new("topics.happypus", "topics.happypus.old");
    let mut topics = TopicHandler::new(&topic_writer.read_list().unwrap());

    if args.is_empty() {
        run_program(&mut topics, &topic_writer);
    } else {
        execute_args(&mut topics, &args);
    }
    topic_writer.write(topics.get_topics()).unwrap();
    topic_writer.overwrite_old().unwrap();
}
