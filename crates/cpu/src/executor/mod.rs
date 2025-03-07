use crate::instruction::DecodedInstruction;

pub trait InstructionExecutor {
    fn execute(&mut self, instruction: DecodedInstruction) -> Result<(), String>;
}

pub mod arithmetic;
