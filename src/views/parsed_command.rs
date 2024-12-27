pub struct ParsedCommand {
    command: String,
    args: Vec<String>,
}

impl ParsedCommand {
    pub fn parse_from_args(args: &[String]) -> Self {
        let (command, args): (String, Vec<String>) = match args.len() {
            0 => (String::new(), Vec::new()),
            1 => (args[0].to_string(), Vec::new()),
            _ => (args[0].to_string(), args[1..].to_vec()),
        };
        Self { command, args }
    }

    pub fn parse_from_line(mut line: &str) -> Self {
        line = line.trim();
        let index: usize;
        if let Some(i) = line.find(' ') {
            index = i
        } else {
            return ParsedCommand {
                command: line.to_string(),
                args: Vec::new(),
            };
        }
        let (command, rest) = line.split_at(index);
        let mut args_list = Vec::new();
        let mut current_word = String::new();
        let mut in_quotes = false;

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

    pub fn is_empty(&self) -> bool {
        self.command.is_empty()
    }

    pub fn command(&self) -> &String {
        &self.command
    }

    pub fn args(&self) -> &Vec<String> {
        &self.args
    }
}
