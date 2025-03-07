use super::InstructionExecutor;
use crate::cpu::CPU;
use crate::instruction::{AddressMode, DecodedInstruction, Instruction};

impl InstructionExecutor for CPU {
    fn execute(&mut self, decoded: DecodedInstruction) -> Result<(), String> {
        match decoded.info.instruction {
            Instruction::ADC(mode) => self.adc(mode, decoded.operand),
            _ => Err(format!(
                "Invalid instruction for arithmetic executor: {:?}",
                decoded.info.instruction
            )),
        }
    }
}

impl CPU {
    fn adc(&mut self, mode: AddressMode, operand: u16) -> Result<(), String> {
        // 여기에 ADC 구현
        println!("[CPU] Executing ADC with operand: {:?}", operand);
        // TODO: 실제 ADC 로직 구현
        Ok(())
    }
}
