// ## FIB
// program_writer
//     .add_instruction(InstructionSet::LoadImmediate, &[reg_0, 0])    // Load 0 into reg_0
//     .add_instruction(InstructionSet::LoadImmediate, &[reg_1, 1]);    // Load 1 into reg_1
// for _ in 0..5 {
// program_writer
//     .add_instruction(InstructionSet::MoveRegister, &[reg_1, reg_2]) // Move reg_1 into reg_2
//     .add_instruction(InstructionSet::AddReg, &[reg_1, reg_0])       // Add reg_0 and reg_1 and store result in reg_1
//     .add_instruction(InstructionSet::MoveRegister, &[reg_2, reg_0]); // Move reg_2 into reg_0
// }
// program_writer
//     .add_instruction(InstructionSet::Halt, &[]);                                       // Halt the program