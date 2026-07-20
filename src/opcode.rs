use crate::error::LoxError;

#[repr(u8)]
#[derive(Debug, Clone, Copy)]
pub enum Opcode {
    //> op-constant
    Constant,
    //< op-constant
    //> Types of Values literal-ops
    Nil,
    True,
    False,
    //< Types of Values literal-ops
    //> Global Variables pop-op
    Pop,
    //< Global Variables pop-op
    //> Local Variables get-local-op
    GetLocal,
    //< Local Variables get-local-op
    //> Local Variables set-local-op
    SetLocal,
    //< Local Variables set-local-op
    //> Global Variables get-global-op
    GetGlobal,
    //< Global Variables get-global-op
    //> Global Variables define-global-op
    DefineGlobal,
    //< Global Variables define-global-op
    //> Global Variables set-global-op
    SetGlobal,
    //< Global Variables set-global-op
    //> Closures upvalue-ops
    GetUpvalue,
    SetUpvalue,
    //< Closures upvalue-ops
    //> Classes and Instances property-ops
    GetProperty,
    SetProperty,
    //< Classes and Instances property-ops
    //> Superclasses get-super-op
    GetSuper,
    //< Superclasses get-super-op
    //> Types of Values comparison-ops
    Equal,
    Greater,
    Less,
    //< Types of Values comparison-ops
    //> A Virtual Machine binary-ops
    Add,
    Subtract,
    Multiply,
    Divide,
    //> Types of Values not-op
    Not,
    //< Types of Values not-op
    //< A Virtual Machine binary-ops
    //> A Virtual Machine negate-op
    Negate,
    //< A Virtual Machine negate-op
    //> Global Variables op-print
    Print,
    //< Global Variables op-print
    //> Jumping Back and Forth jump-op
    Jump,
    //< Jumping Back and Forth jump-op
    //> Jumping Back and Forth jump-if-false-op
    JumpIfFalse,
    //< Jumping Back and Forth jump-if-false-op
    //> Jumping Back and Forth loop-op
    Loop,
    //< Jumping Back and Forth loop-op
    //> Calls and Functions op-call
    Call,
    //< Calls and Functions op-call
    //> Methods and Initializers invoke-op
    Invoke,
    //< Methods and Initializers invoke-op
    //> Superclasses super-invoke-op
    SuperInvoke,
    //< Superclasses super-invoke-op
    //> Closures closure-op
    Closure,
    //< Closures closure-op
    //> Closures close-upvalue-op
    CloseUpValue,
    //< Closures close-upvalue-op
    Return,
    //> Classes and Instances class-op
    Class,
    //< Classes and Instances class-op
    //> Superclasses inherit-op
    Inherit,
    //< Superclasses inherit-op
    //> Methods and Initializers method-op
    Method, //< Methods and Initializers method-op}
}
impl TryFrom<u8> for Opcode {
    type Error = LoxError;
    fn try_from(value: u8) -> Result<Self, Self::Error> {
        match value {
            0 => Ok(Self::Constant),
            1 => Ok(Self::Nil),
            2 => Ok(Self::True),
            3 => Ok(Self::False),
            4 => Ok(Self::Pop),
            5 => Ok(Self::GetLocal),
            6 => Ok(Self::SetLocal),
            7 => Ok(Self::GetGlobal),
            8 => Ok(Self::DefineGlobal),
            9 => Ok(Self::SetGlobal),
            10 => Ok(Self::GetUpvalue),
            11 => Ok(Self::SetUpvalue),
            12 => Ok(Self::GetProperty),
            13 => Ok(Self::SetProperty),
            14 => Ok(Self::GetSuper),
            15 => Ok(Self::Equal),
            16 => Ok(Self::Greater),
            17 => Ok(Self::Less),
            18 => Ok(Self::Add),
            19 => Ok(Self::Subtract),
            20 => Ok(Self::Multiply),
            21 => Ok(Self::Divide),
            22 => Ok(Self::Not),
            23 => Ok(Self::Negate),
            24 => Ok(Self::Print),
            25 => Ok(Self::Jump),
            26 => Ok(Self::JumpIfFalse),
            27 => Ok(Self::Loop),
            28 => Ok(Self::Call),
            29 => Ok(Self::Invoke),
            30 => Ok(Self::SuperInvoke),
            31 => Ok(Self::Closure),
            32 => Ok(Self::CloseUpValue),
            33 => Ok(Self::Return),
            34 => Ok(Self::Class),
            35 => Ok(Self::Inherit),
            36 => Ok(Self::Method),
            _ => Err(LoxError::VMError {
                msg: format!("uknown opcode found : {}", value),
            }),
        }
    }
}

impl From<Opcode> for u8 {
    fn from(value: Opcode) -> Self {
        value as u8
    }
}
