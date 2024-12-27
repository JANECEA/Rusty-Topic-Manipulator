#[derive(Copy, Clone)]
pub enum ArgCommand {
    Add,
    Pick,
    Remove,
    Entries,
    List,
    Switch,
}

impl ArgCommand {
    pub const ALL_COMMANDS: [&'static str; 6] =
        ["add", "pick", "remove", "entries", "list", "switch"];

    pub fn from_str(command: &str) -> Option<ArgCommand> {
        match command {
            "add" => Some(ArgCommand::Add),
            "pick" => Some(ArgCommand::Pick),
            "remove" => Some(ArgCommand::Remove),
            "entries" => Some(ArgCommand::Entries),
            "list" => Some(ArgCommand::List),
            "switch" => Some(ArgCommand::Switch),
            _ => None,
        }
    }

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
    Switch,
    Exit,
}

impl RuntimeCommand {
    pub const ALL_COMMANDS: [&'static str; 7] =
        ["add", "pick", "remove", "undo", "redo", "switch", "exit"];

    pub fn from_str(command: &str) -> Option<RuntimeCommand> {
        match command {
            "add" => Some(RuntimeCommand::Add),
            "pick" => Some(RuntimeCommand::Pick),
            "remove" => Some(RuntimeCommand::Remove),
            "undo" => Some(RuntimeCommand::Undo),
            "redo" => Some(RuntimeCommand::Redo),
            "switch" => Some(RuntimeCommand::Switch),
            "exit" => Some(RuntimeCommand::Exit),
            _ => None,
        }
    }

    pub fn as_str(&self) -> &'static str {
        RuntimeCommand::ALL_COMMANDS[*self as usize]
    }
}

pub enum CommandResult {
    Success,
    Fail(String),
}
