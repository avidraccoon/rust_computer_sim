use crate::computer::{flag_invert, SubInstructions, FLAG_NONE, ZERO_FLAG, GREATER_FLAG};
use crate::writers::InstructionSetWriter;
use crate::InstructionSet;



pub fn add_instructions(instruction_set_writer: &mut InstructionSetWriter, mut ref_reg: impl FnMut(&str) -> u8, cpu_data_size: u8) {

    let reg_a = ref_reg("reg_a");
    let reg_b = ref_reg("reg_b");
    let reg_c = ref_reg("reg_c");
    let i_temp_0 = ref_reg("instruction_temp_0");
    let i_temp_1 = ref_reg("instruction_temp_1");
    let i_temp_2 = ref_reg("instruction_temp_2");
    let i_temp_3 = ref_reg("instruction_temp_3");
    let program_counter = ref_reg("program_counter");
    let stack_pointer = ref_reg("stack_pointer");
    let base_pointer = ref_reg("base_pointer");
    let flags = ref_reg("flags");

    instruction_set_writer.add_instruction(InstructionSet::NoOperation, 0);

    instruction_set_writer.add_instruction(InstructionSet::Halt, 0)
        .add_sub_instruction(SubInstructions::Halt);

    instruction_set_writer.add_instruction(InstructionSet::LoadFromMemory, 2)
        .add_sub_instruction(SubInstructions::LoadImmediate(1))
        .add_sub_instruction(SubInstructions::SetMemoryAddress)
        .add_sub_instruction(SubInstructions::LoadFromMemory)
        .add_sub_instruction(SubInstructions::StoreToRegister(2))
        .add_sub_instruction(SubInstructions::StepProgramMemory(2));

    instruction_set_writer.add_instruction(InstructionSet::StoreToMemory, 2)
        .add_sub_instruction(SubInstructions::LoadFromRegister(1))
        .add_sub_instruction(SubInstructions::LoadImmediate(2))
        .add_sub_instruction(SubInstructions::SetMemoryAddress)
        .add_sub_instruction(SubInstructions::StoreToMemory)
        .add_sub_instruction(SubInstructions::StepProgramMemory(2));

    instruction_set_writer.add_instruction(InstructionSet::LoadFromMemoryReg, 2)
        .add_sub_instruction(SubInstructions::LoadFromRegister(1))
        .add_sub_instruction(SubInstructions::SetMemoryAddress)
        .add_sub_instruction(SubInstructions::LoadFromMemory)
        .add_sub_instruction(SubInstructions::StoreToRegister(2))
        .add_sub_instruction(SubInstructions::StepProgramMemory(2));

    instruction_set_writer.add_instruction(InstructionSet::StoreToMemoryReg, 2)
        .add_sub_instruction(SubInstructions::LoadFromRegister(2))
        .add_sub_instruction(SubInstructions::SetMemoryAddress)
        .add_sub_instruction(SubInstructions::LoadFromRegister(1))
        .add_sub_instruction(SubInstructions::StoreToMemory)
        .add_sub_instruction(SubInstructions::StepProgramMemory(2));

    instruction_set_writer.add_instruction(InstructionSet::AddImmediate, 2)
        .add_sub_instruction(SubInstructions::LoadFromRegister(1))
        .add_sub_instruction(SubInstructions::StoreToRegisterInternal(reg_a))
        .add_sub_instruction(SubInstructions::LoadImmediate(2))
        .add_sub_instruction(SubInstructions::StoreToRegisterInternal(reg_b))
        .add_sub_instruction(SubInstructions::Add)
        .add_sub_instruction(SubInstructions::StoreToRegister(1))
        .add_sub_instruction(SubInstructions::StepProgramMemory(2));

    instruction_set_writer.add_instruction(InstructionSet::AddReg, 2)
        .add_sub_instruction(SubInstructions::LoadFromRegister(1))
        .add_sub_instruction(SubInstructions::StoreToRegisterInternal(reg_a))
        .add_sub_instruction(SubInstructions::LoadFromRegister(2))
        .add_sub_instruction(SubInstructions::StoreToRegisterInternal(reg_b))
        .add_sub_instruction(SubInstructions::Add)
        .add_sub_instruction(SubInstructions::StoreToRegister(1))
        .add_sub_instruction(SubInstructions::StepProgramMemory(2));

    instruction_set_writer.add_instruction(InstructionSet::SubImmediate, 2)
        .add_sub_instruction(SubInstructions::LoadFromRegister(1))
        .add_sub_instruction(SubInstructions::StoreToRegisterInternal(reg_a))
        .add_sub_instruction(SubInstructions::LoadImmediate(2))
        .add_sub_instruction(SubInstructions::StoreToRegisterInternal(reg_b))
        .add_sub_instruction(SubInstructions::Sub)
        .add_sub_instruction(SubInstructions::StoreToRegister(1))
        .add_sub_instruction(SubInstructions::StepProgramMemory(2));

    instruction_set_writer.add_instruction(InstructionSet::SubReg, 2)
        .add_sub_instruction(SubInstructions::LoadFromRegister(1))
        .add_sub_instruction(SubInstructions::StoreToRegisterInternal(reg_a))
        .add_sub_instruction(SubInstructions::LoadFromRegister(2))
        .add_sub_instruction(SubInstructions::StoreToRegisterInternal(reg_b))
        .add_sub_instruction(SubInstructions::Sub)
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

    instruction_set_writer.add_instruction(InstructionSet::Jump, 1)
        // Start of address storing
        // saving base_pointer
        .add_sub_instruction(SubInstructions::LoadFromRegisterInternal(base_pointer))
        .add_sub_instruction(SubInstructions::PushToStack)
        // saving flags
        .add_sub_instruction(SubInstructions::LoadFromRegisterInternal(flags))
        .add_sub_instruction(SubInstructions::PushToStack)
        // calulating program counter at end of current instruction
        .add_sub_instruction(SubInstructions::LoadFromRegisterInternal(program_counter))
        .add_sub_instruction(SubInstructions::StoreToRegisterInternal(reg_a))
        .add_sub_instruction(SubInstructions::LoadImmediateInternal(3))
        .add_sub_instruction(SubInstructions::StoreToRegisterInternal(reg_b))
        .add_sub_instruction(SubInstructions::Add)
        // Return flags to saved state
        .add_sub_instruction(SubInstructions::PopFromStack)
        .add_sub_instruction(SubInstructions::StoreToRegisterInternal(flags))
        // Storing calculated return address
        .add_sub_instruction(SubInstructions::LoadFromRegister(reg_a))
        .add_sub_instruction(SubInstructions::PushToStack)
        // END of address storing
        .add_sub_instruction(SubInstructions::LoadImmediate(1))
        .add_sub_instruction(SubInstructions::Jump);

    instruction_set_writer.add_instruction(InstructionSet::JumpReg, 1)
        // TODO: Maybe make use of the stack.
        // Start of address storing
        .add_sub_instruction(SubInstructions::LoadFromRegister(program_counter))
        // .add_sub_instruction(SubInstructions::StoreToRegisterInternal(return_address))
        // END of address storing
        .add_sub_instruction(SubInstructions::LoadFromRegister(1))
        .add_sub_instruction(SubInstructions::Jump);

    let equal_mask_true         = ZERO_FLAG.clone();
    let equal_mask_false        = GREATER_FLAG.clone();
    let not_equal_mask_true     = FLAG_NONE.clone();
    let not_equal_mask_false    = ZERO_FLAG.clone();
    let greater_mask_true       = GREATER_FLAG.clone();
    let greater_mask_false      = ZERO_FLAG.clone();
    let less_mask_true          = FLAG_NONE.clone();
    let less_mask_false         = ZERO_FLAG.clone() | GREATER_FLAG.clone();
    let less_equal_mask_true    = FLAG_NONE.clone();
    let less_equal_mask_false   = GREATER_FLAG.clone();
    let great_equal_mask_true   = less_equal_mask_true.clone();
    let great_equal_mask_false  = less_equal_mask_false.clone();

    instruction_set_writer.add_instruction(InstructionSet::JumpEqual, 3)
        .add_sub_instruction(SubInstructions::LoadFromRegister(2))
        .add_sub_instruction(SubInstructions::StoreToRegisterInternal(reg_a))
        .add_sub_instruction(SubInstructions::LoadImmediate(3))
        .add_sub_instruction(SubInstructions::StoreToRegisterInternal(reg_b))
        .add_sub_instruction(SubInstructions::Compare)
        // TODO: Maybe make use of the stack.
        // Start of address storing
        .add_sub_instruction(SubInstructions::LoadFromRegister(program_counter))
        // .add_sub_instruction(SubInstructions::StoreToRegisterInternal(return_address))
        // END of address storing
        .add_sub_instruction(SubInstructions::LoadFromRegister(1))
        .add_sub_instruction(SubInstructions::JumpIfFlag(equal_mask_true.clone(), equal_mask_false.clone()))
        .add_sub_instruction(SubInstructions::StepProgramMemory(3));

    instruction_set_writer.add_instruction(InstructionSet::JumpEqualReg, 3)
        .add_sub_instruction(SubInstructions::LoadFromRegister(2))
        .add_sub_instruction(SubInstructions::StoreToRegisterInternal(reg_a))
        .add_sub_instruction(SubInstructions::LoadFromRegister(3))
        .add_sub_instruction(SubInstructions::StoreToRegisterInternal(reg_b))
        .add_sub_instruction(SubInstructions::Compare)
        // TODO: Maybe make use of the stack.
        // Start of address storing
        .add_sub_instruction(SubInstructions::LoadFromRegister(program_counter))
        // .add_sub_instruction(SubInstructions::StoreToRegisterInternal(return_address))
        // END of address storing
        .add_sub_instruction(SubInstructions::LoadFromRegister(1))
        .add_sub_instruction(SubInstructions::JumpIfFlag(equal_mask_true.clone(), equal_mask_false.clone()))
        .add_sub_instruction(SubInstructions::StepProgramMemory(3));

    instruction_set_writer.add_instruction(InstructionSet::JumpNotEqual, 3)
        .add_sub_instruction(SubInstructions::LoadFromRegister(2))
        .add_sub_instruction(SubInstructions::StoreToRegisterInternal(reg_a))
        .add_sub_instruction(SubInstructions::LoadImmediate(3))
        .add_sub_instruction(SubInstructions::StoreToRegisterInternal(reg_b))
        .add_sub_instruction(SubInstructions::Compare)
        // TODO: Maybe make use of the stack.
        // Start of address storing
        .add_sub_instruction(SubInstructions::LoadFromRegister(program_counter))
        // .add_sub_instruction(SubInstructions::StoreToRegisterInternal(return_address))
        // END of address storing
        .add_sub_instruction(SubInstructions::LoadFromRegister(1))
        .add_sub_instruction(SubInstructions::JumpIfFlag(not_equal_mask_true.clone(), not_equal_mask_false.clone()))
        .add_sub_instruction(SubInstructions::StepProgramMemory(3));

    instruction_set_writer.add_instruction(InstructionSet::JumpNotEqualReg, 3)
        .add_sub_instruction(SubInstructions::LoadFromRegister(2))
        .add_sub_instruction(SubInstructions::StoreToRegisterInternal(reg_a))
        .add_sub_instruction(SubInstructions::LoadFromRegister(3))
        .add_sub_instruction(SubInstructions::StoreToRegisterInternal(reg_b))
        .add_sub_instruction(SubInstructions::Compare)
        // TODO: Maybe make use of the stack.
        // Start of address storing
        .add_sub_instruction(SubInstructions::LoadFromRegister(program_counter))
        // .add_sub_instruction(SubInstructions::StoreToRegisterInternal(return_address))
        // END of address storing
        .add_sub_instruction(SubInstructions::LoadFromRegister(1))
        .add_sub_instruction(SubInstructions::JumpIfFlag(not_equal_mask_true.clone(), not_equal_mask_false.clone()))
        .add_sub_instruction(SubInstructions::StepProgramMemory(3));

    instruction_set_writer.add_instruction(InstructionSet::JumpGreaterThan, 3)
        .add_sub_instruction(SubInstructions::LoadFromRegister(2))
        .add_sub_instruction(SubInstructions::StoreToRegisterInternal(reg_a))
        .add_sub_instruction(SubInstructions::LoadImmediate(3))
        .add_sub_instruction(SubInstructions::StoreToRegisterInternal(reg_b))
        .add_sub_instruction(SubInstructions::Compare)
        // TODO: Maybe make use of the stack.
        // Start of address storing
        .add_sub_instruction(SubInstructions::LoadFromRegister(program_counter))
        // .add_sub_instruction(SubInstructions::StoreToRegisterInternal(return_address))
        // END of address storing
        .add_sub_instruction(SubInstructions::LoadFromRegister(1))
        .add_sub_instruction(SubInstructions::JumpIfFlag(greater_mask_true.clone(), greater_mask_false.clone()))
        .add_sub_instruction(SubInstructions::StepProgramMemory(3));

    instruction_set_writer.add_instruction(InstructionSet::JumpGreaterThanReg, 3)
        .add_sub_instruction(SubInstructions::LoadFromRegister(2))
        .add_sub_instruction(SubInstructions::StoreToRegisterInternal(reg_a))
        .add_sub_instruction(SubInstructions::LoadFromRegister(3))
        .add_sub_instruction(SubInstructions::StoreToRegisterInternal(reg_b))
        .add_sub_instruction(SubInstructions::Compare)
        // TODO: Maybe make use of the stack.
        // Start of address storing
        .add_sub_instruction(SubInstructions::LoadFromRegister(program_counter))
        // .add_sub_instruction(SubInstructions::StoreToRegisterInternal(return_address))
        // END of address storing
        .add_sub_instruction(SubInstructions::LoadFromRegister(1))
        .add_sub_instruction(SubInstructions::JumpIfFlag(greater_mask_true.clone(), greater_mask_false.clone()))
        .add_sub_instruction(SubInstructions::StepProgramMemory(3));

    instruction_set_writer.add_instruction(InstructionSet::JumpLessThan, 3)
        .add_sub_instruction(SubInstructions::LoadFromRegister(2))
        .add_sub_instruction(SubInstructions::StoreToRegisterInternal(reg_a))
        .add_sub_instruction(SubInstructions::LoadImmediate(3))
        .add_sub_instruction(SubInstructions::StoreToRegisterInternal(reg_b))
        .add_sub_instruction(SubInstructions::Compare)
        // TODO: Maybe make use of the stack.
        // Start of address storing
        .add_sub_instruction(SubInstructions::LoadFromRegister(program_counter))
        // .add_sub_instruction(SubInstructions::StoreToRegisterInternal(return_address))
        // END of address storing
        .add_sub_instruction(SubInstructions::LoadFromRegister(1))
        .add_sub_instruction(SubInstructions::JumpIfFlag(less_mask_true.clone(), less_mask_false.clone()))
        .add_sub_instruction(SubInstructions::StepProgramMemory(3));

    instruction_set_writer.add_instruction(InstructionSet::JumpLessThanReg, 3)
        .add_sub_instruction(SubInstructions::LoadFromRegister(2))
        .add_sub_instruction(SubInstructions::StoreToRegisterInternal(reg_a))
        .add_sub_instruction(SubInstructions::LoadFromRegister(3))
        .add_sub_instruction(SubInstructions::StoreToRegisterInternal(reg_b))
        .add_sub_instruction(SubInstructions::Compare)
        // TODO: Maybe make use of the stack.
        // Start of address storing
        .add_sub_instruction(SubInstructions::LoadFromRegister(program_counter))
        // .add_sub_instruction(SubInstructions::StoreToRegisterInternal(return_address))
        // END of address storing
        .add_sub_instruction(SubInstructions::LoadFromRegister(1))
        .add_sub_instruction(SubInstructions::JumpIfFlag(less_mask_true.clone(), less_mask_false.clone()))
        .add_sub_instruction(SubInstructions::StepProgramMemory(3));

    instruction_set_writer.add_instruction(InstructionSet::JumpLessEqual, 3)
        .add_sub_instruction(SubInstructions::LoadFromRegister(2))
        .add_sub_instruction(SubInstructions::StoreToRegisterInternal(reg_a))
        .add_sub_instruction(SubInstructions::LoadImmediate(3))
        .add_sub_instruction(SubInstructions::StoreToRegisterInternal(reg_b))
        .add_sub_instruction(SubInstructions::Compare)
        // TODO: Maybe make use of the stack.
        // Start of address storing
        .add_sub_instruction(SubInstructions::LoadFromRegister(program_counter))
        // .add_sub_instruction(SubInstructions::StoreToRegisterInternal(return_address))
        // END of address storing
        .add_sub_instruction(SubInstructions::LoadFromRegister(1))
        .add_sub_instruction(SubInstructions::JumpIfFlag(less_equal_mask_true.clone(), less_equal_mask_false.clone()))
        .add_sub_instruction(SubInstructions::StepProgramMemory(3));

    instruction_set_writer.add_instruction(InstructionSet::JumpLessEqualReg, 3)
        .add_sub_instruction(SubInstructions::LoadFromRegister(2))
        .add_sub_instruction(SubInstructions::StoreToRegisterInternal(reg_a))
        .add_sub_instruction(SubInstructions::LoadFromRegister(3))
        .add_sub_instruction(SubInstructions::StoreToRegisterInternal(reg_b))
        .add_sub_instruction(SubInstructions::Compare)
        // TODO: Maybe make use of the stack.
        // Start of address storing
        .add_sub_instruction(SubInstructions::LoadFromRegister(program_counter))
        // .add_sub_instruction(SubInstructions::StoreToRegisterInternal(return_address))
        // END of address storing
        .add_sub_instruction(SubInstructions::LoadFromRegister(1))
        .add_sub_instruction(SubInstructions::JumpIfFlag(less_equal_mask_true.clone(), less_equal_mask_false.clone()))
        .add_sub_instruction(SubInstructions::StepProgramMemory(3));

    instruction_set_writer.add_instruction(InstructionSet::JumpGreaterEqual, 3)
        .add_sub_instruction(SubInstructions::LoadFromRegister(2))
        .add_sub_instruction(SubInstructions::StoreToRegisterInternal(reg_a))
        .add_sub_instruction(SubInstructions::LoadImmediate(3))
        .add_sub_instruction(SubInstructions::StoreToRegisterInternal(reg_b))
        .add_sub_instruction(SubInstructions::Compare)
        // TODO: Maybe make use of the stack.
        // Start of address storing
        .add_sub_instruction(SubInstructions::LoadFromRegister(program_counter))
        // .add_sub_instruction(SubInstructions::StoreToRegisterInternal(return_address))
        // END of address storing
        .add_sub_instruction(SubInstructions::LoadFromRegister(1))
        .add_sub_instruction(SubInstructions::JumpIfFlag(great_equal_mask_true.clone(), great_equal_mask_false.clone()))
        .add_sub_instruction(SubInstructions::StepProgramMemory(3));

    instruction_set_writer.add_instruction(InstructionSet::JumpGreaterEqualReg, 3)
        .add_sub_instruction(SubInstructions::LoadFromRegister(2))
        .add_sub_instruction(SubInstructions::StoreToRegisterInternal(reg_a))
        .add_sub_instruction(SubInstructions::LoadFromRegister(3))
        .add_sub_instruction(SubInstructions::StoreToRegisterInternal(reg_b))
        .add_sub_instruction(SubInstructions::Compare)
        // Start of address storing
        // saving base_pointer
        .add_sub_instruction(SubInstructions::LoadFromRegisterInternal(base_pointer))
        .add_sub_instruction(SubInstructions::PushToStack)
        // saving flags
        .add_sub_instruction(SubInstructions::LoadFromRegisterInternal(flags))
        .add_sub_instruction(SubInstructions::PushToStack)
        // calulating program counter at end of current instruction
        .add_sub_instruction(SubInstructions::LoadFromRegisterInternal(program_counter))
        .add_sub_instruction(SubInstructions::StoreToRegisterInternal(reg_a))
        .add_sub_instruction(SubInstructions::LoadImmediateInternal(3))
        .add_sub_instruction(SubInstructions::StoreToRegisterInternal(reg_b))
        .add_sub_instruction(SubInstructions::Add)
        // Return flags to saved state
        .add_sub_instruction(SubInstructions::PopFromStack)
        .add_sub_instruction(SubInstructions::StoreToRegisterInternal(flags))
        // Storing calculated return address
        .add_sub_instruction(SubInstructions::LoadFromRegister(reg_a))
        .add_sub_instruction(SubInstructions::PushToStack)
        // END of address storing
        .add_sub_instruction(SubInstructions::LoadFromRegister(1))
        .add_sub_instruction(SubInstructions::JumpIfFlag(great_equal_mask_true.clone(), great_equal_mask_false.clone()))
        .add_sub_instruction(SubInstructions::StepProgramMemory(3));

    instruction_set_writer.add_instruction(InstructionSet::Return, 0)
        .add_sub_instruction(SubInstructions::PopFromStack) // Get return address
        .add_sub_instruction(SubInstructions::StoreToRegisterInternal(program_counter))
        .add_sub_instruction(SubInstructions::PopFromStack)//Recover from saved
        .add_sub_instruction(SubInstructions::)


}