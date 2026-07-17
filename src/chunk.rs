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
        let instruction = Opcode::from(self.code[offset]);
        match instruction {
            Opcode::OpReturn => self.simple_instruction(&instruction, offset),
            _ => {
                println!("unknown instruction: {:?}", instruction);
                offset + 1
            }
        }
    }
    fn simple_instruction(&self, instruction: &Opcode, offset: usize) -> usize {
        println!("{:?}", instruction);
        offset + 1
    }
}
