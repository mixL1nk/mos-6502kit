use types::{AddressModeValue, Instruction};

/// Instruction size calculator
pub struct InstructionSizeCalculator;

impl InstructionSizeCalculator {
    pub fn get_instruction_size(instruction: &Instruction) -> u16 {
        match instruction {
            // Single byte instructions
            Instruction::TAX
            | Instruction::TXA
            | Instruction::TAY
            | Instruction::TYA
            | Instruction::INX
            | Instruction::INY
            | Instruction::DEX
            | Instruction::DEY
            | Instruction::CLC
            | Instruction::SEC
            | Instruction::CLI
            | Instruction::SEI
            | Instruction::CLV
            | Instruction::CLD
            | Instruction::SED
            | Instruction::NOP
            | Instruction::BRK
            | Instruction::RTI
            | Instruction::RTS
            | Instruction::PHA
            | Instruction::PLA
            | Instruction::PHP
            | Instruction::PLP
            | Instruction::TSX
            | Instruction::TXS => 1,

            // Branch instructions (always 2 bytes)
            Instruction::BCC(_)
            | Instruction::BCS(_)
            | Instruction::BEQ(_)
            | Instruction::BNE(_)
            | Instruction::BMI(_)
            | Instruction::BPL(_)
            | Instruction::BVC(_)
            | Instruction::BVS(_) => 2,

            // Shift and rotate instructions
            Instruction::LSR(mode)
            | Instruction::ASL(mode)
            | Instruction::ROL(mode)
            | Instruction::ROR(mode) => match mode {
                AddressModeValue::Accumulator => 1,
                AddressModeValue::ZeroPage(_) | AddressModeValue::ZeroPageX(_) => 2,
                AddressModeValue::Absolute(_) | AddressModeValue::AbsoluteX(_) => 3,
                _ => 2,
            },

            // Increment and decrement instructions
            Instruction::INC(mode) | Instruction::DEC(mode) => match mode {
                AddressModeValue::ZeroPage(_) | AddressModeValue::ZeroPageX(_) => 2,
                AddressModeValue::Absolute(_) | AddressModeValue::AbsoluteX(_) => 3,
                _ => 2,
            },

            // Addressing mode dependent instructions
            Instruction::LDA(mode)
            | Instruction::LDX(mode)
            | Instruction::LDY(mode)
            | Instruction::STA(mode)
            | Instruction::STX(mode)
            | Instruction::STY(mode)
            | Instruction::ADC(mode)
            | Instruction::SBC(mode)
            | Instruction::AND(mode)
            | Instruction::ORA(mode)
            | Instruction::EOR(mode)
            | Instruction::CMP(mode)
            | Instruction::CPX(mode)
            | Instruction::CPY(mode)
            | Instruction::BIT(mode) => match mode {
                AddressModeValue::Immediate(_)
                | AddressModeValue::ZeroPage(_)
                | AddressModeValue::ZeroPageX(_)
                | AddressModeValue::ZeroPageY(_)
                | AddressModeValue::IndirectX(_)
                | AddressModeValue::IndirectY(_) => 2,

                AddressModeValue::Absolute(_)
                | AddressModeValue::AbsoluteX(_)
                | AddressModeValue::AbsoluteY(_) => 3,

                AddressModeValue::Accumulator | AddressModeValue::Implied => 1,

                AddressModeValue::Indirect(_) => 3,
            },

            // Jump instructions
            Instruction::JMP(mode) => match mode {
                AddressModeValue::Absolute(_) | AddressModeValue::Indirect(_) => 3,
                _ => 3, // Default for JMP
            },

            Instruction::JSR(mode) => match mode {
                AddressModeValue::Absolute(_) => 3,
                _ => 3, // Default for JSR
            },
        }
    }

    pub fn estimate_instruction_size(mnemonic: &str) -> u16 {
        match mnemonic {
            // Single byte instructions
            "INX" | "INY" | "DEX" | "DEY" | "TAX" | "TXA" | "TAY" | "TYA" | "CLC" | "SEC"
            | "CLI" | "SEI" | "CLV" | "CLD" | "SED" | "NOP" | "BRK" | "RTI" | "RTS" | "PHA"
            | "PLA" | "PHP" | "PLP" | "TSX" | "TXS" => 1,

            // Branch instructions (always 2 bytes)
            "BCC" | "BCS" | "BEQ" | "BNE" | "BMI" | "BPL" | "BVC" | "BVS" => 2,

            // Load/Store instructions (usually 2 bytes)
            "LDA" | "LDX" | "LDY" | "STA" | "STX" | "STY" => 2,

            // Compare instructions (usually 2 bytes)
            "CMP" | "CPX" | "CPY" => 2,

            // Arithmetic/Logical instructions (usually 2 bytes)
            "ADC" | "SBC" | "AND" | "ORA" | "EOR" => 2,

            // Shift/Rotate instructions (usually 2 bytes)
            "ASL" | "LSR" | "ROL" | "ROR" => 2,

            // Increment/Decrement instructions (usually 2 bytes)
            "INC" | "DEC" => 2,

            // Bit Test instructions
            "BIT" => 2,

            // Jump instructions (always 3 bytes)
            "JMP" | "JSR" => 3,

            // Default for other instructions (conservative estimate)
            _ => 2,
        }
    }
}
