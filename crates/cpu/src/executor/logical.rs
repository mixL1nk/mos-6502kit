use crate::{
    CPU, RegisterData, RegisterType,
    instruction::{AddressMode, DecodedInstruction, Instruction},
    register::StatusRegister,
};
use common::Result;
use error::Error;

impl CPU {
    pub(super) fn execute_logical(&mut self, decoded: DecodedInstruction) -> Result<()> {
        println!(
            "[CPU] Executing logical instruction: {:?}",
            decoded.info.instruction
        );
        match decoded.info.instruction {
            Instruction::AND(mode) => self.and(mode, decoded),
            Instruction::ORA(mode) => self.ora(mode, decoded),
            Instruction::EOR(mode) => self.eor(mode, decoded),
            Instruction::BIT(mode) => self.bit(mode, decoded),
            _ => Err(Error::InvalidInstruction {
                inst_type: "logical",
            }),
        }
    }

    fn and(&mut self, _mode: AddressMode, decode: DecodedInstruction) -> Result<()> {
        println!("[CPU] Executing AND with operand: 0x{:04X}", decode.operand);
        let value = decode.operand as u8;
        let a = self.get_value(RegisterType::A).as_u8();
        let result = a & value;

        self.set_value(RegisterType::A, RegisterData::Bit8(result));
        self.update_nz_flags(result);
        Ok(())
    }

    fn ora(&mut self, _mode: AddressMode, decode: DecodedInstruction) -> Result<()> {
        println!("[CPU] Executing ORA with operand: 0x{:04X}", decode.operand);
        let value = decode.operand as u8;
        let a = self.get_value(RegisterType::A).as_u8();
        let result = a | value;

        self.set_value(RegisterType::A, RegisterData::Bit8(result));
        self.update_nz_flags(result);
        Ok(())
    }

    fn eor(&mut self, _mode: AddressMode, decode: DecodedInstruction) -> Result<()> {
        println!("[CPU] Executing EOR with operand: 0x{:04X}", decode.operand);
        let value = decode.operand as u8;
        let a = self.get_value(RegisterType::A).as_u8();
        let result = a ^ value;

        self.set_value(RegisterType::A, RegisterData::Bit8(result));
        self.update_nz_flags(result);
        Ok(())
    }

    fn bit(&mut self, _mode: AddressMode, decode: DecodedInstruction) -> Result<()> {
        println!("[CPU] Executing BIT with operand: 0x{:04X}", decode.operand);
        let value = decode.operand as u8;
        let a = self.get_value(RegisterType::A).as_u8();
        let result = a & value;

        self.set_flag(StatusRegister::NEGATIVE, (value & 0x80) != 0);
        self.set_flag(StatusRegister::OVERFLOW, (value & 0x40) != 0);
        self.set_flag(StatusRegister::ZERO, result == 0);

        Ok(())
    }
}
