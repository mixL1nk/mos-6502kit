use crate::lexer::Lexer;
use crate::parser::Parser;
use common::Result;
use types::Instruction;

pub struct Assembler {
    output: Vec<u8>,
}

impl Default for Assembler {
    fn default() -> Self {
        Self::new()
    }
}

impl Assembler {
    pub fn new() -> Self {
        Assembler { output: Vec::new() }
    }

    pub fn assemble(&mut self, source: &'static str) -> Result<Vec<Instruction>> {
        let lexer = Lexer::new(source);
        let tokens = lexer.tokenize()?;
        let mut parser = Parser::new(tokens);
        let instructions = parser.parse()?;

        Ok(instructions)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use types::{AddressMode, Instruction};
    #[test]
    fn test_assemble_load_store() {
        let mut assembler = Assembler::new();

        // 즉시 주소 지정
        let source = "LDA #$42";
        let instructions = assembler.assemble(source).unwrap();

        assert_eq!(instructions.len(), 1);
        assert!(matches!(
            instructions[0],
            Instruction::LDA(AddressMode::Immediate)
        ));

        // 제로 페이지
        let source = "LDA $42";
        let instructions = assembler.assemble(source).unwrap();
        assert_eq!(instructions.len(), 1);
        assert!(matches!(
            instructions[0],
            Instruction::LDA(AddressMode::ZeroPage)
        ));

        // 제로 페이지 X
        let source = "LDA $42,X";
        let instructions = assembler.assemble(source).unwrap();
        assert_eq!(instructions.len(), 1);
        assert!(matches!(
            instructions[0],
            Instruction::LDA(AddressMode::ZeroPageX)
        ));

        // 간접 X
        let source = "LDA ($42,X)";
        let instructions = assembler.assemble(source).unwrap();
        assert_eq!(instructions.len(), 1);
        assert!(matches!(
            instructions[0],
            Instruction::LDA(AddressMode::IndirectX)
        ));

        // 간접 Y
        let source = "LDA ($42),Y";
        let instructions = assembler.assemble(source).unwrap();
        assert_eq!(instructions.len(), 1);
        assert!(matches!(
            instructions[0],
            Instruction::LDA(AddressMode::IndirectY)
        ));
    }

    #[test]
    fn test_assemble_transfers() {
        let mut assembler = Assembler::new();
        let source = "
            TAX
            TXA
            TAY
            TYA
            TSX
            TXS
        ";
        let instructions = assembler.assemble(source).unwrap();
        assert_eq!(instructions.len(), 6);
        assert!(matches!(instructions[0], Instruction::TAX));
        assert!(matches!(instructions[1], Instruction::TXA));
        assert!(matches!(instructions[2], Instruction::TAY));
        assert!(matches!(instructions[3], Instruction::TYA));
        assert!(matches!(instructions[4], Instruction::TSX));
        assert!(matches!(instructions[5], Instruction::TXS));
    }

    #[test]
    fn test_assemble_stack() {
        let mut assembler = Assembler::new();
        let source = "
            PHA
            PLA
            PHP
            PLP
        ";
        let instructions = assembler.assemble(source).unwrap();
        assert_eq!(instructions.len(), 4);
        assert!(matches!(instructions[0], Instruction::PHA));
        assert!(matches!(instructions[1], Instruction::PLA));
        assert!(matches!(instructions[2], Instruction::PHP));
        assert!(matches!(instructions[3], Instruction::PLP));
    }

    #[test]
    fn test_assemble_store() {
        let mut assembler = Assembler::new();
        let source = "
            STA $42    ; 제로 페이지
            STX $43    ; 제로 페이지
            STY $44    ; 제로 페이지
            STA $45,X  ; 제로 페이지 X
            STX $46,Y  ; 제로 페이지 Y
            STY $47,X  ; 제로 페이지 X
        ";
        let instructions = assembler.assemble(source).unwrap();
        assert_eq!(instructions.len(), 6);
        assert!(matches!(
            instructions[0],
            Instruction::STA(AddressMode::ZeroPage)
        ));
        assert!(matches!(
            instructions[1],
            Instruction::STX(AddressMode::ZeroPage)
        ));
        assert!(matches!(
            instructions[2],
            Instruction::STY(AddressMode::ZeroPage)
        ));
        assert!(matches!(
            instructions[3],
            Instruction::STA(AddressMode::ZeroPageX)
        ));
        assert!(matches!(
            instructions[4],
            Instruction::STX(AddressMode::ZeroPageY)
        ));
        assert!(matches!(
            instructions[5],
            Instruction::STY(AddressMode::ZeroPageX)
        ));
    }

    #[test]
    fn test_assemble_complex_program() {
        let mut assembler = Assembler::new();
        let source = "
            ; 간단한 루프 프로그램
            LDA #$00    ; 초기값 0 로드
            TAX         ; X 레지스터로 전송
            loop:
                TXA     ; A 레지스터로 전송
                PHA     ; 스택에 저장
                INX     ; X 증가
                CPX #$0A; X가 10인지 비교
                BNE loop; 같지 않으면 반복
        ";
        let instructions = assembler.assemble(source).unwrap();

        // 현재는 레이블을 처리하지 않으므로 실제 명령어만 확인
        assert!(matches!(
            instructions[0],
            Instruction::LDA(AddressMode::Immediate)
        ));
        assert!(matches!(instructions[1], Instruction::TAX));
        assert!(matches!(instructions[2], Instruction::TXA));
        assert!(matches!(instructions[3], Instruction::PHA));
    }

    #[test]
    fn test_assemble_arithmetic() {
        let mut assembler = Assembler::new();
        let source = "
            ADC #$42    ; 즉시
            ADC $42     ; 제로 페이지
            ADC $42,X   ; 제로 페이지 X
            SBC #$42    ; 즉시
            SBC $42     ; 제로 페이지
            AND #$42    ; 즉시
            AND $42     ; 제로 페이지
            ORA #$42    ; 즉시
            ORA $42     ; 제로 페이지
            EOR #$42    ; 즉시
            EOR $42     ; 제로 페이지
        ";
        let instructions = assembler.assemble(source).unwrap();
        assert_eq!(instructions.len(), 11);
        assert!(matches!(
            instructions[0],
            Instruction::ADC(AddressMode::Immediate)
        ));
        assert!(matches!(
            instructions[1],
            Instruction::ADC(AddressMode::ZeroPage)
        ));
        assert!(matches!(
            instructions[2],
            Instruction::ADC(AddressMode::ZeroPageX)
        ));
        assert!(matches!(
            instructions[3],
            Instruction::SBC(AddressMode::Immediate)
        ));
        assert!(matches!(
            instructions[4],
            Instruction::SBC(AddressMode::ZeroPage)
        ));
        assert!(matches!(
            instructions[5],
            Instruction::AND(AddressMode::Immediate)
        ));
        assert!(matches!(
            instructions[6],
            Instruction::AND(AddressMode::ZeroPage)
        ));
        assert!(matches!(
            instructions[7],
            Instruction::ORA(AddressMode::Immediate)
        ));
        assert!(matches!(
            instructions[8],
            Instruction::ORA(AddressMode::ZeroPage)
        ));
        assert!(matches!(
            instructions[9],
            Instruction::EOR(AddressMode::Immediate)
        ));
        assert!(matches!(
            instructions[10],
            Instruction::EOR(AddressMode::ZeroPage)
        ));
    }

    #[test]
    fn test_assemble_compare() {
        let mut assembler = Assembler::new();
        let source = "
            CMP #$42    ; 즉시
            CMP $42     ; 제로 페이지
            CMP $42,X   ; 제로 페이지 X
            CPX #$42    ; 즉시
            CPX $42     ; 제로 페이지
            CPY #$42    ; 즉시
            CPY $42     ; 제로 페이지
        ";
        let instructions = assembler.assemble(source).unwrap();
        assert_eq!(instructions.len(), 7);
        assert!(matches!(
            instructions[0],
            Instruction::CMP(AddressMode::Immediate)
        ));
        assert!(matches!(
            instructions[1],
            Instruction::CMP(AddressMode::ZeroPage)
        ));
        assert!(matches!(
            instructions[2],
            Instruction::CMP(AddressMode::ZeroPageX)
        ));
        assert!(matches!(
            instructions[3],
            Instruction::CPX(AddressMode::Immediate)
        ));
        assert!(matches!(
            instructions[4],
            Instruction::CPX(AddressMode::ZeroPage)
        ));
        assert!(matches!(
            instructions[5],
            Instruction::CPY(AddressMode::Immediate)
        ));
        assert!(matches!(
            instructions[6],
            Instruction::CPY(AddressMode::ZeroPage)
        ));
    }

    #[test]
    fn test_assemble_branch() {
        let mut assembler = Assembler::new();
        let source = "
            BCC $10   ; 캐리 클리어
            BCS $20   ; 캐리 세트
            BEQ $30   ; 제로
            BNE $40   ; 낫 제로
            BMI $50   ; 마이너스
            BPL $60   ; 플러스
            BVC $70   ; 오버플로우 클리어
            BVS $80   ; 오버플로우 세트
        ";
        let instructions = assembler.assemble(source).unwrap();
        assert_eq!(instructions.len(), 8);
        assert!(matches!(instructions[0], Instruction::BCC(0x10)));
        assert!(matches!(instructions[1], Instruction::BCS(0x20)));
        assert!(matches!(instructions[2], Instruction::BEQ(0x30)));
        assert!(matches!(instructions[3], Instruction::BNE(0x40)));
        assert!(matches!(instructions[4], Instruction::BMI(0x50)));
        assert!(matches!(instructions[5], Instruction::BPL(0x60)));
        assert!(matches!(instructions[6], Instruction::BVC(0x70i8)));
        assert!(matches!(instructions[7], Instruction::BVS(-128i8)));
    }

    #[test]
    fn test_assemble_jump() {
        let mut assembler = Assembler::new();
        let source = "
            JMP $4242   ; 절대
            JMP ($4242) ; 간접
            JSR $4242   ; 절대
        ";
        let instructions = assembler.assemble(source).unwrap();
        assert_eq!(instructions.len(), 3);
        assert!(matches!(
            instructions[0],
            Instruction::JMP(AddressMode::Absolute)
        ));
        assert!(matches!(
            instructions[1],
            Instruction::JMP(AddressMode::Indirect)
        ));
        assert!(matches!(
            instructions[2],
            Instruction::JSR(AddressMode::Absolute)
        ));
    }

    #[test]
    fn test_assemble_increment_decrement() {
        let mut assembler = Assembler::new();
        let source = "
            INC $42     ; 제로 페이지
            INC $42,X   ; 제로 페이지 X
            DEC $42     ; 제로 페이지
            DEC $42,X   ; 제로 페이지 X
            INX         ; 묵시
            INY         ; 묵시
            DEX         ; 묵시
            DEY         ; 묵시
        ";
        let instructions = assembler.assemble(source).unwrap();
        assert_eq!(instructions.len(), 8);
        assert!(matches!(
            instructions[0],
            Instruction::INC(AddressMode::ZeroPage)
        ));
        assert!(matches!(
            instructions[1],
            Instruction::INC(AddressMode::ZeroPageX)
        ));
        assert!(matches!(
            instructions[2],
            Instruction::DEC(AddressMode::ZeroPage)
        ));
        assert!(matches!(
            instructions[3],
            Instruction::DEC(AddressMode::ZeroPageX)
        ));
        assert!(matches!(instructions[4], Instruction::INX));
        assert!(matches!(instructions[5], Instruction::INY));
        assert!(matches!(instructions[6], Instruction::DEX));
        assert!(matches!(instructions[7], Instruction::DEY));
    }

    #[test]
    fn test_assemble_flags() {
        let mut assembler = Assembler::new();
        let source = "
            CLC    ; 캐리 클리어
            SEC    ; 캐리 세트
            CLI    ; 인터럽트 클리어
            SEI    ; 인터럽트 세트
            CLV    ; 오버플로우 클리어
            CLD    ; 데시멀 클리어
            SED    ; 데시멀 세트
        ";
        let instructions = assembler.assemble(source).unwrap();
        assert_eq!(instructions.len(), 7);
        assert!(matches!(instructions[0], Instruction::CLC));
        assert!(matches!(instructions[1], Instruction::SEC));
        assert!(matches!(instructions[2], Instruction::CLI));
        assert!(matches!(instructions[3], Instruction::SEI));
        assert!(matches!(instructions[4], Instruction::CLV));
        assert!(matches!(instructions[5], Instruction::CLD));
        assert!(matches!(instructions[6], Instruction::SED));
    }
}
