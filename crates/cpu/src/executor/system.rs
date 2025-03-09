use crate::{CPU, RegisterData, RegisterType, instruction::DecodedInstruction};
use common::Result;
use error::Error;
use types::Instruction;
impl CPU {
    pub(super) fn execute_system(&mut self, decoded: DecodedInstruction) -> Result<()> {
        println!(
            "[CPU] Executing system instruction: {:?}",
            decoded.instruction
        );
        match decoded.instruction {
            Instruction::BRK => self.brk(),
            Instruction::RTI => self.rti(),
            Instruction::NOP => self.nop(),
            _ => Err(Error::InvalidInstruction {
                inst_type: "system",
            }),
        }
    }

    fn brk(&mut self) -> Result<()> {
        println!("[CPU] Executing BRK");
        let pc = self.get_pc();
        let mut p = self.get_value(RegisterType::P).as_u8();

        // Push PC+2 onto stack (PC+1 for the BRK instruction, +1 for the padding byte)
        self.stack_push_u16(pc.wrapping_add(2))?;

        // Set Break flag and push status register onto stack
        p |= 0x10; // Set Break flag
        self.stack_push(p)?;

        // Set Interrupt Disable flag while preserving other flags
        p |= 0x04; // Set Interrupt Disable flag
        self.set_value(RegisterType::P, RegisterData::Bit8(p));

        // Halt CPU with BRK reason
        self.halt_with_reason(crate::cpu::InterruptType::BRK);

        Ok(())
    }

    fn rti(&mut self) -> Result<()> {
        println!("[CPU] Executing RTI");

        // Pull status register from stack (ignore Break and Unused flags)
        let status = self.stack_pull()?;
        self.set_value(RegisterType::P, RegisterData::Bit8(status & 0xEF));

        // Pull program counter from stack
        let pc = self.stack_pull_u16()?;
        self.set_pc(pc);

        Ok(())
    }

    fn nop(&mut self) -> Result<()> {
        println!("[CPU] Executing NOP");
        Ok(())
    }
}
