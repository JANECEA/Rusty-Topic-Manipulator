#[derive(Copy, Clone)]
pub enum ArgCommand {
    Add,
    Pick,
    Remove,
    Entries,
    List,
    Switch,
}

pub trait StrEnum {
    fn as_str(&self) -> &'static str;

    fn from_str(command: &str) -> Option<Self>
    where
        Self: Sized;
}

impl StrEnum for ArgCommand {
    fn as_str(&self) -> &'static str {
        Self::ALL_COMMANDS[*self as usize]
    }

    fn from_str(command: &str) -> Option<Self>
    where
        Self: Sized,
    {
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
}

impl ArgCommand {
    pub const ALL_COMMANDS: [&'static str; 6] =
        ["add", "pick", "remove", "entries", "list", "switch"];
}

#[derive(Copy, Clone)]
pub enum RuntimeCommand {
    Add,
    Pick,
    Remove,
    Undo,
    Redo,
    Switch,
    Refresh,
    Exit,
}

impl StrEnum for RuntimeCommand {
    fn as_str(&self) -> &'static str {
        Self::ALL_COMMANDS[*self as usize]
    }

    fn from_str(command: &str) -> Option<Self>
    where
        Self: Sized,
    {
        match command {
            "add" => Some(RuntimeCommand::Add),
            "pick" => Some(RuntimeCommand::Pick),
            "remove" => Some(RuntimeCommand::Remove),
            "undo" => Some(RuntimeCommand::Undo),
            "redo" => Some(RuntimeCommand::Redo),
            "switch" => Some(RuntimeCommand::Switch),
            "refresh" => Some(RuntimeCommand::Refresh),
            "exit" => Some(RuntimeCommand::Exit),
            _ => None,
        }
    }
}

impl RuntimeCommand {
    pub const ALL_COMMANDS: [&'static str; 8] = [
        "add", "pick", "remove", "undo", "redo", "switch", "refresh", "exit",
    ];
}

pub enum CommandResult {
    Success,
    Fail(String),
}
