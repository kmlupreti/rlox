// #![allow(unused)]
use crate::opcode::Opcode;

type Value = u64;

#[derive(Default)]
pub struct Chunk {
    code: Vec<u8>,
    constants: Vec<Value>,
}
impl Chunk {
    pub fn new() -> Self {
        Self::default()
    }
    pub fn write<T>(&mut self, byte: T)
    where
        T: Into<u8>,
    {
        self.code.push(byte.into());
    }
    pub fn add_constant(&mut self, constant: Value) {
        self.constants.push(constant);
    }
    pub fn disassemble(&self, name: char) {
        println!("== {} ==\n", name);
        let mut offset = 0;
        while offset < self.code.len() {
            offset = self.disassemble_instruction(offset);
        }
    }
    fn disassemble_instruction(&self, offset: usize) -> usize {
        print!("{:04} ", offset);
        let opcode = Opcode::from(self.code[offset]);
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
        let constant_index = self.code[offset + 1];
        print!("{:<16?} {:>4} '", opcode, constant_index);
        println!("{}", self.constants[constant_index as usize]);
        offset + 2
    }
}
