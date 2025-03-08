use crate::{AddressMode, CycleInfo, Instruction, InstructionInfo};
use std::collections::HashMap;
use std::sync::LazyLock;

/// 옵코드와 명령어 정보의 매핑
pub static OPCODE_MAP: LazyLock<HashMap<u8, InstructionInfo>> = LazyLock::new(|| {
    let mut map = HashMap::new();

    // LDA 명령어
    map.insert(
        0xA9,
        InstructionInfo::new(Instruction::LDA(AddressMode::Immediate), CycleInfo::new(2)),
    );
    map.insert(
        0xA5,
        InstructionInfo::new(Instruction::LDA(AddressMode::ZeroPage), CycleInfo::new(3)),
    );
    map.insert(
        0xB5,
        InstructionInfo::new(Instruction::LDA(AddressMode::ZeroPageX), CycleInfo::new(4)),
    );
    map.insert(
        0xAD,
        InstructionInfo::new(Instruction::LDA(AddressMode::Absolute), CycleInfo::new(4)),
    );
    map.insert(
        0xBD,
        InstructionInfo::new(Instruction::LDA(AddressMode::AbsoluteX), CycleInfo::new(4)),
    );
    map.insert(
        0xB9,
        InstructionInfo::new(Instruction::LDA(AddressMode::AbsoluteY), CycleInfo::new(4)),
    );
    map.insert(
        0xA1,
        InstructionInfo::new(Instruction::LDA(AddressMode::IndirectX), CycleInfo::new(6)),
    );
    map.insert(
        0xB1,
        InstructionInfo::new(Instruction::LDA(AddressMode::IndirectY), CycleInfo::new(5)),
    );

    // LDX 명령어
    map.insert(
        0xA2,
        InstructionInfo::new(Instruction::LDX(AddressMode::Immediate), CycleInfo::new(2)),
    );
    map.insert(
        0xA6,
        InstructionInfo::new(Instruction::LDX(AddressMode::ZeroPage), CycleInfo::new(3)),
    );
    map.insert(
        0xB6,
        InstructionInfo::new(Instruction::LDX(AddressMode::ZeroPageY), CycleInfo::new(4)),
    );
    map.insert(
        0xAE,
        InstructionInfo::new(Instruction::LDX(AddressMode::Absolute), CycleInfo::new(4)),
    );
    map.insert(
        0xBE,
        InstructionInfo::new(Instruction::LDX(AddressMode::AbsoluteY), CycleInfo::new(4)),
    );

    // LDY 명령어
    map.insert(
        0xA0,
        InstructionInfo::new(Instruction::LDY(AddressMode::Immediate), CycleInfo::new(2)),
    );
    map.insert(
        0xA4,
        InstructionInfo::new(Instruction::LDY(AddressMode::ZeroPage), CycleInfo::new(3)),
    );
    map.insert(
        0xB4,
        InstructionInfo::new(Instruction::LDY(AddressMode::ZeroPageX), CycleInfo::new(4)),
    );
    map.insert(
        0xAC,
        InstructionInfo::new(Instruction::LDY(AddressMode::Absolute), CycleInfo::new(4)),
    );
    map.insert(
        0xBC,
        InstructionInfo::new(Instruction::LDY(AddressMode::AbsoluteX), CycleInfo::new(4)),
    );

    // STA 명령어
    map.insert(
        0x85,
        InstructionInfo::new(Instruction::STA(AddressMode::ZeroPage), CycleInfo::new(3)),
    );
    map.insert(
        0x95,
        InstructionInfo::new(Instruction::STA(AddressMode::ZeroPageX), CycleInfo::new(4)),
    );
    map.insert(
        0x8D,
        InstructionInfo::new(Instruction::STA(AddressMode::Absolute), CycleInfo::new(4)),
    );
    map.insert(
        0x9D,
        InstructionInfo::new(Instruction::STA(AddressMode::AbsoluteX), CycleInfo::new(5)),
    );
    map.insert(
        0x99,
        InstructionInfo::new(Instruction::STA(AddressMode::AbsoluteY), CycleInfo::new(5)),
    );
    map.insert(
        0x81,
        InstructionInfo::new(Instruction::STA(AddressMode::IndirectX), CycleInfo::new(6)),
    );
    map.insert(
        0x91,
        InstructionInfo::new(Instruction::STA(AddressMode::IndirectY), CycleInfo::new(6)),
    );

    // 스택 연산
    map.insert(
        0x48,
        InstructionInfo::new(Instruction::PHA, CycleInfo::new(3)),
    );
    map.insert(
        0x68,
        InstructionInfo::new(Instruction::PLA, CycleInfo::new(4)),
    );
    map.insert(
        0x08,
        InstructionInfo::new(Instruction::PHP, CycleInfo::new(3)),
    );
    map.insert(
        0x28,
        InstructionInfo::new(Instruction::PLP, CycleInfo::new(4)),
    );

    // 레지스터 연산
    map.insert(
        0xAA,
        InstructionInfo::new(Instruction::TAX, CycleInfo::new(2)),
    );
    map.insert(
        0x8A,
        InstructionInfo::new(Instruction::TXA, CycleInfo::new(2)),
    );
    map.insert(
        0xA8,
        InstructionInfo::new(Instruction::TAY, CycleInfo::new(2)),
    );
    map.insert(
        0x98,
        InstructionInfo::new(Instruction::TYA, CycleInfo::new(2)),
    );
    map.insert(
        0xBA,
        InstructionInfo::new(Instruction::TSX, CycleInfo::new(2)),
    );
    map.insert(
        0x9A,
        InstructionInfo::new(Instruction::TXS, CycleInfo::new(2)),
    );

    // 증감 연산
    map.insert(
        0xE6,
        InstructionInfo::new(Instruction::INC(AddressMode::ZeroPage), CycleInfo::new(5)),
    );
    map.insert(
        0xF6,
        InstructionInfo::new(Instruction::INC(AddressMode::ZeroPageX), CycleInfo::new(6)),
    );
    map.insert(
        0xEE,
        InstructionInfo::new(Instruction::INC(AddressMode::Absolute), CycleInfo::new(6)),
    );
    map.insert(
        0xFE,
        InstructionInfo::new(Instruction::INC(AddressMode::AbsoluteX), CycleInfo::new(7)),
    );
    map.insert(
        0xE8,
        InstructionInfo::new(Instruction::INX, CycleInfo::new(2)),
    );
    map.insert(
        0xC8,
        InstructionInfo::new(Instruction::INY, CycleInfo::new(2)),
    );

    map.insert(
        0xC6,
        InstructionInfo::new(Instruction::DEC(AddressMode::ZeroPage), CycleInfo::new(5)),
    );
    map.insert(
        0xD6,
        InstructionInfo::new(Instruction::DEC(AddressMode::ZeroPageX), CycleInfo::new(6)),
    );
    map.insert(
        0xCE,
        InstructionInfo::new(Instruction::DEC(AddressMode::Absolute), CycleInfo::new(6)),
    );
    map.insert(
        0xDE,
        InstructionInfo::new(Instruction::DEC(AddressMode::AbsoluteX), CycleInfo::new(7)),
    );
    map.insert(
        0xCA,
        InstructionInfo::new(Instruction::DEX, CycleInfo::new(2)),
    );
    map.insert(
        0x88,
        InstructionInfo::new(Instruction::DEY, CycleInfo::new(2)),
    );

    // 분기 명령
    map.insert(
        0x90,
        InstructionInfo::new(Instruction::BCC, CycleInfo::new(2)),
    );
    map.insert(
        0xB0,
        InstructionInfo::new(Instruction::BCS, CycleInfo::new(2)),
    );
    map.insert(
        0xF0,
        InstructionInfo::new(Instruction::BEQ, CycleInfo::new(2)),
    );
    map.insert(
        0xD0,
        InstructionInfo::new(Instruction::BNE, CycleInfo::new(2)),
    );
    map.insert(
        0x30,
        InstructionInfo::new(Instruction::BMI, CycleInfo::new(2)),
    );
    map.insert(
        0x10,
        InstructionInfo::new(Instruction::BPL, CycleInfo::new(2)),
    );
    map.insert(
        0x50,
        InstructionInfo::new(Instruction::BVC, CycleInfo::new(2)),
    );
    map.insert(
        0x70,
        InstructionInfo::new(Instruction::BVS, CycleInfo::new(2)),
    );

    // 점프/서브루틴
    map.insert(
        0x4C,
        InstructionInfo::new(Instruction::JMP(AddressMode::Absolute), CycleInfo::new(3)),
    );
    map.insert(
        0x6C,
        InstructionInfo::new(Instruction::JMP(AddressMode::Indirect), CycleInfo::new(5)),
    );
    map.insert(
        0x20,
        InstructionInfo::new(Instruction::JSR(AddressMode::Absolute), CycleInfo::new(6)),
    );
    map.insert(
        0x60,
        InstructionInfo::new(Instruction::RTS, CycleInfo::new(6)),
    );

    // 인터럽트
    map.insert(
        0x00,
        InstructionInfo::new(Instruction::BRK, CycleInfo::new(7)),
    );
    map.insert(
        0x40,
        InstructionInfo::new(Instruction::RTI, CycleInfo::new(6)),
    );

    // 기타
    map.insert(
        0x18,
        InstructionInfo::new(Instruction::CLC, CycleInfo::new(2)),
    );
    map.insert(
        0x38,
        InstructionInfo::new(Instruction::SEC, CycleInfo::new(2)),
    );
    map.insert(
        0x58,
        InstructionInfo::new(Instruction::CLI, CycleInfo::new(2)),
    );
    map.insert(
        0x78,
        InstructionInfo::new(Instruction::SEI, CycleInfo::new(2)),
    );
    map.insert(
        0xD8,
        InstructionInfo::new(Instruction::CLD, CycleInfo::new(2)),
    );
    map.insert(
        0xF8,
        InstructionInfo::new(Instruction::SED, CycleInfo::new(2)),
    );
    map.insert(
        0xB8,
        InstructionInfo::new(Instruction::CLV, CycleInfo::new(2)),
    );
    map.insert(
        0xEA,
        InstructionInfo::new(Instruction::NOP, CycleInfo::new(2)),
    );

    // BIT 명령어
    map.insert(
        0x24,
        InstructionInfo::new(Instruction::BIT(AddressMode::ZeroPage), CycleInfo::new(3)),
    );
    map.insert(
        0x2C,
        InstructionInfo::new(Instruction::BIT(AddressMode::Absolute), CycleInfo::new(4)),
    );

    map
});

/// 명령어와 주소 모드로부터 옵코드 정보 찾기
pub fn get_opcode_info(instruction: Instruction) -> Option<InstructionInfo> {
    match instruction {
        Instruction::LDA(mode) => match mode {
            AddressMode::Immediate => OPCODE_MAP.get(&0xA9).copied(),
            AddressMode::ZeroPage => OPCODE_MAP.get(&0xA5).copied(),
            AddressMode::ZeroPageX => OPCODE_MAP.get(&0xB5).copied(),
            AddressMode::Absolute => OPCODE_MAP.get(&0xAD).copied(),
            AddressMode::AbsoluteX => OPCODE_MAP.get(&0xBD).copied(),
            AddressMode::AbsoluteY => OPCODE_MAP.get(&0xB9).copied(),
            AddressMode::IndirectX => OPCODE_MAP.get(&0xA1).copied(),
            AddressMode::IndirectY => OPCODE_MAP.get(&0xB1).copied(),
            _ => None,
        },
        Instruction::LDX(mode) => match mode {
            AddressMode::Immediate => OPCODE_MAP.get(&0xA2).copied(),
            AddressMode::ZeroPage => OPCODE_MAP.get(&0xA6).copied(),
            AddressMode::ZeroPageY => OPCODE_MAP.get(&0xB6).copied(),
            AddressMode::Absolute => OPCODE_MAP.get(&0xAE).copied(),
            AddressMode::AbsoluteY => OPCODE_MAP.get(&0xBE).copied(),
            _ => None,
        },
        Instruction::LDY(mode) => match mode {
            AddressMode::Immediate => OPCODE_MAP.get(&0xA0).copied(),
            AddressMode::ZeroPage => OPCODE_MAP.get(&0xA4).copied(),
            AddressMode::ZeroPageX => OPCODE_MAP.get(&0xB4).copied(),
            AddressMode::Absolute => OPCODE_MAP.get(&0xAC).copied(),
            AddressMode::AbsoluteX => OPCODE_MAP.get(&0xBC).copied(),
            _ => None,
        },
        Instruction::STA(mode) => match mode {
            AddressMode::ZeroPage => OPCODE_MAP.get(&0x85).copied(),
            AddressMode::ZeroPageX => OPCODE_MAP.get(&0x95).copied(),
            AddressMode::Absolute => OPCODE_MAP.get(&0x8D).copied(),
            AddressMode::AbsoluteX => OPCODE_MAP.get(&0x9D).copied(),
            AddressMode::AbsoluteY => OPCODE_MAP.get(&0x99).copied(),
            AddressMode::IndirectX => OPCODE_MAP.get(&0x81).copied(),
            AddressMode::IndirectY => OPCODE_MAP.get(&0x91).copied(),
            _ => None,
        },
        Instruction::INC(mode) => match mode {
            AddressMode::ZeroPage => OPCODE_MAP.get(&0xE6).copied(),
            AddressMode::ZeroPageX => OPCODE_MAP.get(&0xF6).copied(),
            AddressMode::Absolute => OPCODE_MAP.get(&0xEE).copied(),
            AddressMode::AbsoluteX => OPCODE_MAP.get(&0xFE).copied(),
            _ => None,
        },
        Instruction::DEC(mode) => match mode {
            AddressMode::ZeroPage => OPCODE_MAP.get(&0xC6).copied(),
            AddressMode::ZeroPageX => OPCODE_MAP.get(&0xD6).copied(),
            AddressMode::Absolute => OPCODE_MAP.get(&0xCE).copied(),
            AddressMode::AbsoluteX => OPCODE_MAP.get(&0xDE).copied(),
            _ => None,
        },
        Instruction::JMP(mode) => match mode {
            AddressMode::Absolute => OPCODE_MAP.get(&0x4C).copied(),
            AddressMode::Indirect => OPCODE_MAP.get(&0x6C).copied(),
            _ => None,
        },
        Instruction::JSR(AddressMode::Absolute) => OPCODE_MAP.get(&0x20).copied(),
        Instruction::BIT(mode) => match mode {
            AddressMode::ZeroPage => OPCODE_MAP.get(&0x24).copied(),
            AddressMode::Absolute => OPCODE_MAP.get(&0x2C).copied(),
            _ => None,
        },
        Instruction::PHA => OPCODE_MAP.get(&0x48).copied(),
        Instruction::PLA => OPCODE_MAP.get(&0x68).copied(),
        Instruction::PHP => OPCODE_MAP.get(&0x08).copied(),
        Instruction::PLP => OPCODE_MAP.get(&0x28).copied(),
        Instruction::TAX => OPCODE_MAP.get(&0xAA).copied(),
        Instruction::TXA => OPCODE_MAP.get(&0x8A).copied(),
        Instruction::TAY => OPCODE_MAP.get(&0xA8).copied(),
        Instruction::TYA => OPCODE_MAP.get(&0x98).copied(),
        Instruction::TSX => OPCODE_MAP.get(&0xBA).copied(),
        Instruction::TXS => OPCODE_MAP.get(&0x9A).copied(),
        Instruction::INX => OPCODE_MAP.get(&0xE8).copied(),
        Instruction::INY => OPCODE_MAP.get(&0xC8).copied(),
        Instruction::DEX => OPCODE_MAP.get(&0xCA).copied(),
        Instruction::DEY => OPCODE_MAP.get(&0x88).copied(),
        Instruction::BCC => OPCODE_MAP.get(&0x90).copied(),
        Instruction::BCS => OPCODE_MAP.get(&0xB0).copied(),
        Instruction::BEQ => OPCODE_MAP.get(&0xF0).copied(),
        Instruction::BNE => OPCODE_MAP.get(&0xD0).copied(),
        Instruction::BMI => OPCODE_MAP.get(&0x30).copied(),
        Instruction::BPL => OPCODE_MAP.get(&0x10).copied(),
        Instruction::BVC => OPCODE_MAP.get(&0x50).copied(),
        Instruction::BVS => OPCODE_MAP.get(&0x70).copied(),
        Instruction::RTS => OPCODE_MAP.get(&0x60).copied(),
        Instruction::BRK => OPCODE_MAP.get(&0x00).copied(),
        Instruction::RTI => OPCODE_MAP.get(&0x40).copied(),
        Instruction::CLC => OPCODE_MAP.get(&0x18).copied(),
        Instruction::SEC => OPCODE_MAP.get(&0x38).copied(),
        Instruction::CLI => OPCODE_MAP.get(&0x58).copied(),
        Instruction::SEI => OPCODE_MAP.get(&0x78).copied(),
        Instruction::CLD => OPCODE_MAP.get(&0xD8).copied(),
        Instruction::SED => OPCODE_MAP.get(&0xF8).copied(),
        Instruction::CLV => OPCODE_MAP.get(&0xB8).copied(),
        Instruction::NOP => OPCODE_MAP.get(&0xEA).copied(),
        _ => None,
    }
}
