use types::{AddressModeValue, Instruction};

pub struct InstructionFormatter;

impl InstructionFormatter {
    pub fn format_mnemonic(
        base_mnemonic: &str,
        instruction: &Instruction,
        operand: &[u8],
        pc: u16,
    ) -> String {
        match instruction {
            // 누산기 어드레싱 모드
            Instruction::ASL(AddressModeValue::Accumulator)
            | Instruction::LSR(AddressModeValue::Accumulator)
            | Instruction::ROL(AddressModeValue::Accumulator)
            | Instruction::ROR(AddressModeValue::Accumulator) => {
                format!("{} A", base_mnemonic)
            }

            // 묵시적 어드레싱 모드 (오퍼랜드 없음)
            Instruction::INX
            | Instruction::INY
            | Instruction::DEX
            | Instruction::DEY
            | Instruction::TAX
            | Instruction::TXA
            | Instruction::TAY
            | Instruction::TYA
            | Instruction::TSX
            | Instruction::TXS
            | Instruction::PHA
            | Instruction::PLA
            | Instruction::PHP
            | Instruction::PLP
            | Instruction::CLC
            | Instruction::SEC
            | Instruction::CLI
            | Instruction::SEI
            | Instruction::CLV
            | Instruction::CLD
            | Instruction::SED
            | Instruction::BRK
            | Instruction::RTI
            | Instruction::RTS
            | Instruction::NOP => base_mnemonic.to_string(),

            // 즉시 어드레싱 모드 (#)
            Instruction::LDA(AddressModeValue::Immediate(_))
            | Instruction::LDX(AddressModeValue::Immediate(_))
            | Instruction::LDY(AddressModeValue::Immediate(_))
            | Instruction::ADC(AddressModeValue::Immediate(_))
            | Instruction::SBC(AddressModeValue::Immediate(_))
            | Instruction::AND(AddressModeValue::Immediate(_))
            | Instruction::ORA(AddressModeValue::Immediate(_))
            | Instruction::EOR(AddressModeValue::Immediate(_))
            | Instruction::CMP(AddressModeValue::Immediate(_))
            | Instruction::CPX(AddressModeValue::Immediate(_))
            | Instruction::CPY(AddressModeValue::Immediate(_)) => {
                if !operand.is_empty() {
                    format!("{} #${:02X}", base_mnemonic, operand[0])
                } else {
                    format!("{} #$??", base_mnemonic)
                }
            }

            // 제로 페이지 어드레싱 모드
            Instruction::LDA(AddressModeValue::ZeroPage(_))
            | Instruction::LDX(AddressModeValue::ZeroPage(_))
            | Instruction::LDY(AddressModeValue::ZeroPage(_))
            | Instruction::STA(AddressModeValue::ZeroPage(_))
            | Instruction::STX(AddressModeValue::ZeroPage(_))
            | Instruction::STY(AddressModeValue::ZeroPage(_))
            | Instruction::ADC(AddressModeValue::ZeroPage(_))
            | Instruction::SBC(AddressModeValue::ZeroPage(_))
            | Instruction::AND(AddressModeValue::ZeroPage(_))
            | Instruction::ORA(AddressModeValue::ZeroPage(_))
            | Instruction::EOR(AddressModeValue::ZeroPage(_))
            | Instruction::ASL(AddressModeValue::ZeroPage(_))
            | Instruction::LSR(AddressModeValue::ZeroPage(_))
            | Instruction::ROL(AddressModeValue::ZeroPage(_))
            | Instruction::ROR(AddressModeValue::ZeroPage(_))
            | Instruction::INC(AddressModeValue::ZeroPage(_))
            | Instruction::DEC(AddressModeValue::ZeroPage(_))
            | Instruction::CMP(AddressModeValue::ZeroPage(_))
            | Instruction::CPX(AddressModeValue::ZeroPage(_))
            | Instruction::CPY(AddressModeValue::ZeroPage(_))
            | Instruction::BIT(AddressModeValue::ZeroPage(_)) => {
                if !operand.is_empty() {
                    format!("{} ${:02X}", base_mnemonic, operand[0])
                } else {
                    format!("{} $??", base_mnemonic)
                }
            }

            // 제로 페이지 X 인덱스 어드레싱 모드
            Instruction::LDA(AddressModeValue::ZeroPageX(_))
            | Instruction::LDY(AddressModeValue::ZeroPageX(_))
            | Instruction::STA(AddressModeValue::ZeroPageX(_))
            | Instruction::STY(AddressModeValue::ZeroPageX(_))
            | Instruction::ADC(AddressModeValue::ZeroPageX(_))
            | Instruction::SBC(AddressModeValue::ZeroPageX(_))
            | Instruction::AND(AddressModeValue::ZeroPageX(_))
            | Instruction::ORA(AddressModeValue::ZeroPageX(_))
            | Instruction::EOR(AddressModeValue::ZeroPageX(_))
            | Instruction::ASL(AddressModeValue::ZeroPageX(_))
            | Instruction::LSR(AddressModeValue::ZeroPageX(_))
            | Instruction::ROL(AddressModeValue::ZeroPageX(_))
            | Instruction::ROR(AddressModeValue::ZeroPageX(_))
            | Instruction::INC(AddressModeValue::ZeroPageX(_))
            | Instruction::DEC(AddressModeValue::ZeroPageX(_))
            | Instruction::CMP(AddressModeValue::ZeroPageX(_)) => {
                if !operand.is_empty() {
                    format!("{} ${:02X},X", base_mnemonic, operand[0])
                } else {
                    format!("{} $??,X", base_mnemonic)
                }
            }

            // 제로 페이지 Y 인덱스 어드레싱 모드
            Instruction::LDX(AddressModeValue::ZeroPageY(_))
            | Instruction::STX(AddressModeValue::ZeroPageY(_)) => {
                if !operand.is_empty() {
                    format!("{} ${:02X},Y", base_mnemonic, operand[0])
                } else {
                    format!("{} $??,Y", base_mnemonic)
                }
            }

            // 절대 어드레싱 모드
            Instruction::LDA(AddressModeValue::Absolute(_))
            | Instruction::LDX(AddressModeValue::Absolute(_))
            | Instruction::LDY(AddressModeValue::Absolute(_))
            | Instruction::STA(AddressModeValue::Absolute(_))
            | Instruction::STX(AddressModeValue::Absolute(_))
            | Instruction::STY(AddressModeValue::Absolute(_))
            | Instruction::ADC(AddressModeValue::Absolute(_))
            | Instruction::SBC(AddressModeValue::Absolute(_))
            | Instruction::AND(AddressModeValue::Absolute(_))
            | Instruction::ORA(AddressModeValue::Absolute(_))
            | Instruction::EOR(AddressModeValue::Absolute(_))
            | Instruction::ASL(AddressModeValue::Absolute(_))
            | Instruction::LSR(AddressModeValue::Absolute(_))
            | Instruction::ROL(AddressModeValue::Absolute(_))
            | Instruction::ROR(AddressModeValue::Absolute(_))
            | Instruction::INC(AddressModeValue::Absolute(_))
            | Instruction::DEC(AddressModeValue::Absolute(_))
            | Instruction::CMP(AddressModeValue::Absolute(_))
            | Instruction::CPX(AddressModeValue::Absolute(_))
            | Instruction::CPY(AddressModeValue::Absolute(_))
            | Instruction::BIT(AddressModeValue::Absolute(_))
            | Instruction::JMP(AddressModeValue::Absolute(_))
            | Instruction::JSR(AddressModeValue::Absolute(_)) => {
                if operand.len() >= 2 {
                    let addr = ((operand[1] as u16) << 8) | operand[0] as u16;
                    format!("{} ${:04X}", base_mnemonic, addr)
                } else {
                    format!("{} $????", base_mnemonic)
                }
            }

            // 절대 X 인덱스 어드레싱 모드
            Instruction::LDA(AddressModeValue::AbsoluteX(_))
            | Instruction::LDY(AddressModeValue::AbsoluteX(_))
            | Instruction::STA(AddressModeValue::AbsoluteX(_))
            | Instruction::ADC(AddressModeValue::AbsoluteX(_))
            | Instruction::SBC(AddressModeValue::AbsoluteX(_))
            | Instruction::AND(AddressModeValue::AbsoluteX(_))
            | Instruction::ORA(AddressModeValue::AbsoluteX(_))
            | Instruction::EOR(AddressModeValue::AbsoluteX(_))
            | Instruction::ASL(AddressModeValue::AbsoluteX(_))
            | Instruction::LSR(AddressModeValue::AbsoluteX(_))
            | Instruction::ROL(AddressModeValue::AbsoluteX(_))
            | Instruction::ROR(AddressModeValue::AbsoluteX(_))
            | Instruction::INC(AddressModeValue::AbsoluteX(_))
            | Instruction::DEC(AddressModeValue::AbsoluteX(_))
            | Instruction::CMP(AddressModeValue::AbsoluteX(_)) => {
                if operand.len() >= 2 {
                    let addr = ((operand[1] as u16) << 8) | operand[0] as u16;
                    format!("{} ${:04X},X", base_mnemonic, addr)
                } else {
                    format!("{} $????,X", base_mnemonic)
                }
            }

            // 절대 Y 인덱스 어드레싱 모드
            Instruction::LDA(AddressModeValue::AbsoluteY(_))
            | Instruction::LDX(AddressModeValue::AbsoluteY(_))
            | Instruction::STA(AddressModeValue::AbsoluteY(_))
            | Instruction::ADC(AddressModeValue::AbsoluteY(_))
            | Instruction::SBC(AddressModeValue::AbsoluteY(_))
            | Instruction::AND(AddressModeValue::AbsoluteY(_))
            | Instruction::ORA(AddressModeValue::AbsoluteY(_))
            | Instruction::EOR(AddressModeValue::AbsoluteY(_))
            | Instruction::CMP(AddressModeValue::AbsoluteY(_)) => {
                if operand.len() >= 2 {
                    let addr = ((operand[1] as u16) << 8) | operand[0] as u16;
                    format!("{} ${:04X},Y", base_mnemonic, addr)
                } else {
                    format!("{} $????,Y", base_mnemonic)
                }
            }

            // 간접 어드레싱 모드
            Instruction::JMP(AddressModeValue::Indirect(_)) => {
                if operand.len() >= 2 {
                    let addr = ((operand[1] as u16) << 8) | operand[0] as u16;
                    format!("{} (${:04X})", base_mnemonic, addr)
                } else {
                    format!("{} ($????)", base_mnemonic)
                }
            }

            // X 인덱스 간접 어드레싱 모드
            Instruction::LDA(AddressModeValue::IndirectX(_))
            | Instruction::STA(AddressModeValue::IndirectX(_))
            | Instruction::ADC(AddressModeValue::IndirectX(_))
            | Instruction::SBC(AddressModeValue::IndirectX(_))
            | Instruction::AND(AddressModeValue::IndirectX(_))
            | Instruction::ORA(AddressModeValue::IndirectX(_))
            | Instruction::EOR(AddressModeValue::IndirectX(_))
            | Instruction::CMP(AddressModeValue::IndirectX(_)) => {
                if !operand.is_empty() {
                    format!("{} (${:02X},X)", base_mnemonic, operand[0])
                } else {
                    format!("{} ($??,X)", base_mnemonic)
                }
            }

            // 간접 Y 인덱스 어드레싱 모드
            Instruction::LDA(AddressModeValue::IndirectY(_))
            | Instruction::STA(AddressModeValue::IndirectY(_))
            | Instruction::ADC(AddressModeValue::IndirectY(_))
            | Instruction::SBC(AddressModeValue::IndirectY(_))
            | Instruction::AND(AddressModeValue::IndirectY(_))
            | Instruction::ORA(AddressModeValue::IndirectY(_))
            | Instruction::EOR(AddressModeValue::IndirectY(_))
            | Instruction::CMP(AddressModeValue::IndirectY(_)) => {
                if !operand.is_empty() {
                    format!("{} (${:02X}),Y", base_mnemonic, operand[0])
                } else {
                    format!("{} ($??),Y", base_mnemonic)
                }
            }

            // 상대 어드레싱 모드 (분기 명령어)
            Instruction::BCC(_)
            | Instruction::BCS(_)
            | Instruction::BEQ(_)
            | Instruction::BNE(_)
            | Instruction::BMI(_)
            | Instruction::BPL(_)
            | Instruction::BVC(_)
            | Instruction::BVS(_) => {
                if !operand.is_empty() {
                    // 상대 주소 계산 (부호 있는 오프셋)
                    let offset = operand[0] as i8;
                    // 6502 CPU에서 상대 주소는 현재 명령어의 다음 명령어 주소(PC+2)에서 오프셋을 더함
                    // PC는 이미 명령어 바이트(1바이트) 다음으로 증가했으므로 PC+1+offset이 정확한 계산
                    let target_addr = ((pc as i32) + 1 + (offset as i32)) & 0xFFFF;
                    format!("{} ${:04X}", base_mnemonic, target_addr)
                } else {
                    format!("{} $????", base_mnemonic)
                }
            }

            // 기타 경우 (기본 포맷)
            _ => {
                if !operand.is_empty() {
                    let operand_str = operand
                        .iter()
                        .map(|&b| format!("${:02X}", b))
                        .collect::<Vec<_>>()
                        .join(" ");
                    format!("{} {}", base_mnemonic, operand_str)
                } else {
                    base_mnemonic.to_string()
                }
            }
        }
    }
}
