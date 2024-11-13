use crate::undo_redo_handler::UndoRedoHandler;
use rand::{rngs::ThreadRng, Rng};

#[derive(Copy, Clone)]
pub enum Command {
    Add,
    Pick,
    Remove,
    Undo,
    Redo,
    Exit,
}

impl Command {
    pub const ALL_COMMANDS: [&'static str; 6] = ["add", "pick", "remove", "undo", "redo", "exit"];

    pub fn from_str(command: &str) -> Option<Command> {
        match command {
            "add" => Some(Command::Add),
            "pick" => Some(Command::Pick),
            "remove" => Some(Command::Remove),
            "undo" => Some(Command::Undo),
            "redo" => Some(Command::Redo),
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

pub struct TopicHandler {
    topic_history: UndoRedoHandler<Vec<String>>,
    state: Vec<String>,
    has_changed: bool,
    can_continue: bool,
    chosen_topic: Option<(String, usize)>,
    rng: ThreadRng,
}

impl TopicHandler {
    pub fn new(state: &[String]) -> Self {
        let mut undo_redo_handler: UndoRedoHandler<Vec<String>> = UndoRedoHandler::new();
        undo_redo_handler.add_new_node(state.to_vec());
        TopicHandler {
            state: state.to_vec(),
            has_changed: true,
            can_continue: true,
            rng: rand::thread_rng(),
            topic_history: undo_redo_handler,
            chosen_topic: None,
        }
    }

    pub fn should_rerender(&mut self) -> bool {
        let changed: bool = self.has_changed;
        self.has_changed = false;
        changed
    }

    pub fn can_continue(&self) -> bool {
        self.can_continue
    }

    pub fn get_topics(&self) -> &[String] {
        self.state.as_slice()
    }

    pub fn get_chosen_topic(&self) -> Option<&String> {
        if let Some((ref topic, _index)) = self.chosen_topic {
            Some(topic)
        } else {
            None
        }
    }

    pub fn exit(&mut self) -> CommandResult {
        self.can_continue = false;
        CommandResult::Success
    }

    pub fn remove_topics(&mut self, args: &[String]) -> CommandResult {
        if args.is_empty() {
            return CommandResult::Fail("Missing arguments: indices".to_string());
        }
        let mut indices: Vec<usize> = vec![0; args.len()];
        for (i, str_index) in args.iter().enumerate() {
            if let Ok(index) = str_index.parse::<usize>() {
                if index < self.state.len() + 1 {
                    indices[i] = index - 1;
                    continue;
                }
            }
            return CommandResult::Fail(format!("Wrong argument: {}", args[i]));
        }
        let mut to_remove: Vec<bool> = vec![false; self.state.len()];
        let mut new_topics: Vec<String> = Vec::new();
        for i in indices {
            to_remove[i] = true;
        }
        for (i, topic) in self.state.iter().enumerate() {
            if !to_remove[i] {
                new_topics.push(topic.clone());
            }
        }
        self.state.clone_from(&new_topics);
        self.topic_history.add_new_node(new_topics);
        self.has_changed = true;
        CommandResult::Success
    }

    pub fn add_topics(&mut self, args: &[String]) -> CommandResult {
        if args.is_empty() {
            return CommandResult::Fail("Missing arguments: topics".to_string());
        }
        for topic in args {
            self.state.push(topic.clone());
        }
        self.topic_history.add_new_node(self.state.clone());
        self.has_changed = true;
        CommandResult::Success
    }

    pub fn pick_random(&mut self) -> CommandResult {
        if self.state.is_empty() {
            return CommandResult::Fail("Not enough topics".to_string());
        }
        let index: usize = self.rng.gen_range(1..=self.state.len());
        self.chosen_topic = Some((self.state[index - 1].clone(), index - 1));
        CommandResult::Success
    }

    pub fn remove_chosen_topic(&mut self) -> CommandResult {
        if let Some((_topic, index)) = &self.chosen_topic {
            if *index >= self.state.len() {
                return CommandResult::Fail(format!("Wrong index: {}", index));
            }
            self.state.remove(*index);
            self.topic_history.add_new_node(self.state.clone());
            self.has_changed = true;
            CommandResult::Success
        } else {
            CommandResult::Fail("No topic has been chosen".to_string())
        }
    }

    pub fn undo(&mut self) -> CommandResult {
        if let Some(state) = self.topic_history.get_previous() {
            self.state.clone_from(state);
            self.topic_history.move_to_previous();
            self.has_changed = true;
            CommandResult::Success
        } else {
            CommandResult::Fail("Already at the oldest change".to_string())
        }
    }

    pub fn redo(&mut self) -> CommandResult {
        if let Some(state) = self.topic_history.get_next() {
            self.state.clone_from(state);
            self.topic_history.move_to_next();
            self.has_changed = true;
            CommandResult::Success
        } else {
            CommandResult::Fail("Already at the newest change".to_string())
        }
    }
}
