mod address_mode;
mod instruction;
pub mod opcode;

pub use address_mode::AddressMode;
pub use instruction::{CycleInfo, Instruction, InstructionInfo};
pub use opcode::{OPCODE_MAP, get_opcode_info};

/// 값을 포함한 어드레싱 모드
#[derive(Debug, PartialEq, Clone, Copy)]
pub enum AddressModeValue {
    Accumulator,
    Immediate(u8),
    ZeroPage(u8),
    ZeroPageX(u8),
    ZeroPageY(u8),
    Absolute(u16),
    AbsoluteX(u16),
    AbsoluteY(u16),
    Indirect(u16),
    IndirectX(u8),
    IndirectY(u8),
    Implied,
}

impl AddressModeValue {
    /// 어드레싱 모드 타입만 반환합니다 (값 제외)
    pub fn get_mode(&self) -> AddressMode {
        match self {
            Self::Accumulator => AddressMode::Accumulator,
            Self::Immediate(_) => AddressMode::Immediate,
            Self::ZeroPage(_) => AddressMode::ZeroPage,
            Self::ZeroPageX(_) => AddressMode::ZeroPageX,
            Self::ZeroPageY(_) => AddressMode::ZeroPageY,
            Self::Absolute(_) => AddressMode::Absolute,
            Self::AbsoluteX(_) => AddressMode::AbsoluteX,
            Self::AbsoluteY(_) => AddressMode::AbsoluteY,
            Self::Indirect(_) => AddressMode::Indirect,
            Self::IndirectX(_) => AddressMode::IndirectX,
            Self::IndirectY(_) => AddressMode::IndirectY,
            Self::Implied => AddressMode::Implied,
        }
    }

    /// 주소 모드의 오퍼랜드 크기를 바이트 단위로 반환
    pub fn operand_size(&self) -> u8 {
        match self {
            Self::Implied | Self::Accumulator => 0,
            Self::Immediate(_)
            | Self::ZeroPage(_)
            | Self::ZeroPageX(_)
            | Self::ZeroPageY(_)
            | Self::IndirectX(_)
            | Self::IndirectY(_) => 1,
            Self::Absolute(_) | Self::AbsoluteX(_) | Self::AbsoluteY(_) | Self::Indirect(_) => 2,
        }
    }

    /// 값을 가져옵니다
    pub fn get_value_u8(&self) -> u8 {
        match self {
            Self::Immediate(val)
            | Self::ZeroPage(val)
            | Self::ZeroPageX(val)
            | Self::ZeroPageY(val)
            | Self::IndirectX(val)
            | Self::IndirectY(val) => *val,
            Self::Absolute(val)
            | Self::AbsoluteX(val)
            | Self::AbsoluteY(val)
            | Self::Indirect(val) => (*val & 0xFF) as u8,
            _ => 0,
        }
    }

    /// 2바이트 값을 가져옵니다
    pub fn get_value_u16(&self) -> u16 {
        match self {
            Self::Absolute(val)
            | Self::AbsoluteX(val)
            | Self::AbsoluteY(val)
            | Self::Indirect(val) => *val,
            Self::Immediate(val)
            | Self::ZeroPage(val)
            | Self::ZeroPageX(val)
            | Self::ZeroPageY(val)
            | Self::IndirectX(val)
            | Self::IndirectY(val) => *val as u16,
            _ => 0,
        }
    }
}
