use common::Result;
use error::Error;
use types::{AddressModeValue, Instruction};

/// 어드레싱 모드별 opcode를 저장하는 구조체
#[derive(Debug, Clone, Copy)]
struct AddressModeOpcodes {
    indirect_x: u8,  // (Indirect,X)
    zero_page: u8,   // Zero Page
    immediate: u8,   // Immediate
    absolute: u8,    // Absolute
    indirect_y: u8,  // (Indirect),Y
    zero_page_x: u8, // Zero Page,X
    absolute_y: u8,  // Absolute,Y
    absolute_x: u8,  // Absolute,X
}

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
                AddressModeOpcodes {
                    indirect_x: 0xA1,
                    zero_page: 0xA5,
                    immediate: 0xA9,
                    absolute: 0xAD,
                    indirect_y: 0xB1,
                    zero_page_x: 0xB5,
                    absolute_y: 0xB9,
                    absolute_x: 0xBD,
                },
                mode,
            )?,
            Instruction::LDX(mode) => self.generate_opcode_with_address(
                AddressModeOpcodes {
                    indirect_x: 0,
                    zero_page: 0xA6,
                    immediate: 0xA2,
                    absolute: 0xAE,
                    indirect_y: 0,
                    zero_page_x: 0,
                    absolute_y: 0xBE,
                    absolute_x: 0,
                },
                mode,
            )?,
            Instruction::LDY(mode) => self.generate_opcode_with_address(
                AddressModeOpcodes {
                    indirect_x: 0,
                    zero_page: 0xA4,
                    immediate: 0xA0,
                    absolute: 0xAC,
                    indirect_y: 0,
                    zero_page_x: 0xB4,
                    absolute_y: 0,
                    absolute_x: 0xBC,
                },
                mode,
            )?,
            Instruction::STA(mode) => self.generate_opcode_with_address(
                AddressModeOpcodes {
                    indirect_x: 0x81,
                    zero_page: 0x85,
                    immediate: 0,
                    absolute: 0x8D,
                    indirect_y: 0x91,
                    zero_page_x: 0x95,
                    absolute_y: 0x99,
                    absolute_x: 0x9D,
                },
                mode,
            )?,
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
                AddressModeOpcodes {
                    indirect_x: 0x61,
                    zero_page: 0x65,
                    immediate: 0x69,
                    absolute: 0x6D,
                    indirect_y: 0x71,
                    zero_page_x: 0x75,
                    absolute_y: 0x79,
                    absolute_x: 0x7D,
                },
                mode,
            )?,
            Instruction::SBC(mode) => self.generate_opcode_with_address(
                AddressModeOpcodes {
                    indirect_x: 0xE1,
                    zero_page: 0xE5,
                    immediate: 0xE9,
                    absolute: 0xED,
                    indirect_y: 0xF1,
                    zero_page_x: 0xF5,
                    absolute_y: 0xF9,
                    absolute_x: 0xFD,
                },
                mode,
            )?,
            Instruction::AND(mode) => self.generate_opcode_with_address(
                AddressModeOpcodes {
                    indirect_x: 0x21,
                    zero_page: 0x25,
                    immediate: 0x29,
                    absolute: 0x2D,
                    indirect_y: 0x31,
                    zero_page_x: 0x35,
                    absolute_y: 0x39,
                    absolute_x: 0x3D,
                },
                mode,
            )?,
            Instruction::ORA(mode) => self.generate_opcode_with_address(
                AddressModeOpcodes {
                    indirect_x: 0x01,
                    zero_page: 0x05,
                    immediate: 0x09,
                    absolute: 0x0D,
                    indirect_y: 0x11,
                    zero_page_x: 0x15,
                    absolute_y: 0x19,
                    absolute_x: 0x1D,
                },
                mode,
            )?,
            Instruction::EOR(mode) => self.generate_opcode_with_address(
                AddressModeOpcodes {
                    indirect_x: 0x41,
                    zero_page: 0x45,
                    immediate: 0x49,
                    absolute: 0x4D,
                    indirect_y: 0x51,
                    zero_page_x: 0x55,
                    absolute_y: 0x59,
                    absolute_x: 0x5D,
                },
                mode,
            )?,

            // 비교 명령어
            Instruction::CMP(mode) => self.generate_opcode_with_address(
                AddressModeOpcodes {
                    indirect_x: 0xC1,
                    zero_page: 0xC5,
                    immediate: 0xC9,
                    absolute: 0xCD,
                    indirect_y: 0xD1,
                    zero_page_x: 0xD5,
                    absolute_y: 0xD9,
                    absolute_x: 0xDD,
                },
                mode,
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

    /// 다양한 어드레싱 모드에 따라 적절한 opcode를 선택하여 생성합니다
    fn generate_opcode_with_address(
        &mut self,
        opcodes: AddressModeOpcodes,
        mode: AddressModeValue,
    ) -> Result<()> {
        match mode {
            AddressModeValue::IndirectX(_) => {
                self.emit_opcode_with_byte(opcodes.indirect_x, mode)?
            }
            AddressModeValue::ZeroPage(_) => self.emit_opcode_with_byte(opcodes.zero_page, mode)?,
            AddressModeValue::Immediate(_) => {
                self.emit_opcode_with_byte(opcodes.immediate, mode)?
            }
            AddressModeValue::Absolute(_) => self.emit_opcode_with_word(opcodes.absolute, mode)?,
            AddressModeValue::IndirectY(_) => {
                self.emit_opcode_with_byte(opcodes.indirect_y, mode)?
            }
            AddressModeValue::ZeroPageX(_) => {
                self.emit_opcode_with_byte(opcodes.zero_page_x, mode)?
            }
            AddressModeValue::AbsoluteY(_) => {
                self.emit_opcode_with_word(opcodes.absolute_y, mode)?
            }
            AddressModeValue::AbsoluteX(_) => {
                self.emit_opcode_with_word(opcodes.absolute_x, mode)?
            }
            _ => return Err(Error::InvalidAddressingMode("Instruction")),
        }
        Ok(())
    }
}
