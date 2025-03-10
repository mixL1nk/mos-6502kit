use common::Result;
use types::OPCODE_MAP;
pub use types::{AddressModeValue, Instruction, InstructionInfo};

/// 명령어 디코딩 결과를 담는 구조체
#[derive(Debug, Clone)]
pub struct DecodedInstruction {
    pub instruction: Instruction,
    pub bytes_count: u8,
    pub operand_value: u16,
    pub cycles: u8,
}

impl DecodedInstruction {
    pub fn new(instruction: Instruction, bytes_count: u8, operand_value: u16, cycles: u8) -> Self {
        Self {
            instruction,
            bytes_count,
            operand_value,
            cycles,
        }
    }
}

#[derive(Debug, Clone)]
pub struct Fetch {
    pub instruction_info: InstructionInfo,
    pub operand: Vec<u8>,
    pub opcode: u8,
}

impl Fetch {
    pub fn new(instruction_info: InstructionInfo, operand: Vec<u8>, opcode: u8) -> Self {
        Self {
            instruction_info,
            operand,
            opcode,
        }
    }

    pub fn to_operand_u16(&self) -> u16 {
        match self.operand.len() {
            0 => 0,
            1 => self.operand[0] as u16,
            2 => (self.operand[0] as u16) | ((self.operand[1] as u16) << 8),
            _ => panic!("잘못된 오퍼랜드 크기: {}", self.operand.len()),
        }
    }

}

/// 명령어 디코더
#[derive(Debug, Clone)]
pub struct InstructionDecoder;

impl Default for InstructionDecoder {
    fn default() -> Self {
        Self::new()
    }
}

impl InstructionDecoder {
    pub fn new() -> Self {
        Self
    }

    pub fn get_instruction_info(&self, opcode: u8) -> Option<InstructionInfo> {
        OPCODE_MAP.get(&opcode).copied()
    }

    pub fn decode(&self, fetch: Fetch) -> Result<DecodedInstruction> {
        let info = fetch.instruction_info;
        let operand = fetch.operand;
        // TODO: 출력을 Debug 모드에서만 하도록 수정
        // println!("[DEBUG] opcode: {:?}", info.instruction);
        // println!("[DEBUG] operand: {:?}", operand);
        // 어드레싱 모드 추출 및 수정
        let address_mode: AddressModeValue = match info.instruction {
            Instruction::LDA(mode)
            | Instruction::LDX(mode)
            | Instruction::LDY(mode)
            | Instruction::STA(mode)
            | Instruction::STX(mode)
            | Instruction::STY(mode)
            | Instruction::ADC(mode)
            | Instruction::SBC(mode)
            | Instruction::AND(mode)
            | Instruction::ORA(mode)
            | Instruction::EOR(mode)
            | Instruction::CMP(mode)
            | Instruction::CPX(mode)
            | Instruction::CPY(mode)
            | Instruction::INC(mode)
            | Instruction::DEC(mode)
            | Instruction::ASL(mode)
            | Instruction::LSR(mode)
            | Instruction::ROL(mode)
            | Instruction::ROR(mode)
            | Instruction::JMP(mode)
            | Instruction::JSR(mode)
            | Instruction::BIT(mode) => match mode {
                AddressModeValue::Immediate(_) => AddressModeValue::Immediate(operand[0]),
                AddressModeValue::Absolute(_) => {
                    let addr = ((operand[1] as u16) << 8) | (operand[0] as u16);
                    AddressModeValue::Absolute(addr)
                }
                AddressModeValue::ZeroPage(_) => AddressModeValue::ZeroPage(operand[0]),
                AddressModeValue::ZeroPageX(_) => AddressModeValue::ZeroPageX(operand[0]),
                AddressModeValue::ZeroPageY(_) => AddressModeValue::ZeroPageY(operand[0]),
                AddressModeValue::AbsoluteX(_) => {
                    let addr = ((operand[1] as u16) << 8) | (operand[0] as u16);
                    AddressModeValue::AbsoluteX(addr)
                }
                AddressModeValue::AbsoluteY(_) => {
                    let addr = ((operand[1] as u16) << 8) | (operand[0] as u16);
                    AddressModeValue::AbsoluteY(addr)
                }
                AddressModeValue::IndirectX(_) => AddressModeValue::IndirectX(operand[0]),
                AddressModeValue::IndirectY(_) => AddressModeValue::IndirectY(operand[0]),
                AddressModeValue::Indirect(_) => {
                    let addr = ((operand[1] as u16) << 8) | (operand[0] as u16);
                    AddressModeValue::Indirect(addr)
                }
                _ => mode,
            },
            _ => AddressModeValue::Implied,
        };

        // 명령어 재구성
        let instruction = match info.instruction {
            Instruction::LDA(_) => Instruction::LDA(address_mode),
            Instruction::LDX(_) => Instruction::LDX(address_mode),
            Instruction::LDY(_) => Instruction::LDY(address_mode),
            Instruction::STA(_) => Instruction::STA(address_mode),
            Instruction::STX(_) => Instruction::STX(address_mode),
            Instruction::STY(_) => Instruction::STY(address_mode),
            Instruction::ADC(_) => Instruction::ADC(address_mode),
            Instruction::SBC(_) => Instruction::SBC(address_mode),
            Instruction::AND(_) => Instruction::AND(address_mode),
            Instruction::ORA(_) => Instruction::ORA(address_mode),
            Instruction::EOR(_) => Instruction::EOR(address_mode),
            Instruction::CMP(_) => Instruction::CMP(address_mode),
            Instruction::CPX(_) => Instruction::CPX(address_mode),
            Instruction::CPY(_) => Instruction::CPY(address_mode),
            Instruction::INC(_) => Instruction::INC(address_mode),
            Instruction::DEC(_) => Instruction::DEC(address_mode),
            Instruction::ASL(_) => Instruction::ASL(address_mode),
            Instruction::LSR(_) => Instruction::LSR(address_mode),
            Instruction::ROL(_) => Instruction::ROL(address_mode),
            Instruction::ROR(_) => Instruction::ROR(address_mode),
            Instruction::JMP(_) => Instruction::JMP(address_mode),
            Instruction::JSR(_) => Instruction::JSR(address_mode),
            Instruction::BIT(_) => Instruction::BIT(address_mode),
            _ => info.instruction,
        };

        // 명령어 크기와 피연산자 값, 페이지 경계 계산
        let (bytes_count, operand_value, page_crossed) = match address_mode {
            AddressModeValue::Immediate(_) => (2, operand[0] as u16, false),
            AddressModeValue::ZeroPage(_) => (2, operand[0] as u16, false),
            AddressModeValue::ZeroPageX(_) | AddressModeValue::ZeroPageY(_) => {
                (2, operand[0] as u16, false)
            }
            AddressModeValue::Absolute(_) => {
                (3, ((operand[1] as u16) << 8) | operand[0] as u16, false)
            }
            AddressModeValue::AbsoluteX(_) | AddressModeValue::AbsoluteY(_) => {
                let base = ((operand[1] as u16) << 8) | operand[0] as u16;
                let crosses = (base & 0xFF00) != ((base + 1) & 0xFF00);
                (3, base, crosses)
            }
            AddressModeValue::Indirect(_) => {
                (3, ((operand[1] as u16) << 8) | operand[0] as u16, false)
            }
            AddressModeValue::IndirectX(_) => (2, operand[0] as u16, false),
            AddressModeValue::IndirectY(_) => {
                let base = operand[0] as u16;
                let crosses = (base & 0xFF00) != ((base + 1) & 0xFF00);
                (2, base, crosses)
            }
            AddressModeValue::Accumulator | AddressModeValue::Implied => (1, 0, false),
        };

        // 분기 명령어인 경우 operand_value를 signed byte로 처리
        let operand_value = match info.instruction {
            Instruction::BCC(_)
            | Instruction::BCS(_)
            | Instruction::BEQ(_)
            | Instruction::BNE(_)
            | Instruction::BMI(_)
            | Instruction::BPL(_)
            | Instruction::BVC(_)
            | Instruction::BVS(_) => operand[0] as i8 as u16,
            _ => operand_value,
        };

        // 총 사이클 수 계산
        let mut cycles = info.cycles.base_cycles;
        if page_crossed {
            cycles += 1;
        }
        if info.cycles.branch_taken {
            cycles += 1;
        }
        Ok(DecodedInstruction::new(
            instruction,
            bytes_count,
            operand_value,
            cycles,
        ))
    }
}
