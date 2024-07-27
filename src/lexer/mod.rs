use crate::{
    token::{immediate::Immediate, Token},
    tokenizer::Tokenizer,
};

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
        while !self.tokenizer.is_eof() {
            self.parse_line();
        }
    }

    fn parse_line(&mut self) {
        let mut tokens = self.tokenizer.consume_line();

        if tokens.is_empty() {
            return;
        }

        // first replace any labels with their addresses
        replace_label_ref(&mut tokens, &self.symbol_table);

        for token in tokens {
            println!("{:?}", token);
        }
    }
}

fn replace_label_ref(tokens: &mut [Token], symbol_table: &Symbolizer) {
    for token in tokens.iter_mut() {
        if let Token::LABELREF(label) = token {
            let address = symbol_table.get_address(label);
            if address.is_none() {
                panic!("Label {} not found in symbol table", label);
            }

            println!("#{}", &address.unwrap().value.to_string());
            let immediate = Immediate::new(address.unwrap().value.to_string()).unwrap();

            println!("Replacing label {} with immediate {:?}", label, immediate);

            *token = Token::IMMEDIATE(immediate);
        }
    }
}
