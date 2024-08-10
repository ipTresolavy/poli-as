use lexer::symbolizer::Symbolizer;
use reader::Reader;
use utils::debug_u32;

use crate::tokenizer::Tokenizer;

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

    let reader = Reader::new("hello.txt");

    let tokenizer = Tokenizer::new(reader);

    let mut lexer = lexer::Lexer::new(tokenizer, symbolizer);

    let program = lexer.parse();

    for op in program {
        let res = op.to_machine_code();

        println!("{:?}", res.to_debug_string());
    }

    // let mut cpu = emulator::cpu::Cpu::new();
    //
    // cpu.load_program(program);
    // cpu.run();
}
