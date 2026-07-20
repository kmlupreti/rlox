use crate::{
    error::{LoxError, LoxResult},
    opcode::Opcode,
};

pub type Value = f64;
pub type ByteResult = LoxResult<u8>;
pub type ValueResult = LoxResult<Value>;

#[derive(Default, Clone)]
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
    pub fn read_byte(&self, byte_index: usize) -> ByteResult {
        if self.bytecodes.is_empty() {
            Err(LoxError::VMError {
                msg: String::from("no bytecode in the chunk"),
            })
        } else {
            if byte_index < self.bytecodes.len() {
                Ok(self.bytecodes[byte_index])
            } else {
                Err(LoxError::VMError {
                    msg: format!(
                        "byte index {byte_index} is invalid as length of bytecodes is {}",
                        self.bytecodes.len()
                    ),
                })
            }
        }
    }
    pub fn add_constant(&mut self, constant: Value) -> usize {
        self.constants.push(constant);
        self.constants.len() - 1
    }
    pub fn read_constant(&self, constant_index: usize) -> ValueResult {
        if self.constants.is_empty() {
            Err(LoxError::VMError {
                msg: String::from("no constants in the chunk"),
            })
        } else {
            Ok(self.constants[constant_index])
        }
    }
    pub fn disassemble(&self, name: char) -> LoxResult<()> {
        println!("== {} ==\n", name);
        let mut offset = 0;
        while offset < self.bytecodes.len() {
            offset = self.disassemble_instruction(offset)?;
        }
        Ok(())
    }
    pub fn disassemble_instruction(&self, offset: usize) -> LoxResult<usize> {
        if offset > 0 && self.lines[offset] == self.lines[offset - 1] {
            print!("   | ");
        } else {
            print!("{:04} ", self.lines[offset]);
        }
        print!("{:04} ", offset);
        let opcode = Opcode::try_from(self.bytecodes[offset])?;
        match opcode {
            Opcode::Constant => self.constant_instruction(&opcode, offset),
            _ => self.simple_instruction(&opcode, offset),
        }
    }
    fn simple_instruction(&self, opcode: &Opcode, offset: usize) -> LoxResult<usize> {
        println!("{:?}", opcode);
        Ok(offset + 1)
    }
    fn constant_instruction(&self, opcode: &Opcode, offset: usize) -> LoxResult<usize> {
        let constant_index = self.read_byte(offset + 1)? as usize;
        print!("{:<16?} {:>4}", opcode, constant_index);
        println!(" '{}'", self.read_constant(constant_index)?);
        Ok(offset + 2)
    }
}
