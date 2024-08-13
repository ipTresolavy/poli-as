use lexer::symbolizer::Symbolizer;
use reader::Reader;

use crate::tokenizer::Tokenizer;

pub mod assembler;
pub mod elf;
pub mod emulator;
pub mod lexer;
pub mod reader;
pub mod token;
pub mod tokenizer;
pub mod utils;

fn main() {
    let reader = Reader::new("hello.txt");

    let tokenizer = Tokenizer::new(reader);

    let mut symbolizer = Symbolizer::new(tokenizer);

    symbolizer.symbolize();

    println!("{:?}", symbolizer.symbol_table);

    let reader = Reader::new("hello.txt");

    let tokenizer = Tokenizer::new(reader);

    let mut assembler = assembler::Assembler::new(tokenizer, symbolizer.symbol_table);

    assembler.assemble();
}
