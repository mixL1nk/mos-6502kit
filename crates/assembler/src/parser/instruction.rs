use common::Result;
use error::Error;
use types::{AddressModeValue, Instruction};

/// 명령어 파싱 전략 트레이트
pub trait InstructionStrategy: Send + Sync {
    fn parse(&self, parser: &InstructionParser, mode: AddressModeValue) -> Result<Instruction>;
}

/// 명령어 파서
pub struct InstructionParser {
    current_address: u16,
}

impl Default for InstructionParser {
    fn default() -> Self {
        Self::new()
    }
}

impl InstructionParser {
    pub fn new() -> Self {
        Self { current_address: 0 }
    }

    pub fn set_current_address(&mut self, address: u16) {
        self.current_address = address;
    }

    pub fn parse_implicit(&self, instruction: Instruction) -> Result<Instruction> {
        Ok(instruction)
    }

    pub fn parse_branch(
        &self,
        create_instruction: impl FnOnce(i8) -> Instruction,
        target_address: u16,
    ) -> Result<Instruction> {
        let next_pc = self.current_address.wrapping_add(2);
        let offset = target_address as i32 - next_pc as i32;

        println!(
            "[parse_branch] offset: 0x{:0x}, target_address: 0x{:0x}, 0xnext_pc: {:0x},
        current: 0x{:0x}",
            offset, target_address, next_pc, self.current_address
        );

        if !(-128..=127).contains(&offset) {
            return Err(Error::AssemblerBranchOutOfRange(
                "Branch target too far".to_string(),
            ));
        }

        Ok(create_instruction(offset as i8))
    }

    pub fn parse_addressing_mode(
        &self,
        mode: AddressModeValue,
        create_instruction: impl FnOnce(AddressModeValue) -> Result<Instruction>,
    ) -> Result<Instruction> {
        create_instruction(mode)
    }
}

// 로드 명령어들
pub struct LDAStrategy;
impl InstructionStrategy for LDAStrategy {
    fn parse(&self, _parser: &InstructionParser, mode: AddressModeValue) -> Result<Instruction> {
        Ok(Instruction::LDA(mode))
    }
}

pub struct LDXStrategy;
impl InstructionStrategy for LDXStrategy {
    fn parse(&self, _parser: &InstructionParser, mode: AddressModeValue) -> Result<Instruction> {
        Ok(Instruction::LDX(mode))
    }
}

pub struct LDYStrategy;
impl InstructionStrategy for LDYStrategy {
    fn parse(&self, _parser: &InstructionParser, mode: AddressModeValue) -> Result<Instruction> {
        Ok(Instruction::LDY(mode))
    }
}

// 스토어 명령어들
pub struct STAStrategy;
impl InstructionStrategy for STAStrategy {
    fn parse(&self, _parser: &InstructionParser, mode: AddressModeValue) -> Result<Instruction> {
        Ok(Instruction::STA(mode))
    }
}

pub struct STXStrategy;
impl InstructionStrategy for STXStrategy {
    fn parse(&self, _parser: &InstructionParser, mode: AddressModeValue) -> Result<Instruction> {
        Ok(Instruction::STX(mode))
    }
}

pub struct STYStrategy;
impl InstructionStrategy for STYStrategy {
    fn parse(&self, _parser: &InstructionParser, mode: AddressModeValue) -> Result<Instruction> {
        Ok(Instruction::STY(mode))
    }
}

// 단일 바이트 명령어들
pub struct SingleByteStrategy(pub &'static str);
impl InstructionStrategy for SingleByteStrategy {
    fn parse(&self, _parser: &InstructionParser, _mode: AddressModeValue) -> Result<Instruction> {
        match self.0 {
            "INX" => Ok(Instruction::INX),
            "INY" => Ok(Instruction::INY),
            "DEX" => Ok(Instruction::DEX),
            "DEY" => Ok(Instruction::DEY),
            "TAX" => Ok(Instruction::TAX),
            "TXA" => Ok(Instruction::TXA),
            "TAY" => Ok(Instruction::TAY),
            "TYA" => Ok(Instruction::TYA),
            "CLC" => Ok(Instruction::CLC),
            "SEC" => Ok(Instruction::SEC),
            "CLI" => Ok(Instruction::CLI),
            "SEI" => Ok(Instruction::SEI),
            "CLV" => Ok(Instruction::CLV),
            "CLD" => Ok(Instruction::CLD),
            "SED" => Ok(Instruction::SED),
            "NOP" => Ok(Instruction::NOP),
            "BRK" => Ok(Instruction::BRK),
            "RTI" => Ok(Instruction::RTI),
            "RTS" => Ok(Instruction::RTS),
            "PHA" => Ok(Instruction::PHA),
            "PLA" => Ok(Instruction::PLA),
            "PHP" => Ok(Instruction::PHP),
            "PLP" => Ok(Instruction::PLP),
            "TSX" => Ok(Instruction::TSX),
            "TXS" => Ok(Instruction::TXS),
            _ => unreachable!("Unknown single byte instruction"),
        }
    }
}

/// 분기 명령어 전략
pub struct BranchStrategy;
impl InstructionStrategy for BranchStrategy {
    fn parse(&self, parser: &InstructionParser, mode: AddressModeValue) -> Result<Instruction> {
        match mode {
            AddressModeValue::Absolute(addr) => parser.parse_branch(Instruction::BEQ, addr),
            _ => Err(Error::InvalidAddressingMode("Branch")),
        }
    }
}

// 시프트 명령어들
pub struct LSRStrategy;
impl InstructionStrategy for LSRStrategy {
    fn parse(&self, _parser: &InstructionParser, mode: AddressModeValue) -> Result<Instruction> {
        Ok(Instruction::LSR(mode))
    }
}

pub struct ASLStrategy;
impl InstructionStrategy for ASLStrategy {
    fn parse(&self, _parser: &InstructionParser, mode: AddressModeValue) -> Result<Instruction> {
        Ok(Instruction::ASL(mode))
    }
}

// 산술 연산 명령어들
pub struct ADCStrategy;
impl InstructionStrategy for ADCStrategy {
    fn parse(&self, _parser: &InstructionParser, mode: AddressModeValue) -> Result<Instruction> {
        Ok(Instruction::ADC(mode))
    }
}

// 논리 연산 명령어들
pub struct ANDStrategy;
impl InstructionStrategy for ANDStrategy {
    fn parse(&self, _parser: &InstructionParser, mode: AddressModeValue) -> Result<Instruction> {
        Ok(Instruction::AND(mode))
    }
}

pub struct ORAStrategy;
impl InstructionStrategy for ORAStrategy {
    fn parse(&self, _parser: &InstructionParser, mode: AddressModeValue) -> Result<Instruction> {
        Ok(Instruction::ORA(mode))
    }
}

pub struct EORStrategy;
impl InstructionStrategy for EORStrategy {
    fn parse(&self, _parser: &InstructionParser, mode: AddressModeValue) -> Result<Instruction> {
        Ok(Instruction::EOR(mode))
    }
}

// 비교 명령어들
pub struct CPYStrategy;
impl InstructionStrategy for CPYStrategy {
    fn parse(&self, _parser: &InstructionParser, mode: AddressModeValue) -> Result<Instruction> {
        Ok(Instruction::CPY(mode))
    }
}

pub struct CPXStrategy;
impl InstructionStrategy for CPXStrategy {
    fn parse(&self, _parser: &InstructionParser, mode: AddressModeValue) -> Result<Instruction> {
        Ok(Instruction::CPX(mode))
    }
}

// 분기 명령어들
pub struct BNEStrategy;
impl InstructionStrategy for BNEStrategy {
    fn parse(&self, parser: &InstructionParser, mode: AddressModeValue) -> Result<Instruction> {
        match mode {
            AddressModeValue::Absolute(target) => parser.parse_branch(Instruction::BNE, target),
            _ => Err(Error::InvalidAddressingMode("BNE")),
        }
    }
}

pub struct CMPStrategy;

impl InstructionStrategy for CMPStrategy {
    fn parse(&self, _parser: &InstructionParser, mode: AddressModeValue) -> Result<Instruction> {
        Ok(Instruction::CMP(mode))
    }
}
