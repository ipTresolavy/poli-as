use crate::{token::Token, tokenizer::Tokenizer};

use self::symbolizer::Symbolizer;

pub mod expression;
pub mod machine_code_builder;
pub mod symbolizer;

pub struct Lexer {
    tokenizer: Tokenizer,
    symbol_table: Symbolizer,
}

impl Lexer {
    pub fn new(tokenizer: Tokenizer, symbol_table: Symbolizer) -> Self {
        Lexer {
            tokenizer,
            symbol_table,
        }
    }

    pub fn parse(&mut self) {
        let tokens = self.tokenizer.consume_line();
        for token in tokens {
            if let Token::LABELREF(label) = token {
                self.symbol_table.get_address(&label);
            }
        }
    }
}
