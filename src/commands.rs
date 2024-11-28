#[derive(Copy, Clone)]
pub enum ArgCommand {
    Add,
    Pick,
    Remove,
    Topics,
}

impl ArgCommand {
    pub const ALL_COMMANDS: [&'static str; 4] = ["add", "pick", "remove", "topics"];

    pub fn from_str(command: &str) -> Option<ArgCommand> {
        match command {
            "add" => Some(ArgCommand::Add),
            "pick" => Some(ArgCommand::Pick),
            "remove" => Some(ArgCommand::Remove),
            "topics" => Some(ArgCommand::Topics),
            _ => None,
        }
    }

    #[allow(dead_code)]
    pub fn as_str(&self) -> &'static str {
        ArgCommand::ALL_COMMANDS[*self as usize]
    }
}

#[derive(Copy, Clone)]
pub enum RuntimeCommand {
    Add,
    Pick,
    Remove,
    Undo,
    Redo,
    List,
    Switch,
    Exit,
}

impl RuntimeCommand {
    pub const ALL_COMMANDS: [&'static str; 8] = [
        "add", "pick", "remove", "undo", "redo", "list", "switch", "exit",
    ];

    pub fn from_str(command: &str) -> Option<RuntimeCommand> {
        match command {
            "add" => Some(RuntimeCommand::Add),
            "pick" => Some(RuntimeCommand::Pick),
            "remove" => Some(RuntimeCommand::Remove),
            "undo" => Some(RuntimeCommand::Undo),
            "redo" => Some(RuntimeCommand::Redo),
            "list" => Some(RuntimeCommand::List),
            "switch" => Some(RuntimeCommand::Switch),
            "exit" => Some(RuntimeCommand::Exit),
            _ => None,
        }
    }

    #[allow(dead_code)]
    pub fn as_str(&self) -> &'static str {
        RuntimeCommand::ALL_COMMANDS[*self as usize]
    }
}

pub enum CommandResult {
    Success,
    Fail(String),
}
