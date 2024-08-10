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
    file_name: String,
    streaming_buffer: StreamingBuffer<BufWriter<File>>,
}

impl<'a> ElfWriter<'a> {
    fn new(file_name: String) -> ElfWriter<'a> {
        let endianess = Endianness::Little;
        let object = Object::new(object::BinaryFormat::Elf, Architecture::Arm, endianess);
        let file = File::create(file_name.clone()).expect("Was not able to create output file");
        let buf_writer = BufWriter::new(file);
        let streaming_buffer = StreamingBuffer::new(buf_writer);

        ElfWriter {
            object,
            file_name,
            streaming_buffer,
        }
    }

    fn write(&mut self) -> Writer {
        Writer::new(Endianness::Little, false, &mut self.streaming_buffer)
    }
}
