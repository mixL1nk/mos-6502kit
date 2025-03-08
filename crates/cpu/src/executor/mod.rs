pub mod arithmetic;
pub mod compare;
pub mod flag;
pub mod jump;
pub mod logical;
pub mod shift;
pub mod stack;
pub mod system;
pub mod transfer;

use crate::CPU;
use crate::instruction::DecodedInstruction;
use types::Instruction;

impl CPU {
    pub(super) fn execute(&mut self, decoded: DecodedInstruction) -> common::Result<()> {
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

            // Logical instructions
            Instruction::AND(_)
            | Instruction::ORA(_)
            | Instruction::EOR(_)
            | Instruction::BIT(_) => self.execute_logical(decoded),

            // Stack instructions
            Instruction::PHA | Instruction::PHP | Instruction::PLA | Instruction::PLP => {
                self.execute_stack(decoded)
            }

            // Jump and branch instructions
            Instruction::JMP(_)
            | Instruction::JSR(_)
            | Instruction::RTS
            | Instruction::BCC(_)
            | Instruction::BCS(_)
            | Instruction::BEQ(_)
            | Instruction::BNE(_)
            | Instruction::BMI(_)
            | Instruction::BPL(_)
            | Instruction::BVC(_)
            | Instruction::BVS(_) => self.execute_jump(decoded),

            // Shift and rotate instructions
            Instruction::ASL(_)
            | Instruction::LSR(_)
            | Instruction::ROL(_)
            | Instruction::ROR(_) => self.execute_shift(decoded),

            // System instructions
            Instruction::BRK | Instruction::RTI | Instruction::NOP => self.execute_system(decoded),

            // Compare instructions
            Instruction::CMP(_) | Instruction::CPX(_) | Instruction::CPY(_) => {
                self.execute_compare(decoded)
            }

            // Flag instructions
            Instruction::CLC
            | Instruction::CLD
            | Instruction::CLI
            | Instruction::CLV
            | Instruction::SEC
            | Instruction::SED
            | Instruction::SEI => self.execute_flag(decoded),
        }
    }
}
