use crate::{
    cpu::CPU,
    instruction::DecodedInstruction,
    register::{RegisterData, RegisterType, StatusRegister},
};
use common::Result;
use error::Error;
use types::{AddressModeValue, Instruction};

/// 논리 연산 구현
pub trait LogicalOperation {
    /// AND - Logical AND with Accumulator
    fn and(&mut self, mode: AddressModeValue, decode: DecodedInstruction) -> Result<()>;

    /// ORA - Logical OR with Accumulator
    fn ora(&mut self, mode: AddressModeValue, decode: DecodedInstruction) -> Result<()>;

    /// EOR - Logical Exclusive OR with Accumulator
    fn eor(&mut self, mode: AddressModeValue, decode: DecodedInstruction) -> Result<()>;

    /// BIT - Test Bits in Memory with Accumulator
    fn bit(&mut self, mode: AddressModeValue, decode: DecodedInstruction) -> Result<()>;
}

impl LogicalOperation for CPU {
    fn and(&mut self, mode: AddressModeValue, _decode: DecodedInstruction) -> Result<()> {
        println!("[CPU] Executing AND with mode: {:?}", mode);

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
            _ => return Err(Error::InvalidAddressingMode("AND")),
        };

        let a = self.get_value(RegisterType::A).as_u8();
        let result = a & value;
        self.set_value(RegisterType::A, RegisterData::Bit8(result));

        // Update flags
        self.set_flag(StatusRegister::ZERO, result == 0);
        self.set_flag(StatusRegister::NEGATIVE, result & 0x80 != 0);

        Ok(())
    }

    fn ora(&mut self, mode: AddressModeValue, _decode: DecodedInstruction) -> Result<()> {
        println!("[CPU] Executing ORA with mode: {:?}", mode);

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
            _ => return Err(Error::InvalidAddressingMode("ORA")),
        };

        let a = self.get_value(RegisterType::A).as_u8();
        let result = a | value;
        self.set_value(RegisterType::A, RegisterData::Bit8(result));

        // Update flags
        self.set_flag(StatusRegister::ZERO, result == 0);
        self.set_flag(StatusRegister::NEGATIVE, result & 0x80 != 0);

        Ok(())
    }

    fn eor(&mut self, mode: AddressModeValue, _decode: DecodedInstruction) -> Result<()> {
        println!("[CPU] Executing EOR with mode: {:?}", mode);

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
            _ => return Err(Error::InvalidAddressingMode("EOR")),
        };

        let a = self.get_value(RegisterType::A).as_u8();
        let result = a ^ value;
        self.set_value(RegisterType::A, RegisterData::Bit8(result));

        // Update flags
        self.set_flag(StatusRegister::ZERO, result == 0);
        self.set_flag(StatusRegister::NEGATIVE, result & 0x80 != 0);

        Ok(())
    }

    fn bit(&mut self, mode: AddressModeValue, _decode: DecodedInstruction) -> Result<()> {
        println!("[CPU] Executing BIT with mode: {:?}", mode);

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
            _ => return Err(Error::InvalidAddressingMode("BIT")),
        };

        let a = self.get_value(RegisterType::A).as_u8();
        let result = a & value;

        // Set Zero flag based on result of AND operation
        self.set_flag(StatusRegister::ZERO, result == 0);

        // For non-immediate addressing modes, set N and V flags from memory value
        if !matches!(mode, AddressModeValue::Immediate(_)) {
            // Set Negative flag from bit 7 of memory value
            self.set_flag(StatusRegister::NEGATIVE, value & 0x80 != 0);
            // Set Overflow flag from bit 6 of memory value
            self.set_flag(StatusRegister::OVERFLOW, value & 0x40 != 0);
        }

        Ok(())
    }
}

impl CPU {
    pub(super) fn execute_logical(&mut self, decoded: DecodedInstruction) -> Result<()> {
        println!(
            "[CPU] Executing logical instruction: {:?}",
            decoded.instruction
        );
        match decoded.instruction {
            Instruction::AND(mode) => self.and(mode, decoded),
            Instruction::ORA(mode) => self.ora(mode, decoded),
            Instruction::EOR(mode) => self.eor(mode, decoded),
            Instruction::BIT(mode) => self.bit(mode, decoded),
            _ => Err(Error::InvalidInstruction {
                inst_type: "logical",
            }),
        }
    }

    fn and(&mut self, mode: AddressModeValue, _decode: DecodedInstruction) -> Result<()> {
        println!("[CPU] Executing AND with mode: {:?}", mode);

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
            _ => return Err(Error::InvalidAddressingMode("AND")),
        };

        let a = self.get_value(RegisterType::A).as_u8();
        let result = a & value;
        self.set_value(RegisterType::A, RegisterData::Bit8(result));

        // Update flags
        self.set_flag(StatusRegister::ZERO, result == 0);
        self.set_flag(StatusRegister::NEGATIVE, result & 0x80 != 0);

        Ok(())
    }

    fn ora(&mut self, mode: AddressModeValue, _decode: DecodedInstruction) -> Result<()> {
        println!("[CPU] Executing ORA with mode: {:?}", mode);

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
            _ => return Err(Error::InvalidAddressingMode("ORA")),
        };

        let a = self.get_value(RegisterType::A).as_u8();
        let result = a | value;
        self.set_value(RegisterType::A, RegisterData::Bit8(result));

        // Update flags
        self.set_flag(StatusRegister::ZERO, result == 0);
        self.set_flag(StatusRegister::NEGATIVE, result & 0x80 != 0);

        Ok(())
    }

    fn eor(&mut self, mode: AddressModeValue, _decode: DecodedInstruction) -> Result<()> {
        println!("[CPU] Executing EOR with mode: {:?}", mode);

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
            _ => return Err(Error::InvalidAddressingMode("EOR")),
        };

        let a = self.get_value(RegisterType::A).as_u8();
        let result = a ^ value;
        self.set_value(RegisterType::A, RegisterData::Bit8(result));

        // Update flags
        self.set_flag(StatusRegister::ZERO, result == 0);
        self.set_flag(StatusRegister::NEGATIVE, result & 0x80 != 0);

        Ok(())
    }

    fn bit(&mut self, mode: AddressModeValue, decode: DecodedInstruction) -> Result<()> {
        println!(
            "[CPU] Executing BIT with operand: 0x{:04X}",
            decode.operand_value
        );
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
            _ => return Err(Error::InvalidAddressingMode("BIT")),
        };

        let a = self.get_value(RegisterType::A).as_u8();
        let result = a & value;
        self.set_flag(StatusRegister::ZERO, result == 0);
        if !matches!(mode, AddressModeValue::Immediate(_)) {
            // Set Negative flag from bit 7 of memory value
            self.set_flag(StatusRegister::NEGATIVE, value & 0x80 != 0);
            // Set Overflow flag from bit 6 of memory value
            self.set_flag(StatusRegister::OVERFLOW, value & 0x40 != 0);
        }
        Ok(())
    }
}
