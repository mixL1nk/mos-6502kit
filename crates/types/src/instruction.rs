use super::AddressModeValue;
use derive_more::Display;
/// 명령어 집합
#[derive(Debug, PartialEq, Clone, Copy, Display)]
pub enum Instruction {
    // 로드/스토어 명령어
    LDA(AddressModeValue),
    LDX(AddressModeValue),
    LDY(AddressModeValue),
    STA(AddressModeValue),
    STX(AddressModeValue),
    STY(AddressModeValue),

    // 산술/논리 연산
    ADC(AddressModeValue),
    SBC(AddressModeValue),
    AND(AddressModeValue),
    ORA(AddressModeValue),
    EOR(AddressModeValue),

    // 시프트/회전
    ASL(AddressModeValue),
    LSR(AddressModeValue),
    ROL(AddressModeValue),
    ROR(AddressModeValue),

    // 증감
    INC(AddressModeValue),
    DEC(AddressModeValue),
    INX,
    INY,
    DEX,
    DEY,

    // 비교
    CMP(AddressModeValue),
    CPX(AddressModeValue),
    CPY(AddressModeValue),
    BIT(AddressModeValue),

    // 점프/분기
    JMP(AddressModeValue),
    JSR(AddressModeValue),
    RTS,
    BCC(i8),
    BCS(i8),
    BEQ(i8),
    BNE(i8),
    BMI(i8),
    BPL(i8),
    BVC(i8),
    BVS(i8),

    // 레지스터 전송
    TAX,
    TXA,
    TAY,
    TYA,
    TSX,
    TXS,

    // 스택 처리
    PHA,
    PLA,
    PHP,
    PLP,

    // 플래그 설정
    CLC,
    SEC,
    CLI,
    SEI,
    CLV,
    CLD,
    SED,

    // 기타
    BRK,
    RTI,
    NOP,
}

#[derive(Debug, Clone, Copy)]
pub struct InstructionInfo {
    pub instruction: Instruction,
    pub cycles: CycleInfo,
}

impl InstructionInfo {
    pub fn new(instruction: Instruction, cycles: CycleInfo) -> Self {
        Self {
            instruction,
            cycles,
        }
    }
    pub fn get_operand_size(&self) -> u8 {
        match &self.instruction {
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
            | Instruction::BIT(mode)
            | Instruction::INC(mode)
            | Instruction::DEC(mode)
            | Instruction::ASL(mode)
            | Instruction::LSR(mode)
            | Instruction::ROL(mode)
            | Instruction::ROR(mode) => match mode {
                AddressModeValue::Immediate(_)
                | AddressModeValue::ZeroPage(_)
                | AddressModeValue::ZeroPageX(_)
                | AddressModeValue::ZeroPageY(_)
                | AddressModeValue::IndirectX(_)
                | AddressModeValue::IndirectY(_) => 1,
                AddressModeValue::Absolute(_)
                | AddressModeValue::AbsoluteX(_)
                | AddressModeValue::AbsoluteY(_)
                | AddressModeValue::Indirect(_) => 2,
                AddressModeValue::Accumulator | AddressModeValue::Implied => 0,
            },
            Instruction::JMP(mode) | Instruction::JSR(mode) => match mode {
                AddressModeValue::Absolute(_) | AddressModeValue::Indirect(_) => 2,
                AddressModeValue::Immediate(_)
                | AddressModeValue::ZeroPage(_)
                | AddressModeValue::ZeroPageX(_)
                | AddressModeValue::ZeroPageY(_)
                | AddressModeValue::IndirectX(_)
                | AddressModeValue::IndirectY(_)
                | AddressModeValue::AbsoluteX(_)
                | AddressModeValue::AbsoluteY(_)
                | AddressModeValue::Accumulator
                | AddressModeValue::Implied => 0,
            },
            Instruction::BCC(_)
            | Instruction::BCS(_)
            | Instruction::BEQ(_)
            | Instruction::BNE(_)
            | Instruction::BMI(_)
            | Instruction::BPL(_)
            | Instruction::BVC(_)
            | Instruction::BVS(_) => 1,
            _ => 0,
        }
    }
}

#[derive(Debug, Clone, Copy)]
pub struct CycleInfo {
    pub base_cycles: u8,    // 기본 사이클 수
    pub page_cross: bool,   // 페이지 크로스 여부
    pub branch_taken: bool, // 분기 명령 실행 여부
}

impl CycleInfo {
    pub fn new(base_cycles: u8) -> Self {
        Self {
            base_cycles,
            page_cross: false,
            branch_taken: false,
        }
    }
    pub fn with_page_cross(mut self) -> Self {
        self.page_cross = true;
        self
    }

    pub fn with_branch_taken(mut self) -> Self {
        self.branch_taken = true;
        self
    }

    pub fn total_cycles(&self) -> u8 {
        let mut cycles = self.base_cycles;
        if self.page_cross {
            cycles += 1;
        }
        if self.branch_taken {
            cycles += 1;
        }
        cycles
    }
}
