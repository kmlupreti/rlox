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
impl From<u8> for Opcode {
    fn from(value: u8) -> Self {
        match value {
            0 => Self::OpConstant,
            1 => Self::OpNil,
            2 => Self::OpTrue,
            3 => Self::OpFalse,
            4 => Self::OpPop,
            5 => Self::OpGetLocal,
            6 => Self::OpSetLocal,
            7 => Self::OpGetGlobal,
            8 => Self::OpDefineGlobal,
            9 => Self::OpSetGlobal,
            10 => Self::OpGetUpvalue,
            11 => Self::OpSetUpvalue,
            12 => Self::OpGetProperty,
            13 => Self::OpSetProperty,
            14 => Self::OpGetSuper,
            15 => Self::OpEqual,
            16 => Self::OpGreater,
            17 => Self::OpLess,
            18 => Self::OpAdd,
            19 => Self::OpSubtract,
            20 => Self::OpMultiply,
            21 => Self::OpDivide,
            22 => Self::OpNot,
            23 => Self::OpNegate,
            24 => Self::OpPrint,
            25 => Self::OpJump,
            26 => Self::OpJumpIfFalse,
            27 => Self::OpLoop,
            28 => Self::OpCall,
            29 => Self::OpInvoke,
            30 => Self::OpSuperInvoke,
            31 => Self::OpClosure,
            32 => Self::OpCloseUpValue,
            33 => Self::OpReturn,
            34 => Self::OpClass,
            35 => Self::OpInherit,
            36 => Self::OpMethod,
            _ => panic!("Unknown raw opcode value: {}", value),
        }
    }
}

impl From<Opcode> for u8 {
    fn from(value: Opcode) -> Self {
        value as u8
    }
}
