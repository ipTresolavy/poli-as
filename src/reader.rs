use std::{fs::File, os::unix::fs::FileExt};

pub struct Reader {
    file: File,
    position: u64,
}

impl Reader {
    pub fn new(path: &str) -> Reader {
        let file = File::open(path).expect("File not found");

        Reader { file, position: 0 }
    }

    pub fn is_eof(&self) -> bool {
        self.peek_char().is_none()
    }

    pub fn consume_whitespace(&mut self) {
        while let Some(c) = self.peek_char() {
            if !self.is_whitespace(c) {
                break;
            }
            self.next_char();
        }
    }

    pub fn consume_line(&mut self) -> String {
        let mut acc = String::new();

        while let Some(c) = self.peek_char() {
            if c == '\n' {
                self.next_char();
                break;
            }
            acc.push(c);
            self.next_char();
        }

        acc
    }

    // We may need to change this in the future
    fn is_whitespace(&mut self, c: char) -> bool {
        c.is_whitespace() || c == ','
    }

    fn next_char(&mut self) -> Option<char> {
        let curr_char = self.read_at_position(self.position);
        self.position += 1;

        curr_char
    }

    pub fn consume_to_whitespace(&mut self) -> Option<String> {
        let mut result = String::new();

        while let Some(c) = self.peek_char() {
            if self.is_whitespace(c) {
                break;
            }

            result.push(c);

            if self.is_separator_char(self.peek_next_char()) {
                self.position += 1;
                break;
            }

            self.next_char();
        }

        if result.is_empty() {
            None
        } else {
            Some(result)
        }
    }

    fn is_separator_char(&self, c: Option<char>) -> bool {
        c.is_none() || c.unwrap() == '-' || c.unwrap() == ','
    }

    pub fn peek_char(&self) -> Option<char> {
        self.read_at_position(self.position)
    }

    fn peek_next_char(&self) -> Option<char> {
        self.read_at_position(self.position + 1)
    }

    fn read_at_position(&self, position: u64) -> Option<char> {
        let mut buf = [0; 1];
        let result = self.file.read_at(&mut buf, position);
        result.ok()?;

        match buf[0] {
            0 => None,
            _ => Some(buf[0] as char),
        }
    }
}
