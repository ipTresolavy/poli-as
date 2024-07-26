use reader::Reader;
use token::Token;

use crate::tokenizer::Tokenizer;

pub mod reader;
pub mod token;
pub mod tokenizer;

fn main() {
    let mut reader = Reader::new("hello.txt");

    reader.consume_whitespace();
    let mut tokenizer = Tokenizer::new(reader);

    for _ in 0..10 {
        let line_tokens = tokenizer.consume_line();
        println!("{:?}", line_tokens);
    }
}
