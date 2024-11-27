use crate::{
    commands::{Command, CommandResult},
    console::console_handler::ConsoleHandler,
    topic_handler::TopicHandler,
};
use arboard::Clipboard;
use crossterm::{style::Stylize, terminal};
use std::io::{self, Write};

pub struct RuntimeConsoleHandler {
    all_commands: String,
    clipboard: Option<Clipboard>,
}

impl ConsoleHandler for RuntimeConsoleHandler {
    fn pick_topic(&mut self, topics: &mut TopicHandler) -> CommandResult {
        let mut result: CommandResult = topics.pick_random();

        if let Some(topic) = topics.get_chosen_topic() {
            self.copy_topic_to_clipboard(topic);
            print!("{}", "Chosen topic: ".blue());
            println!("{}", topic);
            print!("{}", "Remove topic [y/N]: ".green());
            if self.confirm() {
                result = topics.remove_chosen_topic();
            }
        }
        result
    }

    fn render(&self, list: &[String], banner: &str) {
        clearscreen::clear().unwrap();
        println!("{}", banner.dark_magenta());
        for (index, topic) in list.iter().enumerate() {
            println!("{0:>2}. {1}", index + 1, topic);
        }
        println!(
            "\n{} {}",
            "available commands:".dark_grey(),
            self.all_commands.as_str().green()
        );
        println!();
        if let Ok((width, _height)) = terminal::size() {
            for _ in 0..width {
                print!("{}", '='.dark_grey());
            }
        }
        println!("\n");
    }

    fn print_error(&self, message: &str) {
        eprintln!("{}", message.red())
    }
}

impl RuntimeConsoleHandler {
    pub fn new() -> Self {
        Self {
            all_commands: Command::RUNTIME_COMMANDS.join(", "),
            clipboard: if let Ok(clipboard) = Clipboard::new() {
                Some(clipboard)
            } else {
                None
            },
        }
    }

    pub fn read_line() -> Option<String> {
        let mut line: String = String::new();
        if io::stdin().read_line(&mut line).is_ok() {
            Some(line)
        } else {
            None
        }
    }

    fn copy_topic_to_clipboard(&mut self, topic: &str) {
        if let Some(clipboard) = &mut self.clipboard {
            _ = clipboard.set_text(topic);
        }
    }

    fn confirm(&self) -> bool {
        _ = io::stdout().flush();
        let mut input: String = String::new();

        if io::stdin().read_line(&mut input).is_ok() {
            input.starts_with('y')
        } else {
            false
        }
    }
}
