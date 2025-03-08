use crate::address_mode::AddressMode;

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
    BCC, // Branch if Carry Clear
    BCS, // Branch if Carry Set
    BEQ, // Branch if Equal
    BNE, // Branch if Not Equal
    BMI, // Branch if Minus
    BPL, // Branch if Plus
    BVC, // Branch if Overflow Clear
    BVS, // Branch if Overflow Set

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

    // 논리 연산
    BIT(AddressMode), // Bit Test
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
            Instruction::LDA(mode) => mode.operand_size(),
            Instruction::LDX(mode) => mode.operand_size(),
            Instruction::LDY(mode) => mode.operand_size(),
            Instruction::STA(mode) => mode.operand_size(),
            Instruction::INC(mode) => mode.operand_size(),
            Instruction::DEC(mode) => mode.operand_size(),
            Instruction::JMP(mode) => mode.operand_size(),
            Instruction::JSR(mode) => mode.operand_size(),
            _ => 0, // 기본적으로 Operand가 없는 경우
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
