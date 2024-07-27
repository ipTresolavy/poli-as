use crate::tokenizer::Tokenizer;

pub mod expression;
pub mod machine_code_builder;
pub mod symbolizer;

pub struct Lexer {
    tokenizer: Tokenizer,
}

impl Lexer {
    pub fn new(tokenizer: Tokenizer) -> Self {
        Lexer { tokenizer }
    }

    pub fn parse(&mut self) {
        let tokens = self.tokenizer.consume_line();
        for token in tokens {
            println!("{:?}", token);
        }
    }
}
