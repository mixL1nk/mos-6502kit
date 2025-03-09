use common::Result;
use error::Error;
use types::{AddressModeValue, Instruction};

/// 6502 명령어를 기계어로 변환하는 생성기
pub struct Generator {
    pub code: Vec<u8>,
    pub org: u16, // 코드의 시작 주소
}

impl Generator {
    /// 새로운 Generator 인스턴스를 생성합니다
    pub fn new(org: u16) -> Self {
        Generator {
            code: Vec::new(),
            org,
        }
    }

    /// 명령어 목록을 기계어로 변환합니다
    pub fn generate(&mut self, instructions: Vec<Instruction>) -> Result<Vec<u8>> {
        self.code.clear();

        for instruction in instructions {
            self.generate_instruction(instruction)?;
        }

        Ok(self.code.clone())
    }

    /// 단일 명령어를 기계어로 변환합니다
    fn generate_instruction(&mut self, instruction: Instruction) -> Result<()> {
        match instruction {
            // 로드/스토어 명령어
            Instruction::LDA(mode) => self.generate_opcode_with_address(
                0xA1, 0xA5, 0xA9, 0xAD, 0xB1, 0xB5, 0xB9, 0xBD, mode,
            )?,
            Instruction::LDX(mode) => {
                self.generate_opcode_with_address(0, 0xA6, 0xA2, 0xAE, 0, 0, 0xBE, 0, mode)?
            }
            Instruction::LDY(mode) => {
                self.generate_opcode_with_address(0, 0xA4, 0xA0, 0xAC, 0, 0xB4, 0, 0xBC, mode)?
            }
            Instruction::STA(mode) => self
                .generate_opcode_with_address(0x81, 0x85, 0, 0x8D, 0x91, 0x95, 0x99, 0x9D, mode)?,
            Instruction::STX(mode) => match mode {
                AddressModeValue::ZeroPage(_) => self.emit_opcode_with_byte(0x86, mode)?,
                AddressModeValue::ZeroPageY(_) => self.emit_opcode_with_byte(0x96, mode)?,
                AddressModeValue::Absolute(_) => self.emit_opcode_with_word(0x8E, mode)?,
                _ => return Err(Error::InvalidAddressingMode("STX")),
            },
            Instruction::STY(mode) => match mode {
                AddressModeValue::ZeroPage(_) => self.emit_opcode_with_byte(0x84, mode)?,
                AddressModeValue::ZeroPageX(_) => self.emit_opcode_with_byte(0x94, mode)?,
                AddressModeValue::Absolute(_) => self.emit_opcode_with_word(0x8C, mode)?,
                _ => return Err(Error::InvalidAddressingMode("STY")),
            },

            // 산술/논리 연산
            Instruction::ADC(mode) => self.generate_opcode_with_address(
                0x61, 0x65, 0x69, 0x6D, 0x71, 0x75, 0x79, 0x7D, mode,
            )?,
            Instruction::SBC(mode) => self.generate_opcode_with_address(
                0xE1, 0xE5, 0xE9, 0xED, 0xF1, 0xF5, 0xF9, 0xFD, mode,
            )?,
            Instruction::AND(mode) => self.generate_opcode_with_address(
                0x21, 0x25, 0x29, 0x2D, 0x31, 0x35, 0x39, 0x3D, mode,
            )?,
            Instruction::ORA(mode) => self.generate_opcode_with_address(
                0x01, 0x05, 0x09, 0x0D, 0x11, 0x15, 0x19, 0x1D, mode,
            )?,
            Instruction::EOR(mode) => self.generate_opcode_with_address(
                0x41, 0x45, 0x49, 0x4D, 0x51, 0x55, 0x59, 0x5D, mode,
            )?,

            // 비교 명령어
            Instruction::CMP(mode) => self.generate_opcode_with_address(
                0xC1, 0xC5, 0xC9, 0xCD, 0xD1, 0xD5, 0xD9, 0xDD, mode,
            )?,
            Instruction::CPX(mode) => match mode {
                AddressModeValue::Immediate(_) => self.emit_opcode_with_byte(0xE0, mode)?,
                AddressModeValue::ZeroPage(_) => self.emit_opcode_with_byte(0xE4, mode)?,
                AddressModeValue::Absolute(_) => self.emit_opcode_with_word(0xEC, mode)?,
                _ => return Err(Error::InvalidAddressingMode("CPX")),
            },
            Instruction::CPY(mode) => match mode {
                AddressModeValue::Immediate(_) => self.emit_opcode_with_byte(0xC0, mode)?,
                AddressModeValue::ZeroPage(_) => self.emit_opcode_with_byte(0xC4, mode)?,
                AddressModeValue::Absolute(_) => self.emit_opcode_with_word(0xCC, mode)?,
                _ => return Err(Error::InvalidAddressingMode("CPY")),
            },

            // 분기 명령어 (상대 모드)
            Instruction::BCC(offset) => self.emit_opcode_with_offset(0x90, offset)?,
            Instruction::BCS(offset) => self.emit_opcode_with_offset(0xB0, offset)?,
            Instruction::BEQ(offset) => self.emit_opcode_with_offset(0xF0, offset)?,
            Instruction::BNE(offset) => self.emit_opcode_with_offset(0xD0, offset)?,
            Instruction::BMI(offset) => self.emit_opcode_with_offset(0x30, offset)?,
            Instruction::BPL(offset) => self.emit_opcode_with_offset(0x10, offset)?,
            Instruction::BVC(offset) => self.emit_opcode_with_offset(0x50, offset)?,
            Instruction::BVS(offset) => self.emit_opcode_with_offset(0x70, offset)?,

            // 점프 명령어
            Instruction::JMP(mode) => match mode {
                AddressModeValue::Absolute(_) => self.emit_opcode_with_word(0x4C, mode)?,
                AddressModeValue::Indirect(_) => self.emit_opcode_with_word(0x6C, mode)?,
                _ => return Err(Error::InvalidAddressingMode("JMP")),
            },
            Instruction::JSR(mode) => match mode {
                AddressModeValue::Absolute(_) => self.emit_opcode_with_word(0x20, mode)?,
                _ => return Err(Error::InvalidAddressingMode("JSR")),
            },

            // 증감 명령어
            Instruction::INC(mode) => match mode {
                AddressModeValue::ZeroPage(_) => self.emit_opcode_with_byte(0xE6, mode)?,
                AddressModeValue::ZeroPageX(_) => self.emit_opcode_with_byte(0xF6, mode)?,
                AddressModeValue::Absolute(_) => self.emit_opcode_with_word(0xEE, mode)?,
                AddressModeValue::AbsoluteX(_) => self.emit_opcode_with_word(0xFE, mode)?,
                _ => return Err(Error::InvalidAddressingMode("INC")),
            },
            Instruction::DEC(mode) => match mode {
                AddressModeValue::ZeroPage(_) => self.emit_opcode_with_byte(0xC6, mode)?,
                AddressModeValue::ZeroPageX(_) => self.emit_opcode_with_byte(0xD6, mode)?,
                AddressModeValue::Absolute(_) => self.emit_opcode_with_word(0xCE, mode)?,
                AddressModeValue::AbsoluteX(_) => self.emit_opcode_with_word(0xDE, mode)?,
                _ => return Err(Error::InvalidAddressingMode("DEC")),
            },

            // 시프트/회전 명령어
            Instruction::ASL(mode) => match mode {
                AddressModeValue::Accumulator => self.emit_opcode(0x0A)?,
                AddressModeValue::ZeroPage(_) => self.emit_opcode_with_byte(0x06, mode)?,
                AddressModeValue::ZeroPageX(_) => self.emit_opcode_with_byte(0x16, mode)?,
                AddressModeValue::Absolute(_) => self.emit_opcode_with_word(0x0E, mode)?,
                AddressModeValue::AbsoluteX(_) => self.emit_opcode_with_word(0x1E, mode)?,
                _ => return Err(Error::InvalidAddressingMode("ASL")),
            },
            Instruction::LSR(mode) => match mode {
                AddressModeValue::Accumulator => self.emit_opcode(0x4A)?,
                AddressModeValue::ZeroPage(_) => self.emit_opcode_with_byte(0x46, mode)?,
                AddressModeValue::ZeroPageX(_) => self.emit_opcode_with_byte(0x56, mode)?,
                AddressModeValue::Absolute(_) => self.emit_opcode_with_word(0x4E, mode)?,
                AddressModeValue::AbsoluteX(_) => self.emit_opcode_with_word(0x5E, mode)?,
                _ => return Err(Error::InvalidAddressingMode("LSR")),
            },
            Instruction::ROL(mode) => match mode {
                AddressModeValue::Accumulator => self.emit_opcode(0x2A)?,
                AddressModeValue::ZeroPage(_) => self.emit_opcode_with_byte(0x26, mode)?,
                AddressModeValue::ZeroPageX(_) => self.emit_opcode_with_byte(0x36, mode)?,
                AddressModeValue::Absolute(_) => self.emit_opcode_with_word(0x2E, mode)?,
                AddressModeValue::AbsoluteX(_) => self.emit_opcode_with_word(0x3E, mode)?,
                _ => return Err(Error::InvalidAddressingMode("ROL")),
            },
            Instruction::ROR(mode) => match mode {
                AddressModeValue::Accumulator => self.emit_opcode(0x6A)?,
                AddressModeValue::ZeroPage(_) => self.emit_opcode_with_byte(0x66, mode)?,
                AddressModeValue::ZeroPageX(_) => self.emit_opcode_with_byte(0x76, mode)?,
                AddressModeValue::Absolute(_) => self.emit_opcode_with_word(0x6E, mode)?,
                AddressModeValue::AbsoluteX(_) => self.emit_opcode_with_word(0x7E, mode)?,
                _ => return Err(Error::InvalidAddressingMode("ROR")),
            },

            // 비트 테스트
            Instruction::BIT(mode) => match mode {
                AddressModeValue::ZeroPage(_) => self.emit_opcode_with_byte(0x24, mode)?,
                AddressModeValue::Absolute(_) => self.emit_opcode_with_word(0x2C, mode)?,
                _ => return Err(Error::InvalidAddressingMode("BIT")),
            },

            // 단일 바이트 명령어
            Instruction::INX => self.emit_opcode(0xE8)?,
            Instruction::INY => self.emit_opcode(0xC8)?,
            Instruction::DEX => self.emit_opcode(0xCA)?,
            Instruction::DEY => self.emit_opcode(0x88)?,
            Instruction::TAX => self.emit_opcode(0xAA)?,
            Instruction::TXA => self.emit_opcode(0x8A)?,
            Instruction::TAY => self.emit_opcode(0xA8)?,
            Instruction::TYA => self.emit_opcode(0x98)?,
            Instruction::TSX => self.emit_opcode(0xBA)?,
            Instruction::TXS => self.emit_opcode(0x9A)?,
            Instruction::PHA => self.emit_opcode(0x48)?,
            Instruction::PLA => self.emit_opcode(0x68)?,
            Instruction::PHP => self.emit_opcode(0x08)?,
            Instruction::PLP => self.emit_opcode(0x28)?,
            Instruction::CLC => self.emit_opcode(0x18)?,
            Instruction::SEC => self.emit_opcode(0x38)?,
            Instruction::CLI => self.emit_opcode(0x58)?,
            Instruction::SEI => self.emit_opcode(0x78)?,
            Instruction::CLD => self.emit_opcode(0xD8)?,
            Instruction::SED => self.emit_opcode(0xF8)?,
            Instruction::CLV => self.emit_opcode(0xB8)?,
            Instruction::NOP => self.emit_opcode(0xEA)?,
            Instruction::BRK => self.emit_opcode(0x00)?,
            Instruction::RTI => self.emit_opcode(0x40)?,
            Instruction::RTS => self.emit_opcode(0x60)?,
        }

        Ok(())
    }

    /// 단일 바이트 명령어(opcode)를 추가합니다
    fn emit_opcode(&mut self, opcode: u8) -> Result<()> {
        self.code.push(opcode);
        Ok(())
    }

    /// opcode와 1바이트 피연산자를 추가합니다
    fn emit_opcode_with_byte(&mut self, opcode: u8, mode: AddressModeValue) -> Result<()> {
        self.code.push(opcode);

        // 실제 피연산자 값 사용
        match mode {
            AddressModeValue::Immediate(value)
            | AddressModeValue::ZeroPage(value)
            | AddressModeValue::ZeroPageX(value)
            | AddressModeValue::ZeroPageY(value)
            | AddressModeValue::IndirectX(value)
            | AddressModeValue::IndirectY(value) => {
                self.code.push(value);
            }
            _ => {
                // 적절한 어드레싱 모드가 아닌 경우, 기본값이나 오류 처리
                self.code.push(0);
            }
        }

        Ok(())
    }

    /// opcode와 2바이트 피연산자를 추가합니다 (little-endian)
    fn emit_opcode_with_word(&mut self, opcode: u8, mode: AddressModeValue) -> Result<()> {
        self.code.push(opcode);

        // 실제 피연산자 값 사용 (little-endian)
        match mode {
            AddressModeValue::Absolute(value)
            | AddressModeValue::AbsoluteX(value)
            | AddressModeValue::AbsoluteY(value)
            | AddressModeValue::Indirect(value) => {
                self.code.push((value & 0xFF) as u8); // 하위 바이트
                self.code.push(((value >> 8) & 0xFF) as u8); // 상위 바이트
            }
            _ => {
                // 적절한 어드레싱 모드가 아닌 경우, 기본값이나 오류 처리
                self.code.push(0);
                self.code.push(0);
            }
        }

        Ok(())
    }

    /// 분기 명령어를 위한 opcode와 오프셋을 추가합니다
    fn emit_opcode_with_offset(&mut self, opcode: u8, offset: i8) -> Result<()> {
        self.code.push(opcode);
        self.code.push(offset as u8);
        Ok(())
    }

    //TODO: 인자가 너무 많다. 방식을 바꿔야 할거 같다.
    /// 다양한 어드레싱 모드에 따라 적절한 opcode를 선택하여 생성합니다
    fn generate_opcode_with_address(
        &mut self,
        ind_x: u8, // (Indirect,X)
        zp: u8,    // Zero Page
        imm: u8,   // Immediate
        abs: u8,   // Absolute
        ind_y: u8, // (Indirect),Y
        zp_x: u8,  // Zero Page,X
        abs_y: u8, // Absolute,Y
        abs_x: u8, // Absolute,X
        mode: AddressModeValue,
    ) -> Result<()> {
        match mode {
            AddressModeValue::IndirectX(_) => self.emit_opcode_with_byte(ind_x, mode)?,
            AddressModeValue::ZeroPage(_) => self.emit_opcode_with_byte(zp, mode)?,
            AddressModeValue::Immediate(_) => self.emit_opcode_with_byte(imm, mode)?,
            AddressModeValue::Absolute(_) => self.emit_opcode_with_word(abs, mode)?,
            AddressModeValue::IndirectY(_) => self.emit_opcode_with_byte(ind_y, mode)?,
            AddressModeValue::ZeroPageX(_) => self.emit_opcode_with_byte(zp_x, mode)?,
            AddressModeValue::AbsoluteY(_) => self.emit_opcode_with_word(abs_y, mode)?,
            AddressModeValue::AbsoluteX(_) => self.emit_opcode_with_word(abs_x, mode)?,
            _ => return Err(Error::InvalidAddressingMode("Instruction")),
        }
        Ok(())
    }
}
