pub struct UndoRedoHandler<T> {
    list: Vec<T>,
    current_index: usize,
    end_index: usize,
    at_end: bool,
}

impl<T> UndoRedoHandler<T> {
    pub fn new() -> Self {
        UndoRedoHandler {
            list: Vec::new(),
            current_index: 0,
            end_index: 0,
            at_end: true,
        }
    }

    pub fn add_new_node(&mut self, data: T) {
        if !self.is_head() {
            self.current_index += 1;
        }
        if self.current_index >= self.list.len() {
            self.list.push(data);
        } else {
            self.list[self.current_index] = data;
        }
        self.end_index = self.current_index;
        self.at_end = false;
    }

    pub fn is_tail(&self) -> bool {
        self.current_index == 0 && self.at_end
    }

    pub fn is_head(&self) -> bool {
        self.current_index >= self.end_index && self.at_end
    }

    pub fn get_current(&self) -> Option<&T> {
        if self.at_end {
            return None;
        }
        Some(&self.list[self.current_index])
    }

    pub fn get_previous(&self) -> Option<&T> {
        if self.current_index == 0 {
            return None;
        }
        Some(&self.list[self.current_index - 1])
    }

    pub fn get_next(&self) -> Option<&T> {
        if self.current_index == self.end_index {
            return None;
        }
        Some(&self.list[self.current_index + 1])
    }

    pub fn move_to_previous(&mut self) -> bool {
        if self.current_index > 0 {
            if !self.at_end {
                self.current_index -= 1;
            }
            self.at_end = false;
            return true;
        }
        if self.at_end {
            return false;
        }
        self.at_end = true;
        true
    }

    pub fn move_to_next(&mut self) -> bool {
        if self.current_index < self.end_index {
            if !self.at_end {
                self.current_index += 1;
            }
            self.at_end = false;
            return true;
        }
        if self.at_end {
            return false;
        }
        self.at_end = true;
        true
    }

    pub fn remove_node(&mut self, go_to_previous: bool) -> Result<(), &'static str> {
        if self.is_head() {
            return Err("Invalid operation: trying to remove head.");
        }
        if self.is_tail() {
            return Err("Invalid operation: trying to remove tail.");
        }
        self.list.remove(self.current_index);
        self.end_index -= 1;
        if self.current_index > self.end_index {
            self.current_index = self.end_index;
            self.at_end = true;
        }
        if go_to_previous && !self.move_to_previous() {
            return Err("Unexpected error occured.");
        }
        Ok(())
    }
}
