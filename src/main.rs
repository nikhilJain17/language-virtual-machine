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

const REGISTER_COUNT:usize = instruction::REGISTER_COUNT;

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
            _ if opcode == Opcode::Add as u16 => add_instr(instruction, &mut registers),
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

/// Module for implementation of the lc3 instructions 

/// Sign extension utility function
fn sign_extend(num:u16, count:u16) -> u16 {
    // if the msb is 1, sign extend with 1s
    if (num >> (count - 1)) & 1 != 0 {
        return num | (0xffff << count);
    }
    num
}

/// Update condition flags any time a value is written to a register 
fn update_flags(reg: usize, registers: &mut [u16; REGISTER_COUNT]) {
    if registers[reg] == 0 {
        registers[Register::COND as usize] = ConditionFlags::Zero as u16;
    }
    else if registers[reg] >> 15 == 1 { // MSB = 1 means negative in 2s complement
        registers[Register::COND as usize] = ConditionFlags::Neg as u16;
    }
    else {
        registers[Register::COND as usize] = ConditionFlags::Pos as u16;
    }
}

/// Handles ADD DR, SR1, SR2 and ADD DR, SR1, imm5
pub fn add_instr(instr: u16, registers: &mut [u16; REGISTER_COUNT]) {
    let dr:usize = ((instr >> 9) & 0x7) as usize;
    let sr1:usize = ((instr >> 6) & 0x7) as usize;
    let imm_flag:u16 = (instr >> 5) & 0x1;

    if imm_flag == 1 {
        // add immediate
        let imm:u16 = sign_extend(instr & 0x1F, 5);
        registers[dr] = registers[sr1] + imm
    }
    else {
        let sr2:usize = (instr & 0x7) as usize;
        registers[dr] = registers[sr1] + registers[sr2];
    }
    
    update_flags(dr, registers);
    
}

pub fn and_instr() {

}
pub fn not_instr() {

}
pub fn branch_instr() {

}
pub fn jump_instr() {

}
pub fn jumpreg_instr() {

}
pub fn load_instr() {

}

pub fn store_instr() {

}

pub fn loadreg_instr() {

}

pub fn loadindr_instr() {

}
pub fn lea_instr() {

}
pub fn trap_instr() {

}

pub fn storeg_instr() {

}
pub fn storeindr_instr() {

}

