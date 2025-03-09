use crate::{
    cpu::CPU,
    instruction::DecodedInstruction,
    register::{RegisterData, RegisterType, StatusRegister},
};
use common::Result;
use error::Error;
use types::{AddressModeValue, Instruction};

/// 시프트 연산 구현
pub trait ShiftOperation {
    /// ASL - Arithmetic Shift Left
    /// C <- [76543210] <- 0
    ///
    /// N Z C I D V
    /// + + + - - -
    fn asl(&mut self, mode: AddressModeValue, decode: DecodedInstruction) -> Result<()>;

    /// LSR - Logical Shift Right
    /// 0 -> [76543210] -> C
    ///
    /// N Z C I D V
    /// 0 + + - - -
    fn lsr(&mut self, mode: AddressModeValue, decode: DecodedInstruction) -> Result<()>;

    /// ROL - Rotate Left
    /// C <- [76543210] <- C
    ///
    /// N Z C I D V
    /// + + + - - -
    fn rol(&mut self, mode: AddressModeValue, decode: DecodedInstruction) -> Result<()>;

    /// ROR - Rotate Right
    /// C -> [76543210] -> C
    ///
    /// N Z C I D V
    /// + + + - - -
    fn ror(&mut self, mode: AddressModeValue, decode: DecodedInstruction) -> Result<()>;
}

impl ShiftOperation for CPU {
    fn asl(&mut self, mode: AddressModeValue, _decode: DecodedInstruction) -> Result<()> {
        println!("[CPU] Executing ASL with mode: {:?}", mode);

        match mode {
            AddressModeValue::Accumulator => {
                let value = self.get_value(RegisterType::A).as_u8();
                let result = value << 1;
                self.set_value(RegisterType::A, RegisterData::Bit8(result));
                self.set_flag(StatusRegister::CARRY, value & 0x80 != 0);
                self.set_flag(StatusRegister::ZERO, result == 0);
                self.set_flag(StatusRegister::NEGATIVE, result & 0x80 != 0);
            }
            AddressModeValue::ZeroPage(addr) => {
                let value = self.read_memory(addr as u16)?;
                let result = value << 1;
                self.write_memory(addr as u16, result)?;
                self.set_flag(StatusRegister::CARRY, value & 0x80 != 0);
                self.set_flag(StatusRegister::ZERO, result == 0);
                self.set_flag(StatusRegister::NEGATIVE, result & 0x80 != 0);
            }
            AddressModeValue::ZeroPageX(addr) => {
                let effective_addr =
                    addr.wrapping_add(self.get_value(RegisterType::X).as_u8()) as u16;
                let value = self.read_memory(effective_addr)?;
                let result = value << 1;
                self.write_memory(effective_addr, result)?;
                self.set_flag(StatusRegister::CARRY, value & 0x80 != 0);
                self.set_flag(StatusRegister::ZERO, result == 0);
                self.set_flag(StatusRegister::NEGATIVE, result & 0x80 != 0);
            }
            AddressModeValue::Absolute(addr) => {
                let value = self.read_memory(addr)?;
                let result = value << 1;
                self.write_memory(addr, result)?;
                self.set_flag(StatusRegister::CARRY, value & 0x80 != 0);
                self.set_flag(StatusRegister::ZERO, result == 0);
                self.set_flag(StatusRegister::NEGATIVE, result & 0x80 != 0);
            }
            AddressModeValue::AbsoluteX(addr) => {
                let x = self.get_value(RegisterType::X).as_u8() as u16;
                let effective_addr = addr.wrapping_add(x);
                let value = self.read_memory(effective_addr)?;
                let result = value << 1;
                self.write_memory(effective_addr, result)?;
                self.set_flag(StatusRegister::CARRY, value & 0x80 != 0);
                self.set_flag(StatusRegister::ZERO, result == 0);
                self.set_flag(StatusRegister::NEGATIVE, result & 0x80 != 0);
            }
            _ => return Err(Error::InvalidAddressingMode("ASL")),
        }

        Ok(())
    }

    fn lsr(&mut self, mode: AddressModeValue, _decode: DecodedInstruction) -> Result<()> {
        println!("[CPU] Executing LSR with mode: {:?}", mode);

        match mode {
            AddressModeValue::Accumulator => {
                let value = self.get_value(RegisterType::A).as_u8();
                let result = value >> 1;
                self.set_value(RegisterType::A, RegisterData::Bit8(result));
                self.set_flag(StatusRegister::CARRY, value & 0x01 != 0);
                self.set_flag(StatusRegister::ZERO, result == 0);
                self.set_flag(StatusRegister::NEGATIVE, false);
            }
            AddressModeValue::ZeroPage(addr) => {
                let value = self.read_memory(addr as u16)?;
                let result = value >> 1;
                self.write_memory(addr as u16, result)?;
                self.set_flag(StatusRegister::CARRY, value & 0x01 != 0);
                self.set_flag(StatusRegister::ZERO, result == 0);
                self.set_flag(StatusRegister::NEGATIVE, false);
            }
            AddressModeValue::ZeroPageX(addr) => {
                let effective_addr =
                    addr.wrapping_add(self.get_value(RegisterType::X).as_u8()) as u16;
                let value = self.read_memory(effective_addr)?;
                let result = value >> 1;
                self.write_memory(effective_addr, result)?;
                self.set_flag(StatusRegister::CARRY, value & 0x01 != 0);
                self.set_flag(StatusRegister::ZERO, result == 0);
                self.set_flag(StatusRegister::NEGATIVE, false);
            }
            AddressModeValue::Absolute(addr) => {
                let value = self.read_memory(addr)?;
                let result = value >> 1;
                self.write_memory(addr, result)?;
                self.set_flag(StatusRegister::CARRY, value & 0x01 != 0);
                self.set_flag(StatusRegister::ZERO, result == 0);
                self.set_flag(StatusRegister::NEGATIVE, false);
            }
            AddressModeValue::AbsoluteX(addr) => {
                let x = self.get_value(RegisterType::X).as_u8() as u16;
                let effective_addr = addr.wrapping_add(x);
                let value = self.read_memory(effective_addr)?;
                let result = value >> 1;
                self.write_memory(effective_addr, result)?;
                self.set_flag(StatusRegister::CARRY, value & 0x01 != 0);
                self.set_flag(StatusRegister::ZERO, result == 0);
                self.set_flag(StatusRegister::NEGATIVE, false);
            }
            _ => return Err(Error::InvalidAddressingMode("LSR")),
        }

        Ok(())
    }

    fn rol(&mut self, mode: AddressModeValue, _decode: DecodedInstruction) -> Result<()> {
        println!("[CPU] Executing ROL with mode: {:?}", mode);

        let carry = if self.get_flag(StatusRegister::CARRY) {
            1
        } else {
            0
        };

        match mode {
            AddressModeValue::Accumulator => {
                let value = self.get_value(RegisterType::A).as_u8();
                let result = (value << 1) | carry;
                self.set_value(RegisterType::A, RegisterData::Bit8(result));
                self.set_flag(StatusRegister::CARRY, value & 0x80 != 0);
                self.set_flag(StatusRegister::ZERO, result == 0);
                self.set_flag(StatusRegister::NEGATIVE, result & 0x80 != 0);
            }
            AddressModeValue::ZeroPage(addr) => {
                let value = self.read_memory(addr as u16)?;
                let result = (value << 1) | carry;
                self.write_memory(addr as u16, result)?;
                self.set_flag(StatusRegister::CARRY, value & 0x80 != 0);
                self.set_flag(StatusRegister::ZERO, result == 0);
                self.set_flag(StatusRegister::NEGATIVE, result & 0x80 != 0);
            }
            AddressModeValue::ZeroPageX(addr) => {
                let effective_addr =
                    addr.wrapping_add(self.get_value(RegisterType::X).as_u8()) as u16;
                let value = self.read_memory(effective_addr)?;
                let result = (value << 1) | carry;
                self.write_memory(effective_addr, result)?;
                self.set_flag(StatusRegister::CARRY, value & 0x80 != 0);
                self.set_flag(StatusRegister::ZERO, result == 0);
                self.set_flag(StatusRegister::NEGATIVE, result & 0x80 != 0);
            }
            AddressModeValue::Absolute(addr) => {
                let value = self.read_memory(addr)?;
                let result = (value << 1) | carry;
                self.write_memory(addr, result)?;
                self.set_flag(StatusRegister::CARRY, value & 0x80 != 0);
                self.set_flag(StatusRegister::ZERO, result == 0);
                self.set_flag(StatusRegister::NEGATIVE, result & 0x80 != 0);
            }
            AddressModeValue::AbsoluteX(addr) => {
                let x = self.get_value(RegisterType::X).as_u8() as u16;
                let effective_addr = addr.wrapping_add(x);
                let value = self.read_memory(effective_addr)?;
                let result = (value << 1) | carry;
                self.write_memory(effective_addr, result)?;
                self.set_flag(StatusRegister::CARRY, value & 0x80 != 0);
                self.set_flag(StatusRegister::ZERO, result == 0);
                self.set_flag(StatusRegister::NEGATIVE, result & 0x80 != 0);
            }
            _ => return Err(Error::InvalidAddressingMode("ROL")),
        }

        Ok(())
    }

    fn ror(&mut self, mode: AddressModeValue, _decode: DecodedInstruction) -> Result<()> {
        println!("[CPU] Executing ROR with mode: {:?}", mode);

        let carry = if self.get_flag(StatusRegister::CARRY) {
            0x80
        } else {
            0
        };

        match mode {
            AddressModeValue::Accumulator => {
                let value = self.get_value(RegisterType::A).as_u8();
                let result = (value >> 1) | carry;
                self.set_value(RegisterType::A, RegisterData::Bit8(result));
                self.set_flag(StatusRegister::CARRY, value & 0x01 != 0);
                self.set_flag(StatusRegister::ZERO, result == 0);
                self.set_flag(StatusRegister::NEGATIVE, result & 0x80 != 0);
            }
            AddressModeValue::ZeroPage(addr) => {
                let value = self.read_memory(addr as u16)?;
                let result = (value >> 1) | carry;
                self.write_memory(addr as u16, result)?;
                self.set_flag(StatusRegister::CARRY, value & 0x01 != 0);
                self.set_flag(StatusRegister::ZERO, result == 0);
                self.set_flag(StatusRegister::NEGATIVE, result & 0x80 != 0);
            }
            AddressModeValue::ZeroPageX(addr) => {
                let effective_addr =
                    addr.wrapping_add(self.get_value(RegisterType::X).as_u8()) as u16;
                let value = self.read_memory(effective_addr)?;
                let result = (value >> 1) | carry;
                self.write_memory(effective_addr, result)?;
                self.set_flag(StatusRegister::CARRY, value & 0x01 != 0);
                self.set_flag(StatusRegister::ZERO, result == 0);
                self.set_flag(StatusRegister::NEGATIVE, result & 0x80 != 0);
            }
            AddressModeValue::Absolute(addr) => {
                let value = self.read_memory(addr)?;
                let result = (value >> 1) | carry;
                self.write_memory(addr, result)?;
                self.set_flag(StatusRegister::CARRY, value & 0x01 != 0);
                self.set_flag(StatusRegister::ZERO, result == 0);
                self.set_flag(StatusRegister::NEGATIVE, result & 0x80 != 0);
            }
            AddressModeValue::AbsoluteX(addr) => {
                let x = self.get_value(RegisterType::X).as_u8() as u16;
                let effective_addr = addr.wrapping_add(x);
                let value = self.read_memory(effective_addr)?;
                let result = (value >> 1) | carry;
                self.write_memory(effective_addr, result)?;
                self.set_flag(StatusRegister::CARRY, value & 0x01 != 0);
                self.set_flag(StatusRegister::ZERO, result == 0);
                self.set_flag(StatusRegister::NEGATIVE, result & 0x80 != 0);
            }
            _ => return Err(Error::InvalidAddressingMode("ROR")),
        }

        Ok(())
    }
}

impl CPU {
    pub(super) fn execute_shift(&mut self, decoded: DecodedInstruction) -> Result<()> {
        println!(
            "[CPU] Executing shift instruction: {:?}",
            decoded.instruction
        );
        match decoded.instruction {
            Instruction::ASL(mode) => self.asl(mode, decoded),
            Instruction::LSR(mode) => self.lsr(mode, decoded),
            Instruction::ROL(mode) => self.rol(mode, decoded),
            Instruction::ROR(mode) => self.ror(mode, decoded),
            _ => Err(Error::InvalidInstruction { inst_type: "shift" }),
        }
    }
}
