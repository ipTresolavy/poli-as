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

use clap::{Arg, Command};

fn main() {
    let matches = Command::new("poli-as")
        .version("1.0")
        .author("Thiago Souza e Igor Pontes Tresolavy")
        .about("Assembler para o armv7")
        .arg(
            Arg::new("input")
                .short('i')
                .long("input")
                .value_name("FILE"),
        )
        .arg(
            Arg::new("output")
                .short('o')
                .long("output")
                .value_name("FILE"),
        )
        .get_matches();

    let output_file_name = matches.get_one::<String>("output");
    // Accessing values
    if let Some(input) = matches.get_one::<String>("input") {
        let reader = Reader::new(input);

        let tokenizer = Tokenizer::new(reader);

        let mut symbolizer = Symbolizer::new(tokenizer);

        symbolizer.symbolize();

        let reader = Reader::new("hello.txt");

        let tokenizer = Tokenizer::new(reader);

        let mut assembler = assembler::Assembler::new(tokenizer, symbolizer.symbol_table);

        assembler.assemble(output_file_name);
    } else {
        println!("No input file provided");
    }
}
