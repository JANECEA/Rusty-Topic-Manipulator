pub struct ParsedCommand {
    command: String,
    args: Vec<String>,
}

impl ParsedCommand {
    pub fn parse_from_args(args: &[String]) -> Self {
        let (command, args): (String, Vec<String>) = if args.is_empty() {
            (String::new(), Vec::new())
        } else if args.len() == 1 {
            (args[0].to_string(), Vec::new())
        } else {
            (args[0].to_string(), args[1..].to_vec())
        };
        Self { command, args }
    }

    pub fn parse_from_line(line: &str) -> Self {
        let index: usize;
        if let Some(i) = line.find(' ') {
            index = i
        } else {
            return ParsedCommand {
                command: line.to_string(),
                args: vec![],
            };
        }
        let (command, rest) = line.split_at(index);
        let mut args_list = vec![];
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

    pub fn get_command(&self) -> &String {
        &self.command
    }

    pub fn get_args(&self) -> &Vec<String> {
        &self.args
    }
}
