mod instruction;

struct Machine {
    program : Vec<i16>,
    memory : Vec<i16>,
    registers : [i16;REGISTER_COUNT],
}

impl Machine {
    fn new(program : Vec<i16>, memory : Vec<i16>, registers : [i16;REGISTER_COUNT]) -> Machine {
        return Machine {
            program, memory, registers
        }
    }
}

enum Register {
    R0 = 0,
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

const REGISTER_COUNT:usize = 11;


enum Opcode {
    Branch = 0 as isize,
    Add,
    And,
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
    // Initialize {program, machine, memory, registers}
    let program = load_program("./program.out");
    let mut memory: Vec<u16> = vec![];
    let mut registers: [u16; REGISTER_COUNT] = [0; REGISTER_COUNT];
    registers[Register::PC as usize] = 0x3000;

    loop {
        // Fetch instruction
        let instruction: u16 = mem_read(registers[Register::PC as usize]);
        let opcode: u16 = instruction >> 12;

        match opcode {
            // why can't an enum just be cast to a u16, am i doing something wrong
            _ if opcode == Opcode::Branch as u16 => instruction::branch_instr(),
            _ if opcode == Opcode::Add as u16 => instruction::add_instr(),
            _ if opcode == Opcode::Load as u16 => instruction::load_instr(),
            _ if opcode == Opcode::Store as u16 => instruction::store_instr(),
            _ if opcode == Opcode::JumpRegister as u16 => instruction::jumpreg_instr(),
            _ if opcode == Opcode::And as u16 => instruction::and_instr(),
            _ if opcode == Opcode::LoadRegister as u16 => instruction::loadreg_instr(),
            _ if opcode == Opcode::StoreRegister as u16 => instruction::storeg_instr(),
            _ if opcode == Opcode::BitwiseNot as u16 => instruction::not_instr(),
            _ if opcode == Opcode::LoadIndirect as u16 => instruction::loadindr_instr(),
            _ if opcode == Opcode::StoreIndirect as u16 => instruction::store_instr(),
            _ if opcode == Opcode::Jump as u16 => instruction::jump_instr(),
            _ if opcode == Opcode::LoadEffectiveAddress as u16 => instruction::lea_instr(),
            _ if opcode == Opcode::Trap as u16 => instruction::trap_instr(),
            _ if opcode == Opcode::RTI as u16 => (),
            _ if opcode == Opcode::RES as u16 => (),
            _ => {
                println!("[error] Invalid instruction, terminating program");
                break;
            },
            
        }

    }

}

/// Load in the program from the command line
fn load_program(path : &str) -> Vec<u16> {
    assert!(path.len() > 0);
    return vec![1, 2, 3];
}

/// Read from memory
fn mem_read(addr : u16) -> u16 {
    return 0x130;
}
