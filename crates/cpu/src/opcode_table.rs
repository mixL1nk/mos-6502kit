use crate::instruction::{AddressMode, CycleInfo, Instruction, InstructionInfo};

/// opcode 테이블 초기화 함수
pub fn initialize_opcode_table() -> [Option<InstructionInfo>; 256] {
    let mut table = [None; 256];

    // LDA 명령어
    table[0xA9] = Some(InstructionInfo::new(
        Instruction::LDA(AddressMode::Immediate),
        CycleInfo::new(2),
    ));
    table[0xA5] = Some(InstructionInfo::new(
        Instruction::LDA(AddressMode::ZeroPage),
        CycleInfo::new(3),
    ));
    table[0xB5] = Some(InstructionInfo::new(
        Instruction::LDA(AddressMode::ZeroPageX),
        CycleInfo::new(4),
    ));
    table[0xAD] = Some(InstructionInfo::new(
        Instruction::LDA(AddressMode::Absolute),
        CycleInfo::new(4),
    ));
    table[0xBD] = Some(InstructionInfo::new(
        Instruction::LDA(AddressMode::AbsoluteX),
        CycleInfo::new(4),
    ));
    table[0xB9] = Some(InstructionInfo::new(
        Instruction::LDA(AddressMode::AbsoluteY),
        CycleInfo::new(4),
    ));
    table[0xA1] = Some(InstructionInfo::new(
        Instruction::LDA(AddressMode::IndirectX),
        CycleInfo::new(6),
    ));
    table[0xB1] = Some(InstructionInfo::new(
        Instruction::LDA(AddressMode::IndirectY),
        CycleInfo::new(5),
    ));

    // LDX 명령어
    table[0xA2] = Some(InstructionInfo::new(
        Instruction::LDX(AddressMode::Immediate),
        CycleInfo::new(2),
    ));
    table[0xA6] = Some(InstructionInfo::new(
        Instruction::LDX(AddressMode::ZeroPage),
        CycleInfo::new(3),
    ));
    table[0xB6] = Some(InstructionInfo::new(
        Instruction::LDX(AddressMode::ZeroPageY),
        CycleInfo::new(4),
    ));
    table[0xAE] = Some(InstructionInfo::new(
        Instruction::LDX(AddressMode::Absolute),
        CycleInfo::new(4),
    ));
    table[0xBE] = Some(InstructionInfo::new(
        Instruction::LDX(AddressMode::AbsoluteY),
        CycleInfo::new(4),
    ));

    // LDY 명령어
    table[0xA0] = Some(InstructionInfo::new(
        Instruction::LDY(AddressMode::Immediate),
        CycleInfo::new(2),
    ));
    table[0xA4] = Some(InstructionInfo::new(
        Instruction::LDY(AddressMode::ZeroPage),
        CycleInfo::new(3),
    ));
    table[0xB4] = Some(InstructionInfo::new(
        Instruction::LDY(AddressMode::ZeroPageX),
        CycleInfo::new(4),
    ));
    table[0xAC] = Some(InstructionInfo::new(
        Instruction::LDY(AddressMode::Absolute),
        CycleInfo::new(4),
    ));
    table[0xBC] = Some(InstructionInfo::new(
        Instruction::LDY(AddressMode::AbsoluteX),
        CycleInfo::new(4),
    ));

    // STA 명령어
    table[0x85] = Some(InstructionInfo::new(
        Instruction::STA(AddressMode::ZeroPage),
        CycleInfo::new(3),
    ));
    table[0x95] = Some(InstructionInfo::new(
        Instruction::STA(AddressMode::ZeroPageX),
        CycleInfo::new(4),
    ));
    table[0x8D] = Some(InstructionInfo::new(
        Instruction::STA(AddressMode::Absolute),
        CycleInfo::new(4),
    ));
    table[0x9D] = Some(InstructionInfo::new(
        Instruction::STA(AddressMode::AbsoluteX),
        CycleInfo::new(5),
    ));
    table[0x99] = Some(InstructionInfo::new(
        Instruction::STA(AddressMode::AbsoluteY),
        CycleInfo::new(5),
    ));
    table[0x81] = Some(InstructionInfo::new(
        Instruction::STA(AddressMode::IndirectX),
        CycleInfo::new(6),
    ));
    table[0x91] = Some(InstructionInfo::new(
        Instruction::STA(AddressMode::IndirectY),
        CycleInfo::new(6),
    ));

    // 스택 연산
    table[0x48] = Some(InstructionInfo::new(Instruction::PHA, CycleInfo::new(3)));
    table[0x68] = Some(InstructionInfo::new(Instruction::PLA, CycleInfo::new(4)));
    table[0x08] = Some(InstructionInfo::new(Instruction::PHP, CycleInfo::new(3)));
    table[0x28] = Some(InstructionInfo::new(Instruction::PLP, CycleInfo::new(4)));

    // 레지스터 연산
    table[0xAA] = Some(InstructionInfo::new(Instruction::TAX, CycleInfo::new(2)));
    table[0x8A] = Some(InstructionInfo::new(Instruction::TXA, CycleInfo::new(2)));
    table[0xA8] = Some(InstructionInfo::new(Instruction::TAY, CycleInfo::new(2)));
    table[0x98] = Some(InstructionInfo::new(Instruction::TYA, CycleInfo::new(2)));
    table[0xBA] = Some(InstructionInfo::new(Instruction::TSX, CycleInfo::new(2)));
    table[0x9A] = Some(InstructionInfo::new(Instruction::TXS, CycleInfo::new(2)));

    // 증감 연산
    table[0xE6] = Some(InstructionInfo::new(
        Instruction::INC(AddressMode::ZeroPage),
        CycleInfo::new(5),
    ));
    table[0xF6] = Some(InstructionInfo::new(
        Instruction::INC(AddressMode::ZeroPageX),
        CycleInfo::new(6),
    ));
    table[0xEE] = Some(InstructionInfo::new(
        Instruction::INC(AddressMode::Absolute),
        CycleInfo::new(6),
    ));
    table[0xFE] = Some(InstructionInfo::new(
        Instruction::INC(AddressMode::AbsoluteX),
        CycleInfo::new(7),
    ));
    table[0xE8] = Some(InstructionInfo::new(Instruction::INX, CycleInfo::new(2)));
    table[0xC8] = Some(InstructionInfo::new(Instruction::INY, CycleInfo::new(2)));

    table[0xC6] = Some(InstructionInfo::new(
        Instruction::DEC(AddressMode::ZeroPage),
        CycleInfo::new(5),
    ));
    table[0xD6] = Some(InstructionInfo::new(
        Instruction::DEC(AddressMode::ZeroPageX),
        CycleInfo::new(6),
    ));
    table[0xCE] = Some(InstructionInfo::new(
        Instruction::DEC(AddressMode::Absolute),
        CycleInfo::new(6),
    ));
    table[0xDE] = Some(InstructionInfo::new(
        Instruction::DEC(AddressMode::AbsoluteX),
        CycleInfo::new(7),
    ));
    table[0xCA] = Some(InstructionInfo::new(Instruction::DEX, CycleInfo::new(2)));
    table[0x88] = Some(InstructionInfo::new(Instruction::DEY, CycleInfo::new(2)));

    // 분기 명령
    table[0x90] = Some(InstructionInfo::new(Instruction::BCC, CycleInfo::new(2)));
    table[0xB0] = Some(InstructionInfo::new(Instruction::BCS, CycleInfo::new(2)));
    table[0xF0] = Some(InstructionInfo::new(Instruction::BEQ, CycleInfo::new(2)));
    table[0xD0] = Some(InstructionInfo::new(Instruction::BNE, CycleInfo::new(2)));
    table[0x30] = Some(InstructionInfo::new(Instruction::BMI, CycleInfo::new(2)));
    table[0x10] = Some(InstructionInfo::new(Instruction::BPL, CycleInfo::new(2)));
    table[0x50] = Some(InstructionInfo::new(Instruction::BVC, CycleInfo::new(2)));
    table[0x70] = Some(InstructionInfo::new(Instruction::BVS, CycleInfo::new(2)));

    // 점프/서브루틴
    table[0x4C] = Some(InstructionInfo::new(
        Instruction::JMP(AddressMode::Absolute),
        CycleInfo::new(3),
    ));
    table[0x6C] = Some(InstructionInfo::new(
        Instruction::JMP(AddressMode::Indirect),
        CycleInfo::new(5),
    ));
    table[0x20] = Some(InstructionInfo::new(
        Instruction::JSR(AddressMode::Absolute),
        CycleInfo::new(6),
    ));
    table[0x60] = Some(InstructionInfo::new(Instruction::RTS, CycleInfo::new(6)));

    // 인터럽트
    table[0x00] = Some(InstructionInfo::new(Instruction::BRK, CycleInfo::new(7)));
    table[0x40] = Some(InstructionInfo::new(Instruction::RTI, CycleInfo::new(6)));

    // 기타
    table[0x18] = Some(InstructionInfo::new(Instruction::CLC, CycleInfo::new(2)));
    table[0x38] = Some(InstructionInfo::new(Instruction::SEC, CycleInfo::new(2)));
    table[0x58] = Some(InstructionInfo::new(Instruction::CLI, CycleInfo::new(2)));
    table[0x78] = Some(InstructionInfo::new(Instruction::SEI, CycleInfo::new(2)));
    table[0xD8] = Some(InstructionInfo::new(Instruction::CLD, CycleInfo::new(2)));
    table[0xF8] = Some(InstructionInfo::new(Instruction::SED, CycleInfo::new(2)));
    table[0xB8] = Some(InstructionInfo::new(Instruction::CLV, CycleInfo::new(2)));
    table[0xEA] = Some(InstructionInfo::new(Instruction::NOP, CycleInfo::new(2)));

    // BIT 명령어
    table[0x24] = Some(InstructionInfo::new(
        Instruction::BIT(AddressMode::ZeroPage),
        CycleInfo::new(3),
    ));
    table[0x2C] = Some(InstructionInfo::new(
        Instruction::BIT(AddressMode::Absolute),
        CycleInfo::new(4),
    ));

    table
}
