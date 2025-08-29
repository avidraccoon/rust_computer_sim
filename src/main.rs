// use computer::*;
use colored::*;
mod computer;
mod writers;
mod instructions;

use computer::{Memory, Storage, CPU, Instruction, SubInstructions};
use num_bigint::BigUint;
use num_traits::{One, Zero};
use std::{collections::HashMap, mem, net::AddrParseError, time::Duration};
use instructions::add_instructions;
// Define a constant for the sub_instructions Vec

#[repr(u8)]
#[derive(Hash, Eq, PartialEq, Debug)]
enum InstructionSet {
    NoOperation,        // No args
    Halt,               // No args
    LoadFromMemory,     // #Addr #Reg
    StoreToMemory,      // #Reg #Addr
    LoadFromMemoryReg,  // #Reg/Addr #Reg
    StoreToMemoryReg,   // #Reg #Reg/Addr
    AddImmediate,       // #Reg #Imm
    AddReg,             // #Reg #Reg
    LoadImmediate,      // #Reg #Imm
    MoveRegister,       // #Reg #Reg
    PushImmediate,      // #Imm
    PushReg,            // #Reg
    PopReg,             // #Reg
    Jump,               // #Addr
    JumpEqual,          // #Reg/Addr #Reg (A) #Imm (B)
    JumpNotEqual,       // #Reg/Addr #Reg (A) #Imm (B)
    JumpGreaterThan,    // #Reg/Addr #Reg (A) #Imm (B)
    JumpLessThan,       // #Reg/Addr #Reg (A) #Imm (B)
    JumpLessEqual,      // #Reg/Addr #Reg (A) #Imm (B)
    JumpGreaterEqual,   // #Reg/Addr #Reg (A) #Imm (B)
    JumpReg,            // #Reg/Addr
    JumpEqualReg,       // #Reg/Addr #Reg (A) #Reg (B)
    JumpNotEqualReg,    // #Reg/Addr #Reg (A) #Reg (B)
    JumpGreaterThanReg, // #Reg/Addr #Reg (A) #Reg (B)
    JumpLessThanReg,    // #Reg/Addr #Reg (A) #Reg (B)
    JumpLessEqualReg,   // #Reg/Addr #Reg (A) #Reg (B)
    JumpGreaterEqualReg,// #Reg/Addr #Reg (A) #Reg (B)
}

impl Into<u8> for InstructionSet {
    fn into(self) -> u8 {
        self as u8
    }
}

fn main() {
    let mut cpu = computer::create_default_cpu(64, 0);
    let mut ref_reg = |name: &str| {
        cpu.registers.name_to_u8(&name)
    };
    let mut instruction_set_writer = writers::InstructionSetWriter::new();

    add_instructions(&mut instruction_set_writer, &mut ref_reg, cpu.cpu_data_size);

    let reg_0 = ref_reg("reg_0");
    let reg_1 = ref_reg("reg_1");
    let reg_2 = ref_reg("reg_2");

    cpu.set_instruction_set(instruction_set_writer.build());

    let mut program_writer = writers::ProgramWriter::new(cpu.instruction_set.clone());

    program_writer
        .add_instruction(InstructionSet::LoadImmediate, &[reg_0, 0])    // Load 0 into reg_0
        .add_instruction(InstructionSet::LoadImmediate, &[reg_1, 1]);    // Load 1 into reg_1
    
    program_writer
        .add_instruction(InstructionSet::PushImmediate, &[1])
        .add_instruction(InstructionSet::PushImmediate, &[2])
        .add_instruction(InstructionSet::PushImmediate, &[3])
        .add_instruction(InstructionSet::PopReg, &[reg_2]);

    program_writer
        .add_instruction(InstructionSet::Halt, &[]);                                       // Halt the program

    let program = program_writer.build();
    cpu.memory.write_chunk(0, program.as_slice());
    let bytes_per_row = BigUint::from(8u32);
    let print_at_end_of_op = false;
    let clear_screen = true;
    let sleep = true;
    let sleep_time_after_op = Duration::from_millis(1000);
    let sleep_time_after_sub_op = Duration::from_millis(100);
    let mut instruction = None;
    let mut op_code_address = BigUint::from(0u32);

    
    for i in 1..50 {
        cpu.clock();
        
        if let Some(opcode) = cpu.current_opcode {
            if let Some(instruct) = cpu.instruction_set.get(&opcode) {
                instruction.replace(instruct.clone());
            } else {
                panic!("Invalid opcode encountered: {}", opcode);
            }
        }else {
            // Update Op Code Address
            op_code_address = cpu.get_program_counter();
        }

        print_status(&cpu, i, print_at_end_of_op, clear_screen, instruction.as_ref(), &op_code_address, &bytes_per_row);
        
        if sleep {
            wait_after_step(cpu.current_opcode.is_none(), sleep_time_after_op, sleep_time_after_sub_op);
        }

        if cpu.is_halted() {
            break;
        }
    }
}

fn print_status(cpu: &CPU, i: u32, print_at_end_of_op: bool, clear_screen: bool, instruction: Option<&Instruction>, op_code_address: &BigUint, bytes_per_row: &BigUint) {
    let pc_color = Color::BrightYellow;
    let op_code_color = Color::Red;
    let arg_colors = vec![
        Color::Blue,
        Color::Green,
        Color::Yellow,
        Color::Cyan
    ];
    if !print_at_end_of_op || (cpu.current_opcode.is_none()){
        if clear_screen {
            clearscreen::clear().expect("failed to clear screen");
        }
        let counter = cpu.get_program_counter();
        let accumulator = cpu.get_accumulator();
        println!("Cycle {}", i);
        println!("Halted: {}", cpu.is_halted());
        println!("Program Counter: {}", counter);
        println!("Current Opcode: {:?}", cpu.current_opcode);
        println!("Current Sub Step: {} / {}", cpu.current_sub_step, instruction.map_or(0, |instr| instr.sub_instructions.len()));
        println!("Accumulator: {:?}", accumulator);
        println!("Registers: {:?}", cpu.register_data);
        let memory_snapshot = cpu.memory.read_chunk(0, 64);
        println!("Memory Snapshot:\n");
        let mut arg_index = 0;
        let arg_count = match instruction {
            Some(ref instr) => instr.args,
            None => 0,
        };
        let mut memory_counter = BigUint::zero();
        for byte in memory_snapshot{
            let mut byte_text = ColoredString::from(format!("{:02x} ", byte));
            if memory_counter == counter {
                byte_text.bgcolor = Some(pc_color);
            }
            if &memory_counter == op_code_address {
                byte_text.fgcolor = Some(op_code_color);
            }else if &memory_counter > op_code_address && arg_index < arg_count as usize {
                byte_text.fgcolor = Some(arg_colors[arg_index]);
                arg_index = (arg_index + 1) % arg_colors.len();
            }
            memory_counter += BigUint::one();
            if &memory_counter % bytes_per_row == BigUint::zero() {
                println!("{}", byte_text);
            }else {
                print!("{}", byte_text);
            }
        }
        println!("Reg 0: {:?}, Reg 1: {:?}, Reg 2: {:?}\n", cpu.read_register_string("reg_0"), cpu.read_register_string("reg_1"), cpu.read_register_string("reg_2"));
    }
}

fn wait_after_step(op_step: bool, sleep_time_after_op: Duration, sleep_time_after_sub_op: Duration) {
    if op_step {
        if sleep_time_after_op > Duration::from_millis(0) {
            std::thread::sleep(sleep_time_after_op);
        }
    }else {
        if sleep_time_after_sub_op > Duration::from_millis(0) {
            std::thread::sleep(sleep_time_after_sub_op);
        }
    }
}