use common::Result;
use types::{Instruction, OPCODE_MAP};

use crate::formatter::InstructionFormatter;
use crate::instruction::DInstruction;

pub struct Disassembler {
    memory: Vec<u8>,
    pc: u16,
}

impl Default for Disassembler {
    fn default() -> Self {
        Self::new(vec![])
    }
}

impl Disassembler {
    pub fn new(memory: Vec<u8>) -> Self {
        Self { memory, pc: 0 }
    }

    pub fn disassemble(&mut self) -> Result<Vec<DInstruction>> {
        let mut instructions = Vec::<DInstruction>::new();
        while self.pc < self.memory.len() as u16 {
            let opcode = self.memory[self.pc as usize];
            let instruction = self.decode_instruction(opcode)?;
            instructions.push(instruction);
        }
        Ok(instructions)
    }

    fn decode_instruction(&mut self, opcode: u8) -> Result<DInstruction> {
        let address = self.pc;
        let inst_info = OPCODE_MAP.get(&opcode).unwrap();
        // 명령어 기본 이름 추출 (LDA, LDX, INX 등)
        let base_mnemonic = match inst_info.instruction {
            Instruction::LDA(_) => "LDA",
            Instruction::LDX(_) => "LDX",
            Instruction::LDY(_) => "LDY",
            Instruction::STA(_) => "STA",
            Instruction::STX(_) => "STX",
            Instruction::STY(_) => "STY",
            Instruction::ADC(_) => "ADC",
            Instruction::SBC(_) => "SBC",
            Instruction::AND(_) => "AND",
            Instruction::ORA(_) => "ORA",
            Instruction::EOR(_) => "EOR",
            Instruction::ASL(_) => "ASL",
            Instruction::LSR(_) => "LSR",
            Instruction::ROL(_) => "ROL",
            Instruction::ROR(_) => "ROR",
            Instruction::INC(_) => "INC",
            Instruction::DEC(_) => "DEC",
            Instruction::CMP(_) => "CMP",
            Instruction::CPX(_) => "CPX",
            Instruction::CPY(_) => "CPY",
            Instruction::BIT(_) => "BIT",
            Instruction::JMP(_) => "JMP",
            Instruction::JSR(_) => "JSR",
            Instruction::BCC(_) => "BCC",
            Instruction::BCS(_) => "BCS",
            Instruction::BEQ(_) => "BEQ",
            Instruction::BNE(_) => "BNE",
            Instruction::BMI(_) => "BMI",
            Instruction::BPL(_) => "BPL",
            Instruction::BVC(_) => "BVC",
            Instruction::BVS(_) => "BVS",
            Instruction::INX => "INX",
            Instruction::INY => "INY",
            Instruction::DEX => "DEX",
            Instruction::DEY => "DEY",
            Instruction::TAX => "TAX",
            Instruction::TXA => "TXA",
            Instruction::TAY => "TAY",
            Instruction::TYA => "TYA",
            Instruction::TSX => "TSX",
            Instruction::TXS => "TXS",
            Instruction::PHA => "PHA",
            Instruction::PLA => "PLA",
            Instruction::PHP => "PHP",
            Instruction::PLP => "PLP",
            Instruction::CLC => "CLC",
            Instruction::SEC => "SEC",
            Instruction::CLI => "CLI",
            Instruction::SEI => "SEI",
            Instruction::CLV => "CLV",
            Instruction::CLD => "CLD",
            Instruction::SED => "SED",
            Instruction::BRK => "BRK",
            Instruction::RTI => "RTI",
            Instruction::RTS => "RTS",
            Instruction::NOP => "NOP",
        };

        // PC 증가 (명령어 바이트 다음으로)
        self.pc += 1;

        // 오퍼랜드 크기 가져오기
        let operand_size = inst_info.get_operand_size();

        // 오퍼랜드 가져오기
        let operand =
            if operand_size > 0 && (self.pc + operand_size as u16) <= self.memory.len() as u16 {
                self.memory[self.pc as usize..(self.pc as usize + operand_size as usize)].to_vec()
            } else {
                Vec::new()
            };

        let formatted_mnemonic = InstructionFormatter::format_mnemonic(
            base_mnemonic,
            &inst_info.instruction,
            &operand,
            self.pc,
        );

        // PC 증가 (오퍼랜드 다음으로)
        self.pc += operand_size as u16;

        let mut base_instruction = Vec::<u8>::new();
        base_instruction.push(opcode);
        base_instruction.extend(operand.clone());
        let base_instruction = base_instruction
            .iter()
            .map(|b| format!("{:02X}", b))
            .collect::<Vec<_>>()
            .join("");

        let instruction = DInstruction::new(
            opcode,
            operand,
            formatted_mnemonic,
            base_instruction,
            address,
        );

        Ok(instruction)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_disassemble() {
        let memory = vec![
            0xa2, 0x00, 0xa0, 0x00, 0xe8, 0xc8, 0xc0, 0x05, 0xd0, 0xfa, 0xa9, 0x42,
        ];
        let mut disassembler = Disassembler::new(memory);
        let instructions = disassembler.disassemble().unwrap();
        assert!(!instructions.is_empty());
        println!("instructions: {:?}", instructions);

        // 첫 번째 명령어 확인 (LDX #$00)
        assert_eq!(instructions[0].opcode, 0xa2);
        assert_eq!(instructions[0].mnemonic, "LDX #$00");

        // INX 명령어 확인
        let inx_inst = instructions.iter().find(|i| i.opcode == 0xe8).unwrap();
        assert_eq!(inx_inst.mnemonic, "INX");

        // CPY #$05 명령어 확인
        let cpy_inst = instructions.iter().find(|i| i.opcode == 0xc0).unwrap();
        assert_eq!(cpy_inst.mnemonic, "CPY #$05");

        // BNE 명령어 확인 (상대 주소 계산 확인)
        let bne_inst = instructions.iter().find(|i| i.opcode == 0xd0).unwrap();
        assert!(bne_inst.mnemonic.starts_with("BNE $"));
    }
}
