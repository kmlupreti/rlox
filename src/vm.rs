use crate::{
    chunk::{ByteResult, Chunk, Value, ValueResult},
    error::{LoxError, LoxResult},
    opcode::Opcode,
};
use std::ops::Neg;

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
                Opcode::Constant => {
                    let constant = self.read_current_constant()?;
                    self.stack.push(constant);
                }
                Opcode::Nil => {
                    todo!()
                }
                Opcode::True => {
                    todo!()
                }
                Opcode::False => {
                    todo!()
                }
                Opcode::Pop => {
                    todo!()
                }
                Opcode::GetLocal => {
                    todo!()
                }
                Opcode::SetLocal => {
                    todo!()
                }
                Opcode::GetGlobal => {
                    todo!()
                }
                Opcode::DefineGlobal => {
                    todo!()
                }
                Opcode::SetGlobal => {
                    todo!()
                }
                Opcode::GetUpvalue => {
                    todo!()
                }
                Opcode::SetUpvalue => {
                    todo!()
                }
                Opcode::GetProperty => {
                    todo!()
                }
                Opcode::SetProperty => {
                    todo!()
                }
                Opcode::GetSuper => {
                    todo!()
                }
                Opcode::Equal => {
                    todo!()
                }
                Opcode::Greater => {
                    todo!()
                }
                Opcode::Less => {
                    todo!()
                }
                Opcode::Add => {
                    if let Some(b) = self.stack.pop()
                        && let Some(a) = self.stack.pop()
                    {
                        self.stack.push(a + b);
                    }
                }
                Opcode::Subtract => {
                    if let Some(b) = self.stack.pop()
                        && let Some(a) = self.stack.pop()
                    {
                        self.stack.push(a - b);
                    }
                }
                Opcode::Multiply => {
                    if let Some(b) = self.stack.pop()
                        && let Some(a) = self.stack.pop()
                    {
                        self.stack.push(a * b);
                    }
                }
                Opcode::Divide => {
                    if let Some(b) = self.stack.pop()
                        && let Some(a) = self.stack.pop()
                    {
                        self.stack.push(a / b);
                    }
                }
                Opcode::Not => {
                    todo!()
                }
                Opcode::Negate => {
                    if self.stack.is_empty() {
                        return Err(LoxError::VMError {
                            msg: String::from("failed to negate as stack is empty"),
                        });
                    } else {
                        *self.stack.last_mut().unwrap() = self.stack.last().unwrap().neg();
                    }
                }
                Opcode::Print => {
                    todo!()
                }
                Opcode::Jump => {
                    todo!()
                }
                Opcode::JumpIfFalse => {
                    todo!()
                }
                Opcode::Loop => {
                    todo!()
                }
                Opcode::Call => {
                    todo!()
                }
                Opcode::Invoke => {
                    todo!()
                }
                Opcode::SuperInvoke => {
                    todo!()
                }
                Opcode::Closure => {
                    todo!()
                }
                Opcode::CloseUpValue => {
                    todo!()
                }
                Opcode::Return => {
                    if let Some(value) = self.stack.pop() {
                        println!("{}", value);
                    }
                    break;
                }
                Opcode::Class => {
                    todo!()
                }
                Opcode::Inherit => {
                    todo!()
                }
                Opcode::Method => {
                    todo!()
                }
            }
        }
        Ok(())
    }
}
