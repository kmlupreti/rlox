use crate::error::LoxError;

#[repr(u8)]
#[derive(Debug, Clone, Copy)]
pub enum Opcode {
    //> op-constant
    OpConstant,
    //< op-constant
    //> Types of Values literal-ops
    OpNil,
    OpTrue,
    OpFalse,
    //< Types of Values literal-ops
    //> Global Variables pop-op
    OpPop,
    //< Global Variables pop-op
    //> Local Variables get-local-op
    OpGetLocal,
    //< Local Variables get-local-op
    //> Local Variables set-local-op
    OpSetLocal,
    //< Local Variables set-local-op
    //> Global Variables get-global-op
    OpGetGlobal,
    //< Global Variables get-global-op
    //> Global Variables define-global-op
    OpDefineGlobal,
    //< Global Variables define-global-op
    //> Global Variables set-global-op
    OpSetGlobal,
    //< Global Variables set-global-op
    //> Closures upvalue-ops
    OpGetUpvalue,
    OpSetUpvalue,
    //< Closures upvalue-ops
    //> Classes and Instances property-ops
    OpGetProperty,
    OpSetProperty,
    //< Classes and Instances property-ops
    //> Superclasses get-super-op
    OpGetSuper,
    //< Superclasses get-super-op
    //> Types of Values comparison-ops
    OpEqual,
    OpGreater,
    OpLess,
    //< Types of Values comparison-ops
    //> A Virtual Machine binary-ops
    OpAdd,
    OpSubtract,
    OpMultiply,
    OpDivide,
    //> Types of Values not-op
    OpNot,
    //< Types of Values not-op
    //< A Virtual Machine binary-ops
    //> A Virtual Machine negate-op
    OpNegate,
    //< A Virtual Machine negate-op
    //> Global Variables op-print
    OpPrint,
    //< Global Variables op-print
    //> Jumping Back and Forth jump-op
    OpJump,
    //< Jumping Back and Forth jump-op
    //> Jumping Back and Forth jump-if-false-op
    OpJumpIfFalse,
    //< Jumping Back and Forth jump-if-false-op
    //> Jumping Back and Forth loop-op
    OpLoop,
    //< Jumping Back and Forth loop-op
    //> Calls and Functions op-call
    OpCall,
    //< Calls and Functions op-call
    //> Methods and Initializers invoke-op
    OpInvoke,
    //< Methods and Initializers invoke-op
    //> Superclasses super-invoke-op
    OpSuperInvoke,
    //< Superclasses super-invoke-op
    //> Closures closure-op
    OpClosure,
    //< Closures closure-op
    //> Closures close-upvalue-op
    OpCloseUpValue,
    //< Closures close-upvalue-op
    OpReturn,
    //> Classes and Instances class-op
    OpClass,
    //< Classes and Instances class-op
    //> Superclasses inherit-op
    OpInherit,
    //< Superclasses inherit-op
    //> Methods and Initializers method-op
    OpMethod, //< Methods and Initializers method-op}
}
impl TryFrom<u8> for Opcode {
    type Error = LoxError;
    fn try_from(value: u8) -> Result<Self, Self::Error> {
        match value {
            0 => Ok(Self::OpConstant),
            1 => Ok(Self::OpNil),
            2 => Ok(Self::OpTrue),
            3 => Ok(Self::OpFalse),
            4 => Ok(Self::OpPop),
            5 => Ok(Self::OpGetLocal),
            6 => Ok(Self::OpSetLocal),
            7 => Ok(Self::OpGetGlobal),
            8 => Ok(Self::OpDefineGlobal),
            9 => Ok(Self::OpSetGlobal),
            10 => Ok(Self::OpGetUpvalue),
            11 => Ok(Self::OpSetUpvalue),
            12 => Ok(Self::OpGetProperty),
            13 => Ok(Self::OpSetProperty),
            14 => Ok(Self::OpGetSuper),
            15 => Ok(Self::OpEqual),
            16 => Ok(Self::OpGreater),
            17 => Ok(Self::OpLess),
            18 => Ok(Self::OpAdd),
            19 => Ok(Self::OpSubtract),
            20 => Ok(Self::OpMultiply),
            21 => Ok(Self::OpDivide),
            22 => Ok(Self::OpNot),
            23 => Ok(Self::OpNegate),
            24 => Ok(Self::OpPrint),
            25 => Ok(Self::OpJump),
            26 => Ok(Self::OpJumpIfFalse),
            27 => Ok(Self::OpLoop),
            28 => Ok(Self::OpCall),
            29 => Ok(Self::OpInvoke),
            30 => Ok(Self::OpSuperInvoke),
            31 => Ok(Self::OpClosure),
            32 => Ok(Self::OpCloseUpValue),
            33 => Ok(Self::OpReturn),
            34 => Ok(Self::OpClass),
            35 => Ok(Self::OpInherit),
            36 => Ok(Self::OpMethod),
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
