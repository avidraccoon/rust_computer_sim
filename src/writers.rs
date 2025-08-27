use std::collections::HashMap;

use crate::computer::{Instruction, SubInstructions};

pub struct InstructionBuilder {
    sub_instructions: Vec<SubInstructions>,
    args: u8,
}

impl InstructionBuilder {
    pub fn new(args: u8) -> Self {
        InstructionBuilder {
            sub_instructions: Vec::new(),
            args,
        }
    }

    pub fn add_sub_instruction(&mut self, sub_instruction: SubInstructions) -> &mut Self {
        self.sub_instructions.push(sub_instruction);
        self
    }

    pub fn build(self) -> Instruction {
        Instruction {
            sub_instructions: self.sub_instructions,
            args: self.args,
        }
    }
}

pub struct InstructionSetWriter {
    instruction_set: HashMap<u8, InstructionBuilder>,
}

impl InstructionSetWriter {
    pub fn new() -> Self {
        InstructionSetWriter {
            instruction_set: HashMap::new(),
        }
    }

    pub fn add_instruction<T>(&mut self, opcode: T, args: u8) -> &mut InstructionBuilder
    where
        T: Into<u8>,
    {
        let u8_opcode: u8 = opcode.into();
        self.instruction_set.insert(u8_opcode, InstructionBuilder::new(args));
        self.instruction_set.get_mut(&u8_opcode).unwrap()
    }

    pub fn build(self) -> HashMap<u8, Instruction> {
        self.instruction_set
            .into_iter()
            .map(|(opcode, builder)| (opcode, builder.build()))
            .collect()
    }
}

pub struct ProgramWriter {
    instruction_set: HashMap<u8, Instruction>,
    program: Vec<u8>,
}

impl ProgramWriter {
    pub fn new(instruction_set: HashMap<u8, Instruction>) -> Self {
        ProgramWriter {
            instruction_set,
            program: Vec::new(),
        }
    }

    pub fn add_instruction<T>(&mut self, opcode: T, args: &[u8]) -> &mut Self
    where
        T: Into<u8>,
    {
        let u8_opcode: u8 = opcode.into();
        if let Some(instruction) = self.instruction_set.get(&u8_opcode) {
            if args.len() != instruction.args as usize {
                panic!("Incorrect number of arguments for opcode {}", u8_opcode);
            }
            self.program.push(u8_opcode);
            self.program.extend_from_slice(args);
        } else {
            panic!("Opcode {} not found in instruction set", u8_opcode);
        }
        self
    }

    pub fn build(self) -> Vec<u8> {
        self.program
    }
}