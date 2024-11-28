mod commands;
mod console;
mod settings;
mod topic_handler;
mod undo_redo_handler;
mod writer;

use commands::{ArgCommand, CommandResult, RuntimeCommand};
use console::{
    args_console_handler::ArgsConsoleHandler, console_handler::ConsoleHandler,
    parsed_command::ParsedCommand, runtime_console_handler::RuntimeConsoleHandler,
};
use settings::{BannerColor, Settings};
use topic_handler::TopicHandler;
use writer::{file_handler::LocalTopicFileHandler, topic_writer::TopicWriter};

fn pass_arg_command(
    parsed_command: &ParsedCommand,
    topics: &mut TopicHandler,
    console_handler: &mut impl ConsoleHandler,
) -> CommandResult {
    match ArgCommand::from_str(parsed_command.get_command()) {
        Some(command) => match command {
            ArgCommand::Add => topics.add_topics(parsed_command.get_args()),
            ArgCommand::Pick => console_handler.pick_topic(topics),
            ArgCommand::Remove => topics.remove_topics(parsed_command.get_args()),
            ArgCommand::Topics => {
                console_handler.render(topics.get_topics(), "", BannerColor::White);
                CommandResult::Success
            }
        },
        None => CommandResult::Fail(format!("Unknown command: {}", parsed_command.get_command())),
    }
}

fn pass_runtime_command(
    parsed_command: &ParsedCommand,
    topics: &mut TopicHandler,
    console_handler: &mut impl ConsoleHandler,
) -> CommandResult {
    match RuntimeCommand::from_str(parsed_command.get_command()) {
        Some(command) => match command {
            RuntimeCommand::Add => topics.add_topics(parsed_command.get_args()),
            RuntimeCommand::Pick => console_handler.pick_topic(topics),
            RuntimeCommand::Remove => topics.remove_topics(parsed_command.get_args()),
            RuntimeCommand::Undo => topics.undo(),
            RuntimeCommand::Redo => topics.redo(),
            RuntimeCommand::List => todo!(),
            RuntimeCommand::Switch => todo!(),
            RuntimeCommand::Exit => topics.exit(),
        },
        None => CommandResult::Fail(format!("Unknown command: {}", parsed_command.get_command())),
    }
}

fn run_program(topics: &mut TopicHandler, topic_writer: &impl TopicWriter) {
    let mut console_handler = RuntimeConsoleHandler::new();

    loop {
        if topics.should_rerender() {
            topic_writer.try_write(topics.get_topics());
            console_handler.render(
                topics.get_topics(),
                topic_writer.get_banner(),
                settings::BannerColor::DarkMagenta,
            );
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
    let mut settings: Settings =
        settings::parse_json_file(writer::file_handler::SETTINGS_FILE_NAME).unwrap();
    run(args);
}

fn run(args: Vec<String>) {
    let topic_writer = LocalTopicFileHandler::new("topics.happypus", "topics.happypus.old");
    let mut topics = TopicHandler::new(&topic_writer.read_list().unwrap());

    if args.is_empty() {
        run_program(&mut topics, &topic_writer);
    } else {
        execute_args(&mut topics, &args);
    }
    topic_writer.write(topics.get_topics()).unwrap();
    topic_writer.overwrite_old().unwrap();
}
