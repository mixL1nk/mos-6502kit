use bitflags::bitflags;

/// 레지스터 타입 (A, X, Y, P, S, PC)
#[derive(Debug, Clone, Copy)]
pub enum RegisterType {
    A,
    X,
    Y,
    P,
    S,
    PC,
}

/// 레지스터 데이터 (8비트 또는 16비트)
#[derive(Debug, PartialEq, Eq, Clone, Copy)]
pub enum RegisterData {
    Bit8(u8),
    Bit16(u16),
}

/// 공통 인터페이스: 레지스터 값 읽기/쓰기
pub trait RegisterValue {
    type Value;
    fn get(&self) -> Self::Value;
    fn set(&mut self, value: Self::Value);
}

/// MOS 6502 일반 레지스터 종류
#[derive(Debug, Clone, Copy)]
pub enum GeneralRegisterType {
    A,
    X,
    Y,
}

/// MOS 6502 일반 레지스터 (A, X, Y)
/// 내부에 (u8) 값 보관
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum GeneralRegister {
    A(u8),
    X(u8),
    Y(u8),
}

impl GeneralRegister {
    /// 새 일반 레지스터를 생성
    pub fn new(reg_type: GeneralRegisterType, value: u8) -> Self {
        match reg_type {
            GeneralRegisterType::A => Self::A(value),
            GeneralRegisterType::X => Self::X(value),
            GeneralRegisterType::Y => Self::Y(value),
        }
    }
}

/// `RegisterValue` 트레이트 구현
impl RegisterValue for GeneralRegister {
    type Value = u8;

    fn get(&self) -> u8 {
        match self {
            Self::A(value) | Self::X(value) | Self::Y(value) => *value,
        }
    }

    fn set(&mut self, value: u8) {
        match self {
            Self::A(v) => *v = value,
            Self::X(v) => *v = value,
            Self::Y(v) => *v = value,
        }
    }
}

bitflags! {
    /// MOS 6502 스테이터스 레지스터 플래그
    #[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
    pub struct StatusRegister: u8 {
        const CARRY             = 0b00000001;
        const ZERO              = 0b00000010;
        const INTERRUPT_DISABLE = 0b00000100;
        const DECIMAL           = 0b00001000;
        const BREAK             = 0b00010000;
        const UNUSED            = 0b00100000;
        const OVERFLOW          = 0b01000000;
        const NEGATIVE          = 0b10000000;
    }
}

/// StatusRegister에서 항상 유지해야 할 UNUSED 플래그
const UNUSED_FLAG: u8 = StatusRegister::UNUSED.bits();

impl Default for StatusRegister {
    fn default() -> Self {
        // 초기값으로 UNUSED 플래그만 설정
        StatusRegister::UNUSED
    }
}

/// MOS 6502 특수 레지스터 (P: StatusRegister, S: Stack Pointer)
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum SpecialRegister8 {
    P(StatusRegister),
    S(u8),
}

/// MOS 6502 특수 레지스터 (PC: Program Counter)
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum SpecialRegister16 {
    PC(u16),
}

/// 8비트 특수 레지스터 종류
#[derive(Debug, Clone, Copy)]
pub enum SpecialRegister8Type {
    P,
    S,
}

/// 16비트 특수 레지스터 종류
#[derive(Debug, Clone, Copy)]
pub enum SpecialRegister16Type {
    PC,
}

impl SpecialRegister8 {
    /// 8비트 특수 레지스터 생성
    /// P(StatusRegister)는 기본값을 사용하고,
    /// S(Stack Pointer)는 주어진 값으로 초기화
    pub fn new(reg_type: SpecialRegister8Type, value: u8) -> Self {
        match reg_type {
            SpecialRegister8Type::P => Self::P(StatusRegister::default()),
            SpecialRegister8Type::S => Self::S(value),
        }
    }
}

impl SpecialRegister16 {
    /// 16비트 특수 레지스터 생성
    pub fn new(reg_type: SpecialRegister16Type, value: u16) -> Self {
        match reg_type {
            SpecialRegister16Type::PC => Self::PC(value),
        }
    }
}

/// `RegisterValue` 트레이트 구현 (8비트 특수 레지스터)
impl RegisterValue for SpecialRegister8 {
    type Value = u8;

    fn get(&self) -> u8 {
        match self {
            Self::P(status) => status.bits(),
            Self::S(value) => *value,
        }
    }

    fn set(&mut self, value: u8) {
        match self {
            Self::P(status) => {
                // 항상 UNUSED_FLAG를 유지하기 위해 OR 연산
                *status = StatusRegister::from_bits_truncate(value | UNUSED_FLAG);
            }
            Self::S(v) => *v = value,
        }
    }
}

/// `RegisterValue` 트레이트 구현 (16비트 특수 레지스터)
impl RegisterValue for SpecialRegister16 {
    type Value = u16;

    fn get(&self) -> u16 {
        match self {
            Self::PC(value) => *value,
        }
    }

    fn set(&mut self, value: u16) {
        match self {
            Self::PC(v) => *v = value,
        }
    }
}

/// CPU 레지스터 집합
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct Registers {
    pub a: GeneralRegister,
    pub x: GeneralRegister,
    pub y: GeneralRegister,
    pub p: SpecialRegister8,
    pub s: SpecialRegister8,
    pub pc: SpecialRegister16,
}

impl Default for Registers {
    fn default() -> Self {
        Self {
            a: GeneralRegister::new(GeneralRegisterType::A, 0),
            x: GeneralRegister::new(GeneralRegisterType::X, 0),
            y: GeneralRegister::new(GeneralRegisterType::Y, 0),
            p: SpecialRegister8::new(SpecialRegister8Type::P, 0),
            s: SpecialRegister8::new(SpecialRegister8Type::S, 0xFD), // 스택 포인터 초기값
            pc: SpecialRegister16::new(SpecialRegister16Type::PC, 0),
        }
    }
}

impl Registers {
    /// 레지스터 값 읽기
    pub fn get_value(&self, reg: RegisterType) -> RegisterData {
        match reg {
            RegisterType::A => RegisterData::Bit8(self.a.get()),
            RegisterType::X => RegisterData::Bit8(self.x.get()),
            RegisterType::Y => RegisterData::Bit8(self.y.get()),
            RegisterType::P => RegisterData::Bit8(self.p.get()),
            RegisterType::S => RegisterData::Bit8(self.s.get()),
            RegisterType::PC => RegisterData::Bit16(self.pc.get()),
        }
    }
    /// 레지스터 값 설정
    pub fn set_value(&mut self, reg: RegisterType, value: RegisterData) {
        match reg {
            RegisterType::A => self.a.set(value.as_u8()),
            RegisterType::X => self.x.set(value.as_u8()),
            RegisterType::Y => self.y.set(value.as_u8()),
            RegisterType::P => self.p.set(value.as_u8()),
            RegisterType::S => self.s.set(value.as_u8()),
            RegisterType::PC => self.pc.set(value.as_u16()),
        }
    }
}

impl RegisterData {
    /// 8비트 값으로 변환
    pub fn as_u8(&self) -> u8 {
        match self {
            Self::Bit8(value) => *value,
            Self::Bit16(value) => *value as u8,
        }
    }

    /// 16비트 값으로 변환
    pub fn as_u16(&self) -> u16 {
        match self {
            Self::Bit8(value) => *value as u16,
            Self::Bit16(value) => *value,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_general_register() {
        let mut reg = GeneralRegister::new(GeneralRegisterType::A, 0x10);
        assert_eq!(reg.get(), 0x10);
        reg.set(0x20);
        assert_eq!(reg.get(), 0x20);

        // 경계값 테스트
        let mut reg_x = GeneralRegister::new(GeneralRegisterType::X, 0);
        assert_eq!(reg_x.get(), 0);
        reg_x.set(255);
        assert_eq!(reg_x.get(), 255);
    }

    #[test]
    fn test_special_register() {
        // 16비트 레지스터(PC) 테스트
        let mut reg_pc = SpecialRegister16::new(SpecialRegister16Type::PC, 0x1000);
        assert_eq!(reg_pc.get(), 0x1000);
        reg_pc.set(0x2000);
        assert_eq!(reg_pc.get(), 0x2000);

        // 8비트 레지스터(P) 테스트
        let mut reg_p = SpecialRegister8::new(SpecialRegister8Type::P, 0x34);
        // P는 기본값(UNUSED_FLAG)으로만 초기화하므로 0x34 무시
        assert_eq!(reg_p.get(), 0b0010_0000);

        reg_p.set(0x78);
        assert_eq!(reg_p.get(), 0x78);

        // 8비트 레지스터(S) 테스트
        let mut reg_s = SpecialRegister8::new(SpecialRegister8Type::S, 0);
        assert_eq!(reg_s.get(), 0);
        reg_s.set(0xFF);
        assert_eq!(reg_s.get(), 0xFF);
    }

    #[test]
    fn test_registers() {
        let regs = Registers::default();
        assert_eq!(regs.a.get(), 0);
        assert_eq!(regs.x.get(), 0);
        assert_eq!(regs.y.get(), 0);
        assert_eq!(regs.p.get(), StatusRegister::UNUSED.bits());
        assert_eq!(regs.s.get(), 0xFD);
        assert_eq!(regs.pc.get(), 0);
    }
}
