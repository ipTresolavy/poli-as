use crate::{
    lexer::{symbolizer::SymbolTable, Lexer},
    token::{Directive, Token},
    tokenizer::Tokenizer,
};

pub enum Section {
    Text,
    Data,
    Bss,
}

pub struct Assembler {
    pub symbol_table: SymbolTable,
    pub tokenizer: Tokenizer,
    lexer: Lexer,
    current_section: Section,
}

impl Assembler {
    pub fn new(tokenizer: Tokenizer, symbol: SymbolTable) -> Self {
        let lexer = Lexer::new(symbol.clone());
        Assembler {
            lexer,
            symbol_table: symbol,
            tokenizer,
            current_section: Section::Text,
        }
    }

    pub fn assemble(&mut self) {
        while !self.tokenizer.is_eof() {
            let line = self.tokenizer.consume_line();
            self.parse_line(line);
        }
    }

    fn parse_line(&mut self, line: Vec<Token>) {
        if line.is_empty() {
            return;
        }

        if let Token::DIRECTIVE(directive) = &line[0] {
            if is_section_directive(&directive.value) {
                self.change_section(directive);
            }
        }
        if has_instruction(&line) {
            let op = self.lexer.parse_line(line).unwrap();

            println!("{:?}", op.to_machine_code().to_debug_string());
            op.to_machine_code().to_u8_buff();
        } else if has_word_directive(&line) {
            self.parse_word_directive(&line);
        } else {
        }
    }

    fn parse_word_directive(&mut self, line: &[Token]) -> Vec<u8> {
        let words = line
            .iter()
            .filter(|token| token.is_number())
            .map(|token| token.extract_number().unwrap().value)
            .collect::<Vec<u32>>();

        // Collect them in a u8 buffer
        let mut buffer = vec![];
        for word in words {
            buffer.push((word >> 24) as u8);
            buffer.push((word >> 16) as u8);
            buffer.push((word >> 8) as u8);
            buffer.push(word as u8);
        }

        self.lexer.increment_addr((buffer.len() * 4) as u32);

        buffer
    }

    fn change_section(&mut self, directive: &Directive) {
        let Directive { value } = directive;

        match value.as_str() {
            ".text" => self.current_section = Section::Text,
            ".data" => self.current_section = Section::Data,
            ".bss" => self.current_section = Section::Bss,
            _ => panic!("Unknown section directive"),
        };
    }
}

fn is_section_directive(token: &str) -> bool {
    token == ".text" || token == ".data" || token == ".bss"
}

fn has_instruction(line: &[Token]) -> bool {
    line.iter().any(|token| token.is_instruction())
}

fn has_word_directive(line: &[Token]) -> bool {
    line.iter().any(|token| {
        token
            .extract_directive()
            .map_or(false, |d| d.value == ".word")
    })
}
