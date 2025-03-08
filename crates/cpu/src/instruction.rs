use crate::cpu::Fetch;
use common::Result;
use std::sync::LazyLock;
use types::OPCODE_MAP;
use types::{AddressMode, Instruction, InstructionInfo};

/// 명령어 디코딩 결과를 담는 구조체
#[derive(Debug)]
pub struct DecodedInstruction {
    /// 디코딩된 명령어 정보
    pub info: InstructionInfo,
    /// 명령어의 바이트 수 (opcode + operand)
    pub bytes: u8,
    /// 명령어의 operand 값
    pub operand: u16,
    /// 페이지 경계를 넘었는지 여부
    pub page_crossed: bool,
    /// 실제 실행에 필요한 사이클 수
    pub total_cycles: u8,
}

/// 명령어 디코더
#[derive(Debug)]
pub struct InstructionDecoder {
    /// opcode 테이블
    opcode_table: &'static [Option<InstructionInfo>; 256],
}

impl Default for InstructionDecoder {
    fn default() -> Self {
        Self::new()
    }
}

impl InstructionDecoder {
    /// 새로운 디코더 인스턴스 생성
    pub fn new() -> Self {
        static OPCODE_TABLE: LazyLock<[Option<InstructionInfo>; 256]> = LazyLock::new(|| {
            let mut table = [None; 256];

            for (opcode, info) in OPCODE_MAP.iter() {
                table[*opcode as usize] = Some(*info);
            }

            table
        });

        Self {
            opcode_table: &OPCODE_TABLE,
        }
    }

    pub fn get_instruction_info(&self, opcode: u8) -> Option<InstructionInfo> {
        self.opcode_table[opcode as usize]
    }

    /// 명령어 디코딩
    pub fn decode(&self, fetch: Fetch) -> Result<DecodedInstruction> {
        let Fetch {
            instruction_info,
            operand,
        } = fetch;
        let inst = instruction_info.instruction;

        // 주소 모드 추출
        let address_mode = match inst {
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
            | Instruction::BIT(mode) => mode,
            _ => AddressMode::Implied, // 묵시적 주소 모드
        };

        // 명령어 크기와 피연산자 값, 페이지 경계 계산
        let (bytes_count, operand_value, page_crossed) = match address_mode {
            AddressMode::Immediate => (2, operand[0] as u16, false),
            AddressMode::ZeroPage => (2, operand[0] as u16, false),
            AddressMode::ZeroPageX | AddressMode::ZeroPageY => (2, operand[0] as u16, false),
            AddressMode::Absolute => (3, ((operand[1] as u16) << 8) | operand[0] as u16, false),
            AddressMode::AbsoluteX | AddressMode::AbsoluteY => {
                let base = ((operand[1] as u16) << 8) | operand[0] as u16;
                // 페이지 경계를 넘었는지 확인
                // 엄밀히는 인덱스 레지스터를 더했을 때 경계를 넘는지 확인해야 하지만,
                // 실제 인덱스 값을 여기서는 알 수 없으므로 +1로 추정
                let crosses = (base & 0xFF00) != ((base + 1) & 0xFF00);
                (3, base, crosses)
            }
            AddressMode::Indirect => (3, ((operand[1] as u16) << 8) | operand[0] as u16, false),
            AddressMode::IndirectX => (2, operand[0] as u16, false),
            AddressMode::IndirectY => {
                let base = operand[0] as u16;
                let crosses = (base & 0xFF00) != ((base + 1) & 0xFF00);
                (2, base, crosses)
            }
            AddressMode::Relative => (2, operand[0] as u16, false),
            AddressMode::Accumulator | AddressMode::Implied => (1, 0, false),
        };

        // 총 사이클 수 계산
        let mut total_cycles = instruction_info.cycles.base_cycles;
        if page_crossed && instruction_info.cycles.page_cross {
            total_cycles += 1;
        }

        Ok(DecodedInstruction {
            info: instruction_info,
            bytes: bytes_count,
            operand: operand_value,
            page_crossed,
            total_cycles,
        })
    }
}
