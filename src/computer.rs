
use num_bigint::BigUint;
use num_traits::One; // Ensure the trait is in scope for BigUint::one()
use num_traits::ToPrimitive;
use num_traits::Zero;
const USIZE_SIZE: usize = mem::size_of::<usize>();
pub struct Memory {
    data: Vec<u8>,
}

impl Memory {
    pub fn new(size: usize) -> Self {
        Memory {
            data: vec![0; size],
        }
    }

    pub fn read(&self, address: usize) -> u8 {
        self.data[address]
    }

    pub fn write(&mut self, address: usize, value: u8) {
        self.data[address] = value;
    }

    pub fn read_chunk(&self, start: usize, end: usize) -> &[u8] {
        &self.data[start..end]
    }

    pub fn write_chunk(&mut self, start: usize, data: &[u8]) {
        self.data[start..start + data.len()].copy_from_slice(data);
    }

    pub fn clear_chunk(&mut self, start: usize, end: usize) {
        self.data[start..end].fill(0);
    }
}

pub struct Storage {
    data: Vec<u8>,
}
    
impl Storage {
    pub fn new(size: usize) -> Self {
        Storage {
            data: vec![0; size],
        }
    }

    pub fn read(&self, address: usize) -> u8 {
        self.data[address]
    }

    pub fn write(&mut self, address: usize, value: u8) {
        self.data[address] = value;
    }

    pub fn read_chunk(&self, start: usize, end: usize) -> &[u8] {
        &self.data[start..end]
    }

    pub fn write_chunk(&mut self, start: usize, data: &[u8]) {
        self.data[start..start + data.len()].copy_from_slice(data);
    }
}

#[derive(Clone)]
pub enum SubInstructions {
    NoOperation,
    Halt,
    LoadImmediate(u8),
    LoadFromMemory,
    LoadFromRegister(u8),
    LoadFromRegisterInternal(u8),
    SetMemoryAddress,
    StoreToMemory,
    StoreToRegister(u8),
    StoreToRegisterInternal(u8),
    StepProgramMemory(u8),
    Add,
    Sub,
    PushToStack,
    PopFromStack,
    Jump,
    Compare,
    JumpIfFlag(BigUint, BigUint),
    JumpIfNotFlag(BigUint, BigUint),
}

// Implement execution logic for SubInstructions
impl SubInstructions {
    pub fn execute(&self, cpu: &mut CPU) {
        match self {
            SubInstructions::NoOperation => {}
            SubInstructions::Halt => {
                cpu.halted = true;
            }
            SubInstructions::LoadImmediate(data_offset) => {
                let value = cpu.read_program_memory_offset(*data_offset);
                cpu.set_accumulator(BigUint::from(value));
            }
            SubInstructions::LoadFromMemory => {
                // TODO: handle longer numbers
                let address = cpu.read_register_string("memory_address").clone().unwrap()[0] as usize;
                let value = cpu.memory.read_chunk(address, address + cpu.cpu_data_size as usize);
                cpu.set_accumulator(BigUint::from_bytes_be(value));
            }
            SubInstructions::LoadFromRegister(data_offset) => {
                let register = cpu.read_program_memory_offset(*data_offset);
                let bytes = cpu.read_register(register).unwrap_or(&[0]);
                cpu.set_accumulator(BigUint::from_bytes_be(bytes));
            }
            SubInstructions::LoadFromRegisterInternal(register) => {
                let bytes= cpu.read_register(*register).unwrap_or(&[0]).to_vec();
                cpu.set_accumulator(BigUint::from_bytes_be(&bytes));
            }
            SubInstructions::SetMemoryAddress => {
               let value = cpu.get_accumulator_bytes().to_vec();
               let _ = cpu.write_register_string("memory_address", value.as_slice());
            }
            SubInstructions::StoreToMemory => {
                // TODO: handle longer numbers
                let address = cpu.read_register_string("memory_address").clone().unwrap()[0] as usize;
                let value = cpu.get_accumulator_bytes().to_vec();
                cpu.memory.write_chunk(address, value.as_slice());
            }
            SubInstructions::StoreToRegister(data_offset) => {
                let register = cpu.read_program_memory_offset(*data_offset);
                let value = cpu.get_accumulator_bytes().to_vec();
                let _ = cpu.write_register(register, value.as_slice());
            }
            SubInstructions::StoreToRegisterInternal(register) => {
                let value: Vec<u8> = cpu.get_accumulator_bytes().to_vec();
                let _ = cpu.write_register(*register, value.as_slice());
            }
            SubInstructions::StepProgramMemory(steps) => {
                cpu.step_size(*steps);
            }
            SubInstructions::Add => {
                let a_value = BigUint::from_bytes_be(cpu.read_register_string("reg_a").unwrap());
                let b_value = BigUint::from_bytes_be(cpu.read_register_string("reg_b").unwrap());
                let sum_value = a_value + b_value;
                cpu.set_accumulator(sum_value);
            }
            SubInstructions::Sub => {
                let a_value = BigUint::from_bytes_be(cpu.read_register_string("reg_a").unwrap());
                let b_value = BigUint::from_bytes_be(cpu.read_register_string("reg_b").unwrap());
                let sub_value = a_value - b_value;
                cpu.set_accumulator(sub_value);
            }
            SubInstructions::PushToStack => {
                let stack_pointer = BigUint::from_bytes_be(cpu.read_register_string("stack_pointer").unwrap());
                let accumulator_bytes = cpu.get_accumulator_bytes().to_vec();
                let destination = (stack_pointer - cpu.cpu_data_size).to_usize().expect("Stack pointer outside usize range");
                cpu.memory.write_chunk(destination, accumulator_bytes.as_slice());
                let cpu_adjusted = &destination.to_be_bytes()[USIZE_SIZE - cpu.cpu_data_size as usize..];
                let _ = cpu.write_register_string("stack_pointer", cpu_adjusted);
            }
            SubInstructions::PopFromStack => {
                let stack_pointer = BigUint::from_bytes_be(cpu.read_register_string("stack_pointer").unwrap());
                let address = stack_pointer.to_usize().expect("Stack pointer outside usize range");
                let data = cpu.memory.read_chunk(address, address + cpu.cpu_data_size as usize);
                cpu.set_accumulator(BigUint::from_bytes_be(data));
                cpu.memory.write_chunk(address, vec![0; cpu.cpu_data_size as usize].as_slice());
                let cpu_adjusted = &address.to_be_bytes()[USIZE_SIZE - cpu.cpu_data_size as usize..];
                let _ = cpu.write_register_string("stack_pointer", cpu_adjusted);
            }
            SubInstructions::Jump => {
                cpu.set_program_counter(cpu.get_accumulator());
                cpu.current_opcode = None;
            }
            SubInstructions::Compare => {
                // Compare reg_a to reg_b
                // TODO: handle longer numbers
                let a = cpu.read_register_string("reg_a").unwrap()[0];
                let b = cpu.read_register_string("reg_b").unwrap()[0];
                let mut flags = cpu.get_flags();
                if a == b {
                    flags |= ZERO_FLAG.clone();
                    flags &= flag_invert(&GREATER_FLAG );
                } else if a > b {
                    flags |= GREATER_FLAG.clone();
                    flags &= flag_invert(&ZERO_FLAG);
                } else {
                    flags &= flag_invert(&(ZERO_FLAG.clone() | GREATER_FLAG.clone()));
                }
                cpu.set_flags(flags);
            }
            SubInstructions::JumpIfFlag(true_mask, false_mask) => {
                // Jump if all bits in true_mask are set and all bits in false_mask are clear
                let flags = cpu.get_flags();
                let true_condition = (flags.clone() & true_mask) == *true_mask;
                let false_condition = (flags & false_mask) == BigUint::zero();
                if true_condition && false_condition {
                    cpu.set_program_counter(cpu.get_accumulator());
                    cpu.current_opcode = None;
                }
            }

            SubInstructions::JumpIfNotFlag(true_mask, false_mask) => {
                // Jump if NOT (all bits in true_mask are set and all bits in false_mask are clear)
                let flags = cpu.get_flags();
                let true_condition = (flags.clone() & true_mask) == *true_mask;
                let false_condition = (flags & false_mask) == BigUint::zero();
                if !(true_condition && false_condition) {
                    cpu.set_program_counter(cpu.get_accumulator());
                    cpu.current_opcode = None;
                }
            }
        }
    }
}

#[derive(Clone)]
pub struct Instruction {
    pub sub_instructions: Vec<SubInstructions>,
    pub args: u8,
}

#[derive(Clone)]
pub struct Register {
    pub size: usize,
    pub location: usize,
}

pub struct Registers {
    pub total_length: usize,
    pub string_reference: HashMap<String, u8>,
    pub registers: Vec<Register>,
}

impl Registers {
    pub fn new() -> Self {
        Registers {
            total_length: 0,
            string_reference: HashMap::new(),
            registers: Vec::new(),
        }
    }

    pub fn look_up_string(&self, name: &str) -> Option<&Register> {
        let id = self.string_reference.get(name);
        if let Some(id) = id {
            self.look_up_u8(*id)
        } else {
            None
        }
    }

    pub fn look_up_u8(&self, id: u8) -> Option<&Register> {
        self.registers.get(id as usize)
    }

    pub fn name_to_u8(&mut self, name: &str) -> u8 {
        *self.string_reference.get(name).expect("Register Not Found")
    }

    pub fn add_register(&mut self, name: String, size: usize, location: usize) {
        self.total_length = self.total_length.max(location + size);
        self.string_reference.insert(name, self.registers.len() as u8);
        self.registers.push(Register { size, location });
    }
}

use std::mem;
use std::{collections::HashMap, hash::Hash, i32, ops::Sub, vec};

// Truth Table
// 
// L (less than), G (greater than), E(equal), N (No compare)
// Zero Flag    | 0 0 1 1
// Greater Flag | 0 1 0 1
// CompareStatus| L G E N
use lazy_static::lazy_static;

lazy_static! {
    static ref FLAG_NONE: BigUint    = BigUint::from(0b0000_0000u8);
    static ref FLAG_ALL: BigUint     = BigUint::from(0b1111_1111u8);
    static ref ZERO_FLAG: BigUint    = BigUint::from(0b0000_0001u8);
    static ref GREATER_FLAG: BigUint = BigUint::from(0b0000_0010u8);
}

pub fn flag_invert(mask: &BigUint) -> BigUint {
    &*FLAG_ALL - mask
}

pub struct CPU {
    pub register_data: Vec<u8>,
    pub registers: Registers,
    pub memory: Memory,
    pub storage: Storage,
    pub instruction_set: HashMap<u8, Instruction>,
    pub cpu_data_size: u8,
    pub current_opcode: Option<u8>,
    pub current_sub_step: u8,
    pub halted: bool
}

impl CPU {
    pub fn new(registers: Registers, memory: Memory, storage: Storage) -> Self {
        let mut cpu = CPU { 
            register_data: vec![0; registers.total_length], 
            registers, 
            memory, 
            storage, 
            instruction_set: HashMap::new(),
            cpu_data_size: 1,
            current_opcode: None,
            current_sub_step: 0,
            halted: false
        };
        cpu.write_register_string("stack_pointer", &[cpu.memory.data.len() as u8]).unwrap();
        cpu
    }

    pub fn read_register_string(&self, name: &str) -> Option<&[u8]> {
        self.registers.look_up_string(name).and_then(|reg| {
            let start = reg.location;
            let end = start + reg.size;
            self.register_data.get(start..end)
        })
    }

    pub fn read_register(&self, id: u8) -> Option<&[u8]> {
        self.registers.look_up_u8(id).and_then(|reg| {
            let start = reg.location;
            let end = start + reg.size;
            self.register_data.get(start..end)
        })
    }

    pub fn write_register_string(&mut self, name: &str, data: &[u8]) -> Result<(), String> {
        let reg = match self.registers.look_up_string(name) {
            Some(r) => r.clone(),
            None => return Err(format!("Register '{}' not found", name)),
        };
        self.write_register_internal(&reg, data)
    }

    pub fn write_register(&mut self, id: u8, data: &[u8]) -> Result<(), String> {
        let reg = match self.registers.look_up_u8(id) {
            Some(r) => r.clone(),
            None => return Err(format!("Register '{}' not found", id)),
        };
        self.write_register_internal(&reg, data)
    }

    pub fn write_register_internal(&mut self, reg: &Register, data: &[u8]) -> Result<(), String> {
        if data.len() != reg.size {
            return Err(format!("Data size mismatch for register {}: expected {}, got {}", reg.location, reg.size, data.len()));
        }
        let start = reg.location;
        let end = start + reg.size;
        self.register_data[start..end].copy_from_slice(data);
        Ok(())
    }

    pub fn load_to_memory(&mut self, storage_address: usize, memory_address: usize){
        let value = self.storage.read(storage_address);
        self.memory.write(memory_address, value);
    }

    pub fn load_chunk_to_memory(&mut self, start: usize, end: usize) {
        let chunk = self.storage.read_chunk(start, end);
        self.memory.write_chunk(start, chunk);
    }

    pub fn save_to_storage(&mut self, memory_address: usize, storage_address: usize) {
        let value = self.memory.read(memory_address);
        self.storage.write(storage_address, value);
    }

    pub fn save_chunk_to_storage(&mut self, start: usize, end: usize) {
        let chunk = self.memory.read_chunk(start, end);
        self.storage.write_chunk(start, chunk);
    }

    pub fn read_program_memory(&mut self) -> u8 {
        let counter = self.get_program_counter();
        let address = counter.to_usize().expect("Program counter outside usize range");
        self.memory.read(address)
    }

    pub fn read_program_memory_offset(&self, offset: u8) -> u8 {
        let counter = self.get_program_counter();
        let total = counter + BigUint::from(offset);
        let address = total.to_usize().expect("Program counter + offset outside usize range");
        self.memory.read(address)
    }

    pub fn get_program_counter(&self) -> BigUint {
        let bytes = self.read_register_string("program_counter").expect("Program Counter register not found.");
        BigUint::from_bytes_be(bytes)
    }

    pub fn get_program_counter_bytes(&self) -> &[u8] {
        self.read_register_string("program_counter").expect("Program Counter register not found.")
    }

    pub fn set_program_counter(&mut self, value: BigUint) {
        let bytes = value.to_bytes_be();
        self.write_register_string("program_counter", &bytes).expect("Failed to write program counter");
    }

    pub fn get_accumulator(&self) -> BigUint {
        let bytes = self.read_register_string("accumulator").expect("Accumulator register not found.");
        BigUint::from_bytes_be(bytes)
    }

    pub fn get_accumulator_bytes(&self) -> &[u8] {
        self.read_register_string("accumulator").expect("Accumulator register not found.")
    }

    pub fn set_accumulator(&mut self, value: BigUint) {
        let bytes = value.to_bytes_be();
        self.write_register_string("accumulator", &bytes).expect("Failed to write accumulator");
    }

    pub fn get_flags_bytes(&self) -> &[u8] {
        self.read_register_string("flags").expect("Flags register not found.")
    }

    pub fn get_flags(&mut self) -> BigUint {
        let bytes = self.get_flags_bytes();
        BigUint::from_bytes_be(bytes)
    }

    pub fn set_flags(&mut self, value: BigUint) {
        let bytes = value.to_bytes_be();
        self.write_register_string("flags", &bytes).expect("Failed to write flags");
    }

    pub fn step(&mut self) {
        let counter = self.get_program_counter();
        self.set_program_counter(counter + BigUint::one());
    }

    pub fn step_size(&mut self, size: u8) {
        let counter: BigUint = self.get_program_counter();
        self.set_program_counter(counter + BigUint::from(size));
    }

    

    pub fn unhalt(&mut self) {
        self.halted = false;
    }

    pub fn is_halted(&self) -> bool {
        self.halted
    }

    pub fn clock(&mut self) {
        if self.is_halted() {
            return;
        }
        match self.current_opcode {
            Some(op_code) => {
                // Limit the scope of the immutable borrow
                let (sub_instructions_len, sub_instruction) = {
                    let op = self.instruction_set.get(&op_code).expect(format!("Invalid op code {}", op_code).as_str());
                    (op.sub_instructions.len(), op.sub_instructions.get(self.current_sub_step as usize).cloned())
                };
                if self.current_sub_step as usize >= sub_instructions_len {
                    self.current_opcode = None;
                    self.step();
                    return;
                }
                if let Some(sub_instruction) = sub_instruction {
                    sub_instruction.execute(self);
                }
                self.current_sub_step += 1;
            },
            None => {
                self.current_opcode = Some(self.read_program_memory());
                self.current_sub_step = 0;
            }
        }
        
        
    }

    pub fn set_instruction_set(&mut self, instruction_set: HashMap<u8, Instruction>){
        self.instruction_set = instruction_set;
    }
    
}

pub fn create_default_cpu_with_memory(memory: Memory, storage: Storage) -> CPU {
    let mut registers = Registers::new();
    registers.add_register("program_counter".to_string(), 1, 0);
    registers.add_register("accumulator".to_string(), 1, 1);
    registers.add_register("flags".to_string(), 1, 2);
    registers.add_register("stack_pointer".to_string(), 1, 3);
    registers.add_register("return_address".to_string(), 1, 4);
    registers.add_register("memory_pointer".to_string(), 1, 5);
    registers.add_register("instruction_temp_0".to_string(), 1, 6);
    registers.add_register("instruction_temp_1".to_string(), 1, 7);
    registers.add_register("instruction_temp_2".to_string(), 1, 8);
    registers.add_register("instruction_temp_3".to_string(), 1, 9);
    registers.add_register("reg_a".to_string(), 1, 10);
    registers.add_register("reg_b".to_string(), 1, 11);
    registers.add_register("reg_c".to_string(), 1, 12);
    registers.add_register("reg_0".to_string(), 1, 13);
    registers.add_register("reg_1".to_string(), 1, 14);
    registers.add_register("reg_2".to_string(), 1, 15);

    CPU::new(registers, memory, storage)
}

pub fn create_default_cpu(memory_size: usize, storage_size: usize) -> CPU {
    let memory = Memory::new(memory_size);
    let storage = Storage::new(storage_size);
    create_default_cpu_with_memory(memory, storage)
}



