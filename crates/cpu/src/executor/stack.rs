use crate::{CPU, RegisterData, RegisterType, instruction::DecodedInstruction};
use common::Result;
use error::Error;
use types::Instruction;

impl CPU {
    pub(super) fn execute_stack(&mut self, decoded: DecodedInstruction) -> Result<()> {
        println!(
            "[CPU] Executing stack instruction: {:?}",
            decoded.instruction
        );
        match decoded.instruction {
            Instruction::PHA => self.pha(),
            Instruction::PHP => self.php(),
            Instruction::PLA => self.pla(),
            Instruction::PLP => self.plp(),
            _ => Err(Error::InvalidInstruction { inst_type: "stack" }),
        }
    }

    fn pha(&mut self) -> Result<()> {
        // println!("[CPU] Executing PHA");
        let a = self.get_value(RegisterType::A).as_u8();
        self.stack_push(a)
    }

    fn php(&mut self) -> Result<()> {
        // println!("[CPU] Executing PHP");
        let p = self.get_value(RegisterType::P).as_u8();
        // Break and Unused flags are set when pushed
        self.stack_push(p | 0x30)
    }

    fn pla(&mut self) -> Result<()> {
        // println!("[CPU] Executing PLA");
        let value = self.stack_pull()?;
        self.set_value(RegisterType::A, RegisterData::Bit8(value));
        self.update_nz_flags(value);
        Ok(())
    }

    fn plp(&mut self) -> Result<()> {
        // println!("[CPU] Executing PLP");
        let value = self.stack_pull()?;
        // Break and Unused flags are ignored when pulled
        let current_p = self.get_value(RegisterType::P).as_u8();
        let new_p = (value & 0xCF) | (current_p & 0x30);
        self.set_value(RegisterType::P, RegisterData::Bit8(new_p));
        Ok(())
    }
}
