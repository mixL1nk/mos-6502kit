use crate::{CPU, instruction::DecodedInstruction, register::StatusRegister};
use common::Result;
use error::Error;
use types::Instruction;

impl CPU {
    pub(super) fn execute_flag(&mut self, decoded: DecodedInstruction) -> Result<()> {
        println!(
            "[CPU] Executing flag instruction: {:?}",
            decoded.info.instruction
        );

        match decoded.info.instruction {
            // 플래그 해제 명령어
            Instruction::CLC => self.clear_flag(StatusRegister::CARRY),
            Instruction::CLD => self.clear_flag(StatusRegister::DECIMAL),
            Instruction::CLI => self.clear_flag(StatusRegister::INTERRUPT_DISABLE),
            Instruction::CLV => self.clear_flag(StatusRegister::OVERFLOW),

            // 플래그 설정 명령어
            Instruction::SEC => self.set_flag_instruction(StatusRegister::CARRY),
            Instruction::SED => self.set_flag_instruction(StatusRegister::DECIMAL),
            Instruction::SEI => self.set_flag_instruction(StatusRegister::INTERRUPT_DISABLE),

            _ => Err(Error::InvalidInstruction { inst_type: "flag" }),
        }
    }

    // 플래그 해제 공통 메서드
    fn clear_flag(&mut self, flag: StatusRegister) -> Result<()> {
        println!("[CPU] Clearing flag: {:?}", flag);
        let mut status = self.status_flag();
        status.remove(flag);
        self.set_status(status);
        Ok(())
    }

    // 플래그 설정 공통 메서드 (명령어를 위한 메서드)
    // 기존 set_flag와 이름이 겹치지 않도록 set_flag_instruction으로 명명
    fn set_flag_instruction(&mut self, flag: StatusRegister) -> Result<()> {
        println!("[CPU] Setting flag: {:?}", flag);
        let mut status = self.status_flag();
        status.insert(flag);
        self.set_status(status);
        Ok(())
    }
}
