use crossterm::{
    style::*,
    terminal::{self},
};
use std::{
    fs,
    io::{self, BufRead, Write},
    path::PathBuf,
    vec::Vec,
};
mod topic_handler;
mod undo_redo_handler;
use topic_handler::{Command, CommandResult, TopicHandler};

pub fn show_error(message: &str) {
    eprintln!("{}", message.red())
}

fn init_documents_dir() -> PathBuf {
    if let Some(user_dirs) = directories::UserDirs::new() {
        user_dirs
            .document_dir()
            .map_or_else(|| PathBuf::from("/"), PathBuf::from)
    } else {
        PathBuf::from("/")
    }
}

fn main() {
    let documents_dir: PathBuf = init_documents_dir();
    let topics_file_dir: PathBuf = documents_dir.join("MeetingTopics/");
    let topics_file_path: PathBuf = documents_dir.join("MeetingTopics/topics.happypus");
    let topics_file_old_path: PathBuf = documents_dir.join("MeetingTopics/topics.happypus.old");

    let mut topics: TopicHandler =
        TopicHandler::new(&read_list(&topics_file_dir, &topics_file_path).unwrap());
    run_program(&mut topics);
    write(&topics, &topics_file_path, &topics_file_old_path).unwrap();
}

fn run_program(topics: &mut TopicHandler) {
    let mut can_continue: bool = true;
    while can_continue {
        if topics.should_rerender() {
            render(topics);
        }
        let mut line: String = String::new();
        if io::stdin().read_line(&mut line).is_ok() {
            let trimmed_line = line.trim();
            if trimmed_line.is_empty() {
                continue;
            }
            let result: CommandResult = pass_command(&parse_input_line(trimmed_line), topics);
            if !result.ok() {
                show_error(
                    result
                        .error_message()
                        .as_deref()
                        .unwrap_or("An unknown error occurred"),
                );
            }
            can_continue = topics.can_continue();
        }
    }
}

fn pass_command(parsed_line: &ParsedLine, topics: &mut TopicHandler) -> CommandResult {
    if let Some(command) = Command::from_str(&parsed_line.command) {
        match command {
            Command::Add => topics.add_topics(&parsed_line.args),
            Command::Pick => {
                let mut result: CommandResult = topics.pick_random();
                if result.ok() {
                    print!("{}", "Chosen topic: ".blue());
                    println!(
                        "{}",
                        topics.get_chosen_topic().as_deref().unwrap_or("error")
                    );
                    print!("{}", "Remove topic [y/N]: ".green());
                    if confirm() {
                        result = topics.remove_chosen_topic();
                    }
                }
                result
            }
            Command::Remove => topics.remove_topics(&parsed_line.args),
            Command::Undo => topics.undo(),
            Command::Redo => topics.redo(),
            Command::Exit => topics.exit(),
        }
    } else {
        CommandResult::fail(&format!("Unknown command: {}", &parsed_line.command))
    }
}

fn confirm() -> bool {
    io::stdout().flush().unwrap();
    let mut input: String = String::new();
    io::stdin().read_line(&mut input).unwrap();
    input.starts_with('y')
}

fn render(topics: &TopicHandler) {
    clearscreen::clear().unwrap();
    println!("{}",
        r"
         |@@@@@@@'                                                               ##^'     '^##
      @@@@@@@@@@@@@@@       ___  ___  ____  ____ ______ __ __  __   ___        #              '#
    @@@M@@@@@@@@@@@@@@@     ||\\//|| ||    ||    | || | || ||\ ||  // \\      #                 #
   @@@@@@@  @@@  @@@@@@@    || \/ || ||==  ||==    ||   || ||\\|| (( ___     #   .-.       .-.   #
   @@     @@@@@@@     @@    ||    || ||___ ||___   ||   || || \||  \\_||     #   ##-       -##   #
   @@     @@    @     @@                                                     +        '-'        #
    @@@@@@@ @@@ @@@@@@@          ______   ___   ____  __   ___  __         .- #-               .# --.
  @@  @@@@@@@M@@@@@@@   @        | || |  // \\  || \\ ||  //   (( \        +   ##+++++----++###-#   +
  @@@  @@@@@@@@@@@@@  @@@          ||   ((   )) ||_// || ((     \\         '+ #     +      +    +.-'
    @@@@@  @@M@@@ @@@@             ||    \\_//  ||    ||  \\__ \_))          +      #      +     #
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
    for _ in 0..terminal::size().unwrap().0 {
        print!("{}", '='.dark_grey());
    }
    println!("\n");
}

struct ParsedLine {
    command: String,
    args: Vec<String>,
}

fn parse_input_line(line: &str) -> ParsedLine {
    let index: usize;
    match line.find(' ') {
        Some(i) => index = i,
        None => {
            return ParsedLine {
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
    ParsedLine {
        command: command.to_string(),
        args: args_list,
    }
}

fn write(
    topics: &TopicHandler,
    topics_file_path: &PathBuf,
    topics_file_old_path: &PathBuf,
) -> io::Result<()> {
    fs::copy(topics_file_path, topics_file_old_path)?;
    let mut file: fs::File = fs::File::create(topics_file_path)?;
    for line in topics.get_topics() {
        writeln!(file, "{}", line)?;
    }
    Ok(())
}

fn check_files_exist(topics_file_dir: &PathBuf, topics_file_path: &PathBuf) {
    if !topics_file_dir.exists() {
        fs::create_dir_all(topics_file_dir).expect("Failed to create directory");
    }
    if !topics_file_path.exists() {
        fs::File::create(topics_file_path).expect("Failed to create file");
    }
}

fn read_list(topics_file_dir: &PathBuf, topics_file_path: &PathBuf) -> io::Result<Vec<String>> {
    check_files_exist(topics_file_dir, topics_file_path);

    io::BufReader::new(fs::File::open(topics_file_path)?)
        .lines()
        .collect()
}
