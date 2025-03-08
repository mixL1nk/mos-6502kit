use crate::{
    CPU, RegisterData, RegisterType,
    instruction::{DecodedInstruction, Instruction},
};
use common::Result;

impl CPU {
    pub(super) fn execute_system(&mut self, decoded: DecodedInstruction) -> Result<()> {
        println!(
            "[CPU] Executing system instruction: {:?}",
            decoded.info.instruction
        );
        match decoded.info.instruction {
            Instruction::BRK => self.brk(),
            Instruction::RTI => self.rti(),
            Instruction::NOP => self.nop(),
            _ => Err("Invalid system instruction".into()),
        }
    }

    fn brk(&mut self) -> Result<()> {
        println!("[CPU] Executing BRK");
        let pc = self.get_pc();
        let p = self.get_value(RegisterType::P).as_u8();

        // Push PC+2 onto stack (PC+1 for the BRK instruction, +1 for the padding byte)
        self.stack_push_u16(pc.wrapping_add(2))?;

        // Push status register onto stack with Break flag set
        self.stack_push(p | 0x10)?;

        // Set interrupt disable flag
        self.set_value(RegisterType::P, RegisterData::Bit8(p | 0x04));

        // // Load interrupt vector
        // let low = self.read_memory(0xFFFE)?;
        // let high = self.read_memory(0xFFFF)?;
        // let irq_vector = ((high as u16) << 8) | (low as u16);
        // self.set_pc(irq_vector);

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
