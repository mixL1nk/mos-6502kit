use crate::lexer::{Token, TokenInfo};
use common::Result;
use error::Error;
use types::{AddressModeValue, Instruction};

// TODO: 이 방식은 분명 개선이 가능할것 같다. 너무 복잡하고 비효율적인 부분이 많다.
pub struct Parser {
    tokens: Vec<TokenInfo>,
    current: usize,
    labels: std::collections::HashMap<String, u16>,
    current_address: u16,
    org: u16,
}

impl Parser {
    pub fn new(tokens: Vec<TokenInfo>) -> Self {
        Parser {
            tokens,
            current: 0,
            labels: std::collections::HashMap::new(),
            current_address: 0,
            org: 0,
        }
    }

    pub fn set_org(&mut self, org: u16) {
        self.org = org;
    }

    // First pass: collect all labels
    fn collect_labels(&mut self) -> Result<()> {
        let mut current_address = 0u16;
        let mut i = 0;

        while i < self.tokens.len() {
            match &self.tokens[i].token {
                Token::Label(name) => {
                    // Skip collecting 'A' as a label if it follows a shift/rotate instruction
                    let is_accumulator_mode = if i > 0 {
                        match &self.tokens[i - 1].token {
                            Token::Mnemonic(mnemonic) => {
                                let is_shift =
                                    matches!(mnemonic.as_str(), "LSR" | "ASL" | "ROL" | "ROR");
                                is_shift && name == "A"
                            }
                            _ => false,
                        }
                    } else {
                        false
                    };

                    if !is_accumulator_mode {
                        if !self.labels.contains_key(name) {
                            println!(
                                "[DEBUG] Found label '{}' at address ${:04X}",
                                name, current_address
                            );
                            self.labels.insert(name.clone(), current_address);
                        } else {
                            println!(
                                "[DEBUG] Ignoring duplicate label '{}' at address ${:04X} (already defined)",
                                name, current_address
                            );
                        }
                    } else {
                        println!("[DEBUG] Skipping 'A' as it's used in accumulator mode");
                    }
                    i += 1;
                }
                Token::Mnemonic(mnemonic) => {
                    // 명령어의 크기를 고려하여 주소 증가
                    let size = match mnemonic.as_str() {
                        // 2바이트 명령어 (opcode + operand)
                        "BCC" | "BCS" | "BEQ" | "BNE" | "BMI" | "BPL" | "BVC" | "BVS" | "LDA"
                        | "LDX" | "LDY" | "STA" | "STX" | "STY" | "ADC" | "SBC" | "CMP" | "CPX"
                        | "CPY" | "AND" | "ORA" | "EOR" | "BIT" | "INC" | "DEC" => 2,
                        // 시프트/회전 명령어는 다음 토큰이 'A'인 경우 1바이트, 아니면 2바이트
                        "ASL" | "LSR" | "ROL" | "ROR" => {
                            if i + 1 < self.tokens.len() {
                                match &self.tokens[i + 1].token {
                                    Token::Label(name) if name == "A" => 1,
                                    Token::IndexRegister('A') => 1,
                                    _ => 2,
                                }
                            } else {
                                2
                            }
                        }
                        // 3바이트 명령어 (opcode + 2byte address)
                        "JMP" | "JSR" => 3,
                        // 1바이트 명령어 (암시적 어드레싱)
                        _ => 1,
                    };
                    println!(
                        "[DEBUG] Found instruction '{}' size: {} at address ${:04X}",
                        mnemonic, size, current_address
                    );
                    current_address = current_address.wrapping_add(size);
                    i += 1;
                }
                Token::Directive(dir) if dir == ".ORG" => {
                    i += 1; // .ORG 토큰 건너뛰기
                    if let Some(TokenInfo {
                        token: Token::HexNumber(addr),
                        ..
                    }) = self.tokens.get(i)
                    {
                        self.org = *addr;
                        current_address = 0; // .ORG를 만나면 현재 주소를 0으로 리셋
                        println!(
                            "[DEBUG] Found .ORG directive, setting org to ${:04X}",
                            self.org
                        );
                    }
                    i += 1;
                }
                _ => i += 1,
            }
        }

        println!("[DEBUG] Label addresses (relative):");
        for (label, addr) in &self.labels {
            println!("[DEBUG]   {}: ${:04X}", label, addr);
        }

        Ok(())
    }

    pub fn parse(&mut self) -> Result<Vec<Instruction>> {
        // First pass: collect labels
        self.collect_labels()?;

        // Reset position and address for second pass
        self.current = 0;
        self.current_address = 0;
        let mut instructions = Vec::new();

        while !self.is_at_end() {
            if let Some(instruction) = self.parse_instruction()? {
                // Update current_address based on instruction size
                let size = match &instruction {
                    Instruction::JMP(_) | Instruction::JSR(_) => 3,
                    Instruction::BCC(_)
                    | Instruction::BCS(_)
                    | Instruction::BEQ(_)
                    | Instruction::BNE(_)
                    | Instruction::BMI(_)
                    | Instruction::BPL(_)
                    | Instruction::BVC(_)
                    | Instruction::BVS(_)
                    | Instruction::LDA(_)
                    | Instruction::LDX(_)
                    | Instruction::LDY(_)
                    | Instruction::STA(_)
                    | Instruction::STX(_)
                    | Instruction::STY(_)
                    | Instruction::ADC(_)
                    | Instruction::SBC(_)
                    | Instruction::CMP(_)
                    | Instruction::CPX(_)
                    | Instruction::CPY(_)
                    | Instruction::AND(_)
                    | Instruction::ORA(_)
                    | Instruction::EOR(_)
                    | Instruction::BIT(_)
                    | Instruction::ASL(_)
                    | Instruction::LSR(_)
                    | Instruction::ROL(_)
                    | Instruction::ROR(_)
                    | Instruction::INC(_)
                    | Instruction::DEC(_) => 2,
                    _ => 1,
                };
                println!(
                    "[DEBUG] Instruction at ${:04X}: {:?} (size: {})",
                    self.current_address + self.org,
                    instruction,
                    size
                );
                self.current_address = self.current_address.wrapping_add(size);
                instructions.push(instruction);
            }
        }

        Ok(instructions)
    }

    fn parse_instruction(&mut self) -> Result<Option<Instruction>> {
        let token = match self.peek() {
            Some(token) => token,
            None => return Ok(None),
        };

        println!("[DEBUG] Parsing instruction: {:?}", token);

        match &token.token {
            Token::EOL | Token::Whitespace | Token::Comment(_) => {
                self.advance();
                Ok(None)
            }
            Token::Label(_) | Token::Colon => {
                // 레이블이나 콜론을 만나면 다음 토큰으로 진행
                self.advance();
                Ok(None)
            }
            Token::IndexRegister(_)
            | Token::Hash
            | Token::Comma
            | Token::LeftParen
            | Token::RightParen
            | Token::Plus
            | Token::Minus
            | Token::Multiply
            | Token::Divide
            | Token::LeftBracket
            | Token::RightBracket
            | Token::Expression(_)
            | Token::Number(_)
            | Token::HexNumber(_)
            | Token::BinaryNumber(_)
            | Token::HighByte
            | Token::LowByte
            | Token::MacroName(_)
            | Token::MacroParam(_)
            | Token::Dot
            | Token::At => {
                // 다른 모든 토큰들도 다음으로 진행
                self.advance();
                Ok(None)
            }
            Token::Directive(directive) => {
                let directive_str = directive.clone();
                self.advance();
                match directive_str.as_str() {
                    ".ORG" => {
                        self.skip_whitespace();
                        let addr = if let Some(TokenInfo {
                            token: Token::HexNumber(addr),
                            ..
                        }) = self.peek()
                        {
                            let addr_val = *addr;
                            self.advance();
                            self.org = addr_val;
                            Ok(None)
                        } else {
                            Err(Error::InvalidDirectiveOperand(".ORG"))
                        };
                        addr
                    }
                    _ => Ok(None),
                }
            }
            Token::Mnemonic(mnemonic) => {
                match mnemonic.as_str() {
                    // 암묵적(Implied) 어드레싱 모드 명령어 처리
                    "TAX" => self.parse_implicit_instruction(Instruction::TAX),
                    "TXA" => self.parse_implicit_instruction(Instruction::TXA),
                    "TAY" => self.parse_implicit_instruction(Instruction::TAY),
                    "TYA" => self.parse_implicit_instruction(Instruction::TYA),
                    "TSX" => self.parse_implicit_instruction(Instruction::TSX),
                    "TXS" => self.parse_implicit_instruction(Instruction::TXS),
                    "PHA" => self.parse_implicit_instruction(Instruction::PHA),
                    "PLA" => self.parse_implicit_instruction(Instruction::PLA),
                    "PHP" => self.parse_implicit_instruction(Instruction::PHP),
                    "PLP" => self.parse_implicit_instruction(Instruction::PLP),
                    "INX" => self.parse_implicit_instruction(Instruction::INX),
                    "INY" => self.parse_implicit_instruction(Instruction::INY),
                    "DEX" => self.parse_implicit_instruction(Instruction::DEX),
                    "DEY" => self.parse_implicit_instruction(Instruction::DEY),
                    "CLC" => self.parse_implicit_instruction(Instruction::CLC),
                    "SEC" => self.parse_implicit_instruction(Instruction::SEC),
                    "CLI" => self.parse_implicit_instruction(Instruction::CLI),
                    "SEI" => self.parse_implicit_instruction(Instruction::SEI),
                    "CLV" => self.parse_implicit_instruction(Instruction::CLV),
                    "CLD" => self.parse_implicit_instruction(Instruction::CLD),
                    "SED" => self.parse_implicit_instruction(Instruction::SED),
                    "NOP" => self.parse_implicit_instruction(Instruction::NOP),
                    "BRK" => self.parse_implicit_instruction(Instruction::BRK),
                    "RTI" => self.parse_implicit_instruction(Instruction::RTI),
                    "RTS" => self.parse_implicit_instruction(Instruction::RTS),

                    // 분기 명령어 처리
                    "BCC" => self.parse_branch_instruction(Instruction::BCC),
                    "BCS" => self.parse_branch_instruction(Instruction::BCS),
                    "BEQ" => self.parse_branch_instruction(Instruction::BEQ),
                    "BNE" => self.parse_branch_instruction(Instruction::BNE),
                    "BMI" => self.parse_branch_instruction(Instruction::BMI),
                    "BPL" => self.parse_branch_instruction(Instruction::BPL),
                    "BVC" => self.parse_branch_instruction(Instruction::BVC),
                    "BVS" => self.parse_branch_instruction(Instruction::BVS),

                    // 일반 명령어 처리 (어드레싱 모드 필요)
                    "LDA" => self.parse_addressing_mode_instruction(|mode: AddressModeValue| {
                        Ok(Instruction::LDA(mode))
                    }),
                    "LDX" => self.parse_addressing_mode_instruction(|mode: AddressModeValue| {
                        Ok(Instruction::LDX(mode))
                    }),
                    "LDY" => self.parse_addressing_mode_instruction(|mode: AddressModeValue| {
                        Ok(Instruction::LDY(mode))
                    }),
                    "STA" => self.parse_addressing_mode_instruction(|mode: AddressModeValue| {
                        Ok(Instruction::STA(mode))
                    }),
                    "STX" => self.parse_addressing_mode_instruction(|mode: AddressModeValue| {
                        Ok(Instruction::STX(mode))
                    }),
                    "STY" => self.parse_addressing_mode_instruction(|mode: AddressModeValue| {
                        Ok(Instruction::STY(mode))
                    }),

                    // 산술/논리 연산
                    "ADC" => self.parse_addressing_mode_instruction(|mode: AddressModeValue| {
                        Ok(Instruction::ADC(mode))
                    }),
                    "SBC" => self.parse_addressing_mode_instruction(|mode: AddressModeValue| {
                        Ok(Instruction::SBC(mode))
                    }),
                    "AND" => self.parse_addressing_mode_instruction(|mode: AddressModeValue| {
                        Ok(Instruction::AND(mode))
                    }),
                    "ORA" => self.parse_addressing_mode_instruction(|mode: AddressModeValue| {
                        Ok(Instruction::ORA(mode))
                    }),
                    "EOR" => self.parse_addressing_mode_instruction(|mode: AddressModeValue| {
                        Ok(Instruction::EOR(mode))
                    }),
                    "CMP" => self.parse_addressing_mode_instruction(|mode: AddressModeValue| {
                        Ok(Instruction::CMP(mode))
                    }),
                    "CPX" => {
                        self.parse_addressing_mode_instruction(
                            |mode: AddressModeValue| match mode {
                                AddressModeValue::Immediate(_)
                                | AddressModeValue::ZeroPage(_)
                                | AddressModeValue::Absolute(_) => Ok(Instruction::CPX(mode)),
                                _ => Err(Error::InvalidAddressingMode("CPX")),
                            },
                        )
                    }
                    "CPY" => {
                        self.parse_addressing_mode_instruction(
                            |mode: AddressModeValue| match mode {
                                AddressModeValue::Immediate(_)
                                | AddressModeValue::ZeroPage(_)
                                | AddressModeValue::Absolute(_) => Ok(Instruction::CPY(mode)),
                                _ => Err(Error::InvalidAddressingMode("CPY")),
                            },
                        )
                    }

                    // 메모리/레지스터 증감
                    "INC" => self.parse_addressing_mode_instruction(|mode: AddressModeValue| {
                        Ok(Instruction::INC(mode))
                    }),
                    "DEC" => self.parse_addressing_mode_instruction(|mode: AddressModeValue| {
                        Ok(Instruction::DEC(mode))
                    }),

                    // 시프트/회전 명령어
                    "ASL" => self.parse_addressing_mode_instruction(|mode: AddressModeValue| {
                        Ok(Instruction::ASL(mode))
                    }),
                    "LSR" => self.parse_addressing_mode_instruction(|mode: AddressModeValue| {
                        Ok(Instruction::LSR(mode))
                    }),
                    "ROL" => self.parse_addressing_mode_instruction(|mode: AddressModeValue| {
                        Ok(Instruction::ROL(mode))
                    }),
                    "ROR" => self.parse_addressing_mode_instruction(|mode: AddressModeValue| {
                        Ok(Instruction::ROR(mode))
                    }),

                    // 비트 테스트
                    "BIT" => self.parse_addressing_mode_instruction(|mode: AddressModeValue| {
                        Ok(Instruction::BIT(mode))
                    }),

                    // 점프 명령어
                    "JMP" => {
                        self.advance(); // JMP 토큰 소비
                        let mode = self.parse_addressing_mode()?;
                        match mode {
                            AddressModeValue::Absolute(_) | AddressModeValue::Indirect(_) => {
                                Ok(Some(Instruction::JMP(mode)))
                            }
                            _ => Err(Error::InvalidAddressingMode("JMP")),
                        }
                    }
                    "JSR" => {
                        self.advance(); // JSR 토큰 소비
                        let mode = self.parse_addressing_mode()?;
                        match mode {
                            AddressModeValue::Absolute(_) => Ok(Some(Instruction::JSR(mode))),
                            _ => Err(Error::InvalidAddressingMode("JSR")),
                        }
                    }

                    // 기타 명령어들은 추후 구현
                    _ => Ok(None),
                }
            }
        }
    }

    // 암시적(Implied) 어드레싱 모드를 사용하는 명령어 파싱 헬퍼 함수
    fn parse_implicit_instruction(
        &mut self,
        instruction: Instruction,
    ) -> Result<Option<Instruction>> {
        self.advance(); // 토큰 소비
        // 필요한 추가 처리
        Ok(Some(instruction))
    }

    // 분기 명령어 파싱 헬퍼 함수
    fn parse_branch_instruction<F>(&mut self, create_instruction: F) -> Result<Option<Instruction>>
    where
        F: FnOnce(i8) -> Instruction,
    {
        self.advance(); // 명령어 토큰을 건너뜀
        let offset = self.parse_branch_offset()?;
        Ok(Some(create_instruction(offset)))
    }

    // 어드레싱 모드가 필요한 명령어 파싱 헬퍼 함수
    fn parse_addressing_mode_instruction<F>(
        &mut self,
        create_instruction: F,
    ) -> Result<Option<Instruction>>
    where
        F: FnOnce(AddressModeValue) -> Result<Instruction>,
    {
        let mode = self.parse_addressing_mode()?;
        create_instruction(mode).map(Some)
    }

    fn parse_addressing_mode(&mut self) -> Result<AddressModeValue> {
        self.advance(); // 명령어 토큰을 건너뜀
        self.skip_whitespace(); // 공백 건너뛰기

        let token = self.peek().cloned();
        match token {
            Some(TokenInfo { token, .. }) => match token {
                Token::IndexRegister('A') => {
                    println!("[DEBUG] Found accumulator mode (A register)");
                    self.advance(); // A 토큰 소비
                    Ok(AddressModeValue::Accumulator)
                }
                Token::Label(name) if name == "A" => {
                    println!("[DEBUG] Found accumulator mode (A label)");
                    self.advance(); // A 토큰 소비
                    Ok(AddressModeValue::Accumulator)
                }
                Token::Hash => self.parse_immediate_mode(),
                Token::LeftParen => self.parse_indirect_mode(),
                Token::HexNumber(n) => self.parse_absolute_or_zeropage_mode(n),
                Token::Number(n) => self.parse_absolute_or_zeropage_mode(n),
                Token::Label(name) => {
                    // 레이블을 주소로 변환
                    if let Some(&addr) = self.labels.get(&name) {
                        self.advance(); // 레이블 토큰 소비
                        self.parse_absolute_or_zeropage_mode(addr)
                    } else {
                        Err(Error::UndefinedLabel(name))
                    }
                }
                _ => Err(Error::InvalidAddressingMode("Unexpected token")),
            },
            None => Err(Error::UnexpectedEndOfInput),
        }
    }

    fn parse_immediate_mode(&mut self) -> Result<AddressModeValue> {
        println!("[DEBUG] Entering parse_immediate_mode");
        println!("[DEBUG] Current token: {:?}", self.peek());

        self.advance(); // # 토큰 건너뛰기
        self.skip_whitespace();

        println!(
            "[DEBUG] After advancing and skipping whitespace: {:?}",
            self.peek()
        );

        let value = match self.peek() {
            Some(TokenInfo {
                token: Token::HexNumber(n),
                ..
            }) => {
                println!("[DEBUG] Found hex number: {:x}", n);
                *n as u8
            }
            Some(TokenInfo {
                token: Token::Number(n),
                ..
            }) => {
                println!("[DEBUG] Found decimal number: {}", n);
                *n as u8
            }
            _ => {
                println!("[DEBUG] No number found, using 0");
                0
            }
        };

        // 숫자 토큰이 있었다면 건너뛰기
        if matches!(
            self.peek(),
            Some(TokenInfo {
                token: Token::HexNumber(_) | Token::Number(_),
                ..
            })
        ) {
            println!("[DEBUG] Found number token, advancing");
            self.advance();
        }

        println!(
            "[DEBUG] Final token after immediate mode parsing: {:?}",
            self.peek()
        );
        println!("[DEBUG] Immediate value: {}", value);
        Ok(AddressModeValue::Immediate(value))
    }

    fn parse_indirect_mode(&mut self) -> Result<AddressModeValue> {
        self.advance(); // ( 토큰 건너뛰기
        self.skip_whitespace();

        // 주소 파싱
        let value = match self.peek() {
            Some(TokenInfo {
                token: Token::HexNumber(n),
                ..
            }) => *n,
            Some(TokenInfo {
                token: Token::Number(n),
                ..
            }) => *n,
            _ => 0,
        };

        if matches!(
            self.peek(),
            Some(TokenInfo {
                token: Token::HexNumber(_) | Token::Number(_),
                ..
            })
        ) {
            self.advance(); // 숫자 건너뛰기
        }

        self.skip_whitespace();

        match self.peek() {
            Some(TokenInfo {
                token: Token::Comma,
                ..
            }) => {
                // (addr,X) 형식 -> IndirectX
                self.advance(); // , 건너뛰기
                self.skip_whitespace();

                // X 레지스터 확인
                if let Some(TokenInfo {
                    token: Token::IndexRegister('X'),
                    ..
                }) = self.peek()
                {
                    self.advance(); // X 건너뛰기
                    self.skip_whitespace();

                    // ) 확인
                    if let Some(TokenInfo {
                        token: Token::RightParen,
                        ..
                    }) = self.peek()
                    {
                        self.advance(); // ) 건너뛰기
                        return Ok(AddressModeValue::IndirectX(value as u8));
                    }
                }

                Ok(AddressModeValue::IndirectX(value as u8))
            }
            Some(TokenInfo {
                token: Token::RightParen,
                ..
            }) => {
                // (addr) 형식 -> Indirect 또는 IndirectY
                self.advance(); // ) 건너뛰기
                self.skip_whitespace();

                // 콤마와 Y 확인
                if let Some(TokenInfo {
                    token: Token::Comma,
                    ..
                }) = self.peek()
                {
                    self.advance(); // , 건너뛰기
                    self.skip_whitespace();

                    if let Some(TokenInfo {
                        token: Token::IndexRegister('Y'),
                        ..
                    }) = self.peek()
                    {
                        self.advance(); // Y 건너뛰기
                        return Ok(AddressModeValue::IndirectY(value as u8));
                    }
                }

                Ok(AddressModeValue::Indirect(value))
            }
            _ => Ok(AddressModeValue::Indirect(value)),
        }
    }

    fn parse_absolute_or_zeropage_mode(&mut self, value: u16) -> Result<AddressModeValue> {
        self.advance(); // 숫자 건너뛰기
        self.skip_whitespace();

        // 콤마가 있는지 확인 (인덱싱 모드)
        if let Some(TokenInfo {
            token: Token::Comma,
            ..
        }) = self.peek()
        {
            self.advance(); // , 건너뛰기
            self.skip_whitespace();

            // X 또는 Y 레지스터 확인
            match self.peek() {
                Some(TokenInfo {
                    token: Token::IndexRegister('X'),
                    ..
                }) => {
                    self.advance(); // X 건너뛰기
                    if value <= 0xFF {
                        Ok(AddressModeValue::ZeroPageX(value as u8))
                    } else {
                        Ok(AddressModeValue::AbsoluteX(value))
                    }
                }
                Some(TokenInfo {
                    token: Token::IndexRegister('Y'),
                    ..
                }) => {
                    self.advance(); // Y 건너뛰기
                    if value <= 0xFF {
                        Ok(AddressModeValue::ZeroPageY(value as u8))
                    } else {
                        Ok(AddressModeValue::AbsoluteY(value))
                    }
                }
                _ => {
                    if value <= 0xFF {
                        Ok(AddressModeValue::ZeroPage(value as u8))
                    } else {
                        Ok(AddressModeValue::Absolute(value))
                    }
                }
            }
        } else {
            // 인덱싱 없는 모드
            if value <= 0xFF {
                Ok(AddressModeValue::ZeroPage(value as u8))
            } else {
                Ok(AddressModeValue::Absolute(value))
            }
        }
    }

    fn parse_branch_offset(&mut self) -> Result<i8> {
        self.skip_whitespace();

        let token = self.peek().cloned();
        match token {
            Some(TokenInfo {
                token: Token::Label(label),
                ..
            }) => {
                self.advance();
                if let Some(&target_address) = self.labels.get(&label) {
                    // 현재 명령어의 주소
                    let current_pc = self.current_address;
                    // 분기 명령어는 2바이트이므로, 다음 명령어의 주소는 +2
                    let next_pc = current_pc + 2;

                    // 상대 주소 계산 (target - next_pc)
                    let offset = target_address as i32 - next_pc as i32;

                    println!("[DEBUG] Branch offset calculation:");
                    println!("[DEBUG] Current PC: ${:04X}", current_pc + self.org);
                    println!("[DEBUG] Next PC: ${:04X}", next_pc + self.org);
                    println!("[DEBUG] Target address: ${:04X}", target_address);
                    println!("[DEBUG] Calculated offset: {}", offset);

                    // 오프셋이 분기 명령어의 범위(-128 ~ +127)를 벗어나는지 확인
                    if !(-128..=127).contains(&offset) {
                        return Err(Error::BranchOutOfRange(label));
                    }

                    Ok(offset as i8)
                } else {
                    Err(Error::UndefinedLabel(label))
                }
            }
            Some(TokenInfo {
                token: Token::HexNumber(n),
                ..
            }) => {
                self.advance();
                Ok(n as i8)
            }
            Some(TokenInfo {
                token: Token::Number(n),
                ..
            }) => {
                self.advance();
                Ok(n as i8)
            }
            _ => Ok(0),
        }
    }

    // 공백 토큰 건너뛰기 헬퍼 함수
    fn skip_whitespace(&mut self) {
        while let Some(TokenInfo {
            token: Token::Whitespace,
            ..
        }) = self.peek()
        {
            self.advance();
        }
    }

    fn peek(&self) -> Option<&TokenInfo> {
        self.tokens.get(self.current)
    }

    fn advance(&mut self) {
        self.current += 1;
    }

    fn is_at_end(&self) -> bool {
        self.current >= self.tokens.len()
    }
}
