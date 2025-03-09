//! 옵코드 관련 처리를 위한 모듈
use crate::{AddressModeValue, CycleInfo, Instruction, InstructionInfo};
use std::collections::HashMap;

mod tables;
use tables::OPCODE_TABLE;

/// 옵코드 맵 - 초기화 시 정의된 옵코드 테이블을 사용
pub static OPCODE_MAP: std::sync::LazyLock<HashMap<u8, InstructionInfo>> =
    std::sync::LazyLock::new(|| generate_opcode_map());

/// 옵코드 맵 생성
fn generate_opcode_map() -> HashMap<u8, InstructionInfo> {
    let mut map = HashMap::new();
    
    // 테이블에서 정의된 명령어 매핑 추가
    for &(opcode, instruction, base_cycles, page_cross, branch_taken) in OPCODE_TABLE {
        map.insert(
            opcode,
            InstructionInfo::new(
                instruction,
                CycleInfo {
                    base_cycles,
                    page_cross,
                    branch_taken,
                },
            ),
        );
    }
    
    map
}

/// 명령어와 주소 모드로부터 옵코드 정보 찾기
pub fn get_opcode_info(instruction: Instruction) -> Option<InstructionInfo> {
    // 명령어 타입에 따라 적절한 조회 함수 호출
    match instruction {
        Instruction::LDA(mode) => get_opcode_for_mode("LDA", mode),
        Instruction::LDX(mode) => get_opcode_for_mode("LDX", mode),
        Instruction::LDY(mode) => get_opcode_for_mode("LDY", mode),
        Instruction::STA(mode) => get_opcode_for_mode("STA", mode),
        Instruction::STX(mode) => get_opcode_for_mode("STX", mode),
        Instruction::STY(mode) => get_opcode_for_mode("STY", mode),
        Instruction::ADC(mode) => get_opcode_for_mode("ADC", mode),
        Instruction::SBC(mode) => get_opcode_for_mode("SBC", mode),
        Instruction::AND(mode) => get_opcode_for_mode("AND", mode),
        Instruction::ORA(mode) => get_opcode_for_mode("ORA", mode),
        Instruction::EOR(mode) => get_opcode_for_mode("EOR", mode),
        Instruction::CMP(mode) => get_opcode_for_mode("CMP", mode),
        Instruction::CPX(mode) => get_opcode_for_mode("CPX", mode),
        Instruction::CPY(mode) => get_opcode_for_mode("CPY", mode),
        Instruction::BIT(mode) => get_opcode_for_mode("BIT", mode),
        Instruction::ASL(mode) => get_opcode_for_mode("ASL", mode),
        Instruction::LSR(mode) => get_opcode_for_mode("LSR", mode),
        Instruction::ROL(mode) => get_opcode_for_mode("ROL", mode),
        Instruction::ROR(mode) => get_opcode_for_mode("ROR", mode),
        Instruction::INC(mode) => get_opcode_for_mode("INC", mode),
        Instruction::DEC(mode) => get_opcode_for_mode("DEC", mode),
        Instruction::JMP(mode) => match mode {
            AddressModeValue::Absolute(_) => OPCODE_MAP.get(&0x4C).copied(),
            AddressModeValue::Indirect(_) => OPCODE_MAP.get(&0x6C).copied(),
            _ => None,
        },
        // 단일 옵코드 명령어 처리
        Instruction::BCC(_) => OPCODE_MAP.get(&0x90).copied(),
        Instruction::BCS(_) => OPCODE_MAP.get(&0xB0).copied(),
        Instruction::BEQ(_) => OPCODE_MAP.get(&0xF0).copied(),
        Instruction::BNE(_) => OPCODE_MAP.get(&0xD0).copied(),
        Instruction::BMI(_) => OPCODE_MAP.get(&0x30).copied(),
        Instruction::BPL(_) => OPCODE_MAP.get(&0x10).copied(),
        Instruction::BVC(_) => OPCODE_MAP.get(&0x50).copied(),
        Instruction::BVS(_) => OPCODE_MAP.get(&0x70).copied(),
        Instruction::RTS => OPCODE_MAP.get(&0x60).copied(),
        Instruction::JSR(_) => OPCODE_MAP.get(&0x20).copied(),
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
        Instruction::PHA => OPCODE_MAP.get(&0x48).copied(),
        Instruction::PLA => OPCODE_MAP.get(&0x68).copied(),
        Instruction::PHP => OPCODE_MAP.get(&0x08).copied(),
        Instruction::PLP => OPCODE_MAP.get(&0x28).copied(),
    }
}

/// 특정 명령어와 주소 모드에 맞는 옵코드 정보 찾기
fn get_opcode_for_mode(instruction_name: &str, mode: AddressModeValue) -> Option<InstructionInfo> {
    // 주소 모드에 따라 해당 명령어의 옵코드 찾기
    match (instruction_name, mode) {
        // LDA
        ("LDA", AddressModeValue::Immediate(_)) => OPCODE_MAP.get(&0xA9).copied(),
        ("LDA", AddressModeValue::ZeroPage(_)) => OPCODE_MAP.get(&0xA5).copied(),
        ("LDA", AddressModeValue::ZeroPageX(_)) => OPCODE_MAP.get(&0xB5).copied(),
        ("LDA", AddressModeValue::Absolute(_)) => OPCODE_MAP.get(&0xAD).copied(),
        ("LDA", AddressModeValue::AbsoluteX(_)) => OPCODE_MAP.get(&0xBD).copied(),
        ("LDA", AddressModeValue::AbsoluteY(_)) => OPCODE_MAP.get(&0xB9).copied(),
        ("LDA", AddressModeValue::IndirectX(_)) => OPCODE_MAP.get(&0xA1).copied(),
        ("LDA", AddressModeValue::IndirectY(_)) => OPCODE_MAP.get(&0xB1).copied(),
        
        // LDX
        ("LDX", AddressModeValue::Immediate(_)) => OPCODE_MAP.get(&0xA2).copied(),
        ("LDX", AddressModeValue::ZeroPage(_)) => OPCODE_MAP.get(&0xA6).copied(),
        ("LDX", AddressModeValue::ZeroPageY(_)) => OPCODE_MAP.get(&0xB6).copied(),
        ("LDX", AddressModeValue::Absolute(_)) => OPCODE_MAP.get(&0xAE).copied(),
        ("LDX", AddressModeValue::AbsoluteY(_)) => OPCODE_MAP.get(&0xBE).copied(),
        
        // LDY
        ("LDY", AddressModeValue::Immediate(_)) => OPCODE_MAP.get(&0xA0).copied(),
        ("LDY", AddressModeValue::ZeroPage(_)) => OPCODE_MAP.get(&0xA4).copied(),
        ("LDY", AddressModeValue::ZeroPageX(_)) => OPCODE_MAP.get(&0xB4).copied(),
        ("LDY", AddressModeValue::Absolute(_)) => OPCODE_MAP.get(&0xAC).copied(),
        ("LDY", AddressModeValue::AbsoluteX(_)) => OPCODE_MAP.get(&0xBC).copied(),
        
        // STA
        ("STA", AddressModeValue::ZeroPage(_)) => OPCODE_MAP.get(&0x85).copied(),
        ("STA", AddressModeValue::ZeroPageX(_)) => OPCODE_MAP.get(&0x95).copied(),
        ("STA", AddressModeValue::Absolute(_)) => OPCODE_MAP.get(&0x8D).copied(),
        ("STA", AddressModeValue::AbsoluteX(_)) => OPCODE_MAP.get(&0x9D).copied(),
        ("STA", AddressModeValue::AbsoluteY(_)) => OPCODE_MAP.get(&0x99).copied(),
        ("STA", AddressModeValue::IndirectX(_)) => OPCODE_MAP.get(&0x81).copied(),
        ("STA", AddressModeValue::IndirectY(_)) => OPCODE_MAP.get(&0x91).copied(),
        
        // STX
        ("STX", AddressModeValue::ZeroPage(_)) => OPCODE_MAP.get(&0x86).copied(),
        ("STX", AddressModeValue::ZeroPageY(_)) => OPCODE_MAP.get(&0x96).copied(),
        ("STX", AddressModeValue::Absolute(_)) => OPCODE_MAP.get(&0x8E).copied(),
        
        // STY
        ("STY", AddressModeValue::ZeroPage(_)) => OPCODE_MAP.get(&0x84).copied(),
        ("STY", AddressModeValue::ZeroPageX(_)) => OPCODE_MAP.get(&0x94).copied(),
        ("STY", AddressModeValue::Absolute(_)) => OPCODE_MAP.get(&0x8C).copied(),
        
        // ASL, LSR, ROL, ROR - 누산기 모드 추가
        ("ASL", AddressModeValue::Accumulator) => OPCODE_MAP.get(&0x0A).copied(),
        ("ASL", AddressModeValue::ZeroPage(_)) => OPCODE_MAP.get(&0x06).copied(),
        ("ASL", AddressModeValue::ZeroPageX(_)) => OPCODE_MAP.get(&0x16).copied(),
        ("ASL", AddressModeValue::Absolute(_)) => OPCODE_MAP.get(&0x0E).copied(),
        ("ASL", AddressModeValue::AbsoluteX(_)) => OPCODE_MAP.get(&0x1E).copied(),
        
        ("LSR", AddressModeValue::Accumulator) => OPCODE_MAP.get(&0x4A).copied(),
        ("LSR", AddressModeValue::ZeroPage(_)) => OPCODE_MAP.get(&0x46).copied(),
        ("LSR", AddressModeValue::ZeroPageX(_)) => OPCODE_MAP.get(&0x56).copied(),
        ("LSR", AddressModeValue::Absolute(_)) => OPCODE_MAP.get(&0x4E).copied(),
        ("LSR", AddressModeValue::AbsoluteX(_)) => OPCODE_MAP.get(&0x5E).copied(),
        
        ("ROL", AddressModeValue::Accumulator) => OPCODE_MAP.get(&0x2A).copied(),
        ("ROL", AddressModeValue::ZeroPage(_)) => OPCODE_MAP.get(&0x26).copied(),
        ("ROL", AddressModeValue::ZeroPageX(_)) => OPCODE_MAP.get(&0x36).copied(),
        ("ROL", AddressModeValue::Absolute(_)) => OPCODE_MAP.get(&0x2E).copied(),
        ("ROL", AddressModeValue::AbsoluteX(_)) => OPCODE_MAP.get(&0x3E).copied(),
        
        ("ROR", AddressModeValue::Accumulator) => OPCODE_MAP.get(&0x6A).copied(),
        ("ROR", AddressModeValue::ZeroPage(_)) => OPCODE_MAP.get(&0x66).copied(),
        ("ROR", AddressModeValue::ZeroPageX(_)) => OPCODE_MAP.get(&0x76).copied(),
        ("ROR", AddressModeValue::Absolute(_)) => OPCODE_MAP.get(&0x6E).copied(),
        ("ROR", AddressModeValue::AbsoluteX(_)) => OPCODE_MAP.get(&0x7E).copied(),
        
        // INC/DEC
        ("INC", AddressModeValue::ZeroPage(_)) => OPCODE_MAP.get(&0xE6).copied(),
        ("INC", AddressModeValue::ZeroPageX(_)) => OPCODE_MAP.get(&0xF6).copied(),
        ("INC", AddressModeValue::Absolute(_)) => OPCODE_MAP.get(&0xEE).copied(),
        ("INC", AddressModeValue::AbsoluteX(_)) => OPCODE_MAP.get(&0xFE).copied(),
        
        ("DEC", AddressModeValue::ZeroPage(_)) => OPCODE_MAP.get(&0xC6).copied(),
        ("DEC", AddressModeValue::ZeroPageX(_)) => OPCODE_MAP.get(&0xD6).copied(),
        ("DEC", AddressModeValue::Absolute(_)) => OPCODE_MAP.get(&0xCE).copied(),
        ("DEC", AddressModeValue::AbsoluteX(_)) => OPCODE_MAP.get(&0xDE).copied(),
        
        // BIT
        ("BIT", AddressModeValue::ZeroPage(_)) => OPCODE_MAP.get(&0x24).copied(),
        ("BIT", AddressModeValue::Absolute(_)) => OPCODE_MAP.get(&0x2C).copied(),
        
        // 기타 필요한 명령어 매핑을 추가
        // ... (필요한 만큼 추가)
        
        _ => None, // 매치되지 않는 조합
    }
}
