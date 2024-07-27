use lexer::symbolizer::{self, Symbolizer};
use reader::Reader;

use crate::tokenizer::Tokenizer;

pub mod lexer;
pub mod reader;
pub mod token;
pub mod tokenizer;

fn main() {
    let mut reader = Reader::new("hello.txt");

    reader.consume_whitespace();
    let tokenizer = Tokenizer::new(reader);

    let mut symbolizer = Symbolizer::new(tokenizer);

    symbolizer.symbolize();

    println!("{:?}", symbolizer.symbol_table);
}
