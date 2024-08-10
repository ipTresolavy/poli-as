use object::elf::{EM_ARM, ET_REL};
use object::write::elf::Writer;
use object::write::elf::{FileHeader, SectionHeader};
use object::write::{Object, StreamingBuffer, Symbol, SymbolSection};
use object::{Architecture, Endianness};
use std::collections::HashMap;
use std::fs::File;
use std::io::{self, BufWriter, Write};

pub struct ElfWriter<'a> {
    object: Object<'a>,
    writer: Writer<'a>,
    file_name: String,
}

impl<'a> ElfWriter<'a> {
    fn new(file_name: String) -> ElfWriter<'a> {
        let endianess = Endianness::Little;
        let object = Object::new(
            object::BinaryFormat::Elf,
            Architecture::Arm,
            endianess.clone(), // FIXME: really necessary?
        );
        let file = File::create(file_name.clone()).expect("Was not able to create output file");
        let buf_writer = BufWriter::new(file);
        let mut streaming_buffer = StreamingBuffer::new(buf_writer);

        let writer = Writer::new(endianess.clone(), false, &mut streaming_buffer);

        ElfWriter {
            object,
            writer,
            file_name,
        }
    }

    // Additional methods for ElfWriter can be added here
}

// fn main() {
//     let elf_writer = ElfWriter::new("output.elf".to_string());
//     // Additional logic using elf_writer
// }
