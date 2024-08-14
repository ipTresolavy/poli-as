use std::collections::HashMap;

use object::elf::{STB_GLOBAL, STT_NOTYPE};

use crate::{
    elf::{
        elf_writer::ElfWriter,
        section_data::{self, SectionData},
    },
    lexer::{symbolizer::SymbolTable, Lexer},
    token::{Directive, Token},
    tokenizer::Tokenizer,
};

pub struct UnknownRefs {
    pub refs: Vec<String>,
}

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub enum Section {
    Text,
    Data,
    Bss,
}

impl Section {
    pub fn to_string(&self) -> String {
        match self {
            Section::Text => ".text".to_string(),
            Section::Data => ".data".to_string(),
            Section::Bss => ".bss".to_string(),
        }
    }
}

#[derive(Debug, Clone)]
pub struct SectionLookupTable(HashMap<String, usize>);

pub struct Assembler {
    pub symbol_table: SymbolTable,
    pub tokenizer: Tokenizer,
    lexer: Lexer,
    current_section: Section,
    elf_writer: ElfWriter,
    buffer: Vec<u8>,
    section_lookup_table: SectionLookupTable,
    unknown_refs: UnknownRefs,
}

impl Assembler {
    pub fn new(tokenizer: Tokenizer, symbol: SymbolTable) -> Self {
        let lexer = Lexer::new(symbol.clone());
        Assembler {
            lexer,
            symbol_table: symbol,
            tokenizer,
            current_section: Section::Text,
            elf_writer: ElfWriter::new(),
            buffer: vec![],
            section_lookup_table: SectionLookupTable(HashMap::new()),
            unknown_refs: UnknownRefs { refs: vec![] },
        }
    }

    fn clear_buffer(&mut self) {
        self.buffer.clear();
    }

    pub fn assemble(&mut self) {
        while !self.tokenizer.is_eof() {
            let line = self.tokenizer.consume_line();
            self.parse_line(line);
        }

        self.create_symbol_entry();
    }

    fn create_symbol_entry(&mut self) {
        let mut section_data = SectionData::Symbols(vec![]);
        for symbol in self.symbol_table.iter() {
            let section_id = self
                .section_lookup_table
                .clone()
                .0
                .get(&symbol.1.section.to_string())
                .unwrap()
                .to_owned();

            section_data.add_symbol(
                section_id.to_owned(),
                symbol.0.name.clone(),
                symbol.1.address.value,
                0,
                STT_NOTYPE,
                None,
            );
        }

        for unknown_ref in &self.unknown_refs.refs {
            section_data.add_symbol(
                0,
                unknown_ref.to_owned(),
                0,
                0,
                STB_GLOBAL << 4 | STT_NOTYPE,
                Some(0),
            );
        }
    }

    fn parse_line(&mut self, line: Vec<Token>) {
        if line.is_empty() {
            return;
        }

        if let Token::DIRECTIVE(directive) = &line[0] {
            if is_section_directive(&directive.value) {
                let section_data = section_data::SectionData::Bytes(self.buffer.clone());
                let id = self
                    .elf_writer
                    .add_section(self.current_section.to_string(), section_data);

                self.section_lookup_table
                    .0
                    .insert(self.current_section.to_string(), id);

                self.clear_buffer();

                self.change_section(directive);
            }
        }

        self.find_unknown_refs(&line);

        if has_instruction(&line) {
            let op = self.lexer.parse_line(line).unwrap();

            println!("{:?}", op.to_machine_code().to_debug_string());

            self.buffer.extend(&op.to_machine_code().to_u8_buff());
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

    fn find_unknown_refs(&mut self, tokens: &[Token]) {
        for token in tokens {
            if let Token::LABELREF(label) = token {
                let address = self.symbol_table.get_address(label);
                if address.is_none() {
                    self.unknown_refs.refs.push(label.clone());
                }
            }
        }
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
