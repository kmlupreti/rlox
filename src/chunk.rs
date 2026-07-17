use crate::opcode::Opcode;

pub type Value = u64;

#[derive(Default)]
pub struct Chunk {
    bytecodes: Vec<u8>,
    constants: Vec<Value>,
    lines: Vec<usize>,
}
impl Chunk {
    pub fn new() -> Self {
        Self::default()
    }
    pub fn write_byte<T>(&mut self, byte: T, line: usize)
    where
        T: Into<u8>,
    {
        self.bytecodes.push(byte.into());
        self.lines.push(line);
    }
    pub fn read_byte(&self, byte_index: usize) -> u8 {
        self.bytecodes[byte_index]
    }
    pub fn add_constant(&mut self, constant: Value) {
        self.constants.push(constant);
    }
    pub fn read_constant(&self, constant_index: usize) -> Value {
        self.constants[constant_index]
    }
    pub fn disassemble(&self, name: char) {
        println!("== {} ==\n", name);
        let mut offset = 0;
        while offset < self.bytecodes.len() {
            offset = self.disassemble_instruction(offset);
        }
    }
    fn disassemble_instruction(&self, offset: usize) -> usize {
        if offset > 0 && self.lines[offset] == self.lines[offset - 1] {
            print!("   | ");
        } else {
            print!("{:04} ", self.lines[offset]);
        }
        print!("{:04} ", offset);
        let opcode = Opcode::from(self.bytecodes[offset]);
        match opcode {
            Opcode::OpReturn => self.simple_instruction(&opcode, offset),
            Opcode::OpConstant => self.constant_instruction(&opcode, offset),
            _ => {
                println!("unknown instruction: {:?}", opcode);
                offset + 1
            }
        }
    }
    fn simple_instruction(&self, opcode: &Opcode, offset: usize) -> usize {
        println!("{:?}", opcode);
        offset + 1
    }
    fn constant_instruction(&self, opcode: &Opcode, offset: usize) -> usize {
        let constant_index = self.bytecodes[offset + 1];
        print!("{:<16?} {:>4}", opcode, constant_index);
        println!(" '{}'", self.constants[constant_index as usize]);
        offset + 2
    }
}
