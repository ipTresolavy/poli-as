use object::write::elf::Writer;
use object::write::{Object, StreamingBuffer};
use object::{Architecture, Endianness};
use std::fs::File;
use std::io::BufWriter;

pub struct ElfWriter<'a> {
    object: Object<'a>,
    streaming_buffer: StreamingBuffer<BufWriter<File>>,
}

impl<'a> ElfWriter<'a> {
    pub fn new(file_name: String) -> ElfWriter<'a> {
        let endianess = Endianness::Little;
        let object = Object::new(object::BinaryFormat::Elf, Architecture::Arm, endianess);
        let file = File::create(file_name).expect("Was not able to create output file");
        let buf_writer = BufWriter::new(file);
        let streaming_buffer = StreamingBuffer::new(buf_writer);

        ElfWriter {
            object,
            streaming_buffer,
        }
    }

    pub fn write(&mut self) -> Writer {
        Writer::new(Endianness::Little, false, &mut self.streaming_buffer)
    }
}
