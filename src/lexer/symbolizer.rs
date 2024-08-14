use std::collections::HashMap;

use crate::{assembler::Section, token::Directive, tokenizer::Tokenizer};

#[derive(Debug, Clone)]
pub enum Scope {
    Global,
    Local,
}

#[derive(Hash, Eq, PartialEq, Debug, Clone)]
pub struct Symbol {
    pub name: String,
}

impl Symbol {
    pub fn new(name: String) -> Self {
        Symbol { name }
    }
}

#[derive(Debug, Clone)]
pub struct TableRow {
    pub address: Address,
    pub scope: Scope,
    pub section: Section,
}

#[derive(Debug, Clone)]
pub struct Address {
    pub value: u32,
}

impl Address {
    pub fn new(value: u32) -> Self {
        Address { value }
    }
}

#[derive(Debug, Clone)]
pub struct SymbolTable(HashMap<Symbol, TableRow>);

impl SymbolTable {
    pub fn new() -> Self {
        SymbolTable(HashMap::new())
    }

    pub fn get_address(&self, symbol: &str) -> Option<&Address> {
        let symbol = Symbol::new(symbol.to_string());

        self.0.get(&symbol).map(|row| &row.address)
    }

    pub fn iter(&self) -> impl Iterator<Item = (&Symbol, &TableRow)> {
        self.0.iter()
    }
}

impl Default for SymbolTable {
    fn default() -> Self {
        Self::new()
    }
}

pub struct Symbolizer {
    pub symbol_table: SymbolTable,
    tokenizer: Tokenizer,
    addr: u32,
    current_scope: Scope,
    current_section: Section,
}

impl Symbolizer {
    pub fn new(tokenizer: Tokenizer) -> Self {
        Symbolizer {
            symbol_table: SymbolTable(HashMap::new()),
            tokenizer,
            addr: 0,
            current_section: Section::Text,
            current_scope: Scope::Local,
        }
    }

    pub fn symbolize(&mut self) {
        while !self.tokenizer.is_eof() {
            self.symbolize_line();
        }
    }

    fn symbolize_line(&mut self) {
        let tokens = self.tokenizer.consume_line();
        use crate::token::Token;

        for token in &tokens {
            if let Token::DIRECTIVE(label) = token {
                if label.value == ".global" || label.value == "._global" {
                    self.current_scope = Scope::Global;
                } else if label.value == ".text" || label.value == ".data" || label.value == ".bss"
                {
                    self.change_section(label);
                    self.current_scope = Scope::Local;
                } else if label.value == ".word" {
                    self.addr += (4 * (tokens.len() - 1)) as u32;
                }
            }
            if let Token::LABEL(label) = token {
                let symbol = Symbol::new(label.value.clone());
                let address = Address::new(self.addr);
                self.add_symbol(symbol, address);
            }
        }

        self.current_scope = Scope::Local;

        if tokens
            .iter()
            .any(|token| matches!(token, Token::INSTRUCTION(_)))
        {
            self.addr += 4;
        }
    }

    fn add_symbol(&mut self, symbol: Symbol, address: Address) {
        if self.symbol_table.0.contains_key(&symbol) {
            panic!("Symbol already exists in table");
        }

        let row = TableRow {
            address,
            scope: self.current_scope.clone(),
            section: self.current_section.clone(),
        };

        self.symbol_table.0.insert(symbol, row);
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
