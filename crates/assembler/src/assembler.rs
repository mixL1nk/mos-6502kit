use crate::generator::Generator;
use crate::lexer::{Lexer, TokenInfo};
use crate::parser::Parser;
use common::Result;
use types::Instruction;

pub struct Assembler {
    org: u16,
}

impl Default for Assembler {
    fn default() -> Self {
        Self::new(0x8000)
    }
}

impl Assembler {
    pub fn new(org: u16) -> Self {
        Assembler { org }
    }

    pub fn assemble(&self, source: &str) -> Result<Vec<u8>> {
        println!("[DEBUG] Assembling source code: \n{}", source);

        // 1. 어휘 분석 (Lexical Analysis)
        let lexer = Lexer::new(source);
        let tokens = lexer.tokenize()?;

        // println!("[DEBUG] Tokens: {:?}", tokens);

        // 2. 구문 분석 (Parsing)
        let mut parser = Parser::new(tokens);
        let instructions = parser.parse()?;

        // println!("[DEBUG] Instructions: {:?}", instructions);

        // 3. 코드 생성 (Code Generation)
        let mut generator = Generator::new(self.org);
        let machine_code = generator.generate(instructions)?;

        // println!("[DEBUG] Machine code: {:?}", machine_code);

        Ok(machine_code)
    }

    pub fn tokenize(&self, source: &str) -> Result<Vec<TokenInfo>> {
        let lexer = Lexer::new(source);
        let tokens = lexer.tokenize()?;
        Ok(tokens)
    }

    pub fn parse(&self, tokens: Vec<TokenInfo>) -> Result<Vec<Instruction>> {
        let mut parser = Parser::new(tokens);
        parser.set_org(self.org);
        parser.parse()
    }

    pub fn generate(&self, instructions: Vec<Instruction>) -> Result<Vec<u8>> {
        let mut generator = Generator::new(self.org);
        let machine_code = generator.generate(instructions)?;
        Ok(machine_code)
    }

    pub fn set_org(&mut self, org: u16) {
        self.org = org;
    }

    pub fn get_org(&self) -> u16 {
        self.org
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use types::{AddressModeValue, Instruction};
    #[test]
    fn test_assemble_load_store() {
        let assembler = Assembler::default();

        // 즉시 주소 지정
        let source = "LDA #$42";
        let tokens = assembler.tokenize(source).unwrap();
        let instructions = assembler.parse(tokens).unwrap();

        assert_eq!(instructions.len(), 1);
        assert!(matches!(
            instructions[0],
            Instruction::LDA(AddressModeValue::Immediate(_))
        ));

        // 제로 페이지
        let source = "LDA $42";
        let tokens = assembler.tokenize(source).unwrap();
        let instructions = assembler.parse(tokens).unwrap();

        assert_eq!(instructions.len(), 1);
        assert!(matches!(
            instructions[0],
            Instruction::LDA(AddressModeValue::ZeroPage(_))
        ));

        // 제로 페이지 X
        let source = "LDA $42,X";
        let tokens = assembler.tokenize(source).unwrap();
        let instructions = assembler.parse(tokens).unwrap();

        assert_eq!(instructions.len(), 1);
        assert!(matches!(
            instructions[0],
            Instruction::LDA(AddressModeValue::ZeroPageX(_))
        ));

        // 간접 X
        let source = "LDA ($42,X)";
        let tokens = assembler.tokenize(source).unwrap();
        let instructions = assembler.parse(tokens).unwrap();

        assert_eq!(instructions.len(), 1);
        assert!(matches!(
            instructions[0],
            Instruction::LDA(AddressModeValue::IndirectX(_))
        ));

        // 간접 Y
        let source = "LDA ($42),Y";
        let tokens = assembler.tokenize(source).unwrap();
        let instructions = assembler.parse(tokens).unwrap();

        assert_eq!(instructions.len(), 1);
        assert!(matches!(
            instructions[0],
            Instruction::LDA(AddressModeValue::IndirectY(_))
        ));
    }

    #[test]
    fn test_assemble_multiline() {
        let assembler = Assembler::default();

        // 여러 줄의 코드 테스트
        let source = "
            LDA #$42
            TAX
            INY
        ";

        let machine_code = assembler.assemble(source).unwrap();

        // LDA #$42 + TAX + INY = [0xA9, 0x42, 0xAA, 0xC8]
        assert_eq!(machine_code, vec![0xA9, 0x42, 0xAA, 0xC8]);
    }

    #[test]
    fn test_assemble_with_comments() {
        let assembler = Assembler::default();

        // 주석과 빈 줄이 있는 코드
        let source = "
            ; 이것은 주석입니다
            LDA #$42  ; 누산기에 값 로드
            
            ; 빈 줄 다음의 명령어
            TAX       ; A를 X로 전송
        ";

        let machine_code = assembler.assemble(source).unwrap();

        // LDA #$42 + TAX = [0xA9, 0x42, 0xAA]
        assert_eq!(machine_code, vec![0xA9, 0x42, 0xAA]);
    }

    #[test]
    fn test_assemble_indented_code() {
        let assembler = Assembler::default();

        // 들여쓰기가 있는 코드
        let source = "
            LDA #$01
                STA $80    ; 들여쓰기가 더 많음
              LDX #$02     ; 약간 들여쓰기
            TAY
        ";

        let machine_code = assembler.assemble(source).unwrap();

        // LDA #$01 + STA $80 + LDX #$02 + TAY = [0xA9, 0x01, 0x85, 0x80, 0xA2, 0x02, 0xA8]
        assert_eq!(machine_code, vec![0xA9, 0x01, 0x85, 0x80, 0xA2, 0x02, 0xA8]);
    }
}
