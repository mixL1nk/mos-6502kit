use crate::RegisterData;
use crate::RegisterType;
use crate::register::StatusRegister;
use crate::{CPU, instruction::DecodedInstruction};
use common::Result;
use error::Error;
use types::{AddressModeValue, Instruction};

impl CPU {
    pub(super) fn execute_arithmetic(&mut self, decoded: DecodedInstruction) -> Result<()> {
        println!(
            "[CPU] Executing arithmetic instruction: {:?}",
            decoded.instruction
        );
        match decoded.instruction {
            Instruction::ADC(mode) => self.adc(mode, decoded),
            Instruction::SBC(mode) => self.sbc(mode, decoded),
            Instruction::INC(mode) => self.inc(mode, decoded),
            Instruction::DEC(mode) => self.dec(mode, decoded),
            Instruction::INX => self.inx(),
            Instruction::INY => self.iny(),
            Instruction::DEX => self.dex(),
            Instruction::DEY => self.dey(),
            _ => Err(Error::InvalidInstruction {
                inst_type: "arithmetic",
            }),
        }
    }

    /// Perform BCD addition
    fn bcd_add(&mut self, a: u8, b: u8, carry: bool) -> (u8, bool) {
        let mut al = (a & 0x0F) + (b & 0x0F) + (carry as u8);
        if al > 9 {
            al += 6;
        }
        let mut ah = (a >> 4) + (b >> 4) + (if al > 0x0F { 1 } else { 0 });
        if ah > 9 {
            ah += 6;
        }

        let carry_out = ah > 0x0F;
        let result = ((ah & 0x0F) << 4) | (al & 0x0F);

        (result, carry_out)
    }

    /// Perform BCD subtraction
    fn bcd_sub(&mut self, a: u8, b: u8, carry: bool) -> (u8, bool) {
        let mut al = (a & 0x0F).wrapping_sub((b & 0x0F) + (!carry as u8));
        let mut ah = (a >> 4).wrapping_sub((b >> 4) + (if al > 0x0F { 1 } else { 0 }));

        if al > 9 {
            al = (al.wrapping_sub(6)) & 0x0F;
        }
        if ah > 9 {
            ah = (ah.wrapping_sub(6)) & 0x0F;
        }

        let result = (ah << 4) | (al & 0x0F);
        let carry_out = a >= b.wrapping_add(!carry as u8);

        (result, carry_out)
    }

    fn adc(&mut self, mode: AddressModeValue, _decode: DecodedInstruction) -> Result<()> {
        // println!("[CPU] Executing ADC with mode: {:?}", mode);

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
            _ => return Err(Error::InvalidAddressingMode("ADC")),
        };

        let a = self.get_value(RegisterType::A).as_u8();
        let carry = if self.get_flag(StatusRegister::CARRY) {
            1
        } else {
            0
        };

        // BCD 모드 체크
        if self.get_flag(StatusRegister::DECIMAL) {
            let (result, carry_out) = self.bcd_add(a, value, carry == 1);
            self.set_value(RegisterType::A, RegisterData::Bit8(result));
            let overflow = (!(a ^ value) & (a ^ result) & 0x80) != 0;
            self.update_flags_arithmetic(result, carry_out, overflow);
        } else {
            // println!(
            //     "[DEBUG] ADC - A: ${:02X}, M: ${:02X}, C: {}",
            //     a, value, carry
            // );
            let sum = a.wrapping_add(value).wrapping_add(carry);
            let carry_out = (a as u16 + value as u16 + carry as u16) > 0xFF;

            // Calculate overflow
            let overflow = (a & 0x80) == (value & 0x80) && (a & 0x80) != (sum & 0x80);

            // println!(
            //     "[DEBUG] ADC - Result: ${:02X}, Carry: {}, Overflow: {}",
            //     sum, carry_out, overflow
            // );
            self.set_value(RegisterType::A, RegisterData::Bit8(sum));
            self.update_flags_arithmetic(sum, carry_out, overflow);
        }

        Ok(())
    }

    fn sbc(&mut self, mode: AddressModeValue, _decode: DecodedInstruction) -> Result<()> {
        // println!("[CPU] Executing SBC with mode: {:?}", mode);

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
            _ => return Err(Error::InvalidAddressingMode("SBC")),
        };

        let a = self.get_value(RegisterType::A).as_u8();
        let borrow = if self.get_flag(StatusRegister::CARRY) {
            0
        } else {
            1
        };

        // BCD 모드 체크
        if self.get_flag(StatusRegister::DECIMAL) {
            let (result, carry_out) = self.bcd_sub(a, value, borrow == 0);
            self.set_value(RegisterType::A, RegisterData::Bit8(result));
            self.set_flag(StatusRegister::CARRY, carry_out);
            self.set_flag(StatusRegister::ZERO, result == 0);
            self.set_flag(StatusRegister::NEGATIVE, result & 0x80 != 0);
        } else {
            let diff = a as i16 - value as i16 - borrow;
            let result = diff as u8;
            self.set_value(RegisterType::A, RegisterData::Bit8(result));
            self.set_flag(StatusRegister::CARRY, diff >= 0);
            self.set_flag(StatusRegister::ZERO, result == 0);
            self.set_flag(StatusRegister::NEGATIVE, result & 0x80 != 0);
            self.set_flag(
                StatusRegister::OVERFLOW,
                ((a ^ value) & (a ^ result) & 0x80) != 0,
            );
        }

        Ok(())
    }

    fn inc(&mut self, mode: AddressModeValue, _decode: DecodedInstruction) -> Result<()> {
        // println!("[CPU] Executing INC with mode: {:?}", mode);

        let addr = match mode {
            AddressModeValue::ZeroPage(addr) => addr as u16,
            AddressModeValue::ZeroPageX(addr) => {
                let effective_addr = addr.wrapping_add(self.get_value(RegisterType::X).as_u8());
                effective_addr as u16
            }
            AddressModeValue::Absolute(addr) => addr,
            AddressModeValue::AbsoluteX(addr) => {
                let x = self.get_value(RegisterType::X).as_u8() as u16;
                addr.wrapping_add(x)
            }
            _ => return Err(Error::InvalidAddressingMode("INC")),
        };

        let value = self.read_memory(addr)?.wrapping_add(1);
        self.write_memory(addr, value)?;

        // Update flags
        self.set_flag(StatusRegister::ZERO, value == 0);
        self.set_flag(StatusRegister::NEGATIVE, value & 0x80 != 0);

        Ok(())
    }

    fn dec(&mut self, mode: AddressModeValue, _decode: DecodedInstruction) -> Result<()> {
        // println!("[CPU] Executing DEC with mode: {:?}", mode);

        let addr = match mode {
            AddressModeValue::ZeroPage(addr) => addr as u16,
            AddressModeValue::ZeroPageX(addr) => {
                let effective_addr = addr.wrapping_add(self.get_value(RegisterType::X).as_u8());
                effective_addr as u16
            }
            AddressModeValue::Absolute(addr) => addr,
            AddressModeValue::AbsoluteX(addr) => {
                let x = self.get_value(RegisterType::X).as_u8() as u16;
                addr.wrapping_add(x)
            }
            _ => return Err(Error::InvalidAddressingMode("DEC")),
        };

        let value = self.read_memory(addr)?.wrapping_sub(1);
        self.write_memory(addr, value)?;

        // Update flags
        self.set_flag(StatusRegister::ZERO, value == 0);
        self.set_flag(StatusRegister::NEGATIVE, value & 0x80 != 0);

        Ok(())
    }

    fn inx(&mut self) -> Result<()> {
        // println!("[CPU] Executing INX");
        let value = self.get_value(RegisterType::X).as_u8().wrapping_add(1);
        self.set_value(RegisterType::X, RegisterData::Bit8(value));

        // Update flags
        self.set_flag(StatusRegister::ZERO, value == 0);
        self.set_flag(StatusRegister::NEGATIVE, value & 0x80 != 0);

        Ok(())
    }

    fn iny(&mut self) -> Result<()> {
        // println!("[CPU] Executing INY");
        let value = self.get_value(RegisterType::Y).as_u8().wrapping_add(1);
        self.set_value(RegisterType::Y, RegisterData::Bit8(value));

        // Update flags
        self.set_flag(StatusRegister::ZERO, value == 0);
        self.set_flag(StatusRegister::NEGATIVE, value & 0x80 != 0);

        Ok(())
    }

    fn dex(&mut self) -> Result<()> {
        // println!("[CPU] Executing DEX");
        let value = self.get_value(RegisterType::X).as_u8().wrapping_sub(1);
        self.set_value(RegisterType::X, RegisterData::Bit8(value));

        // Update flags
        self.set_flag(StatusRegister::ZERO, value == 0);
        self.set_flag(StatusRegister::NEGATIVE, value & 0x80 != 0);

        Ok(())
    }

    fn dey(&mut self) -> Result<()> {
        println!("[CPU] Executing DEY");
        let value = self.get_value(RegisterType::Y).as_u8().wrapping_sub(1);
        self.set_value(RegisterType::Y, RegisterData::Bit8(value));

        // Update flags
        self.set_flag(StatusRegister::ZERO, value == 0);
        self.set_flag(StatusRegister::NEGATIVE, value & 0x80 != 0);

        Ok(())
    }
}
