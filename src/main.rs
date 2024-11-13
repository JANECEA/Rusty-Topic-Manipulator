use arboard::{self, Clipboard};
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

const SETTINGS_DIR_NAME: &str = "RustyTopicManipulator";
const SETTINGS_FILE_NAME: &str = "settings.json";

struct ParsedCommand {
    command: String,
    args: Vec<String>,
}

impl ParsedCommand {
    fn parse_from_line(line: &str) -> Self {
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
}

struct FileHandler {
    topics_file_dir: PathBuf,
    topics_file_path: PathBuf,
    settings_file_path: PathBuf,
    topics_file_old_path: PathBuf,
}

impl FileHandler {
    fn new() -> Self {
        let documents_dir: PathBuf = FileHandler::init_documents_dir();
        let topics_file_dir: PathBuf = documents_dir.join(SETTINGS_DIR_NAME);
        let topics_file_path: PathBuf = topics_file_dir.join("topics.happypus");
        let topics_file_old_path: PathBuf = topics_file_dir.join("topics.happypus.old");
        let settings_file_path: PathBuf = topics_file_dir.join(SETTINGS_FILE_NAME);
        Self {
            topics_file_dir,
            topics_file_path,
            topics_file_old_path,
            settings_file_path,
        }
    }

    fn write(&self, topics: &TopicHandler) -> io::Result<()> {
        let mut file: fs::File = fs::File::create(&self.topics_file_path)?;
        for line in topics.get_topics() {
            writeln!(file, "{}", line)?;
        }
        Ok(())
    }

    fn try_write(&self, topics: &TopicHandler) {
        if let Ok(mut file) = fs::File::create(&self.topics_file_path) {
            for line in topics.get_topics() {
                _ = writeln!(file, "{}", line);
            }
        }
    }

    fn overwrite_old(&self) -> io::Result<()> {
        fs::copy(&self.topics_file_path, &self.topics_file_old_path)?;
        Ok(())
    }

    fn check_files_exist(&self) {
        if !&self.topics_file_dir.exists() {
            fs::create_dir_all(&self.topics_file_dir).expect("Failed to create directory");
        }
        if !&self.topics_file_path.exists() {
            fs::File::create(&self.topics_file_path).expect("Failed to create file");
        }
    }

    fn load_settings(&self) -> String {
        fs::read_to_string(&self.settings_file_path).unwrap()
    }

    fn read_list(&self) -> io::Result<Vec<String>> {
        self.check_files_exist();

        io::BufReader::new(fs::File::open(&self.topics_file_path)?)
            .lines()
            .collect()
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
}

struct ConsoleHandler {
    clipboard: Option<Clipboard>,
}

impl ConsoleHandler {
    fn new() -> Self {
        Self {
            clipboard: if let Ok(clipboard) = Clipboard::new() {
                Some(clipboard)
            } else {
                None
            },
        }
    }

    fn copy_topic_to_clipboard(&mut self, topic: &str) {
        if let Some(clipboard) = &mut self.clipboard {
            _ = clipboard.set_text(topic);
        }
    }

    fn pick_prompt(&mut self, topics: &mut TopicHandler) -> CommandResult {
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

    fn confirm(&self) -> bool {
        io::stdout().flush().unwrap();
        let mut input: String = String::new();
        io::stdin().read_line(&mut input).unwrap();
        input.starts_with('y')
    }

    fn render(&self, topics: &TopicHandler) {
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
        let mut line: String = String::new();
        if io::stdin().read_line(&mut line).is_err() {
            continue;
        }
        let trimmed_line: &str = line.trim();
        if trimmed_line.is_empty() {
            continue;
        }
        if let CommandResult::Fail(result) = pass_command(
            &ParsedCommand::parse_from_line(trimmed_line),
            topics,
            console_handler,
        ) {
            console_handler.print_error(&result);
        }
        if !topics.can_continue() {
            break;
        }
    }
}

fn pass_command(
    parsed_command: &ParsedCommand,
    topics: &mut TopicHandler,
    console_handler: &mut ConsoleHandler,
) -> CommandResult {
    if let Some(command) = Command::from_str(&parsed_command.command) {
        match command {
            Command::Add => topics.add_topics(&parsed_command.args),
            Command::Pick => console_handler.pick_prompt(topics),
            Command::Remove => topics.remove_topics(&parsed_command.args),
            Command::Undo => topics.undo(),
            Command::Redo => topics.redo(),
            Command::Exit => topics.exit(),
        }
    } else {
        CommandResult::Fail(format!("Unknown command: {}", &parsed_command.command))
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
