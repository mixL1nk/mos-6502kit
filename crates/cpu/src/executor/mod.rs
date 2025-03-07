pub mod arithmetic;
pub mod transfer;

use crate::CPU;
use crate::instruction::{DecodedInstruction, Instruction};

pub trait InstructionExecutor {
    fn execute(&mut self, decoded: DecodedInstruction) -> common::Result<()>;
}

impl InstructionExecutor for CPU {
    fn execute(&mut self, decoded: DecodedInstruction) -> common::Result<()> {
        match decoded.info.instruction {
            // Transfer instructions
            Instruction::LDA(_)
            | Instruction::LDX(_)
            | Instruction::LDY(_)
            | Instruction::STA(_)
            | Instruction::STX(_)
            | Instruction::STY(_)
            | Instruction::TAX
            | Instruction::TAY
            | Instruction::TSX
            | Instruction::TXA
            | Instruction::TXS
            | Instruction::TYA => self.execute_transfer(decoded),

            // Arithmetic instructions
            Instruction::ADC(_)
            | Instruction::SBC(_)
            | Instruction::INC(_)
            | Instruction::DEC(_)
            | Instruction::INX
            | Instruction::INY
            | Instruction::DEX
            | Instruction::DEY => self.execute_arithmetic(decoded),

            _ => Err("Unsupported instruction".into()),
        }
    }
}
