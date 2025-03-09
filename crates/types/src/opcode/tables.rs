//! 6502 프로세서의 옵코드 테이블 정의
//! 
//! 테이블 포맷:
//! (opcode, instruction, base_cycles, page_cross, branch_taken)

use crate::{AddressModeValue, Instruction};

/// 옵코드 테이블 정의
#[rustfmt::skip]
pub(crate) const OPCODE_TABLE: &[(u8, Instruction, u8, bool, bool)] = &[
    // 로드/스토어 명령어
    (0xA9, Instruction::LDA(AddressModeValue::Immediate(0)), 2, false, false),
    (0xA5, Instruction::LDA(AddressModeValue::ZeroPage(0)),   3, false, false),
    (0xB5, Instruction::LDA(AddressModeValue::ZeroPageX(0)),  4, false, false),
    (0xAD, Instruction::LDA(AddressModeValue::Absolute(0)),   4, false, false),
    (0xBD, Instruction::LDA(AddressModeValue::AbsoluteX(0)),  4, true,  false),
    (0xB9, Instruction::LDA(AddressModeValue::AbsoluteY(0)),  4, true,  false),
    (0xA1, Instruction::LDA(AddressModeValue::IndirectX(0)),  6, false, false),
    (0xB1, Instruction::LDA(AddressModeValue::IndirectY(0)),  5, true,  false),
    
    (0xA2, Instruction::LDX(AddressModeValue::Immediate(0)), 2, false, false),
    (0xA6, Instruction::LDX(AddressModeValue::ZeroPage(0)),  3, false, false),
    (0xB6, Instruction::LDX(AddressModeValue::ZeroPageY(0)), 4, false, false),
    (0xAE, Instruction::LDX(AddressModeValue::Absolute(0)),  4, false, false),
    (0xBE, Instruction::LDX(AddressModeValue::AbsoluteY(0)), 4, true,  false),
    
    (0xA0, Instruction::LDY(AddressModeValue::Immediate(0)), 2, false, false),
    (0xA4, Instruction::LDY(AddressModeValue::ZeroPage(0)),  3, false, false),
    (0xB4, Instruction::LDY(AddressModeValue::ZeroPageX(0)), 4, false, false),
    (0xAC, Instruction::LDY(AddressModeValue::Absolute(0)),  4, false, false),
    (0xBC, Instruction::LDY(AddressModeValue::AbsoluteX(0)), 4, true,  false),
    
    (0x85, Instruction::STA(AddressModeValue::ZeroPage(0)),  3, false, false),
    (0x95, Instruction::STA(AddressModeValue::ZeroPageX(0)), 4, false, false),
    (0x8D, Instruction::STA(AddressModeValue::Absolute(0)),  4, false, false),
    (0x9D, Instruction::STA(AddressModeValue::AbsoluteX(0)), 5, false, false),
    (0x99, Instruction::STA(AddressModeValue::AbsoluteY(0)), 5, false, false),
    (0x81, Instruction::STA(AddressModeValue::IndirectX(0)), 6, false, false),
    (0x91, Instruction::STA(AddressModeValue::IndirectY(0)), 6, false, false),
    
    (0x86, Instruction::STX(AddressModeValue::ZeroPage(0)),  3, false, false),
    (0x96, Instruction::STX(AddressModeValue::ZeroPageY(0)), 4, false, false),
    (0x8E, Instruction::STX(AddressModeValue::Absolute(0)),  4, false, false),
    
    (0x84, Instruction::STY(AddressModeValue::ZeroPage(0)),  3, false, false),
    (0x94, Instruction::STY(AddressModeValue::ZeroPageX(0)), 4, false, false),
    (0x8C, Instruction::STY(AddressModeValue::Absolute(0)),  4, false, false),
    
    // 산술/논리 연산
    // ADC - Add with Carry
    (0x69, Instruction::ADC(AddressModeValue::Immediate(0)), 2, false, false),
    (0x65, Instruction::ADC(AddressModeValue::ZeroPage(0)),  3, false, false),
    (0x75, Instruction::ADC(AddressModeValue::ZeroPageX(0)), 4, false, false),
    (0x6D, Instruction::ADC(AddressModeValue::Absolute(0)),  4, false, false),
    (0x7D, Instruction::ADC(AddressModeValue::AbsoluteX(0)), 4, true,  false),
    (0x79, Instruction::ADC(AddressModeValue::AbsoluteY(0)), 4, true,  false),
    (0x61, Instruction::ADC(AddressModeValue::IndirectX(0)), 6, false, false),
    (0x71, Instruction::ADC(AddressModeValue::IndirectY(0)), 5, true,  false),
    
    // SBC - Subtract with Carry
    (0xE9, Instruction::SBC(AddressModeValue::Immediate(0)), 2, false, false),
    (0xE5, Instruction::SBC(AddressModeValue::ZeroPage(0)),  3, false, false),
    (0xF5, Instruction::SBC(AddressModeValue::ZeroPageX(0)), 4, false, false),
    (0xED, Instruction::SBC(AddressModeValue::Absolute(0)),  4, false, false),
    (0xFD, Instruction::SBC(AddressModeValue::AbsoluteX(0)), 4, true,  false),
    (0xF9, Instruction::SBC(AddressModeValue::AbsoluteY(0)), 4, true,  false),
    (0xE1, Instruction::SBC(AddressModeValue::IndirectX(0)), 6, false, false),
    (0xF1, Instruction::SBC(AddressModeValue::IndirectY(0)), 5, true,  false),
    
    // AND - Logical AND
    (0x29, Instruction::AND(AddressModeValue::Immediate(0)), 2, false, false),
    (0x25, Instruction::AND(AddressModeValue::ZeroPage(0)),  3, false, false),
    (0x35, Instruction::AND(AddressModeValue::ZeroPageX(0)), 4, false, false),
    (0x2D, Instruction::AND(AddressModeValue::Absolute(0)),  4, false, false),
    (0x3D, Instruction::AND(AddressModeValue::AbsoluteX(0)), 4, true,  false),
    (0x39, Instruction::AND(AddressModeValue::AbsoluteY(0)), 4, true,  false),
    (0x21, Instruction::AND(AddressModeValue::IndirectX(0)), 6, false, false),
    (0x31, Instruction::AND(AddressModeValue::IndirectY(0)), 5, true,  false),
    
    // ORA - Logical OR
    (0x09, Instruction::ORA(AddressModeValue::Immediate(0)), 2, false, false),
    (0x05, Instruction::ORA(AddressModeValue::ZeroPage(0)),  3, false, false),
    (0x15, Instruction::ORA(AddressModeValue::ZeroPageX(0)), 4, false, false),
    (0x0D, Instruction::ORA(AddressModeValue::Absolute(0)),  4, false, false),
    (0x1D, Instruction::ORA(AddressModeValue::AbsoluteX(0)), 4, true,  false),
    (0x19, Instruction::ORA(AddressModeValue::AbsoluteY(0)), 4, true,  false),
    (0x01, Instruction::ORA(AddressModeValue::IndirectX(0)), 6, false, false),
    (0x11, Instruction::ORA(AddressModeValue::IndirectY(0)), 5, true,  false),
    
    // EOR - Exclusive OR
    (0x49, Instruction::EOR(AddressModeValue::Immediate(0)), 2, false, false),
    (0x45, Instruction::EOR(AddressModeValue::ZeroPage(0)),  3, false, false),
    (0x55, Instruction::EOR(AddressModeValue::ZeroPageX(0)), 4, false, false),
    (0x4D, Instruction::EOR(AddressModeValue::Absolute(0)),  4, false, false),
    (0x5D, Instruction::EOR(AddressModeValue::AbsoluteX(0)), 4, true,  false),
    (0x59, Instruction::EOR(AddressModeValue::AbsoluteY(0)), 4, true,  false),
    (0x41, Instruction::EOR(AddressModeValue::IndirectX(0)), 6, false, false),
    (0x51, Instruction::EOR(AddressModeValue::IndirectY(0)), 5, true,  false),
    
    // 시프트/회전 명령어
    // ASL - Arithmetic Shift Left
    (0x0A, Instruction::ASL(AddressModeValue::Accumulator),   2, false, false),
    (0x06, Instruction::ASL(AddressModeValue::ZeroPage(0)),   5, false, false),
    (0x16, Instruction::ASL(AddressModeValue::ZeroPageX(0)),  6, false, false),
    (0x0E, Instruction::ASL(AddressModeValue::Absolute(0)),   6, false, false),
    (0x1E, Instruction::ASL(AddressModeValue::AbsoluteX(0)),  7, false, false),
    
    // LSR - Logical Shift Right
    (0x4A, Instruction::LSR(AddressModeValue::Accumulator),   2, false, false),
    (0x46, Instruction::LSR(AddressModeValue::ZeroPage(0)),   5, false, false),
    (0x56, Instruction::LSR(AddressModeValue::ZeroPageX(0)),  6, false, false),
    (0x4E, Instruction::LSR(AddressModeValue::Absolute(0)),   6, false, false),
    (0x5E, Instruction::LSR(AddressModeValue::AbsoluteX(0)),  7, false, false),
    
    // ROL - Rotate Left
    (0x2A, Instruction::ROL(AddressModeValue::Accumulator),   2, false, false),
    (0x26, Instruction::ROL(AddressModeValue::ZeroPage(0)),   5, false, false),
    (0x36, Instruction::ROL(AddressModeValue::ZeroPageX(0)),  6, false, false),
    (0x2E, Instruction::ROL(AddressModeValue::Absolute(0)),   6, false, false),
    (0x3E, Instruction::ROL(AddressModeValue::AbsoluteX(0)),  7, false, false),
    
    // ROR - Rotate Right
    (0x6A, Instruction::ROR(AddressModeValue::Accumulator),   2, false, false),
    (0x66, Instruction::ROR(AddressModeValue::ZeroPage(0)),   5, false, false),
    (0x76, Instruction::ROR(AddressModeValue::ZeroPageX(0)),  6, false, false),
    (0x6E, Instruction::ROR(AddressModeValue::Absolute(0)),   6, false, false),
    (0x7E, Instruction::ROR(AddressModeValue::AbsoluteX(0)),  7, false, false),
    
    // 증감 명령어
    // INC - Increment Memory
    (0xE6, Instruction::INC(AddressModeValue::ZeroPage(0)),   5, false, false),
    (0xF6, Instruction::INC(AddressModeValue::ZeroPageX(0)),  6, false, false),
    (0xEE, Instruction::INC(AddressModeValue::Absolute(0)),   6, false, false),
    (0xFE, Instruction::INC(AddressModeValue::AbsoluteX(0)),  7, false, false),
    
    // DEC - Decrement Memory
    (0xC6, Instruction::DEC(AddressModeValue::ZeroPage(0)),   5, false, false),
    (0xD6, Instruction::DEC(AddressModeValue::ZeroPageX(0)),  6, false, false),
    (0xCE, Instruction::DEC(AddressModeValue::Absolute(0)),   6, false, false),
    (0xDE, Instruction::DEC(AddressModeValue::AbsoluteX(0)),  7, false, false),
    
    // INX - Increment X Register
    (0xE8, Instruction::INX, 2, false, false),
    
    // INY - Increment Y Register
    (0xC8, Instruction::INY, 2, false, false),
    
    // DEX - Decrement X Register
    (0xCA, Instruction::DEX, 2, false, false),
    
    // DEY - Decrement Y Register
    (0x88, Instruction::DEY, 2, false, false),
    
    // 비교 명령어
    // CMP - Compare Accumulator
    (0xC9, Instruction::CMP(AddressModeValue::Immediate(0)),  2, false, false),
    (0xC5, Instruction::CMP(AddressModeValue::ZeroPage(0)),   3, false, false),
    (0xD5, Instruction::CMP(AddressModeValue::ZeroPageX(0)),  4, false, false),
    (0xCD, Instruction::CMP(AddressModeValue::Absolute(0)),   4, false, false),
    (0xDD, Instruction::CMP(AddressModeValue::AbsoluteX(0)),  4, true,  false),
    (0xD9, Instruction::CMP(AddressModeValue::AbsoluteY(0)),  4, true,  false),
    (0xC1, Instruction::CMP(AddressModeValue::IndirectX(0)),  6, false, false),
    (0xD1, Instruction::CMP(AddressModeValue::IndirectY(0)),  5, true,  false),
    
    // CPX - Compare X Register
    (0xE0, Instruction::CPX(AddressModeValue::Immediate(0)),  2, false, false),
    (0xE4, Instruction::CPX(AddressModeValue::ZeroPage(0)),   3, false, false),
    (0xEC, Instruction::CPX(AddressModeValue::Absolute(0)),   4, false, false),
    
    // CPY - Compare Y Register
    (0xC0, Instruction::CPY(AddressModeValue::Immediate(0)),  2, false, false),
    (0xC4, Instruction::CPY(AddressModeValue::ZeroPage(0)),   3, false, false),
    (0xCC, Instruction::CPY(AddressModeValue::Absolute(0)),   4, false, false),
    
    // BIT - Bit Test
    (0x24, Instruction::BIT(AddressModeValue::ZeroPage(0)),   3, false, false),
    (0x2C, Instruction::BIT(AddressModeValue::Absolute(0)),   4, false, false),
    
    // 레지스터 전송 명령어
    (0xAA, Instruction::TAX, 2, false, false), // Transfer A to X
    (0x8A, Instruction::TXA, 2, false, false), // Transfer X to A
    (0xA8, Instruction::TAY, 2, false, false), // Transfer A to Y
    (0x98, Instruction::TYA, 2, false, false), // Transfer Y to A
    (0xBA, Instruction::TSX, 2, false, false), // Transfer S to X
    (0x9A, Instruction::TXS, 2, false, false), // Transfer X to S
    
    // 스택 연산 명령어
    (0x48, Instruction::PHA, 3, false, false), // Push Accumulator
    (0x68, Instruction::PLA, 4, false, false), // Pull Accumulator
    (0x08, Instruction::PHP, 3, false, false), // Push Processor Status
    (0x28, Instruction::PLP, 4, false, false), // Pull Processor Status
    
    // 점프/분기 명령어
    // JMP - Jump
    (0x4C, Instruction::JMP(AddressModeValue::Absolute(0)),   3, false, false),
    (0x6C, Instruction::JMP(AddressModeValue::Indirect(0)),   5, false, false),
    
    // JSR - Jump to Subroutine
    (0x20, Instruction::JSR(AddressModeValue::Absolute(0)),   6, false, false),
    
    // RTS - Return from Subroutine
    (0x60, Instruction::RTS, 6, false, false),
    
    // 분기 명령어
    (0x90, Instruction::BCC(0), 2, false, true), // Branch on Carry Clear
    (0xB0, Instruction::BCS(0), 2, false, true), // Branch on Carry Set
    (0xF0, Instruction::BEQ(0), 2, false, true), // Branch on Equal (Z=1)
    (0xD0, Instruction::BNE(0), 2, false, true), // Branch on Not Equal (Z=0)
    (0x30, Instruction::BMI(0), 2, false, true), // Branch on Minus (N=1)
    (0x10, Instruction::BPL(0), 2, false, true), // Branch on Plus (N=0)
    (0x50, Instruction::BVC(0), 2, false, true), // Branch on Overflow Clear (V=0)
    (0x70, Instruction::BVS(0), 2, false, true), // Branch on Overflow Set (V=1)
    
    // 플래그 설정 명령어
    (0x18, Instruction::CLC, 2, false, false), // Clear Carry Flag
    (0x38, Instruction::SEC, 2, false, false), // Set Carry Flag
    (0x58, Instruction::CLI, 2, false, false), // Clear Interrupt Disable
    (0x78, Instruction::SEI, 2, false, false), // Set Interrupt Disable
    (0xB8, Instruction::CLV, 2, false, false), // Clear Overflow Flag
    (0xD8, Instruction::CLD, 2, false, false), // Clear Decimal Mode
    (0xF8, Instruction::SED, 2, false, false), // Set Decimal Mode
    
    // 시스템 명령어
    (0x00, Instruction::BRK, 7, false, false), // Force Interrupt
    (0x40, Instruction::RTI, 6, false, false), // Return from Interrupt
    (0xEA, Instruction::NOP, 2, false, false), // No Operation
];
