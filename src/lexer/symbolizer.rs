use std::collections::HashMap;

use crate::tokenizer::{self, Tokenizer};

#[derive(Hash, Eq, PartialEq, Debug)]
pub struct Symbol {
    pub name: String,
}

impl Symbol {
    pub fn new(name: String) -> Self {
        Symbol { name }
    }
}

#[derive(Debug)]
pub struct Address {
    pub value: u32,
}

impl Address {
    pub fn new(value: u32) -> Self {
        Address { value }
    }
}

#[derive(Debug)]
pub struct SymbolTable(HashMap<Symbol, Address>);

pub struct Symbolizer {
    pub symbol_table: SymbolTable,
    tokenizer: Tokenizer,
    addr: u32,
}

impl Symbolizer {
    pub fn new(tokenizer: Tokenizer) -> Self {
        Symbolizer {
            symbol_table: SymbolTable(HashMap::new()),
            tokenizer,
            addr: 0,
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

        for token in tokens {
            match token {
                Token::LABEL(label) => {
                    let symbol = Symbol::new(label.value);
                    let address = Address::new(self.addr);
                    self.add_symbol(symbol, address);
                    return;
                }
                Token::INSTRUCTION(_) => {
                    self.addr += 4;
                    return;
                }
                _ => {}
            }
        }
    }

    fn add_symbol(&mut self, symbol: Symbol, address: Address) {
        if self.symbol_table.0.contains_key(&symbol) {
            panic!("Symbol already exists in table");
        }

        self.symbol_table.0.insert(symbol, address);
    }
    fn get_address(&self, symbol: &Symbol) -> Option<&Address> {
        self.symbol_table.0.get(symbol)
    }
}
