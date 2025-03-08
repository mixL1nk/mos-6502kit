use crate::{CPU, instruction::DecodedInstruction, register::StatusRegister};
use common::Result;
use error::Error;
use types::{AddressMode, Instruction};

impl CPU {
    pub(super) fn execute_jump(&mut self, decoded: DecodedInstruction) -> Result<()> {
        println!(
            "[CPU] Executing jump instruction: {:?}",
            decoded.info.instruction
        );
        match decoded.info.instruction {
            Instruction::JMP(mode) => self.jmp(mode, decoded),
            Instruction::JSR(mode) => self.jsr(mode, decoded),
            Instruction::RTS => self.rts(),
            // 분기 명령어들을 모두 공통 함수로 처리
            instruction => {
                let (flag, condition) = match instruction {
                    Instruction::BCC => (StatusRegister::CARRY, false),
                    Instruction::BCS => (StatusRegister::CARRY, true),
                    Instruction::BEQ => (StatusRegister::ZERO, true),
                    Instruction::BNE => (StatusRegister::ZERO, false),
                    Instruction::BMI => (StatusRegister::NEGATIVE, true),
                    Instruction::BPL => (StatusRegister::NEGATIVE, false),
                    Instruction::BVC => (StatusRegister::OVERFLOW, false),
                    Instruction::BVS => (StatusRegister::OVERFLOW, true),
                    _ => return Err(Error::InvalidInstruction { inst_type: "jump" }),
                };

                println!(
                    "[CPU] Executing branch instruction: {:?} with offset: 0x{:02X}",
                    instruction, decoded.operand
                );

                self.branch_if_flag(flag, condition, decoded)
            }
        }
    }

    // 플래그 조건에 따라 분기 처리하는 공통 함수
    fn branch_if_flag(
        &mut self,
        flag: StatusRegister,
        expected_state: bool,
        decode: DecodedInstruction,
    ) -> Result<()> {
        // get_flag로 플래그 상태 확인
        let branch_taken = self.get_flag(flag) == expected_state;

        if branch_taken {
            let offset = decode.operand as i8;
            let old_pc = self.get_pc();
            let new_pc = ((old_pc as i32) + (offset as i32)) as u16;

            // 페이지 경계를 넘었는지 확인
            let page_cross = self.check_page_cross(old_pc, new_pc);

            // 사이클 카운트 업데이트
            self.update_cycles(&decode.info, page_cross, branch_taken);

            self.set_pc(new_pc);
        } else {
            // 분기가 발생하지 않은 경우 기본 사이클만
            self.update_cycles(&decode.info, false, false);
        }

        Ok(())
    }

    // 기존 jmp, jsr, rts 함수는 그대로 유지
    fn jmp(&mut self, _mode: AddressMode, decode: DecodedInstruction) -> Result<()> {
        println!("[CPU] Executing JMP to address: 0x{:04X}", decode.operand);
        self.set_pc(decode.operand);
        Ok(())
    }

    fn jsr(&mut self, _mode: AddressMode, decode: DecodedInstruction) -> Result<()> {
        println!("[CPU] Executing JSR to address: 0x{:04X}", decode.operand);
        // Get current PC (points to next instruction)
        let return_addr = self.get_pc().wrapping_sub(1);

        // Push return address onto stack
        self.stack_push_u16(return_addr)?;

        // Jump to subroutine
        self.set_pc(decode.operand);
        Ok(())
    }

    fn rts(&mut self) -> Result<()> {
        println!("[CPU] Executing RTS");

        // Pull return address from stack and add 1
        let return_addr = self.stack_pull_u16()?.wrapping_add(1);
        self.set_pc(return_addr);
        Ok(())
    }
}
