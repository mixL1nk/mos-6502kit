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
    fn cmp(&mut self, _mode: AddressModeValue, decode: DecodedInstruction) -> Result<()> {
        println!(
            "[CPU] Executing CMP with operand: 0x{:02X}",
            decode.operand_value
        );
        let value = match _mode {
            AddressModeValue::Immediate(val) => {
                println!("[DEBUG] CMP immediate value: 0x{:02X}", val);
                val
            }
            _ => {
                let mem_val = self.read_memory(decode.operand_value)?;
                println!("[DEBUG] CMP memory value: 0x{:02X}", mem_val);
                mem_val
            }
        };
        let a = self.get_value(RegisterType::A).as_u8();
        println!("[DEBUG] CMP A register: 0x{:02X}", a);
        let result = a.wrapping_sub(value);
        println!("[DEBUG] CMP result: 0x{:02X}", result);
        println!(
            "[DEBUG] CMP flags before: N={}, Z={}, C={}",
            self.get_flag(StatusRegister::NEGATIVE),
            self.get_flag(StatusRegister::ZERO),
            self.get_flag(StatusRegister::CARRY)
        );
        self.set_flag(StatusRegister::NEGATIVE, result & 0x80 != 0);
        self.set_flag(StatusRegister::ZERO, result == 0);
        self.set_flag(StatusRegister::CARRY, a >= value);
        println!(
            "[DEBUG] CMP flags after: N={}, Z={}, C={}",
            self.get_flag(StatusRegister::NEGATIVE),
            self.get_flag(StatusRegister::ZERO),
            self.get_flag(StatusRegister::CARRY)
        );
        Ok(())
    }

    fn cpx(&mut self, _mode: AddressModeValue, decode: DecodedInstruction) -> Result<()> {
        println!(
            "[CPU] Executing CPX with operand: 0x{:02X}",
            decode.operand_value
        );
        let value = match _mode {
            AddressModeValue::Immediate(val) => val,
            _ => self.read_memory(decode.operand_value)?,
        };
        let x = self.get_value(RegisterType::X).as_u8();
        let result = x.wrapping_sub(value);
        println!("[DEBUG] CPX result: 0x{:02X}", result);
        self.set_flag(StatusRegister::NEGATIVE, result & 0x80 != 0);
        self.set_flag(StatusRegister::ZERO, result == 0);
        self.set_flag(StatusRegister::CARRY, x >= value);
        Ok(())
    }

    fn cpy(&mut self, _mode: AddressModeValue, decode: DecodedInstruction) -> Result<()> {
        println!(
            "[CPU] Executing CPY with operand: 0x{:02X}",
            decode.operand_value
        );
        let value = match _mode {
            AddressModeValue::Immediate(val) => val,
            _ => self.read_memory(decode.operand_value)?,
        };
        let y = self.get_value(RegisterType::Y).as_u8();
        let result = y.wrapping_sub(value);
        println!("[DEBUG] CPY result: 0x{:02X}", result);
        println!("[DEBUG] CPY y: 0x{:02X}", y);
        println!("[DEBUG] CPY value: 0x{:02X}", value);
        // self.update_nz_flags(result);
        self.set_flag(StatusRegister::NEGATIVE, result & 0x80 != 0);
        self.set_flag(StatusRegister::ZERO, result == 0);
        self.set_flag(StatusRegister::CARRY, y >= value);
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
