use crate::{AddressModeValue, CycleInfo, Instruction, InstructionInfo};
use std::collections::HashMap;
use std::sync::LazyLock;

/*

TODO: 공용 타입을 만들기 위해 이 파일을 만들었는데
이 파일에 너무 많은 코드가 들어가게 되었다.
이 파일을 분리하는 것이 좋을 것 같다.
그리고 사용하는 모든 명령어를 여기에 추가하는 것은 좋지 않을 것 같다.

어셈블러/디스어셈블러/에뮬레이터 모두가 사용가능한 타입을 생가하고 만들었는데
복잡성이 자꾸 늘어나는거 같다.

*/

/// 옵코드와 명령어 정보의 매핑
pub static OPCODE_MAP: LazyLock<HashMap<u8, InstructionInfo>> = LazyLock::new(|| {
    let mut map = HashMap::new();

    // LDA 명령어
    map.insert(
        0xA9,
        InstructionInfo::new(
            Instruction::LDA(AddressModeValue::Immediate(0)),
            CycleInfo::new(2),
        ),
    );
    map.insert(
        0xA5,
        InstructionInfo::new(
            Instruction::LDA(AddressModeValue::ZeroPage(0)),
            CycleInfo::new(3),
        ),
    );
    map.insert(
        0xB5,
        InstructionInfo::new(
            Instruction::LDA(AddressModeValue::ZeroPageX(0)),
            CycleInfo::new(4),
        ),
    );
    map.insert(
        0xAD,
        InstructionInfo::new(
            Instruction::LDA(AddressModeValue::Absolute(0)),
            CycleInfo::new(4),
        ),
    );
    map.insert(
        0xBD,
        InstructionInfo::new(
            Instruction::LDA(AddressModeValue::AbsoluteX(0)),
            CycleInfo::new(4),
        ),
    );
    map.insert(
        0xB9,
        InstructionInfo::new(
            Instruction::LDA(AddressModeValue::AbsoluteY(0)),
            CycleInfo::new(4),
        ),
    );
    map.insert(
        0xA1,
        InstructionInfo::new(
            Instruction::LDA(AddressModeValue::IndirectX(0)),
            CycleInfo::new(6),
        ),
    );
    map.insert(
        0xB1,
        InstructionInfo::new(
            Instruction::LDA(AddressModeValue::IndirectY(0)),
            CycleInfo::new(5),
        ),
    );

    // LDX 명령어
    map.insert(
        0xA2,
        InstructionInfo::new(
            Instruction::LDX(AddressModeValue::Immediate(0)),
            CycleInfo::new(2),
        ),
    );
    map.insert(
        0xA6,
        InstructionInfo::new(
            Instruction::LDX(AddressModeValue::ZeroPage(0)),
            CycleInfo::new(3),
        ),
    );
    map.insert(
        0xB6,
        InstructionInfo::new(
            Instruction::LDX(AddressModeValue::ZeroPageY(0)),
            CycleInfo::new(4),
        ),
    );
    map.insert(
        0xAE,
        InstructionInfo::new(
            Instruction::LDX(AddressModeValue::Absolute(0)),
            CycleInfo::new(4),
        ),
    );
    map.insert(
        0xBE,
        InstructionInfo::new(
            Instruction::LDX(AddressModeValue::AbsoluteY(0)),
            CycleInfo::new(4),
        ),
    );

    // LDY 명령어
    map.insert(
        0xA0,
        InstructionInfo::new(
            Instruction::LDY(AddressModeValue::Immediate(0)),
            CycleInfo::new(2),
        ),
    );
    map.insert(
        0xA4,
        InstructionInfo::new(
            Instruction::LDY(AddressModeValue::ZeroPage(0)),
            CycleInfo::new(3),
        ),
    );
    map.insert(
        0xB4,
        InstructionInfo::new(
            Instruction::LDY(AddressModeValue::ZeroPageX(0)),
            CycleInfo::new(4),
        ),
    );
    map.insert(
        0xAC,
        InstructionInfo::new(
            Instruction::LDY(AddressModeValue::Absolute(0)),
            CycleInfo::new(4),
        ),
    );
    map.insert(
        0xBC,
        InstructionInfo::new(
            Instruction::LDY(AddressModeValue::AbsoluteX(0)),
            CycleInfo::new(4),
        ),
    );

    // STA 명령어
    map.insert(
        0x85,
        InstructionInfo::new(
            Instruction::STA(AddressModeValue::ZeroPage(0)),
            CycleInfo::new(3),
        ),
    );
    map.insert(
        0x95,
        InstructionInfo::new(
            Instruction::STA(AddressModeValue::ZeroPageX(0)),
            CycleInfo::new(4),
        ),
    );
    map.insert(
        0x8D,
        InstructionInfo::new(
            Instruction::STA(AddressModeValue::Absolute(0)),
            CycleInfo::new(4),
        ),
    );
    map.insert(
        0x9D,
        InstructionInfo::new(
            Instruction::STA(AddressModeValue::AbsoluteX(0)),
            CycleInfo::new(5),
        ),
    );
    map.insert(
        0x99,
        InstructionInfo::new(
            Instruction::STA(AddressModeValue::AbsoluteY(0)),
            CycleInfo::new(5),
        ),
    );
    map.insert(
        0x81,
        InstructionInfo::new(
            Instruction::STA(AddressModeValue::IndirectX(0)),
            CycleInfo::new(6),
        ),
    );
    map.insert(
        0x91,
        InstructionInfo::new(
            Instruction::STA(AddressModeValue::IndirectY(0)),
            CycleInfo::new(6),
        ),
    );

    // STY 명령어
    map.insert(
        0x84,
        InstructionInfo::new(
            Instruction::STY(AddressModeValue::ZeroPage(0)),
            CycleInfo::new(3),
        ),
    );
    map.insert(
        0x94,
        InstructionInfo::new(
            Instruction::STY(AddressModeValue::ZeroPageX(0)),
            CycleInfo::new(4),
        ),
    );
    map.insert(
        0x8C,
        InstructionInfo::new(
            Instruction::STY(AddressModeValue::Absolute(0)),
            CycleInfo::new(4),
        ),
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
        InstructionInfo::new(
            Instruction::INC(AddressModeValue::ZeroPage(0)),
            CycleInfo::new(5),
        ),
    );
    map.insert(
        0xF6,
        InstructionInfo::new(
            Instruction::INC(AddressModeValue::ZeroPageX(0)),
            CycleInfo::new(6),
        ),
    );
    map.insert(
        0xEE,
        InstructionInfo::new(
            Instruction::INC(AddressModeValue::Absolute(0)),
            CycleInfo::new(6),
        ),
    );
    map.insert(
        0xFE,
        InstructionInfo::new(
            Instruction::INC(AddressModeValue::AbsoluteX(0)),
            CycleInfo::new(7),
        ),
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
        InstructionInfo::new(
            Instruction::DEC(AddressModeValue::ZeroPage(0)),
            CycleInfo::new(5),
        ),
    );
    map.insert(
        0xD6,
        InstructionInfo::new(
            Instruction::DEC(AddressModeValue::ZeroPageX(0)),
            CycleInfo::new(6),
        ),
    );
    map.insert(
        0xCE,
        InstructionInfo::new(
            Instruction::DEC(AddressModeValue::Absolute(0)),
            CycleInfo::new(6),
        ),
    );
    map.insert(
        0xDE,
        InstructionInfo::new(
            Instruction::DEC(AddressModeValue::AbsoluteX(0)),
            CycleInfo::new(7),
        ),
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
        InstructionInfo::new(Instruction::BCC(0), CycleInfo::new(2)),
    );
    map.insert(
        0xB0,
        InstructionInfo::new(Instruction::BCS(0), CycleInfo::new(2)),
    );
    map.insert(
        0xF0,
        InstructionInfo::new(Instruction::BEQ(0), CycleInfo::new(2)),
    );
    map.insert(
        0xD0,
        InstructionInfo::new(Instruction::BNE(0), CycleInfo::new(2)),
    );
    map.insert(
        0x30,
        InstructionInfo::new(Instruction::BMI(0), CycleInfo::new(2)),
    );
    map.insert(
        0x10,
        InstructionInfo::new(Instruction::BPL(0), CycleInfo::new(2)),
    );
    map.insert(
        0x50,
        InstructionInfo::new(Instruction::BVC(0), CycleInfo::new(2)),
    );
    map.insert(
        0x70,
        InstructionInfo::new(Instruction::BVS(0), CycleInfo::new(2)),
    );

    // 점프/서브루틴
    map.insert(
        0x4C,
        InstructionInfo::new(
            Instruction::JMP(AddressModeValue::Absolute(0)),
            CycleInfo::new(3),
        ),
    );
    map.insert(
        0x6C,
        InstructionInfo::new(
            Instruction::JMP(AddressModeValue::Indirect(0)),
            CycleInfo::new(5),
        ),
    );
    map.insert(
        0x20,
        InstructionInfo::new(
            Instruction::JSR(AddressModeValue::Absolute(0)),
            CycleInfo::new(6),
        ),
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
        InstructionInfo::new(
            Instruction::BIT(AddressModeValue::ZeroPage(0)),
            CycleInfo::new(3),
        ),
    );
    map.insert(
        0x2C,
        InstructionInfo::new(
            Instruction::BIT(AddressModeValue::Absolute(0)),
            CycleInfo::new(4),
        ),
    );

    // CMP - Compare Memory with Accumulator
    map.insert(
        0xC9,
        InstructionInfo::new(
            Instruction::CMP(AddressModeValue::Immediate(0)),
            CycleInfo {
                base_cycles: 2,
                page_cross: false,
                branch_taken: false,
            },
        ),
    );
    map.insert(
        0xC5,
        InstructionInfo::new(
            Instruction::CMP(AddressModeValue::ZeroPage(0)),
            CycleInfo {
                base_cycles: 3,
                page_cross: false,
                branch_taken: false,
            },
        ),
    );
    map.insert(
        0xD5,
        InstructionInfo::new(
            Instruction::CMP(AddressModeValue::ZeroPageX(0)),
            CycleInfo {
                base_cycles: 4,
                page_cross: false,
                branch_taken: false,
            },
        ),
    );
    map.insert(
        0xCD,
        InstructionInfo::new(
            Instruction::CMP(AddressModeValue::Absolute(0)),
            CycleInfo {
                base_cycles: 4,
                page_cross: false,
                branch_taken: false,
            },
        ),
    );
    map.insert(
        0xDD,
        InstructionInfo::new(
            Instruction::CMP(AddressModeValue::AbsoluteX(0)),
            CycleInfo {
                base_cycles: 4,
                page_cross: true,
                branch_taken: false,
            }, // * 페이지 경계를 넘을 때 +1 cycle
        ),
    );
    map.insert(
        0xD9,
        InstructionInfo::new(
            Instruction::CMP(AddressModeValue::AbsoluteY(0)),
            CycleInfo {
                base_cycles: 4,
                page_cross: true,
                branch_taken: false,
            }, // * 페이지 경계를 넘을 때 +1 cycle
        ),
    );
    map.insert(
        0xC1,
        InstructionInfo::new(
            Instruction::CMP(AddressModeValue::IndirectX(0)),
            CycleInfo {
                base_cycles: 6,
                page_cross: false,
                branch_taken: false,
            },
        ),
    );
    map.insert(
        0xD1,
        InstructionInfo::new(
            Instruction::CMP(AddressModeValue::IndirectY(0)),
            CycleInfo {
                base_cycles: 5,
                page_cross: true,
                branch_taken: false,
            }, // * 페이지 경계를 넘을 때 +1 cycle
        ),
    );

    // CPX - Compare Memory with X Register
    map.insert(
        0xE0,
        InstructionInfo::new(
            Instruction::CPX(AddressModeValue::Immediate(0)),
            CycleInfo {
                base_cycles: 2,
                page_cross: false,
                branch_taken: false,
            },
        ),
    );
    map.insert(
        0xE4,
        InstructionInfo::new(
            Instruction::CPX(AddressModeValue::ZeroPage(0)),
            CycleInfo {
                base_cycles: 3,
                page_cross: false,
                branch_taken: false,
            },
        ),
    );
    map.insert(
        0xEC,
        InstructionInfo::new(
            Instruction::CPX(AddressModeValue::Absolute(0)),
            CycleInfo {
                base_cycles: 4,
                page_cross: false,
                branch_taken: false,
            },
        ),
    );

    // CPY - Compare Memory with Y Register
    map.insert(
        0xC0,
        InstructionInfo::new(
            Instruction::CPY(AddressModeValue::Immediate(0)),
            CycleInfo {
                base_cycles: 2,
                page_cross: false,
                branch_taken: false,
            },
        ),
    );
    map.insert(
        0xC4,
        InstructionInfo::new(
            Instruction::CPY(AddressModeValue::ZeroPage(0)),
            CycleInfo {
                base_cycles: 3,
                page_cross: false,
                branch_taken: false,
            },
        ),
    );
    map.insert(
        0xCC,
        InstructionInfo::new(
            Instruction::CPY(AddressModeValue::Absolute(0)),
            CycleInfo {
                base_cycles: 4,
                page_cross: false,
                branch_taken: false,
            },
        ),
    );

    // ADC - Add Memory to Accumulator with Carry
    map.insert(
        0x69,
        InstructionInfo::new(
            Instruction::ADC(AddressModeValue::Immediate(0)),
            CycleInfo {
                base_cycles: 2,
                page_cross: false,
                branch_taken: false,
            },
        ),
    );
    map.insert(
        0x65,
        InstructionInfo::new(
            Instruction::ADC(AddressModeValue::ZeroPage(0)),
            CycleInfo {
                base_cycles: 3,
                page_cross: false,
                branch_taken: false,
            },
        ),
    );
    map.insert(
        0x75,
        InstructionInfo::new(
            Instruction::ADC(AddressModeValue::ZeroPageX(0)),
            CycleInfo {
                base_cycles: 4,
                page_cross: false,
                branch_taken: false,
            },
        ),
    );
    map.insert(
        0x6D,
        InstructionInfo::new(
            Instruction::ADC(AddressModeValue::Absolute(0)),
            CycleInfo {
                base_cycles: 4,
                page_cross: false,
                branch_taken: false,
            },
        ),
    );
    map.insert(
        0x7D,
        InstructionInfo::new(
            Instruction::ADC(AddressModeValue::AbsoluteX(0)),
            CycleInfo {
                base_cycles: 4,
                page_cross: true,
                branch_taken: false,
            },
        ),
    );
    map.insert(
        0x79,
        InstructionInfo::new(
            Instruction::ADC(AddressModeValue::AbsoluteY(0)),
            CycleInfo {
                base_cycles: 4,
                page_cross: true,
                branch_taken: false,
            },
        ),
    );
    map.insert(
        0x61,
        InstructionInfo::new(
            Instruction::ADC(AddressModeValue::IndirectX(0)),
            CycleInfo {
                base_cycles: 6,
                page_cross: false,
                branch_taken: false,
            },
        ),
    );
    map.insert(
        0x71,
        InstructionInfo::new(
            Instruction::ADC(AddressModeValue::IndirectY(0)),
            CycleInfo {
                base_cycles: 5,
                page_cross: true,
                branch_taken: false,
            },
        ),
    );

    // LSR - Logical Shift Right
    map.insert(
        0x4A,
        InstructionInfo::new(
            Instruction::LSR(AddressModeValue::Accumulator),
            CycleInfo::new(2),
        ),
    );
    map.insert(
        0x46,
        InstructionInfo::new(
            Instruction::LSR(AddressModeValue::ZeroPage(0)),
            CycleInfo::new(5),
        ),
    );
    map.insert(
        0x56,
        InstructionInfo::new(
            Instruction::LSR(AddressModeValue::ZeroPageX(0)),
            CycleInfo::new(6),
        ),
    );
    map.insert(
        0x4E,
        InstructionInfo::new(
            Instruction::LSR(AddressModeValue::Absolute(0)),
            CycleInfo::new(6),
        ),
    );
    map.insert(
        0x5E,
        InstructionInfo::new(
            Instruction::LSR(AddressModeValue::AbsoluteX(0)),
            CycleInfo::new(7),
        ),
    );

    // ASL - Arithmetic Shift Left
    map.insert(
        0x0A,
        InstructionInfo::new(
            Instruction::ASL(AddressModeValue::Accumulator),
            CycleInfo::new(2),
        ),
    );
    map.insert(
        0x06,
        InstructionInfo::new(
            Instruction::ASL(AddressModeValue::ZeroPage(0)),
            CycleInfo::new(5),
        ),
    );
    map.insert(
        0x16,
        InstructionInfo::new(
            Instruction::ASL(AddressModeValue::ZeroPageX(0)),
            CycleInfo::new(6),
        ),
    );
    map.insert(
        0x0E,
        InstructionInfo::new(
            Instruction::ASL(AddressModeValue::Absolute(0)),
            CycleInfo::new(6),
        ),
    );
    map.insert(
        0x1E,
        InstructionInfo::new(
            Instruction::ASL(AddressModeValue::AbsoluteX(0)),
            CycleInfo::new(7),
        ),
    );

    // ROL - Rotate Left
    map.insert(
        0x2A,
        InstructionInfo::new(
            Instruction::ROL(AddressModeValue::Accumulator),
            CycleInfo::new(2),
        ),
    );
    map.insert(
        0x26,
        InstructionInfo::new(
            Instruction::ROL(AddressModeValue::ZeroPage(0)),
            CycleInfo::new(5),
        ),
    );
    map.insert(
        0x36,
        InstructionInfo::new(
            Instruction::ROL(AddressModeValue::ZeroPageX(0)),
            CycleInfo::new(6),
        ),
    );
    map.insert(
        0x2E,
        InstructionInfo::new(
            Instruction::ROL(AddressModeValue::Absolute(0)),
            CycleInfo::new(6),
        ),
    );
    map.insert(
        0x3E,
        InstructionInfo::new(
            Instruction::ROL(AddressModeValue::AbsoluteX(0)),
            CycleInfo::new(7),
        ),
    );

    // ROR - Rotate Right
    map.insert(
        0x6A,
        InstructionInfo::new(
            Instruction::ROR(AddressModeValue::Accumulator),
            CycleInfo::new(2),
        ),
    );
    map.insert(
        0x66,
        InstructionInfo::new(
            Instruction::ROR(AddressModeValue::ZeroPage(0)),
            CycleInfo::new(5),
        ),
    );
    map.insert(
        0x76,
        InstructionInfo::new(
            Instruction::ROR(AddressModeValue::ZeroPageX(0)),
            CycleInfo::new(6),
        ),
    );
    map.insert(
        0x6E,
        InstructionInfo::new(
            Instruction::ROR(AddressModeValue::Absolute(0)),
            CycleInfo::new(6),
        ),
    );
    map.insert(
        0x7E,
        InstructionInfo::new(
            Instruction::ROR(AddressModeValue::AbsoluteX(0)),
            CycleInfo::new(7),
        ),
    );

    // AND 명령어
    map.insert(
        0x29,
        InstructionInfo::new(
            Instruction::AND(AddressModeValue::Immediate(0)),
            CycleInfo::new(2),
        ),
    );

    // ORA 명령어
    map.insert(
        0x09,
        InstructionInfo::new(
            Instruction::ORA(AddressModeValue::Immediate(0)),
            CycleInfo::new(2),
        ),
    );

    // EOR 명령어
    map.insert(
        0x49,
        InstructionInfo::new(
            Instruction::EOR(AddressModeValue::Immediate(0)),
            CycleInfo::new(2),
        ),
    );

    map.insert(
        0x86,
        InstructionInfo::new(
            Instruction::STX(AddressModeValue::ZeroPage(0)),
            CycleInfo::new(3),
        ),
    );
    map.insert(
        0x96,
        InstructionInfo::new(
            Instruction::STX(AddressModeValue::ZeroPageY(0)),
            CycleInfo::new(4),
        ),
    );
    map.insert(
        0x8E,
        InstructionInfo::new(
            Instruction::STX(AddressModeValue::Absolute(0)),
            CycleInfo::new(4),
        ),
    );

    map
});

/// 명령어와 주소 모드로부터 옵코드 정보 찾기
pub fn get_opcode_info(instruction: Instruction) -> Option<InstructionInfo> {
    match instruction {
        Instruction::LDA(mode) => match mode {
            AddressModeValue::Immediate(_) => OPCODE_MAP.get(&0xA9).copied(),
            AddressModeValue::ZeroPage(_) => OPCODE_MAP.get(&0xA5).copied(),
            AddressModeValue::ZeroPageX(_) => OPCODE_MAP.get(&0xB5).copied(),
            AddressModeValue::Absolute(_) => OPCODE_MAP.get(&0xAD).copied(),
            AddressModeValue::AbsoluteX(_) => OPCODE_MAP.get(&0xBD).copied(),
            AddressModeValue::AbsoluteY(_) => OPCODE_MAP.get(&0xB9).copied(),
            AddressModeValue::IndirectX(_) => OPCODE_MAP.get(&0xA1).copied(),
            AddressModeValue::IndirectY(_) => OPCODE_MAP.get(&0xB1).copied(),
            _ => None,
        },
        Instruction::LDX(mode) => match mode {
            AddressModeValue::Immediate(_) => OPCODE_MAP.get(&0xA2).copied(),
            AddressModeValue::ZeroPage(_) => OPCODE_MAP.get(&0xA6).copied(),
            AddressModeValue::ZeroPageY(_) => OPCODE_MAP.get(&0xB6).copied(),
            AddressModeValue::Absolute(_) => OPCODE_MAP.get(&0xAE).copied(),
            AddressModeValue::AbsoluteY(_) => OPCODE_MAP.get(&0xBE).copied(),
            _ => None,
        },
        Instruction::LDY(mode) => match mode {
            AddressModeValue::Immediate(_) => OPCODE_MAP.get(&0xA0).copied(),
            AddressModeValue::ZeroPage(_) => OPCODE_MAP.get(&0xA4).copied(),
            AddressModeValue::ZeroPageX(_) => OPCODE_MAP.get(&0xB4).copied(),
            AddressModeValue::Absolute(_) => OPCODE_MAP.get(&0xAC).copied(),
            AddressModeValue::AbsoluteX(_) => OPCODE_MAP.get(&0xBC).copied(),
            _ => None,
        },
        Instruction::STA(mode) => match mode {
            AddressModeValue::ZeroPage(_) => OPCODE_MAP.get(&0x85).copied(),
            AddressModeValue::ZeroPageX(_) => OPCODE_MAP.get(&0x95).copied(),
            AddressModeValue::Absolute(_) => OPCODE_MAP.get(&0x8D).copied(),
            AddressModeValue::AbsoluteX(_) => OPCODE_MAP.get(&0x9D).copied(),
            AddressModeValue::AbsoluteY(_) => OPCODE_MAP.get(&0x99).copied(),
            AddressModeValue::IndirectX(_) => OPCODE_MAP.get(&0x81).copied(),
            AddressModeValue::IndirectY(_) => OPCODE_MAP.get(&0x91).copied(),
            _ => None,
        },
        Instruction::STY(mode) => match mode {
            AddressModeValue::ZeroPage(_) => OPCODE_MAP.get(&0x84).copied(),
            AddressModeValue::ZeroPageX(_) => OPCODE_MAP.get(&0x94).copied(),
            AddressModeValue::Absolute(_) => OPCODE_MAP.get(&0x8C).copied(),
            _ => None,
        },
        Instruction::INC(mode) => match mode {
            AddressModeValue::ZeroPage(_) => OPCODE_MAP.get(&0xE6).copied(),
            AddressModeValue::ZeroPageX(_) => OPCODE_MAP.get(&0xF6).copied(),
            AddressModeValue::Absolute(_) => OPCODE_MAP.get(&0xEE).copied(),
            AddressModeValue::AbsoluteX(_) => OPCODE_MAP.get(&0xFE).copied(),
            _ => None,
        },
        Instruction::DEC(mode) => match mode {
            AddressModeValue::ZeroPage(_) => OPCODE_MAP.get(&0xC6).copied(),
            AddressModeValue::ZeroPageX(_) => OPCODE_MAP.get(&0xD6).copied(),
            AddressModeValue::Absolute(_) => OPCODE_MAP.get(&0xCE).copied(),
            AddressModeValue::AbsoluteX(_) => OPCODE_MAP.get(&0xDE).copied(),
            _ => None,
        },
        Instruction::JMP(mode) => match mode {
            AddressModeValue::Absolute(_) => OPCODE_MAP.get(&0x4C).copied(),
            AddressModeValue::Indirect(_) => OPCODE_MAP.get(&0x6C).copied(),
            _ => None,
        },
        Instruction::JSR(_mode) => OPCODE_MAP.get(&0x20).copied(),
        Instruction::BIT(mode) => match mode {
            AddressModeValue::ZeroPage(_) => OPCODE_MAP.get(&0x24).copied(),
            AddressModeValue::Absolute(_) => OPCODE_MAP.get(&0x2C).copied(),
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
        Instruction::BCC(_offset) => OPCODE_MAP.get(&0x90).copied(),
        Instruction::BCS(_offset) => OPCODE_MAP.get(&0xB0).copied(),
        Instruction::BEQ(_offset) => OPCODE_MAP.get(&0xF0).copied(),
        Instruction::BNE(_offset) => OPCODE_MAP.get(&0xD0).copied(),
        Instruction::BMI(_offset) => OPCODE_MAP.get(&0x30).copied(),
        Instruction::BPL(_offset) => OPCODE_MAP.get(&0x10).copied(),
        Instruction::BVC(_offset) => OPCODE_MAP.get(&0x50).copied(),
        Instruction::BVS(_offset) => OPCODE_MAP.get(&0x70).copied(),
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
        Instruction::CMP(mode) => match mode {
            AddressModeValue::Immediate(_) => OPCODE_MAP.get(&0xC9).copied(),
            AddressModeValue::ZeroPage(_) => OPCODE_MAP.get(&0xC5).copied(),
            AddressModeValue::ZeroPageX(_) => OPCODE_MAP.get(&0xD5).copied(),
            AddressModeValue::Absolute(_) => OPCODE_MAP.get(&0xCD).copied(),
            AddressModeValue::AbsoluteX(_) => OPCODE_MAP.get(&0xDD).copied(),
            AddressModeValue::AbsoluteY(_) => OPCODE_MAP.get(&0xD9).copied(),
            AddressModeValue::IndirectX(_) => OPCODE_MAP.get(&0xC1).copied(),
            AddressModeValue::IndirectY(_) => OPCODE_MAP.get(&0xD1).copied(),
            _ => None,
        },
        Instruction::CPX(mode) => match mode {
            AddressModeValue::Immediate(_) => OPCODE_MAP.get(&0xE0).copied(),
            AddressModeValue::ZeroPage(_) => OPCODE_MAP.get(&0xE4).copied(),
            AddressModeValue::Absolute(_) => OPCODE_MAP.get(&0xEC).copied(),
            _ => None,
        },
        Instruction::CPY(mode) => match mode {
            AddressModeValue::Immediate(_) => OPCODE_MAP.get(&0xC0).copied(),
            AddressModeValue::ZeroPage(_) => OPCODE_MAP.get(&0xC4).copied(),
            AddressModeValue::Absolute(_) => OPCODE_MAP.get(&0xCC).copied(),
            _ => None,
        },
        Instruction::ADC(mode) => match mode {
            AddressModeValue::Immediate(_) => OPCODE_MAP.get(&0x69).copied(),
            AddressModeValue::ZeroPage(_) => OPCODE_MAP.get(&0x65).copied(),
            AddressModeValue::ZeroPageX(_) => OPCODE_MAP.get(&0x75).copied(),
            AddressModeValue::Absolute(_) => OPCODE_MAP.get(&0x6D).copied(),
            AddressModeValue::AbsoluteX(_) => OPCODE_MAP.get(&0x7D).copied(),
            AddressModeValue::AbsoluteY(_) => OPCODE_MAP.get(&0x79).copied(),
            AddressModeValue::IndirectX(_) => OPCODE_MAP.get(&0x61).copied(),
            AddressModeValue::IndirectY(_) => OPCODE_MAP.get(&0x71).copied(),
            _ => None,
        },
        Instruction::LSR(mode) => match mode {
            AddressModeValue::Accumulator => OPCODE_MAP.get(&0x4A).copied(),
            AddressModeValue::ZeroPage(_) => OPCODE_MAP.get(&0x46).copied(),
            AddressModeValue::ZeroPageX(_) => OPCODE_MAP.get(&0x56).copied(),
            AddressModeValue::Absolute(_) => OPCODE_MAP.get(&0x4E).copied(),
            AddressModeValue::AbsoluteX(_) => OPCODE_MAP.get(&0x5E).copied(),
            _ => None,
        },
        Instruction::ASL(mode) => match mode {
            AddressModeValue::Accumulator => OPCODE_MAP.get(&0x0A).copied(),
            AddressModeValue::ZeroPage(_) => OPCODE_MAP.get(&0x06).copied(),
            AddressModeValue::ZeroPageX(_) => OPCODE_MAP.get(&0x16).copied(),
            AddressModeValue::Absolute(_) => OPCODE_MAP.get(&0x0E).copied(),
            AddressModeValue::AbsoluteX(_) => OPCODE_MAP.get(&0x1E).copied(),
            _ => None,
        },
        Instruction::ROL(mode) => match mode {
            AddressModeValue::Accumulator => OPCODE_MAP.get(&0x2A).copied(),
            AddressModeValue::ZeroPage(_) => OPCODE_MAP.get(&0x26).copied(),
            AddressModeValue::ZeroPageX(_) => OPCODE_MAP.get(&0x36).copied(),
            AddressModeValue::Absolute(_) => OPCODE_MAP.get(&0x2E).copied(),
            AddressModeValue::AbsoluteX(_) => OPCODE_MAP.get(&0x3E).copied(),
            _ => None,
        },
        Instruction::ROR(mode) => match mode {
            AddressModeValue::Accumulator => OPCODE_MAP.get(&0x6A).copied(),
            AddressModeValue::ZeroPage(_) => OPCODE_MAP.get(&0x66).copied(),
            AddressModeValue::ZeroPageX(_) => OPCODE_MAP.get(&0x76).copied(),
            AddressModeValue::Absolute(_) => OPCODE_MAP.get(&0x6E).copied(),
            AddressModeValue::AbsoluteX(_) => OPCODE_MAP.get(&0x7E).copied(),
            _ => None,
        },
        _ => None,
    }
}
