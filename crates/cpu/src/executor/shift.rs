use crate::{
    CPU, RegisterData, RegisterType, instruction::DecodedInstruction, register::StatusRegister,
};
use common::Result;
use error::Error;
use types::{AddressMode, Instruction};

impl CPU {
    pub(super) fn execute_shift(&mut self, decoded: DecodedInstruction) -> Result<()> {
        println!(
            "[CPU] Executing shift instruction: {:?}",
            decoded.info.instruction
        );
        match decoded.info.instruction {
            Instruction::ASL(mode) => self.asl(mode, decoded),
            Instruction::LSR(mode) => self.lsr(mode, decoded),
            Instruction::ROL(mode) => self.rol(mode, decoded),
            Instruction::ROR(mode) => self.ror(mode, decoded),
            _ => Err(Error::InvalidInstruction { inst_type: "shift" }),
        }
    }

    fn asl(&mut self, mode: AddressMode, decode: DecodedInstruction) -> Result<()> {
        println!("[CPU] Executing ASL with operand: 0x{:04X}", decode.operand);
        match mode {
            AddressMode::Accumulator => {
                let value = self.get_value(RegisterType::A).as_u8();
                let carry = (value & 0x80) != 0;
                let result = value << 1;
                self.set_value(RegisterType::A, RegisterData::Bit8(result));
                self.update_flags_arithmetic(result, carry, false);
                Ok(())
            }
            _ => {
                let addr = decode.operand;
                let value = self.read_memory(addr)?;
                let carry = (value & 0x80) != 0;
                let result = value << 1;
                self.write_memory(addr, result)?;
                self.update_flags_arithmetic(result, carry, false);
                Ok(())
            }
        }
    }

    fn lsr(&mut self, mode: AddressMode, decode: DecodedInstruction) -> Result<()> {
        println!("[CPU] Executing LSR with operand: 0x{:04X}", decode.operand);
        match mode {
            AddressMode::Accumulator => {
                let value = self.get_value(RegisterType::A).as_u8();
                let carry = (value & 0x01) != 0;
                let result = value >> 1;
                self.set_value(RegisterType::A, RegisterData::Bit8(result));
                self.update_flags_arithmetic(result, carry, false);
                Ok(())
            }
            _ => {
                let addr = decode.operand;
                let value = self.read_memory(addr)?;
                let carry = (value & 0x01) != 0;
                let result = value >> 1;
                self.write_memory(addr, result)?;
                self.update_flags_arithmetic(result, carry, false);
                Ok(())
            }
        }
    }

    fn rol(&mut self, mode: AddressMode, decode: DecodedInstruction) -> Result<()> {
        println!("[CPU] Executing ROL with operand: 0x{:04X}", decode.operand);
        let status = self.status_flag();
        let old_carry = status.contains(StatusRegister::CARRY);

        match mode {
            AddressMode::Accumulator => {
                let value = self.get_value(RegisterType::A).as_u8();
                let new_carry = (value & 0x80) != 0;
                let result = (value << 1) | (old_carry as u8);
                self.set_value(RegisterType::A, RegisterData::Bit8(result));
                self.update_flags_arithmetic(result, new_carry, false);
                Ok(())
            }
            _ => {
                let addr = decode.operand;
                let value = self.read_memory(addr)?;
                let new_carry = (value & 0x80) != 0;
                let result = (value << 1) | (old_carry as u8);
                self.write_memory(addr, result)?;
                self.update_flags_arithmetic(result, new_carry, false);
                Ok(())
            }
        }
    }

    fn ror(&mut self, mode: AddressMode, decode: DecodedInstruction) -> Result<()> {
        println!("[CPU] Executing ROR with operand: 0x{:04X}", decode.operand);
        let status = self.status_flag();
        let old_carry = status.contains(StatusRegister::CARRY);

        match mode {
            AddressMode::Accumulator => {
                let value = self.get_value(RegisterType::A).as_u8();
                let new_carry = (value & 0x01) != 0;
                let result = (value >> 1) | ((old_carry as u8) << 7);
                self.set_value(RegisterType::A, RegisterData::Bit8(result));
                self.update_flags_arithmetic(result, new_carry, false);
                Ok(())
            }
            _ => {
                let addr = decode.operand;
                let value = self.read_memory(addr)?;
                let new_carry = (value & 0x01) != 0;
                let result = (value >> 1) | ((old_carry as u8) << 7);
                self.write_memory(addr, result)?;
                self.update_flags_arithmetic(result, new_carry, false);
                Ok(())
            }
        }
    }
}
