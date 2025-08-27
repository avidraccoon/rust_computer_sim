

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

fn add_u8_arrays(a: &[u8], b: &[u8]) -> Vec<u8> {
    let max_len = a.len().max(b.len());
    let mut result = Vec::with_capacity(max_len + 1);
    let mut carry: u16 = 0;

    for i in 0..max_len {
        let digit_a = if i < a.len() { a[a.len() - 1 - i] } else { 0 };
        let digit_b = if i < b.len() { b[b.len() - 1 - i] } else { 0 };

        let sum = digit_a as u16 + digit_b as u16 + carry;
        result.push((sum % 256) as u8);
        carry = sum / 256;
    }

    if carry > 0 {
        result.push(carry as u8);
    }

    result.reverse();
    result
}

#[derive(Clone)]
pub enum SubInstructions {
    NoOperation,
    Halt,
    LoadImmediate(u8),
    LoadFromMemory(u8),
    LoadFromRegister(u8),
    LoadFromRegisterInternal(u8),
    StoreToMemory(u8),
    StoreToRegister(u8),
    StoreToRegisterInternal(u8),
    StepProgramMemory(u8),
    Add,
    PushToStack,
    PopFromStack
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
                cpu.accumulator = [value].to_vec();
            }
            SubInstructions::LoadFromMemory(data_offset) => {
                let address = cpu.read_program_memory_offset(*data_offset) as usize;
                let value = cpu.memory.read_chunk(address, address + cpu.cpu_data_size as usize);
                cpu.accumulator = value.to_vec();
            }
            SubInstructions::LoadFromRegister(data_offset) => {
                let register = cpu.read_program_memory_offset(*data_offset);
                cpu.accumulator = cpu.read_register(register).unwrap_or(&[0]).to_vec();
            }
            SubInstructions::LoadFromRegisterInternal(register) => {
                cpu.accumulator = cpu.read_register(*register).unwrap_or(&[0]).to_vec();
            }
            SubInstructions::StoreToMemory(data_offset) => {
                let address = cpu.read_program_memory_offset(*data_offset) as usize;
                let value = &cpu.accumulator;
                cpu.memory.write_chunk(address, value.as_slice());
            }
            SubInstructions::StoreToRegister(data_offset) => {
                let register = cpu.read_program_memory_offset(*data_offset);
                let value = cpu.accumulator.clone();
                let _ = cpu.write_register(register, &value);
            }
            SubInstructions::StoreToRegisterInternal(register) => {
                let value = cpu.accumulator.clone();
                let _ = cpu.write_register(*register, &value);
            }
            SubInstructions::StepProgramMemory(steps) => {
                cpu.step_size(*steps);
            }
            SubInstructions::Add => {
                let a_value = cpu.read_register_string("reg_a");
                let b_value = cpu.read_register_string("reg_b");
                let sum_value = add_u8_arrays(a_value.unwrap(), b_value.unwrap());
                cpu.accumulator = sum_value;
            }
            SubInstructions::PushToStack => {
                let stack_pointer = cpu.read_register_string("stack_pointer").unwrap()[0] as usize;
                let destination = stack_pointer - cpu.accumulator.len() + 1;
                cpu.memory.write_chunk(destination, cpu.accumulator.as_slice());
                let _ = cpu.write_register_string("stack_pointer", &[destination as u8 - 1]);
            }
            SubInstructions::PopFromStack => {
                let stack_pointer = cpu.read_register_string("stack_pointer").unwrap()[0] as usize + 1;
                cpu.accumulator = cpu.memory.read_chunk(stack_pointer, stack_pointer + cpu.cpu_data_size as usize).to_vec();
                cpu.memory.write_chunk(stack_pointer, vec![0; cpu.cpu_data_size as usize].as_slice());
                let _ = cpu.write_register_string("stack_pointer", &[stack_pointer as u8]);
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

use std::{collections::HashMap, hash::Hash, i32, ops::Sub, vec};

pub struct CPU {
    pub program_counter: usize,
    pub accumulator: Vec<u8>,
    pub register_data: Vec<u8>,
    pub registers: Registers,
    pub memory: Memory,
    pub storage: Storage,
    pub instruction_set: HashMap<u8, Instruction>,
    pub cpu_data_size: u8,

    pub current_opcode: Option<u8>,
    pub current_sub_step: u8,
    pub halted: bool,

}

impl CPU {
    pub fn new(registers: Registers, memory: Memory, storage: Storage) -> Self {
        let mut cpu = CPU { 
            program_counter: 0, 
            accumulator: vec![0], 
            register_data: vec![0; registers.total_length], 
            registers, 
            memory, 
            storage, 
            instruction_set: HashMap::new(),
            cpu_data_size: 1,
            current_opcode: None,
            current_sub_step: 0,
            halted: false,
        };
        cpu.write_register_string("stack_pointer", &[cpu.memory.data.len() as u8 - 1]).unwrap();
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
        let value = self.memory.read(self.program_counter);
        value
    }

    pub fn read_program_memory_offset(&self, offset: u8) -> u8 {
        self.memory.read(self.program_counter + offset as usize)
    }

    pub fn step_size(&mut self, size: u8) {
        self.program_counter += size as usize;
    }

    pub fn step(&mut self) {
        self.program_counter += 1;
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
    registers.add_register("stack_pointer".to_string(), 1, 0);
    registers.add_register("return_address".to_string(), 1, 1);
    registers.add_register("reg_a".to_string(), 1, 2);
    registers.add_register("reg_b".to_string(), 1, 3);
    registers.add_register("reg_0".to_string(), 1, 4);
    registers.add_register("reg_1".to_string(), 1, 5);
    registers.add_register("reg_2".to_string(), 1, 6);

    CPU::new(registers, memory, storage)
}

pub fn create_default_cpu(memory_size: usize, storage_size: usize) -> CPU {
    let memory = Memory::new(memory_size);
    let storage = Storage::new(storage_size);
    create_default_cpu_with_memory(memory, storage)
}



