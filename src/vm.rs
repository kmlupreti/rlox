use crate::{
    chunk::{ByteResult, Chunk, Value, ValueResult},
    error::LoxResult,
    opcode::Opcode,
};

#[derive(Default)]
pub struct VM {
    chunk: Chunk,
    ip: usize,
    stack: Vec<Value>,
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
    fn read_current_byte(&mut self) -> ByteResult {
        let byte = self.chunk.read_byte(self.ip)?;
        self.ip += 1;
        Ok(byte)
    }
    fn read_current_constant(&mut self) -> ValueResult {
        let constant_index = self.read_current_byte()? as usize;
        self.chunk.read_constant(constant_index)
    }
    pub fn run(&mut self) -> LoxResult<()> {
        loop {
            let instruction = Opcode::try_from(self.read_current_byte()?)?;
            match instruction {
                Opcode::OpConstant => {
                    let constant = self.read_current_constant()?;
                    self.stack.push(constant);
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
                    if let Some(value) = self.stack.pop() {
                        self.stack.push(-value);
                    }
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
                Opcode::OpReturn => {
                    if let Some(value) = self.stack.pop() {
                        println!("{}", value);
                    }
                    break;
                }
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
