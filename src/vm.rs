use crate::{
    chunk::{Chunk, Value},
    error::LoxResult,
    opcode::Opcode,
};

#[derive(Default)]
pub struct VM {
    chunk: Chunk,
    ip: usize,
}
impl VM {
    pub fn new() -> Self {
        Self::default()
    }
    pub fn interpret(&mut self, chunk: Chunk) -> LoxResult<()> {
        self.chunk = chunk;
        self.ip = 0;
        self.run()
    }
    fn read_current_byte(&mut self) -> u8 {
        let byte = self.chunk.read_byte(self.ip);
        self.ip += 1;
        byte
    }
    fn read_current_constant(&mut self) -> Value {
        let constant_index = self.read_current_byte() as usize;
        self.chunk.read_constant(constant_index)
    }
    pub fn run(&mut self) -> LoxResult<()> {
        loop {
            let instruction = Opcode::from(self.read_current_byte());
            match instruction {
                Opcode::OpConstant => {
                    println!("{}", self.read_current_constant())
                }
                Opcode::OpNil => {
                    todo!()
                }
                Opcode::OpTrue => {
                    todo!()
                }
                Opcode::OpFalse => {
                    todo!()
                }
                Opcode::OpPop => {
                    todo!()
                }
                Opcode::OpGetLocal => {
                    todo!()
                }
                Opcode::OpSetLocal => {
                    todo!()
                }
                Opcode::OpGetGlobal => {
                    todo!()
                }
                Opcode::OpDefineGlobal => {
                    todo!()
                }
                Opcode::OpSetGlobal => {
                    todo!()
                }
                Opcode::OpGetUpvalue => {
                    todo!()
                }
                Opcode::OpSetUpvalue => {
                    todo!()
                }
                Opcode::OpGetProperty => {
                    todo!()
                }
                Opcode::OpSetProperty => {
                    todo!()
                }
                Opcode::OpGetSuper => {
                    todo!()
                }
                Opcode::OpEqual => {
                    todo!()
                }
                Opcode::OpGreater => {
                    todo!()
                }
                Opcode::OpLess => {
                    todo!()
                }
                Opcode::OpAdd => {
                    todo!()
                }
                Opcode::OpSubtract => {
                    todo!()
                }
                Opcode::OpMultiply => {
                    todo!()
                }
                Opcode::OpDivide => {
                    todo!()
                }
                Opcode::OpNot => {
                    todo!()
                }
                Opcode::OpNegate => {
                    todo!()
                }
                Opcode::OpPrint => {
                    todo!()
                }
                Opcode::OpJump => {
                    todo!()
                }
                Opcode::OpJumpIfFalse => {
                    todo!()
                }
                Opcode::OpLoop => {
                    todo!()
                }
                Opcode::OpCall => {
                    todo!()
                }
                Opcode::OpInvoke => {
                    todo!()
                }
                Opcode::OpSuperInvoke => {
                    todo!()
                }
                Opcode::OpClosure => {
                    todo!()
                }
                Opcode::OpCloseUpValue => {
                    todo!()
                }
                Opcode::OpReturn => break,
                Opcode::OpClass => {
                    todo!()
                }
                Opcode::OpInherit => {
                    todo!()
                }
                Opcode::OpMethod => {
                    todo!()
                }
            }
        }
        Ok(())
    }
}
