use crate::topic_handler::{Command, CommandResult, TopicHandler};
use arboard::Clipboard;
use crossterm::{style::Stylize, terminal};
use std::{
    io::{self, Write},
    vec::Vec,
};

pub struct ParsedCommand {
    command: String,
    args: Vec<String>,
}

impl ParsedCommand {
    pub fn parse_from_line(line: &str) -> Self {
        let index: usize;
        match line.find(' ') {
            Some(i) => index = i,
            None => {
                return ParsedCommand {
                    command: line.to_string(),
                    args: vec![],
                }
            }
        }
        let (command, rest): (&str, &str) = line.split_at(index);
        let mut args_list: Vec<String> = vec![];
        let mut current_word: String = String::new();
        let mut in_quotes: bool = false;

        for ch in rest.trim().chars() {
            match ch {
                '"' => in_quotes = !in_quotes,
                ' ' if !in_quotes => {
                    if !current_word.is_empty() {
                        args_list.push(current_word.clone());
                        current_word.clear();
                    }
                }
                _ => current_word.push(ch),
            }
        }
        args_list.push(current_word.clone());
        Self {
            command: command.to_string(),
            args: args_list,
        }
    }

    pub fn get_command(&self) -> &String {
        &self.command
    }

    pub fn get_args(&self) -> &Vec<String> {
        &self.args
    }
}

pub struct ConsoleHandler {
    clipboard: Option<Clipboard>,
}

impl ConsoleHandler {
    pub fn new() -> Self {
        Self {
            clipboard: if let Ok(clipboard) = Clipboard::new() {
                Some(clipboard)
            } else {
                None
            },
        }
    }

    pub fn copy_topic_to_clipboard(&mut self, topic: &str) {
        if let Some(clipboard) = &mut self.clipboard {
            _ = clipboard.set_text(topic);
        }
    }

    pub fn pick_prompt(&mut self, topics: &mut TopicHandler) -> CommandResult {
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

    pub fn read_line() -> Option<String> {
        let mut line: String = String::new();
        if io::stdin().read_line(&mut line).is_ok() {
            Some(line)
        } else {
            None
        }
    }

    pub fn confirm(&self) -> bool {
        io::stdout().flush().unwrap();
        let mut input: String = String::new();
        io::stdin().read_line(&mut input).unwrap();
        input.starts_with('y')
    }

    pub fn render(&self, topics: &TopicHandler) {
        clearscreen::clear().unwrap();
        println!("{}",
    r"
         |@@@@@@@'                                                               ##^'     '^##
      @@@@@@@@@@@@@@@       ___  ___  ____  ____ ______ __ __  __   ___        #              '#
    @@@M@@@@@@@@@@@@@@@     ||\//|| ||    ||    | || | || ||\ ||  // \      #                 #
   @@@@@@@  @@@  @@@@@@@    || \/ || ||==  ||==    ||   || ||\|| (( ___     #   .-.       .-.   #
   @@     @@@@@@@     @@    ||    || ||___ ||___   ||   || || \||  \_||     #   ##-       -##   #
   @@     @@    @     @@                                                     +        '-'        #
    @@@@@@@ @@@ @@@@@@@          ______   ___   ____  __   ___  __         .- #-               .# --.
  @@  @@@@@@@M@@@@@@@   @        | || |  // \  || \ ||  //   (( \        +   ##+++++----++###-#   +
  @@@  @@@@@@@@@@@@@  @@@          ||   ((   )) ||_// || ((     \         '+ #     +      +    +.-'
    @@@@@  @@M@@@ @@@@             ||    \_//  ||    ||  \__ \_))          +      #      +     #
    @@@@@  @@@@@@ @@@@@@                                                      +    +#      #     #
           @@@@@@                                                              ''#' '-.__.+ '##''
        ".dark_magenta()
);
        for (index, topic) in topics.get_topics().iter().enumerate() {
            println!("{0:>2}. {1}", index + 1, topic);
        }
        println!();
        println!(
            "{} {}",
            "available commands:".dark_grey(),
            Command::ALL_COMMANDS.join(", ").green()
        );
        println!();
        if let Ok((width, _height)) = terminal::size() {
            for _ in 0..width {
                print!("{}", '='.dark_grey());
            }
        }
        println!("\n");
    }

    pub fn print_error(&self, message: &str) {
        eprintln!("{}", message.red())
    }
}
