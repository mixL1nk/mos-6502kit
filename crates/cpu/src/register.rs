// registers.rs
pub trait RegisterValue {
    type Value;
    fn get(&self) -> Self::Value;
    fn set(&mut self, value: Self::Value);
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum GeneralRegister {
    A(u8),
    X(u8),
    Y(u8),
}

impl GeneralRegister {
    pub fn new(reg_type: GeneralRegisterType, value: u8) -> Self {
        match reg_type {
            GeneralRegisterType::A => Self::A(value),
            GeneralRegisterType::X => Self::X(value),
            GeneralRegisterType::Y => Self::Y(value),
        }
    }
}

#[derive(Debug, Clone, Copy)]
pub enum GeneralRegisterType {
    A,
    X,
    Y,
}

impl RegisterValue for GeneralRegister {
    type Value = u8;
    fn get(&self) -> u8 {
        match self {
            Self::A(value) | Self::X(value) | Self::Y(value) => *value,
        }
    }

    fn set(&mut self, value: u8) {
        match self {
            Self::A(v) | Self::X(v) | Self::Y(v) => *v = value,
        }
    }
}

use bitflags::bitflags;

bitflags! {
    #[derive(Debug, Clone, Copy, PartialEq, Eq)]
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

impl Default for StatusRegister {
    fn default() -> Self {
        StatusRegister::UNUSED  // UNUSED 플래그만 설정된 상태로 초기화
    }
}


#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum SpecialRegister8 {
    P(StatusRegister),
    S(u8),
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum SpecialRegister16 {
    PC(u16),
}

#[derive(Debug, Clone, Copy)]
pub enum SpecialRegister8Type { // 8비트 특수 레지스터 타입
    P,
    S,
}

#[derive(Debug, Clone, Copy)]
pub enum SpecialRegister16Type { // 16비트 특수 레지스터 타입
    PC,
}

// 8비트 특수 레지스터용 생성자 (Default trait 사용)
impl SpecialRegister8 {
    pub fn new(reg_type: SpecialRegister8Type, value: u8) -> Self {
        match reg_type {
            SpecialRegister8Type::P => Self::P(StatusRegister::default()), // P는 기본값으로
            SpecialRegister8Type::S => Self::S(value), // S는 주어진 값으로
        }
    }
}

// 16비트 특수 레지스터용 생성자
impl SpecialRegister16 {
    pub fn new(reg_type: SpecialRegister16Type, value: u16) -> Self {
        match reg_type {
            SpecialRegister16Type::PC => Self::PC(value),
        }
    }
}

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
            Self::P(status) => *status = StatusRegister::from_bits_truncate(value | 0b00100000),
            Self::S(v) => *v = value,
        }
    }
}

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
    let mut reg_p = SpecialRegister8::new(SpecialRegister8Type::P, 0x34); // 초기값은 무시됨.
    assert_eq!(reg_p.get(), 0b00100000); //UNUSED flag

    reg_p.set(0x78);
    assert_eq!(reg_p.get(), 0x78);


    // 8비트 레지스터(S) 테스트
    let mut reg_s = SpecialRegister8::new(SpecialRegister8Type::S, 0);
    assert_eq!(reg_s.get(), 0);
    reg_s.set(0xFF);
    assert_eq!(reg_s.get(), 0xFF);
}