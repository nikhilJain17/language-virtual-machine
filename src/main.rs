const REGISTER_COUNT:usize = 11;

struct Machine {
    memory : Vec<i16>,
    registers : [i16;REGISTER_COUNT],
}

enum Register {
    R0,
    R1,
    R2,
    R3,
    R4,
    R5,
    R6,
    R7,
    PC,
    COND,
}

enum Opcode {
    Branch,
    Add,
    Load,
    Store,
    JumpRegister,
    LoadRegister,
    StoreRegister,
    RTI, // Unused
    BitwiseNot,
    LoadIndirect,
    StoreIndirect,
    Jump,
    RES, // Unused
    LoadEffectiveAddress,
    Trap,
}

// Condition Flags are for recently calculated instruction
// for "if x < y { }"
enum ConditionFlags {
    Pos = 1,
    Zero = 2,
    Neg = 4
}

fn main() {
    // Load in program

}
