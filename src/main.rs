// use computer::*;
use colored::*;
mod computer;
mod writers;

use computer::{Memory, Storage, CPU, Instruction, SubInstructions};
use std::{collections::HashMap, mem, net::AddrParseError, time::Duration};

// Define a constant for the sub_instructions Vec

#[repr(u8)]
#[derive(Hash, Eq, PartialEq, Debug)]
enum InstructionSet {
    NoOperation,    // No args
    Halt,           // No args
    LoadFromMemory, // #Addr #Reg
    StoreToMemory,  // #Reg #Addr
    AddImmediate,   // #Reg #Imm
    AddReg,         // #Reg #Reg
    LoadImmediate,  // #Reg #Imm
    MoveRegister,   // #Reg #Reg
    PushImmediate,  // #Imm
    PushReg,        // #Reg
    PopReg,         // #Reg
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

    instruction_set_writer.add_instruction(InstructionSet::NoOperation, 0);

    instruction_set_writer.add_instruction(InstructionSet::Halt, 0)
        .add_sub_instruction(SubInstructions::Halt);

    instruction_set_writer.add_instruction(InstructionSet::LoadFromMemory, 2)
        .add_sub_instruction(SubInstructions::LoadFromMemory(1))
        .add_sub_instruction(SubInstructions::StoreToRegister(2))
        .add_sub_instruction(SubInstructions::StepProgramMemory(2));

    instruction_set_writer.add_instruction(InstructionSet::StoreToMemory, 2)
        .add_sub_instruction(SubInstructions::LoadFromRegister(1))
        .add_sub_instruction(SubInstructions::StoreToMemory(2))
        .add_sub_instruction(SubInstructions::StepProgramMemory(2));

    instruction_set_writer.add_instruction(InstructionSet::AddImmediate, 2)
        .add_sub_instruction(SubInstructions::LoadFromRegister(1))
        .add_sub_instruction(SubInstructions::StoreToRegisterInternal(ref_reg("reg_a")))
        .add_sub_instruction(SubInstructions::LoadImmediate(2))
        .add_sub_instruction(SubInstructions::StoreToRegisterInternal(ref_reg("reg_b")))
        .add_sub_instruction(SubInstructions::Add)
        .add_sub_instruction(SubInstructions::StoreToRegister(1))
        .add_sub_instruction(SubInstructions::StepProgramMemory(2));

    instruction_set_writer.add_instruction(InstructionSet::AddReg, 2)
        .add_sub_instruction(SubInstructions::LoadFromRegister(1))
        .add_sub_instruction(SubInstructions::StoreToRegisterInternal(ref_reg("reg_a")))
        .add_sub_instruction(SubInstructions::LoadFromRegister(2))
        .add_sub_instruction(SubInstructions::StoreToRegisterInternal(ref_reg("reg_b")))
        .add_sub_instruction(SubInstructions::Add)
        .add_sub_instruction(SubInstructions::StoreToRegister(1))
        .add_sub_instruction(SubInstructions::StepProgramMemory(2));

    instruction_set_writer.add_instruction(InstructionSet::LoadImmediate, 2)
        .add_sub_instruction(SubInstructions::LoadImmediate(2))
        .add_sub_instruction(SubInstructions::StoreToRegister(1))
        .add_sub_instruction(SubInstructions::StepProgramMemory(2));

    instruction_set_writer.add_instruction(InstructionSet::MoveRegister, 2)
        .add_sub_instruction(SubInstructions::LoadFromRegister(1))
        .add_sub_instruction(SubInstructions::StoreToRegister(2))
        .add_sub_instruction(SubInstructions::StepProgramMemory(2));

    instruction_set_writer.add_instruction(InstructionSet::PushImmediate, 1)
        .add_sub_instruction(SubInstructions::LoadImmediate(1))
        .add_sub_instruction(SubInstructions::PushToStack)
        .add_sub_instruction(SubInstructions::StepProgramMemory(1));

    instruction_set_writer.add_instruction(InstructionSet::PushReg, 1)
        .add_sub_instruction(SubInstructions::LoadFromRegister(1))
        .add_sub_instruction(SubInstructions::PushToStack)
        .add_sub_instruction(SubInstructions::StepProgramMemory(1));

    instruction_set_writer.add_instruction(InstructionSet::PopReg, 1)
        .add_sub_instruction(SubInstructions::PopFromStack)
        .add_sub_instruction(SubInstructions::StoreToRegister(1))
        .add_sub_instruction(SubInstructions::StepProgramMemory(1));

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
    let bytes_per_row = 8;
    let print_at_end_of_op = false;
    let clear_screen = true;
    let sleep = true;
    let sleep_time_after_op = Duration::from_millis(500);
    let sleep_time_after_sub_op = Duration::from_millis(100);
    let mut instruction = None;
    let mut op_code_address = 0;

    
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
            op_code_address = cpu.program_counter;
        }

        print_status(&cpu, i, print_at_end_of_op, clear_screen, instruction.as_ref(), op_code_address, bytes_per_row);
        
        if sleep {
            wait_after_step(cpu.current_opcode.is_none(), sleep_time_after_op, sleep_time_after_sub_op);
        }

        if cpu.is_halted() {
            break;
        }
    }
}

fn print_status(cpu: &CPU, i: u32, print_at_end_of_op: bool, clear_screen: bool, instruction: Option<&Instruction>, op_code_address: usize, bytes_per_row: usize) {
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
        println!("Cycle {}", i);
        println!("Halted: {}", cpu.is_halted());
        println!("Program Counter: {}", cpu.program_counter);
        println!("Current Opcode: {:?}", cpu.current_opcode);
        println!("Current Sub Step: {} / {}", cpu.current_sub_step, instruction.map_or(0, |instr| instr.sub_instructions.len()));
        println!("Accumulator: {:?}", cpu.accumulator);
        println!("Registers: {:?}", cpu.register_data);
        let memory_snapshot = cpu.memory.read_chunk(0, 64);
        println!("Memory Snapshot:\n");
        let mut arg_index = 0;
        let arg_count = match instruction {
            Some(ref instr) => instr.args,
            None => 0,
        };
        let counter = cpu.program_counter;
        for (i, byte) in memory_snapshot.iter().enumerate() {
            let mut byte_text = ColoredString::from(format!("{:02x} ", byte));
            if i == counter {
                byte_text.bgcolor = Some(pc_color);
            }
            if i == op_code_address {
                byte_text.fgcolor = Some(op_code_color);
            }else if i > op_code_address && arg_index < arg_count as usize {
                byte_text.fgcolor = Some(arg_colors[arg_index]);
                arg_index = (arg_index + 1) % arg_colors.len();
            }

            if (i + 1) % bytes_per_row == 0 {
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