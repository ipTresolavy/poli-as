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
            let operation = self.lexer.parse_line(line);
        } else {
            println!("{:?}", &line);
        }
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
