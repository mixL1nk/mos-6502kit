use crate::RegisterData;
use crate::RegisterType;
use crate::cpu::CPU;
use crate::instruction::{AddressMode, DecodedInstruction, Instruction};
use common::Result;

impl CPU {
    pub(super) fn execute_arithmetic(&mut self, decoded: DecodedInstruction) -> Result<()> {
        println!(
            "[CPU] Executing arithmetic instruction: {:?}",
            decoded.info.instruction
        );
        match decoded.info.instruction {
            Instruction::ADC(mode) => self.adc(mode, decoded),
            Instruction::SBC(mode) => self.sbc(mode, decoded),
            Instruction::INC(mode) => self.inc(mode, decoded),
            Instruction::DEC(mode) => self.dec(mode, decoded),
            Instruction::INX => self.inx(),
            Instruction::INY => self.iny(),
            Instruction::DEX => self.dex(),
            Instruction::DEY => self.dey(),
            _ => Err("Invalid arithmetic instruction".into()),
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

    fn adc(&mut self, mode: AddressMode, decode: DecodedInstruction) -> Result<()> {
        println!(
            "[CPU] Executing ADC with mode: {:?}, operand: 0x{:04X}",
            mode, decode.operand
        );

        let operand = decode.operand as u8;
        let a = self.get(RegisterType::A).as_u8();
        let status = self.get(RegisterType::P).as_u8();
        let carry = (status & 0x01) != 0;
        let decimal_mode = (status & 0x08) != 0;

        if decimal_mode {
            let (result, carry_out) = self.bcd_add(a, operand, carry);
            self.set(RegisterType::A, RegisterData::Bit8(result));
            self.update_bcd_flags(result, carry_out);
        } else {
            let sum = a as u16 + operand as u16 + (carry as u16);
            let result = sum as u8;

            let carry_out = sum > 0xFF;
            let overflow = ((a ^ result) & (operand ^ result) & 0x80) != 0;

            self.set(RegisterType::A, RegisterData::Bit8(result));
            self.update_arithmetic_flags(result, carry_out, overflow);
        }

        Ok(())
    }

    fn sbc(&mut self, _mode: AddressMode, decode: DecodedInstruction) -> Result<()> {
        println!("[CPU] Executing SBC with operand: 0x{:04X}", decode.operand);

        let operand = decode.operand as u8;
        let a = self.get(RegisterType::A).as_u8();
        let status = self.get(RegisterType::P).as_u8();
        let carry = (status & 0x01) != 0;
        let decimal_mode = (status & 0x08) != 0;

        if decimal_mode {
            let (result, carry_out) = self.bcd_sub(a, operand, carry);
            self.set(RegisterType::A, RegisterData::Bit8(result));
            self.update_bcd_flags(result, carry_out);
        } else {
            let operand = operand.wrapping_add(!carry as u8);
            let result = a.wrapping_sub(operand);

            let carry_out = a >= operand;
            let overflow = ((a ^ operand) & (a ^ result) & 0x80) != 0;

            self.set(RegisterType::A, RegisterData::Bit8(result));
            self.update_arithmetic_flags(result, carry_out, overflow);
        }

        Ok(())
    }

    fn inc(&mut self, _mode: AddressMode, decode: DecodedInstruction) -> Result<()> {
        println!("[CPU] Executing INC with operand: 0x{:04X}", decode.operand);

        let addr = decode.operand;
        let value = self.read_memory(addr).map_err(|e| e.to_string())?;
        let result = value.wrapping_add(1);

        self.write_memory(addr, result).map_err(|e| e.to_string())?;
        self.update_nz_flags(result);

        Ok(())
    }

    fn dec(&mut self, _mode: AddressMode, decode: DecodedInstruction) -> Result<()> {
        println!("[CPU] Executing DEC with operand: 0x{:04X}", decode.operand);

        let addr = decode.operand;
        let value = self.read_memory(addr).map_err(|e| e.to_string())?;
        let result = value.wrapping_sub(1);

        self.write_memory(addr, result).map_err(|e| e.to_string())?;
        self.update_nz_flags(result);

        Ok(())
    }

    fn inx(&mut self) -> Result<()> {
        println!("[CPU] Executing INX");

        let x = self.get(RegisterType::X).as_u8();
        let result = x.wrapping_add(1);

        self.set(RegisterType::X, RegisterData::Bit8(result));
        self.update_nz_flags(result);

        Ok(())
    }

    fn iny(&mut self) -> Result<()> {
        println!("[CPU] Executing INY");

        let y = self.get(RegisterType::Y).as_u8();
        let result = y.wrapping_add(1);

        self.set(RegisterType::Y, RegisterData::Bit8(result));
        self.update_nz_flags(result);

        Ok(())
    }

    fn dex(&mut self) -> Result<()> {
        println!("[CPU] Executing DEX");

        let x = self.get(RegisterType::X).as_u8();
        let result = x.wrapping_sub(1);

        self.set(RegisterType::X, RegisterData::Bit8(result));
        self.update_nz_flags(result);

        Ok(())
    }

    fn dey(&mut self) -> Result<()> {
        println!("[CPU] Executing DEY");

        let y = self.get(RegisterType::Y).as_u8();
        let result = y.wrapping_sub(1);

        self.set(RegisterType::Y, RegisterData::Bit8(result));
        self.update_nz_flags(result);

        Ok(())
    }
}
