use crate::computer::SubInstructions;
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
        .add_sub_instruction(SubInstructions::LoadImmediate(1))
        .add_sub_instruction(SubInstructions::Jump);

    instruction_set_writer.add_instruction(InstructionSet::JumpReg, 1)
        .add_sub_instruction(SubInstructions::LoadFromRegister(1))
        .add_sub_instruction(SubInstructions::Jump);
}