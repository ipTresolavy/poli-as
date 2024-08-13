use std::collections::HashMap;

use crate::tokenizer::Tokenizer;

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

        for token in &tokens {
            if let Token::LABEL(label) = token {
                let symbol = Symbol::new(label.value.clone());
                let address = Address::new(self.addr);
                self.add_symbol(symbol, address);
            }
        }

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

        let row = TableRow { address };

        self.symbol_table.0.insert(symbol, row);
    }
}
