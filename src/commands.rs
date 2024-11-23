#[derive(Copy, Clone)]
pub enum Command {
    Add,
    Pick,
    Remove,
    Undo,
    Redo,
    Topics,
    List,
    Switch,
    Exit,
}

impl Command {
    const ALL_COMMANDS: [&'static str; 9] = [
        "add", "pick", "remove", "undo", "redo", "topics", "list", "switch", "exit",
    ];
    pub const RUNTIME_COMMANDS: [&'static str; 8] = [
        "add", "pick", "remove", "undo", "redo", "list", "switch", "exit",
    ];

    pub fn from_str(command: &str) -> Option<Command> {
        match command {
            "add" => Some(Command::Add),
            "pick" => Some(Command::Pick),
            "remove" => Some(Command::Remove),
            "undo" => Some(Command::Undo),
            "redo" => Some(Command::Redo),
            "topics" => Some(Command::Topics),
            "list" => Some(Command::List),
            "switch" => Some(Command::Switch),
            "exit" => Some(Command::Exit),
            _ => None,
        }
    }

    #[allow(dead_code)]
    pub fn as_str(&self) -> &'static str {
        Command::ALL_COMMANDS[*self as usize]
    }
}

pub enum CommandResult {
    Success,
    Fail(String),
}
