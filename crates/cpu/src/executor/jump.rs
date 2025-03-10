use crate::{CPU, instruction::DecodedInstruction, register::StatusRegister};
use common::Result;
use error::Error;
use types::{AddressModeValue, Instruction};

impl CPU {
    pub(super) fn execute_jump(&mut self, decoded: DecodedInstruction) -> Result<()> {
        println!(
            "[CPU] Executing jump instruction: {:?}",
            decoded.instruction
        );
        match decoded.instruction {
            Instruction::JMP(mode) => self.jmp(mode, decoded),
            Instruction::JSR(mode) => self.jsr(mode, decoded),
            Instruction::RTS => self.rts(),
            // 분기 명령어들을 모두 공통 함수로 처리
            instruction => {
                let (flag, condition) = match instruction {
                    Instruction::BCC(_) => (StatusRegister::CARRY, false),
                    Instruction::BCS(_) => (StatusRegister::CARRY, true),
                    Instruction::BEQ(_) => (StatusRegister::ZERO, true),
                    Instruction::BNE(_) => (StatusRegister::ZERO, false),
                    Instruction::BMI(_) => (StatusRegister::NEGATIVE, true),
                    Instruction::BPL(_) => (StatusRegister::NEGATIVE, false),
                    Instruction::BVC(_) => (StatusRegister::OVERFLOW, false),
                    Instruction::BVS(_) => (StatusRegister::OVERFLOW, true),
                    _ => return Err(Error::InvalidInstruction { inst_type: "jump" }),
                };

                println!(
                    "[CPU] Executing branch instruction: {:?} with offset: 0x{:02X}",
                    instruction, decoded.operand_value
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
        let branch_taken = self.get_flag(flag) == expected_state;
        // println!("[DEBUG] Branch taken: {}", branch_taken);

        if branch_taken {
            // 오퍼랜드를 signed byte로 처리
            let offset = decode.operand_value as i8;
            let pc = self.get_pc();

            // PC 상대 주소 계산 - PC는 이미 다음 명령어를 가리키고 있으므로 offset만 더함
            let new_pc = pc.wrapping_add(offset as u16);

            // 페이지 경계를 넘어가는지 확인
            let page_crossed = (pc & 0xFF00) != (new_pc & 0xFF00);

            // 분기 성공 시 1 사이클 추가
            self.add_cycles(1);

            // 페이지 경계를 넘어가면 추가 1 사이클
            if page_crossed {
                self.add_cycles(1);
            }

            // println!(
            //     "[DEBUG] Branch from {:04X} to {:04X} (offset: {:+})",
            //     pc, new_pc, offset
            // );
            // if page_crossed {
            //     println!("[DEBUG] Page boundary crossed, added extra cycle");
            // }

            self.set_pc(new_pc);
        }

        Ok(())
    }

    // 기존 jmp, jsr, rts 함수는 그대로 유지
    fn jmp(&mut self, _mode: AddressModeValue, decode: DecodedInstruction) -> Result<()> {
        // println!(
        //     "[CPU] Executing JMP to address: 0x{:04X}",
        //     decode.operand_value
        // );
        self.set_pc(decode.operand_value);
        Ok(())
    }

    fn jsr(&mut self, _mode: AddressModeValue, decode: DecodedInstruction) -> Result<()> {
        // println!(
        //     "[CPU] Executing JSR to address: 0x{:04X}",
        //     decode.operand_value
        // );
        let return_addr = self.get_pc().wrapping_sub(1);
        self.stack_push_u16(return_addr)?;
        self.set_pc(decode.operand_value);
        Ok(())
    }

    fn rts(&mut self) -> Result<()> {
        // println!("[CPU] Executing RTS");
        let return_addr = self.stack_pull_u16()?.wrapping_add(1);
        self.set_pc(return_addr);
        Ok(())
    }
}
