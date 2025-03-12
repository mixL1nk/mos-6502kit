use crate::{
    cpu::CPU,
    instruction::DecodedInstruction,
    register::{RegisterType, StatusRegister},
};
use common::Result;
use error::Error;
use types::{AddressModeValue, Instruction};

/// 비교 연산 구현
pub trait CompareOperation {
    /// CMP - Compare Memory with Accumulator
    fn cmp(&mut self, mode: AddressModeValue, decode: DecodedInstruction) -> Result<()>;

    /// CPX - Compare Memory with X Register
    fn cpx(&mut self, mode: AddressModeValue, decode: DecodedInstruction) -> Result<()>;

    /// CPY - Compare Memory with Y Register
    fn cpy(&mut self, mode: AddressModeValue, decode: DecodedInstruction) -> Result<()>;
}

impl CompareOperation for CPU {
    fn cmp(&mut self, mode: AddressModeValue, _decode: DecodedInstruction) -> Result<()> {
        // println!("[CPU] Executing CMP with mode: {:?}", mode);

        let value = match mode {
            AddressModeValue::Immediate(val) => val,
            AddressModeValue::ZeroPage(addr) => self.read_memory(addr as u16)?,
            AddressModeValue::ZeroPageX(addr) => {
                let effective_addr =
                    addr.wrapping_add(self.get_value(RegisterType::X).as_u8()) as u16;
                self.read_memory(effective_addr)?
            }
            AddressModeValue::Absolute(addr) => self.read_memory(addr)?,
            AddressModeValue::AbsoluteX(addr) => {
                let x = self.get_value(RegisterType::X).as_u8() as u16;
                let effective_addr = addr.wrapping_add(x);
                self.read_memory(effective_addr)?
            }
            AddressModeValue::AbsoluteY(addr) => {
                let y = self.get_value(RegisterType::Y).as_u8() as u16;
                let effective_addr = addr.wrapping_add(y);
                self.read_memory(effective_addr)?
            }
            AddressModeValue::IndirectX(addr) => {
                let x = self.get_value(RegisterType::X).as_u8();
                let ptr = addr.wrapping_add(x) as u16;
                let low = self.read_memory(ptr)? as u16;
                let high = self.read_memory(ptr.wrapping_add(1))? as u16;
                let effective_addr = (high << 8) | low;
                self.read_memory(effective_addr)?
            }
            AddressModeValue::IndirectY(addr) => {
                let ptr = addr as u16;
                let low = self.read_memory(ptr)? as u16;
                let high = self.read_memory(ptr.wrapping_add(1))? as u16;
                let base_addr = (high << 8) | low;
                let y = self.get_value(RegisterType::Y).as_u8() as u16;
                let effective_addr = base_addr.wrapping_add(y);
                self.read_memory(effective_addr)?
            }
            _ => return Err(Error::InvalidAddressingMode("CMP")),
        };

        let a = self.get_value(RegisterType::A).as_u8();
        let result = a.wrapping_sub(value);

        // Update flags
        self.set_flag(StatusRegister::CARRY, a >= value);
        self.set_flag(StatusRegister::ZERO, a == value);
        self.set_flag(StatusRegister::NEGATIVE, result & 0x80 != 0);

        Ok(())
    }

    fn cpx(&mut self, mode: AddressModeValue, _decode: DecodedInstruction) -> Result<()> {
        // println!("[CPU] Executing CPX with mode: {:?}", mode);

        let value = match mode {
            AddressModeValue::Immediate(val) => val,
            AddressModeValue::ZeroPage(addr) => self.read_memory(addr as u16)?,
            AddressModeValue::Absolute(addr) => self.read_memory(addr)?,
            _ => return Err(Error::InvalidAddressingMode("CPX")),
        };

        let x = self.get_value(RegisterType::X).as_u8();
        let result = x.wrapping_sub(value);

        // Update flags
        self.set_flag(StatusRegister::CARRY, x >= value);
        self.set_flag(StatusRegister::ZERO, x == value);
        self.set_flag(StatusRegister::NEGATIVE, result & 0x80 != 0);

        Ok(())
    }

    fn cpy(&mut self, mode: AddressModeValue, _decode: DecodedInstruction) -> Result<()> {
        // println!("[CPU] Executing CPY with mode: {:?}", mode);

        let value = match mode {
            AddressModeValue::Immediate(val) => val,
            AddressModeValue::ZeroPage(addr) => self.read_memory(addr as u16)?,
            AddressModeValue::Absolute(addr) => self.read_memory(addr)?,
            _ => return Err(Error::InvalidAddressingMode("CPY")),
        };

        let y = self.get_value(RegisterType::Y).as_u8();
        let result = y.wrapping_sub(value);

        // Update flags
        self.set_flag(StatusRegister::CARRY, y >= value);
        self.set_flag(StatusRegister::ZERO, y == value);
        self.set_flag(StatusRegister::NEGATIVE, result & 0x80 != 0);

        Ok(())
    }
}

impl CPU {
    pub(super) fn execute_compare(&mut self, decoded: DecodedInstruction) -> Result<()> {
        println!(
            "[CPU] Executing compare instruction: {:?}",
            decoded.instruction
        );
        match decoded.instruction {
            Instruction::CMP(mode) => self.cmp(mode, decoded),
            Instruction::CPX(mode) => self.cpx(mode, decoded),
            Instruction::CPY(mode) => self.cpy(mode, decoded),
            _ => Err(Error::InvalidInstruction {
                inst_type: "compare",
            }),
        }
    }
}
