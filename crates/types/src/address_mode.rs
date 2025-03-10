/// CPU 명령어의 주소 지정 모드
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum AddressMode {
    Implied,
    Accumulator,
    Immediate,
    ZeroPage,
    ZeroPageX,
    ZeroPageY,
    Absolute,
    AbsoluteX,
    AbsoluteY,
    Indirect,
    IndirectX,
    IndirectY,
    Relative,
}

impl AddressMode {
    /// 주소 모드의 오퍼랜드 크기를 바이트 단위로 반환
    pub fn operand_size(&self) -> u8 {
        match self {
            Self::Implied | Self::Accumulator => 0,
            Self::Immediate
            | Self::ZeroPage
            | Self::ZeroPageX
            | Self::ZeroPageY
            | Self::IndirectX
            | Self::IndirectY
            | Self::Relative => 1,
            Self::Absolute | Self::AbsoluteX | Self::AbsoluteY | Self::Indirect => 2,
        }
    }
}
