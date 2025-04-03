/// Represents the state of the search popup
pub struct SearchState {
    active: bool,
    input: String,
    cursor_position: usize,
}

impl SearchState {
    pub fn new() -> Self {
        Self {
            active: false,
            input: String::new(),
            cursor_position: 0,
        }
    }

    pub fn activate(&mut self) {
        self.active = true;
        self.input.clear();
        self.cursor_position = 0;
    }

    pub fn deactivate(&mut self) -> Option<String> {
        self.active = false;
        if self.input.is_empty() {
            None
        } else {
            Some(self.input.clone())
        }
    }

    pub fn cancel(&mut self) {
        self.active = false;
        self.input.clear();
        self.cursor_position = 0;
    }

    pub fn input(&mut self, c: char) {
        self.input.insert(self.cursor_position, c);
        self.cursor_position += 1;
    }

    pub fn delete(&mut self) {
        if self.cursor_position > 0 {
            self.cursor_position -= 1;
            self.input.remove(self.cursor_position);
        }
    }

    pub fn is_active(&self) -> bool {
        self.active
    }

    pub fn get_input(&self) -> &str {
        &self.input
    }
}