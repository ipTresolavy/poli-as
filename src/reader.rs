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

    pub fn reset(&mut self) {
        self.position = 0;
    }

    pub fn is_eof(&self) -> bool {
        self.peek_char().is_none()
    }

    fn consume_whitespace(&mut self) {
        while let Some(c) = self.peek_char() {
            if !self.is_whitespace(c) {
                break;
            }
            self.next_char();
        }
    }

    pub fn consume_line(&mut self) -> String {
        self.consume_whitespace();

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

    pub fn peek_char(&self) -> Option<char> {
        self.read_at_position(self.position)
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
