use crate::{cpu::Fetch, opcode_table::initialize_opcode_table};
use common::Result;
use error::CPUError;

#[derive(Debug, Clone, Copy)]
pub struct InstructionInfo {
    pub instruction: Instruction,
    pub cycles: CycleInfo,
}

impl AddressMode {
    pub fn operand_size(&self) -> OperandSize {
        match self {
            AddressMode::Implied => OperandSize::Zero,
            AddressMode::Immediate => OperandSize::One,
            AddressMode::ZeroPage => OperandSize::One,
            AddressMode::ZeroPageX => OperandSize::One,
            AddressMode::ZeroPageY => OperandSize::One,
            AddressMode::Absolute => OperandSize::Two,
            AddressMode::AbsoluteX => OperandSize::Two,
            AddressMode::AbsoluteY => OperandSize::Two,
            AddressMode::Indirect => OperandSize::Two,
            AddressMode::IndirectX => OperandSize::One,
            AddressMode::IndirectY => OperandSize::One,
            _ => OperandSize::Zero, // 기본적으로 Operand가 없는 경우
        }
    }
}

impl InstructionInfo {
    pub fn new(instruction: Instruction, cycles: CycleInfo) -> Self {
        Self {
            instruction,
            cycles,
        }
    }

    pub fn get_operand_size(&self) -> OperandSize {
        match &self.instruction {
            Instruction::LDA(mode) => mode.operand_size(),
            Instruction::LDX(mode) => mode.operand_size(),
            Instruction::LDY(mode) => mode.operand_size(),
            Instruction::STA(mode) => mode.operand_size(),
            Instruction::INC(mode) => mode.operand_size(),
            Instruction::DEC(mode) => mode.operand_size(),
            Instruction::JMP(mode) => mode.operand_size(),
            Instruction::JSR(mode) => mode.operand_size(),
            _ => OperandSize::Zero, // 기본적으로 Operand가 없는 경우
        }
    }
}

/// 명령어 디코딩 결과를 담는 구조체
#[derive(Debug)]
pub struct DecodedInstruction {
    /// 디코딩된 명령어 정보
    pub info: InstructionInfo,
    /// 명령어의 바이트 수 (opcode + operand)
    pub bytes: u8,
    /// 명령어의 operand 값
    pub operand: u16,
}

/// 명령어 디코더
#[derive(Debug)]
pub struct InstructionDecoder {
    /// opcode 테이블
    opcode_table: [Option<InstructionInfo>; 256],
}

impl Default for InstructionDecoder {
    fn default() -> Self {
        Self::new()
    }
}

impl InstructionDecoder {
    /// 새로운 디코더 인스턴스 생성
    pub fn new() -> Self {
        Self {
            opcode_table: initialize_opcode_table(),
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
        let (bytes_count, operand_value) = match inst {
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
            | Instruction::ROR(mode) => match mode {
                AddressMode::Immediate => {
                    if operand.is_empty() {
                        return Err(CPUError::InvalidOperand(0).into());
                    }
                    (2, operand[0] as u16)
                }
                AddressMode::ZeroPage => {
                    if operand.is_empty() {
                        return Err(CPUError::InvalidOperand(0).into());
                    }
                    (2, operand[0] as u16)
                }
                AddressMode::ZeroPageX => {
                    if operand.is_empty() {
                        return Err(CPUError::InvalidOperand(0).into());
                    }
                    (2, operand[0] as u16)
                }
                AddressMode::ZeroPageY => {
                    if operand.is_empty() {
                        return Err(CPUError::InvalidOperand(0).into());
                    }
                    (2, operand[0] as u16)
                }
                AddressMode::Absolute => {
                    if operand.len() < 2 {
                        return Err(CPUError::InvalidOperand(0).into());
                    }
                    (3, ((operand[1] as u16) << 8) | operand[0] as u16)
                }
                AddressMode::AbsoluteX => {
                    if operand.len() < 2 {
                        return Err(CPUError::InvalidOperand(0).into());
                    }
                    (3, ((operand[1] as u16) << 8) | operand[0] as u16)
                }
                AddressMode::AbsoluteY => {
                    if operand.len() < 2 {
                        return Err(CPUError::InvalidOperand(0).into());
                    }
                    (3, ((operand[1] as u16) << 8) | operand[0] as u16)
                }
                AddressMode::Indirect => {
                    if operand.len() < 2 {
                        return Err(CPUError::InvalidOperand(0).into());
                    }
                    (3, ((operand[1] as u16) << 8) | operand[0] as u16)
                }
                AddressMode::IndirectX => {
                    if operand.is_empty() {
                        return Err(CPUError::InvalidOperand(0).into());
                    }
                    (2, operand[0] as u16)
                }
                AddressMode::IndirectY => {
                    if operand.is_empty() {
                        return Err(CPUError::InvalidOperand(0).into());
                    }
                    (2, operand[0] as u16)
                }
                AddressMode::Relative => {
                    if operand.is_empty() {
                        return Err(CPUError::InvalidOperand(0).into());
                    }
                    (2, operand[0] as u16)
                }
                AddressMode::Accumulator => (1, 0),
                AddressMode::Implied => (1, 0),
            },
            Instruction::PHA
            | Instruction::PLA
            | Instruction::PHP
            | Instruction::PLP
            | Instruction::TAX
            | Instruction::TXA
            | Instruction::TAY
            | Instruction::TYA
            | Instruction::TSX
            | Instruction::TXS
            | Instruction::INX
            | Instruction::INY
            | Instruction::DEX
            | Instruction::DEY
            | Instruction::CLC
            | Instruction::SEC
            | Instruction::CLI
            | Instruction::SEI
            | Instruction::CLD
            | Instruction::SED
            | Instruction::CLV
            | Instruction::NOP
            | Instruction::BRK
            | Instruction::RTI
            | Instruction::RTS => (1, 0),
            _ => return Err(CPUError::Decode(format!("Invalid instruction: {:?}", inst)).into()),
        };

        Ok(DecodedInstruction {
            info: instruction_info,
            bytes: bytes_count,
            operand: operand_value,
        })
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

#[derive(Debug, Clone, Copy)]
pub enum OperandSize {
    Zero, // Operand 없음
    One,  // 1 바이트 Operand
    Two,  // 2 바이트 Operand
}

#[derive(Debug, Clone, Copy)]
pub enum AddressMode {
    Immediate,   // #$00
    ZeroPage,    // $00
    ZeroPageX,   // $00,X
    ZeroPageY,   // $00,Y
    Absolute,    // $0000
    AbsoluteX,   // $0000,X
    AbsoluteY,   // $0000,Y
    Indirect,    // ($0000)
    IndirectX,   // ($00,X)
    IndirectY,   // ($00),Y
    Relative,    // $00 (for branches)
    Accumulator, // A
    Implied,     // (no operand)
}

#[derive(Debug, Clone, Copy)]
pub enum Instruction {
    // 로드/스토어
    LDA(AddressMode), // Load Accumulator
    LDX(AddressMode), // Load X Register
    LDY(AddressMode), // Load Y Register
    STA(AddressMode), // Store Accumulator
    STX(AddressMode), // Store X Register
    STY(AddressMode), // Store Y Register

    // 스택 연산
    PHA, // Push Accumulator
    PLA, // Pull Accumulator
    PHP, // Push Processor Status
    PLP, // Pull Processor Status

    // 레지스터 연산
    TAX, // Transfer Accumulator to X
    TXA, // Transfer X to Accumulator
    TAY, // Transfer Accumulator to Y
    TYA, // Transfer Y to Accumulator
    TSX, // Transfer Stack Pointer to X
    TXS, // Transfer X to Stack Pointer

    // 산술/논리 연산
    ADC(AddressMode), // Add with Carry
    SBC(AddressMode), // Subtract with Carry
    AND(AddressMode), // Logical AND
    ORA(AddressMode), // Logical OR
    EOR(AddressMode), // Logical Exclusive OR
    CMP(AddressMode), // Compare with Accumulator
    CPX(AddressMode), // Compare with X
    CPY(AddressMode), // Compare with Y

    // 증감 연산
    INC(AddressMode), // Increment Memory
    INX,              // Increment X
    INY,              // Increment Y
    DEC(AddressMode), // Decrement Memory
    DEX,              // Decrement X
    DEY,              // Decrement Y

    // 시프트 연산
    ASL(AddressMode), // Arithmetic Shift Left
    LSR(AddressMode), // Logical Shift Right
    ROL(AddressMode), // Rotate Left
    ROR(AddressMode), // Rotate Right

    // 분기 명령
    BCC(i8), // Branch if Carry Clear
    BCS(i8), // Branch if Carry Set
    BEQ(i8), // Branch if Equal
    BNE(i8), // Branch if Not Equal
    BMI(i8), // Branch if Minus
    BPL(i8), // Branch if Plus
    BVC(i8), // Branch if Overflow Clear
    BVS(i8), // Branch if Overflow Set

    // 점프/서브루틴
    JMP(AddressMode), // Jump
    JSR(AddressMode), // Jump to Subroutine
    RTS,              // Return from Subroutine

    // 인터럽트
    BRK, // Break
    RTI, // Return from Interrupt

    // 기타
    CLC, // Clear Carry Flag
    SEC, // Set Carry Flag
    CLI, // Clear Interrupt Disable
    SEI, // Set Interrupt Disable
    CLD, // Clear Decimal Mode
    SED, // Set Decimal Mode
    CLV, // Clear Overflow Flag
    NOP, // No Operation
}
