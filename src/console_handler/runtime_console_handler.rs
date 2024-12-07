use crate::{
    commands::RuntimeCommand,
    console_handler::ConsoleHandler,
    settings::{BannerColor, List},
};
use arboard::Clipboard;
use crossterm::{style::Stylize, terminal};
use std::io::{self, Write};

pub struct RuntimeConsoleHandler {
    all_commands: String,
    clipboard: Option<Clipboard>,
}

impl ConsoleHandler for RuntimeConsoleHandler {
    fn display_chosen_topic(&mut self, topic: &str) {
        self.copy_topic_to_clipboard(topic);
        print!("{}", "Chosen topic: ".blue());
        println!("{}", topic);
        print!("{}", "Remove topic [y/N]: ".green());
    }

    fn render(&self, list: &[String], banner: &str, color: &BannerColor) {
        _ = clearscreen::clear();
        println!(
            "{}",
            crossterm::style::style(banner).with(color.as_crossterm_color())
        );
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

    fn print_lists(&self, lists: &[List]) {
        for (index, list) in lists.iter().enumerate() {
            println!("{}. {}", (index + 1).to_string().dark_grey(), list.name())
        }
    }
}

impl RuntimeConsoleHandler {
    pub fn new() -> Self {
        Self {
            all_commands: RuntimeCommand::ALL_COMMANDS.join(", "),
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

    pub fn confirm(&self) -> bool {
        _ = io::stdout().flush();
        let mut input: String = String::new();

        if io::stdin().read_line(&mut input).is_ok() {
            input.starts_with('y')
        } else {
            false
        }
    }

    fn copy_topic_to_clipboard(&mut self, topic: &str) {
        if let Some(clipboard) = &mut self.clipboard {
            _ = clipboard.set_text(topic);
        }
    }
}
