use crate::{CPU, RegisterType, instruction::DecodedInstruction, register::StatusRegister};
use common::Result;
use error::Error;
use types::{AddressMode, Instruction};

impl CPU {
    pub(super) fn execute_compare(&mut self, decoded: DecodedInstruction) -> Result<()> {
        println!(
            "[CPU] Executing compare instruction: {:?}",
            decoded.info.instruction
        );
        match decoded.info.instruction {
            Instruction::CMP(mode) => self.cmp(mode, decoded),
            Instruction::CPX(mode) => self.cpx(mode, decoded),
            Instruction::CPY(mode) => self.cpy(mode, decoded),
            _ => Err(Error::InvalidInstruction {
                inst_type: "compare",
            }),
        }
    }

    fn cmp(&mut self, _mode: AddressMode, decode: DecodedInstruction) -> Result<()> {
        println!("[CPU] Executing CMP with operand: 0x{:02X}", decode.operand);
        let value = decode.operand as u8;
        let a = self.get_value(RegisterType::A).as_u8();
        self.compare_values(a, value);
        Ok(())
    }

    fn cpx(&mut self, _mode: AddressMode, decode: DecodedInstruction) -> Result<()> {
        println!("[CPU] Executing CPX with operand: 0x{:02X}", decode.operand);
        let value = decode.operand as u8;
        let x = self.get_value(RegisterType::X).as_u8();
        self.compare_values(x, value);
        Ok(())
    }

    fn cpy(&mut self, _mode: AddressMode, decode: DecodedInstruction) -> Result<()> {
        println!("[CPU] Executing CPY with operand: 0x{:02X}", decode.operand);
        let value = decode.operand as u8;
        let y = self.get_value(RegisterType::Y).as_u8();
        self.compare_values(y, value);
        Ok(())
    }

    /// Helper function to perform comparison and set flags
    /// 개선된 버전: StatusRegister 기능 활용
    fn compare_values(&mut self, reg_value: u8, mem_value: u8) {
        let result = reg_value.wrapping_sub(mem_value);
        // 비교 결과에 따라 각 플래그를 독립적으로 설정
        // Carry 플래그: reg_value >= mem_value 면 SET
        self.set_flag(StatusRegister::CARRY, reg_value >= mem_value);

        // Zero 플래그: reg_value == mem_value 면 SET
        self.set_flag(StatusRegister::ZERO, reg_value == mem_value);

        // Negative 플래그: 결과의 비트 7이 1이면 SET
        self.set_flag(StatusRegister::NEGATIVE, (result & 0x80) != 0);
    }
}
