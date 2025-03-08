use crate::lexer::{Token, TokenInfo};
use common::Result;
use error::Error;
use types::{AddressMode, Instruction};

pub struct Parser {
    tokens: Vec<TokenInfo>,
    current: usize,
}

impl Parser {
    pub fn new(tokens: Vec<TokenInfo>) -> Self {
        Parser { tokens, current: 0 }
    }

    pub fn parse(&mut self) -> Result<Vec<Instruction>> {
        let mut instructions = Vec::new();

        while !self.is_at_end() {
            if let Some(instruction) = self.parse_instruction()? {
                instructions.push(instruction);
            }
            self.advance();
        }

        Ok(instructions)
    }

    fn parse_instruction(&mut self) -> Result<Option<Instruction>> {
        let token = match self.peek() {
            Some(token) => token,
            None => return Ok(None),
        };

        match &token.token {
            Token::Mnemonic(mnemonic) => {
                match mnemonic.as_str() {
                    // 로드/스토어 명령어
                    "LDA" => {
                        let mode = self.parse_addressing_mode()?;
                        Ok(Some(Instruction::LDA(mode)))
                    }
                    "LDX" => {
                        let mode = self.parse_addressing_mode()?;
                        Ok(Some(Instruction::LDX(mode)))
                    }
                    "LDY" => {
                        let mode = self.parse_addressing_mode()?;
                        Ok(Some(Instruction::LDY(mode)))
                    }
                    "STA" => {
                        let mode = self.parse_addressing_mode()?;
                        Ok(Some(Instruction::STA(mode)))
                    }
                    "STX" => {
                        let mode = self.parse_addressing_mode()?;
                        Ok(Some(Instruction::STX(mode)))
                    }
                    "STY" => {
                        let mode = self.parse_addressing_mode()?;
                        Ok(Some(Instruction::STY(mode)))
                    }
                    // 산술/논리 연산 명령어
                    "ADC" => {
                        let mode = self.parse_addressing_mode()?;
                        Ok(Some(Instruction::ADC(mode)))
                    }
                    "SBC" => {
                        let mode = self.parse_addressing_mode()?;
                        Ok(Some(Instruction::SBC(mode)))
                    }
                    "AND" => {
                        let mode = self.parse_addressing_mode()?;
                        Ok(Some(Instruction::AND(mode)))
                    }
                    "ORA" => {
                        let mode = self.parse_addressing_mode()?;
                        Ok(Some(Instruction::ORA(mode)))
                    }
                    "EOR" => {
                        let mode = self.parse_addressing_mode()?;
                        Ok(Some(Instruction::EOR(mode)))
                    }
                    // 비교 명령어
                    "CMP" => {
                        let mode = self.parse_addressing_mode()?;
                        Ok(Some(Instruction::CMP(mode)))
                    }
                    "CPX" => {
                        let mode = self.parse_addressing_mode()?;
                        Ok(Some(Instruction::CPX(mode)))
                    }
                    "CPY" => {
                        let mode = self.parse_addressing_mode()?;
                        Ok(Some(Instruction::CPY(mode)))
                    }
                    // 분기 명령어
                    "BCC" => {
                        self.advance();
                        let offset = self.parse_branch_offset()?;
                        Ok(Some(Instruction::BCC(offset)))
                    }
                    "BCS" => {
                        self.advance();
                        let offset = self.parse_branch_offset()?;
                        Ok(Some(Instruction::BCS(offset)))
                    }
                    "BEQ" => {
                        self.advance();
                        let offset = self.parse_branch_offset()?;
                        Ok(Some(Instruction::BEQ(offset)))
                    }
                    "BNE" => {
                        self.advance();
                        let offset = self.parse_branch_offset()?;
                        Ok(Some(Instruction::BNE(offset)))
                    }
                    "BMI" => {
                        self.advance();
                        let offset = self.parse_branch_offset()?;
                        Ok(Some(Instruction::BMI(offset)))
                    }
                    "BPL" => {
                        self.advance();
                        let offset = self.parse_branch_offset()?;
                        Ok(Some(Instruction::BPL(offset)))
                    }
                    "BVC" => {
                        self.advance();
                        let offset = self.parse_branch_offset()?;
                        Ok(Some(Instruction::BVC(offset)))
                    }
                    "BVS" => {
                        self.advance();
                        let offset = self.parse_branch_offset()?;
                        Ok(Some(Instruction::BVS(offset)))
                    }
                    // 점프 명령어
                    "JMP" => {
                        let mode = self.parse_addressing_mode()?;
                        match mode {
                            AddressMode::Absolute | AddressMode::Indirect => {
                                Ok(Some(Instruction::JMP(mode)))
                            }
                            _ => Err(Error::InvalidAddressingMode("JMP")),
                        }
                    }
                    "JSR" => {
                        let mode = self.parse_addressing_mode()?;
                        match mode {
                            AddressMode::Absolute => Ok(Some(Instruction::JSR(mode))),
                            _ => Err(Error::InvalidAddressingMode("JSR")),
                        }
                    }
                    // 증감 명령어
                    "INC" => {
                        let mode = self.parse_addressing_mode()?;
                        Ok(Some(Instruction::INC(mode)))
                    }
                    "DEC" => {
                        let mode = self.parse_addressing_mode()?;
                        Ok(Some(Instruction::DEC(mode)))
                    }
                    "INX" => {
                        self.advance();
                        Ok(Some(Instruction::INX))
                    }
                    "INY" => {
                        self.advance();
                        Ok(Some(Instruction::INY))
                    }
                    "DEX" => {
                        self.advance();
                        Ok(Some(Instruction::DEX))
                    }
                    "DEY" => {
                        self.advance();
                        Ok(Some(Instruction::DEY))
                    }
                    // 플래그 조작 명령어
                    "CLC" => {
                        self.advance();
                        Ok(Some(Instruction::CLC))
                    }
                    "SEC" => {
                        self.advance();
                        Ok(Some(Instruction::SEC))
                    }
                    "CLI" => {
                        self.advance();
                        Ok(Some(Instruction::CLI))
                    }
                    "SEI" => {
                        self.advance();
                        Ok(Some(Instruction::SEI))
                    }
                    "CLV" => {
                        self.advance();
                        Ok(Some(Instruction::CLV))
                    }
                    "CLD" => {
                        self.advance();
                        Ok(Some(Instruction::CLD))
                    }
                    "SED" => {
                        self.advance();
                        Ok(Some(Instruction::SED))
                    }
                    // 단일 바이트 명령어
                    "TAX" => {
                        self.advance();
                        Ok(Some(Instruction::TAX))
                    }
                    "TXA" => {
                        self.advance();
                        Ok(Some(Instruction::TXA))
                    }
                    "TAY" => {
                        self.advance();
                        Ok(Some(Instruction::TAY))
                    }
                    "TYA" => {
                        self.advance();
                        Ok(Some(Instruction::TYA))
                    }
                    "TSX" => {
                        self.advance();
                        Ok(Some(Instruction::TSX))
                    }
                    "TXS" => {
                        self.advance();
                        Ok(Some(Instruction::TXS))
                    }
                    "PHA" => {
                        self.advance();
                        Ok(Some(Instruction::PHA))
                    }
                    "PLA" => {
                        self.advance();
                        Ok(Some(Instruction::PLA))
                    }
                    "PHP" => {
                        self.advance();
                        Ok(Some(Instruction::PHP))
                    }
                    "PLP" => {
                        self.advance();
                        Ok(Some(Instruction::PLP))
                    }
                    // 기타 명령어들은 추후 구현
                    _ => Ok(None),
                }
            }
            _ => Ok(None),
        }
    }

    fn parse_addressing_mode(&mut self) -> Result<AddressMode> {
        self.advance(); // 명령어 토큰을 건너뜀

        // Whitespace 토큰들을 건너뛰기
        while let Some(token) = self.peek() {
            if !matches!(token.token, Token::Whitespace) {
                break;
            }
            self.advance();
        }

        match self.peek() {
            Some(token) => match &token.token {
                Token::Hash => {
                    self.advance(); // # 토큰을 건너뜀

                    // Whitespace 토큰들을 건너뛰기
                    while let Some(token) = self.peek() {
                        if !matches!(token.token, Token::Whitespace) {
                            break;
                        }
                        self.advance();
                    }

                    match self.peek() {
                        Some(TokenInfo {
                            token: Token::HexNumber(_),
                            ..
                        }) => {
                            self.advance(); // 숫자를 건너뜀
                            Ok(AddressMode::Immediate)
                        }
                        Some(TokenInfo {
                            token: Token::Number(_),
                            ..
                        }) => {
                            self.advance(); // 숫자를 건너뜀
                            Ok(AddressMode::Immediate)
                        }
                        _ => Ok(AddressMode::Immediate), // 에러 처리는 나중에
                    }
                }
                Token::LeftParen => {
                    self.advance(); // ( 토큰을 건너뜀

                    // Whitespace 토큰들을 건너뛰기
                    while let Some(token) = self.peek() {
                        if !matches!(token.token, Token::Whitespace) {
                            break;
                        }
                        self.advance();
                    }

                    // 숫자 토큰 확인
                    match self.peek() {
                        Some(token) => match &token.token {
                            Token::HexNumber(_) => {
                                self.advance(); // 숫자 토큰을 건너뜀

                                // Whitespace 토큰들을 건너뛰기
                                while let Some(token) = self.peek() {
                                    if !matches!(token.token, Token::Whitespace) {
                                        break;
                                    }
                                    self.advance();
                                }

                                // 다음 토큰 확인
                                match self.peek() {
                                    Some(token) => match &token.token {
                                        Token::RightParen => {
                                            self.advance(); // ) 토큰을 건너뜀

                                            // Whitespace 토큰들을 건너뛰기
                                            while let Some(token) = self.peek() {
                                                if !matches!(token.token, Token::Whitespace) {
                                                    break;
                                                }
                                                self.advance();
                                            }

                                            // 콤마와 Y 레지스터 확인
                                            match self.peek() {
                                                Some(token) => match &token.token {
                                                    Token::Comma => {
                                                        self.advance(); // , 토큰을 건너뜀

                                                        // Whitespace 토큰들을 건너뛰기
                                                        while let Some(token) = self.peek() {
                                                            if !matches!(
                                                                token.token,
                                                                Token::Whitespace
                                                            ) {
                                                                break;
                                                            }
                                                            self.advance();
                                                        }

                                                        // Y 레지스터 확인
                                                        match self.peek() {
                                                            Some(token) => match &token.token {
                                                                Token::IndexRegister('Y') => {
                                                                    self.advance(); // Y 토큰을 건너뜀
                                                                    Ok(AddressMode::IndirectY)
                                                                }
                                                                _ => Ok(AddressMode::Indirect),
                                                            },
                                                            None => Ok(AddressMode::Indirect),
                                                        }
                                                    }
                                                    _ => Ok(AddressMode::Indirect),
                                                },
                                                None => Ok(AddressMode::Indirect),
                                            }
                                        }
                                        Token::Comma => {
                                            self.advance(); // , 토큰을 건너뜀

                                            // Whitespace 토큰들을 건너뛰기
                                            while let Some(token) = self.peek() {
                                                if !matches!(token.token, Token::Whitespace) {
                                                    break;
                                                }
                                                self.advance();
                                            }

                                            // X 레지스터 확인
                                            match self.peek() {
                                                Some(token) => match &token.token {
                                                    Token::IndexRegister('X') => {
                                                        self.advance(); // X 토큰을 건너뜀

                                                        // Whitespace 토큰들을 건너뛰기
                                                        while let Some(token) = self.peek() {
                                                            if !matches!(
                                                                token.token,
                                                                Token::Whitespace
                                                            ) {
                                                                break;
                                                            }
                                                            self.advance();
                                                        }

                                                        // ) 토큰 확인
                                                        match self.peek() {
                                                            Some(token) => match &token.token {
                                                                Token::RightParen => {
                                                                    self.advance(); // ) 토큰을 건너뜀
                                                                    Ok(AddressMode::IndirectX)
                                                                }
                                                                _ => Ok(AddressMode::ZeroPageX),
                                                            },
                                                            None => Ok(AddressMode::ZeroPageX),
                                                        }
                                                    }
                                                    _ => Ok(AddressMode::ZeroPage),
                                                },
                                                None => Ok(AddressMode::ZeroPage),
                                            }
                                        }
                                        _ => Ok(AddressMode::ZeroPage),
                                    },
                                    None => Ok(AddressMode::ZeroPage),
                                }
                            }
                            _ => Ok(AddressMode::ZeroPage),
                        },
                        None => Ok(AddressMode::ZeroPage),
                    }
                }
                Token::HexNumber(n) => {
                    let value = *n; // 값을 복사
                    self.advance(); // 숫자 토큰을 건너뜀

                    // Whitespace 토큰들을 건너뛰기
                    while let Some(token) = self.peek() {
                        if !matches!(token.token, Token::Whitespace) {
                            break;
                        }
                        self.advance();
                    }

                    // 다음 토큰 확인
                    match self.peek() {
                        Some(token) => match &token.token {
                            Token::Comma => {
                                self.advance(); // , 토큰을 건너뜀

                                // Whitespace 토큰들을 건너뛰기
                                while let Some(token) = self.peek() {
                                    if !matches!(token.token, Token::Whitespace) {
                                        break;
                                    }
                                    self.advance();
                                }

                                // X 또는 Y 레지스터 확인
                                match self.peek() {
                                    Some(token) => match &token.token {
                                        Token::IndexRegister('X') => {
                                            self.advance(); // X 토큰을 건너뜀
                                            Ok(AddressMode::ZeroPageX)
                                        }
                                        Token::IndexRegister('Y') => {
                                            self.advance(); // Y 토큰을 건너뜀
                                            Ok(AddressMode::ZeroPageY)
                                        }
                                        _ => Ok(AddressMode::ZeroPage),
                                    },
                                    None => Ok(AddressMode::ZeroPage),
                                }
                            }
                            _ => {
                                // $FF 이하면 제로 페이지, 그 이상이면 절대 주소
                                if value <= 0xFF {
                                    Ok(AddressMode::ZeroPage)
                                } else {
                                    Ok(AddressMode::Absolute)
                                }
                            }
                        },
                        None => {
                            // $FF 이하면 제로 페이지, 그 이상이면 절대 주소
                            if value <= 0xFF {
                                Ok(AddressMode::ZeroPage)
                            } else {
                                Ok(AddressMode::Absolute)
                            }
                        }
                    }
                }
                _ => Ok(AddressMode::ZeroPage),
            },
            None => Ok(AddressMode::ZeroPage),
        }
    }

    fn parse_branch_offset(&mut self) -> Result<i8> {
        // Whitespace 토큰들을 건너뛰기
        while let Some(token) = self.peek() {
            if !matches!(token.token, Token::Whitespace) {
                break;
            }
            self.advance();
        }

        let value = match self.peek() {
            Some(TokenInfo {
                token: Token::HexNumber(n),
                ..
            }) => (*n as u8) as i8,
            Some(TokenInfo {
                token: Token::Number(n),
                ..
            }) => (*n as u8) as i8,
            _ => 0i8,
        };

        if value != 0 {
            self.advance();
        }

        Ok(value)
    }

    fn peek_next_is_comma_x(&self) -> bool {
        let mut pos = self.current;

        // Whitespace 토큰들을 건너뛰기
        while let Some(token) = self.tokens.get(pos) {
            if !matches!(token.token, Token::Whitespace) {
                break;
            }
            pos += 1;
        }

        // Comma 토큰 확인
        if let Some(token) = self.tokens.get(pos) {
            if matches!(&token.token, Token::Comma) {
                pos += 1;

                // Whitespace 토큰들을 건너뛰기
                while let Some(token) = self.tokens.get(pos) {
                    if !matches!(token.token, Token::Whitespace) {
                        break;
                    }
                    pos += 1;
                }

                // X 레지스터 토큰 확인
                if let Some(token) = self.tokens.get(pos) {
                    if let Token::IndexRegister(reg) = &token.token {
                        return *reg == 'X';
                    }
                }
            }
        }
        false
    }

    fn peek_next_is_comma_y(&self) -> bool {
        let mut pos = self.current;

        // Whitespace 토큰들을 건너뛰기
        while let Some(token) = self.tokens.get(pos) {
            if !matches!(token.token, Token::Whitespace) {
                break;
            }
            pos += 1;
        }

        // Comma 토큰 확인
        if let Some(token) = self.tokens.get(pos) {
            if matches!(&token.token, Token::Comma) {
                pos += 1;

                // Whitespace 토큰들을 건너뛰기
                while let Some(token) = self.tokens.get(pos) {
                    if !matches!(token.token, Token::Whitespace) {
                        break;
                    }
                    pos += 1;
                }

                // Y 레지스터 토큰 확인
                if let Some(token) = self.tokens.get(pos) {
                    if let Token::IndexRegister(reg) = &token.token {
                        return *reg == 'Y';
                    }
                }
            }
        }
        false
    }

    fn peek(&self) -> Option<&TokenInfo> {
        self.tokens.get(self.current)
    }

    fn peek_next(&self) -> Option<&TokenInfo> {
        self.tokens.get(self.current + 1)
    }

    fn peek_next_next(&self) -> Option<&TokenInfo> {
        self.tokens.get(self.current + 2)
    }

    fn advance(&mut self) {
        self.current += 1;
    }

    fn is_at_end(&self) -> bool {
        self.current >= self.tokens.len()
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::lexer::Token;

    // 헬퍼 함수: 토큰 생성
    fn create_token(token: Token, line: usize, column: usize) -> TokenInfo {
        TokenInfo {
            token,
            line,
            column,
        }
    }

    #[test]
    fn test_parse_lda_immediate() {
        let tokens = vec![
            create_token(Token::Mnemonic("LDA".to_string()), 1, 1),
            create_token(Token::Hash, 1, 5),
            create_token(Token::HexNumber(0x42), 1, 6),
        ];

        let mut parser = Parser::new(tokens);
        let instructions = parser.parse().unwrap();

        assert_eq!(instructions.len(), 1);
        match instructions[0] {
            Instruction::LDA(mode) => assert_eq!(mode, AddressMode::Immediate),
            _ => panic!("Expected LDA instruction"),
        }
    }

    #[test]
    fn test_parse_lda_zero_page() {
        let tokens = vec![
            create_token(Token::Mnemonic("LDA".to_string()), 1, 1),
            create_token(Token::HexNumber(0x42), 1, 5),
        ];

        let mut parser = Parser::new(tokens);
        let instructions = parser.parse().unwrap();

        assert_eq!(instructions.len(), 1);
        match instructions[0] {
            Instruction::LDA(mode) => assert_eq!(mode, AddressMode::ZeroPage),
            _ => panic!("Expected LDA instruction"),
        }
    }

    #[test]
    fn test_parse_lda_zero_page_x() {
        let tokens = vec![
            create_token(Token::Mnemonic("LDA".to_string()), 1, 1),
            create_token(Token::HexNumber(0x42), 1, 5),
            create_token(Token::Comma, 1, 7),
            create_token(Token::IndexRegister('X'), 1, 8),
        ];

        let mut parser = Parser::new(tokens);
        let instructions = parser.parse().unwrap();

        assert_eq!(instructions.len(), 1);
        match instructions[0] {
            Instruction::LDA(mode) => assert_eq!(mode, AddressMode::ZeroPageX),
            _ => panic!("Expected LDA instruction"),
        }
    }

    #[test]
    fn test_parse_lda_indirect_x() {
        let tokens = vec![
            create_token(Token::Mnemonic("LDA".to_string()), 1, 1),
            create_token(Token::LeftParen, 1, 5),
            create_token(Token::HexNumber(0x20), 1, 6),
            create_token(Token::Comma, 1, 8),
            create_token(Token::IndexRegister('X'), 1, 9),
            create_token(Token::RightParen, 1, 10),
        ];

        let mut parser = Parser::new(tokens);
        let instructions = parser.parse().unwrap();

        assert_eq!(instructions.len(), 1);
        match instructions[0] {
            Instruction::LDA(mode) => assert_eq!(mode, AddressMode::IndirectX),
            _ => panic!("Expected LDA instruction"),
        }
    }

    #[test]
    fn test_parse_lda_indirect_y() {
        let tokens = vec![
            create_token(Token::Mnemonic("LDA".to_string()), 1, 1),
            create_token(Token::LeftParen, 1, 5),
            create_token(Token::HexNumber(0x20), 1, 6),
            create_token(Token::RightParen, 1, 8),
            create_token(Token::Comma, 1, 9),
            create_token(Token::IndexRegister('Y'), 1, 10),
        ];

        let mut parser = Parser::new(tokens);
        let instructions = parser.parse().unwrap();

        assert_eq!(instructions.len(), 1);
        match instructions[0] {
            Instruction::LDA(mode) => assert_eq!(mode, AddressMode::IndirectY),
            _ => panic!("Expected LDA instruction"),
        }
    }

    #[test]
    fn test_parse_single_byte_instructions() {
        let test_cases = vec![
            ("TAX", Instruction::TAX),
            ("TXA", Instruction::TXA),
            ("TAY", Instruction::TAY),
            ("TYA", Instruction::TYA),
            ("TSX", Instruction::TSX),
            ("TXS", Instruction::TXS),
            ("PHA", Instruction::PHA),
            ("PLA", Instruction::PLA),
            ("PHP", Instruction::PHP),
            ("PLP", Instruction::PLP),
        ];

        for (mnemonic, _expected_instruction) in test_cases {
            let tokens = vec![create_token(Token::Mnemonic(mnemonic.to_string()), 1, 1)];

            let mut parser = Parser::new(tokens);
            let instructions = parser.parse().unwrap();

            assert_eq!(instructions.len(), 1);
            assert!(matches!(instructions[0], _expected_instruction));
        }
    }

    #[test]
    fn test_parse_store_instructions() {
        // STA 테스트
        let tokens = vec![
            create_token(Token::Mnemonic("STA".to_string()), 1, 1),
            create_token(Token::HexNumber(0x42), 1, 5),
        ];
        let mut parser = Parser::new(tokens);
        let instructions = parser.parse().unwrap();
        assert_eq!(instructions.len(), 1);
        assert!(matches!(
            instructions[0],
            Instruction::STA(AddressMode::ZeroPage)
        ));

        // STX 테스트
        let tokens = vec![
            create_token(Token::Mnemonic("STX".to_string()), 1, 1),
            create_token(Token::HexNumber(0x42), 1, 5),
        ];
        let mut parser = Parser::new(tokens);
        let instructions = parser.parse().unwrap();
        assert_eq!(instructions.len(), 1);
        assert!(matches!(
            instructions[0],
            Instruction::STX(AddressMode::ZeroPage)
        ));

        // STY 테스트
        let tokens = vec![
            create_token(Token::Mnemonic("STY".to_string()), 1, 1),
            create_token(Token::HexNumber(0x42), 1, 5),
        ];
        let mut parser = Parser::new(tokens);
        let instructions = parser.parse().unwrap();
        assert_eq!(instructions.len(), 1);
        assert!(matches!(
            instructions[0],
            Instruction::STY(AddressMode::ZeroPage)
        ));

        // STA Zero Page,X 테스트
        let tokens = vec![
            create_token(Token::Mnemonic("STA".to_string()), 1, 1),
            create_token(Token::HexNumber(0x42), 1, 5),
            create_token(Token::Comma, 1, 7),
            create_token(Token::IndexRegister('X'), 1, 8),
        ];
        let mut parser = Parser::new(tokens);
        let instructions = parser.parse().unwrap();
        assert_eq!(instructions.len(), 1);
        assert!(matches!(
            instructions[0],
            Instruction::STA(AddressMode::ZeroPageX)
        ));

        // STX Zero Page,X 테스트
        let tokens = vec![
            create_token(Token::Mnemonic("STX".to_string()), 1, 1),
            create_token(Token::HexNumber(0x42), 1, 5),
            create_token(Token::Comma, 1, 7),
            create_token(Token::IndexRegister('X'), 1, 8),
        ];
        let mut parser = Parser::new(tokens);
        let instructions = parser.parse().unwrap();
        assert_eq!(instructions.len(), 1);
        assert!(matches!(
            instructions[0],
            Instruction::STX(AddressMode::ZeroPageX)
        ));
    }

    #[test]
    fn test_parse_multiple_instructions() {
        let tokens = vec![
            // LDA #$42
            create_token(Token::Mnemonic("LDA".to_string()), 1, 1),
            create_token(Token::Hash, 1, 5),
            create_token(Token::HexNumber(0x42), 1, 6),
            // TAX
            create_token(Token::Mnemonic("TAX".to_string()), 2, 1),
            // STA $20
            create_token(Token::Mnemonic("STA".to_string()), 3, 1),
            create_token(Token::HexNumber(0x20), 3, 5),
        ];

        let mut parser = Parser::new(tokens);
        let instructions = parser.parse().unwrap();

        assert_eq!(instructions.len(), 3);
        assert!(matches!(
            instructions[0],
            Instruction::LDA(AddressMode::Immediate)
        ));
        assert!(matches!(instructions[1], Instruction::TAX));
        assert!(matches!(
            instructions[2],
            Instruction::STA(AddressMode::ZeroPage)
        ));
    }
}
