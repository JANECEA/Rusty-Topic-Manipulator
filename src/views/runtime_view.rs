use crate::{
    settings::{BannerColor, List},
    views::{parsed_command::ParsedCommand, View},
};
use arboard::Clipboard;
use crossterm::{style::Stylize, terminal};
use std::io::{self, BufRead, Write};

pub struct RuntimeConsoleView<R: BufRead> {
    all_commands: String,
    clipboard: Option<Clipboard>,
    reader: R,
}

impl<R: BufRead> View for RuntimeConsoleView<R> {
    fn display_chosen_topic(&mut self, topic: &str) {
        self.copy_topic_to_clipboard(topic);
        print!("{}", "Chosen topic: ".blue());
        println!("{topic}");
        print!("{}", "Remove topic [y/N]: ".green());
        _ = io::stdout().flush();
    }

    fn print_lists(&mut self, lists: &[List]) {
        for (index, list) in lists.iter().enumerate() {
            println!(
                "{} {}",
                format!("{:>2}.", (index + 1).to_string()).dark_grey(),
                list.name()
            );
        }
        print!("{}", "List name or index: ".green());
        _ = io::stdout().flush();
    }

    fn render(&mut self, list: &[String], banner: &str, color: &BannerColor) {
        _ = clearscreen::clear();
        println!(
            "{}",
            crossterm::style::style(banner).with(color.as_crossterm_color())
        );
        for (index, topic) in list.iter().enumerate() {
            println!(
                "{} {topic}",
                format!("{:>2}.", (index + 1).to_string()).grey(),
            );
        }
        println!(
            "\n{} {}\n",
            "available commands:".dark_grey(),
            self.all_commands.as_str().green()
        );
        if let Ok((width, _height)) = terminal::size() {
            for _ in 0..width {
                print!("{}", '='.dark_grey());
            }
        }
        println!("\n");
    }

    fn print_error(&mut self, message: &str) {
        eprintln!("{}", message.red())
    }

    fn get_input(&mut self) -> Option<ParsedCommand> {
        let mut line: String = String::new();
        let result = self.reader.read_line(&mut line);

        if result.is_ok() && result.unwrap() > 0 {
            Some(ParsedCommand::parse_from_line(&line))
        } else {
            None
        }
    }
}

impl<R: BufRead> RuntimeConsoleView<R> {
    pub fn new(reader: R) -> Self {
        Self {
            all_commands: crate::controllers::commands::RuntimeCommand::ALL_COMMANDS.join(", "),
            clipboard: match Clipboard::new() {
                Ok(clipboard) => Some(clipboard),
                Err(_) => None,
            },
            reader,
        }
    }

    fn copy_topic_to_clipboard(&mut self, topic: &str) {
        if let Some(clipboard) = &mut self.clipboard {
            _ = clipboard.set_text(topic);
        }
    }
}
