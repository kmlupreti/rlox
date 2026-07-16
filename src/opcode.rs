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
