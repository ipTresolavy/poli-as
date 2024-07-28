use lexer::symbolizer::Symbolizer;
use reader::Reader;

use crate::tokenizer::Tokenizer;

pub mod emulator;
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

    let reader = Reader::new("hello.txt");

    let tokenizer = Tokenizer::new(reader);

    let mut lexer = lexer::Lexer::new(tokenizer, symbolizer);

    let program = lexer.parse();

    let mut cpu = emulator::cpu::Cpu::new();

    cpu.load_program(program);
    cpu.run();
}
