use crate::lexer::{Token, TokenInfo};
use crate::parser::addressing::AddressingModeParser;
use crate::parser::instruction::{
    ADCStrategy, ANDStrategy, ASLStrategy, BNEStrategy, CMPStrategy, CPXStrategy, CPYStrategy,
    EORStrategy, InstructionParser, InstructionStrategy, LDAStrategy, LDXStrategy, LDYStrategy,
    LSRStrategy, ORAStrategy, STAStrategy, STXStrategy, STYStrategy, SingleByteStrategy,
};
use common::Result;
use error::Error;
use std::collections::HashMap;
use types::{AddressModeValue, Instruction};

pub struct Parser {
    tokens: Vec<TokenInfo>,
    current: usize,
    instruction_parser: InstructionParser,
    addressing_parser: AddressingModeParser,
    org: u16,
    current_address: u16,
    instruction_strategies: HashMap<&'static str, Box<dyn InstructionStrategy>>,
}
//TODO: 로직이 너무 복잡해졌다.
impl Parser {
    pub fn new(tokens: Vec<TokenInfo>) -> Self {
        let labels = HashMap::new();
        let addressing_parser = AddressingModeParser::new(labels);
        let instruction_parser = InstructionParser::new();

        let mut parser = Self {
            tokens,
            current: 0,
            instruction_parser,
            addressing_parser,
            org: 0x8000,             // 기본값
            current_address: 0x8000, // 기본값
            instruction_strategies: HashMap::new(),
        };

        parser.register_strategies();
        parser
    }

    pub fn set_org(&mut self, org: u16) {
        self.org = org;
        self.current_address = org;
        self.instruction_parser.set_current_address(org);
    }

    pub fn get_org(&self) -> u16 {
        self.org
    }

    fn register_strategies(&mut self) {
        // 로드 명령어
        self.instruction_strategies
            .insert("LDA", Box::new(LDAStrategy));
        self.instruction_strategies
            .insert("LDX", Box::new(LDXStrategy));
        self.instruction_strategies
            .insert("LDY", Box::new(LDYStrategy));

        // 스토어 명령어
        self.instruction_strategies
            .insert("STA", Box::new(STAStrategy));
        self.instruction_strategies
            .insert("STX", Box::new(STXStrategy));
        self.instruction_strategies
            .insert("STY", Box::new(STYStrategy));

        // 시프트 명령어
        self.instruction_strategies
            .insert("LSR", Box::new(LSRStrategy));
        self.instruction_strategies
            .insert("ASL", Box::new(ASLStrategy));

        // 산술 연산 명령어
        self.instruction_strategies
            .insert("ADC", Box::new(ADCStrategy));

        // 논리 연산 명령어
        self.instruction_strategies
            .insert("AND", Box::new(ANDStrategy));
        self.instruction_strategies
            .insert("ORA", Box::new(ORAStrategy));
        self.instruction_strategies
            .insert("EOR", Box::new(EORStrategy));

        // 비교 명령어
        self.instruction_strategies
            .insert("CMP", Box::new(CMPStrategy));
        self.instruction_strategies
            .insert("CPY", Box::new(CPYStrategy));
        self.instruction_strategies
            .insert("CPX", Box::new(CPXStrategy));

        // 분기 명령어
        self.instruction_strategies
            .insert("BNE", Box::new(BNEStrategy));

        // 단일 바이트 명령어
        self.instruction_strategies
            .insert("TAX", Box::new(SingleByteStrategy("TAX")));
        self.instruction_strategies
            .insert("TXA", Box::new(SingleByteStrategy("TXA")));
        self.instruction_strategies
            .insert("TAY", Box::new(SingleByteStrategy("TAY")));
        self.instruction_strategies
            .insert("TYA", Box::new(SingleByteStrategy("TYA")));
        self.instruction_strategies
            .insert("INX", Box::new(SingleByteStrategy("INX")));
        self.instruction_strategies
            .insert("INY", Box::new(SingleByteStrategy("INY")));
        self.instruction_strategies
            .insert("DEX", Box::new(SingleByteStrategy("DEX")));
        self.instruction_strategies
            .insert("DEY", Box::new(SingleByteStrategy("DEY")));
        self.instruction_strategies
            .insert("CLC", Box::new(SingleByteStrategy("CLC")));
        self.instruction_strategies
            .insert("SEC", Box::new(SingleByteStrategy("SEC")));
        self.instruction_strategies
            .insert("CLI", Box::new(SingleByteStrategy("CLI")));
        self.instruction_strategies
            .insert("SEI", Box::new(SingleByteStrategy("SEI")));
        self.instruction_strategies
            .insert("CLV", Box::new(SingleByteStrategy("CLV")));
        self.instruction_strategies
            .insert("CLD", Box::new(SingleByteStrategy("CLD")));
        self.instruction_strategies
            .insert("SED", Box::new(SingleByteStrategy("SED")));
        self.instruction_strategies
            .insert("NOP", Box::new(SingleByteStrategy("NOP")));
        self.instruction_strategies
            .insert("BRK", Box::new(SingleByteStrategy("BRK")));
        self.instruction_strategies
            .insert("RTI", Box::new(SingleByteStrategy("RTI")));
        self.instruction_strategies
            .insert("RTS", Box::new(SingleByteStrategy("RTS")));
        self.instruction_strategies
            .insert("PHA", Box::new(SingleByteStrategy("PHA")));
        self.instruction_strategies
            .insert("PLA", Box::new(SingleByteStrategy("PLA")));
        self.instruction_strategies
            .insert("PHP", Box::new(SingleByteStrategy("PHP")));
        self.instruction_strategies
            .insert("PLP", Box::new(SingleByteStrategy("PLP")));
        self.instruction_strategies
            .insert("TSX", Box::new(SingleByteStrategy("TSX")));
        self.instruction_strategies
            .insert("TXS", Box::new(SingleByteStrategy("TXS")));
    }

    pub fn parse(&mut self) -> Result<Vec<Instruction>> {
        // 첫 번째 패스: 라벨과 .ORG 수집
        self.collect_labels()?;

        // 두 번째 패스: 명령어 생성
        self.reset_for_second_pass();

        let mut instructions = Vec::new();
        while !self.is_at_end() {
            let token = self.peek()?.token.clone();
            match token {
                Token::Directive(dir) if dir == ".ORG" => {
                    self.advance();
                    self.skip_whitespace();

                    match self.peek()?.token {
                        Token::HexNumber(addr) | Token::Number(addr) => {
                            self.org = addr;
                            self.current_address = addr;
                            self.instruction_parser.set_current_address(addr);
                            self.advance();
                        }
                        _ => return Err(Error::AssemblerInvalidDirective(".ORG")),
                    }
                }
                Token::Label(_) | Token::Whitespace | Token::EOL | Token::Comment(_) => {
                    self.advance();
                }
                Token::Mnemonic(_) => {
                    if let Some(instruction) = self.parse_next_instruction()? {
                        instructions.push(instruction);
                    }
                }
                _ => {
                    self.advance();
                }
            }
        }

        Ok(instructions)
    }

    fn collect_labels(&mut self) -> Result<()> {
        self.current = 0;
        self.current_address = self.org;

        while !self.is_at_end() {
            let token = self.peek()?.token.clone();
            match token {
                Token::Directive(dir) if dir == ".ORG" => {
                    self.advance();
                    self.skip_whitespace();

                    match self.peek()?.token {
                        Token::HexNumber(addr) | Token::Number(addr) => {
                            println!("[DEBUG] Found .ORG directive: ${:04X}", addr);
                            self.set_org(addr);
                            self.advance();
                        }
                        _ => return Err(Error::AssemblerInvalidDirective(".ORG")),
                    }
                }
                Token::Label(name) => {
                    println!(
                        "[DEBUG] Found label '{}' at ${:04X}",
                        name, self.current_address
                    );
                    self.addressing_parser
                        .add_label(name.clone(), self.current_address);
                    self.advance();
                }
                Token::Mnemonic(ref m) => {
                    let size = match m.as_str() {
                        // 단일 바이트 명령어
                        "INX" | "INY" | "DEX" | "DEY" | "TAX" | "TXA" | "TAY" | "TYA" | "CLC"
                        | "SEC" | "CLI" | "SEI" | "CLV" | "CLD" | "SED" | "NOP" | "BRK" | "RTI"
                        | "RTS" | "PHA" | "PLA" | "PHP" | "PLP" | "TSX" | "TXS" => 1,

                        // 분기 명령어 (상대 주소 지정)
                        "BCC" | "BCS" | "BEQ" | "BNE" | "BMI" | "BPL" | "BVC" | "BVS" => 2,

                        // 다른 명령어들은 피연산자를 확인
                        _ => {
                            self.advance();
                            self.skip_whitespace();
                            match self.peek()?.token {
                                Token::Hash => 2, // 즉시 주소 지정
                                Token::HexNumber(n) | Token::Number(n) => {
                                    if n <= 0xFF {
                                        2 // 제로 페이지
                                    } else {
                                        3 // 절대 주소
                                    }
                                }
                                _ => 2, // 기본값
                            }
                        }
                    };

                    self.current_address = self.current_address.wrapping_add(size);

                    // 명령어의 피연산자 스킵
                    while !self.is_at_end() {
                        match self.peek()?.token {
                            Token::EOL | Token::Comment(_) => break,
                            _ => self.advance(),
                        }
                    }
                }
                Token::EOL | Token::Comment(_) | Token::Whitespace => {
                    self.advance();
                }
                _ => {
                    self.advance();
                }
            }
        }

        // 디버그 출력: 수집된 모든 라벨
        println!(
            "[DEBUG] Collected labels: {:?}",
            self.addressing_parser.get_labels()
        );

        Ok(())
    }

    fn parse_next_instruction(&mut self) -> Result<Option<Instruction>> {
        let token = self.peek()?.token.clone();
        match token {
            Token::Directive(dir) if dir == ".ORG" => {
                self.advance();
                self.skip_whitespace();
                match self.peek()?.token {
                    Token::HexNumber(addr) | Token::Number(addr) => {
                        self.set_org(addr);
                        self.advance();
                    }
                    _ => return Err(Error::AssemblerInvalidDirective(".ORG")),
                }
                Ok(None)
            }
            Token::Label(name) => {
                self.advance();
                self.addressing_parser.add_label(name, self.current_address);
                Ok(None)
            }
            Token::Mnemonic(m) => {
                let instruction_start = self.current_address;
                self.advance();

                // 단일 바이트 명령어 처리
                match m.as_str() {
                    "INX" | "INY" | "DEX" | "DEY" | "TAX" | "TXA" | "TAY" | "TYA" | "CLC"
                    | "SEC" | "CLI" | "SEI" | "CLV" | "CLD" | "SED" | "NOP" | "BRK" | "RTI"
                    | "RTS" | "PHA" | "PLA" | "PHP" | "PLP" | "TSX" | "TXS" => {
                        self.skip_whitespace();
                        let mode = AddressModeValue::Implied;
                        let strategy = self
                            .instruction_strategies
                            .get(m.as_str())
                            .expect("Strategy should exist for single byte instruction");
                        let instruction = strategy.parse(&self.instruction_parser, mode)?;
                        self.current_address = instruction_start.wrapping_add(1);
                        self.instruction_parser
                            .set_current_address(self.current_address);
                        return Ok(Some(instruction));
                    }
                    _ => {}
                }

                // 다른 명령어 처리
                if self.instruction_strategies.contains_key(m.as_str()) {
                    let mode = self.parse_addressing_mode()?;
                    let strategy = self
                        .instruction_strategies
                        .get(m.as_str())
                        .expect("Strategy was just checked");
                    let instruction = strategy.parse(&self.instruction_parser, mode)?;
                    let size = self.get_instruction_size(&instruction);
                    self.current_address = instruction_start.wrapping_add(size);
                    self.instruction_parser
                        .set_current_address(self.current_address);
                    Ok(Some(instruction))
                } else {
                    let instruction = self.parse_instruction_without_strategy(&m)?;
                    if let Some(ref instr) = instruction {
                        let size = self.get_instruction_size(instr);
                        self.current_address = instruction_start.wrapping_add(size);
                        self.instruction_parser
                            .set_current_address(self.current_address);
                    }
                    Ok(instruction)
                }
            }
            _ => {
                self.advance();
                Ok(None)
            }
        }
    }

    fn parse_instruction_without_strategy(
        &mut self,
        mnemonic: &str,
    ) -> Result<Option<Instruction>> {
        match mnemonic {
            // 암시적 어드레싱 모드 명령어들
            "CLC" => Ok(Some(Instruction::CLC)),
            "SEC" => Ok(Some(Instruction::SEC)),
            "CLI" => Ok(Some(Instruction::CLI)),
            "SEI" => Ok(Some(Instruction::SEI)),
            "CLV" => Ok(Some(Instruction::CLV)),
            "CLD" => Ok(Some(Instruction::CLD)),
            "SED" => Ok(Some(Instruction::SED)),
            "NOP" => Ok(Some(Instruction::NOP)),
            "BRK" => Ok(Some(Instruction::BRK)),
            "RTI" => Ok(Some(Instruction::RTI)),
            "RTS" => Ok(Some(Instruction::RTS)),
            "PHA" => Ok(Some(Instruction::PHA)),
            "PLA" => Ok(Some(Instruction::PLA)),
            "PHP" => Ok(Some(Instruction::PHP)),
            "PLP" => Ok(Some(Instruction::PLP)),
            "TSX" => Ok(Some(Instruction::TSX)),
            "TXS" => Ok(Some(Instruction::TXS)),

            // 분기 명령어들
            "BCC" => {
                self.skip_whitespace();
                let mode = self.parse_branch_target()?;
                match mode {
                    AddressModeValue::Absolute(target) => {
                        let next_pc = self.current_address.wrapping_add(2);
                        let offset = target as i32 - next_pc as i32;
                        if !(-128..=127).contains(&offset) {
                            return Err(Error::AssemblerBranchOutOfRange(
                                "Branch target too far".to_string(),
                            ));
                        }
                        Ok(Some(Instruction::BCC(offset as i8)))
                    }
                    _ => Err(Error::InvalidAddressingMode("BCC")),
                }
            }
            "BCS" => {
                self.skip_whitespace();
                let mode = self.parse_branch_target()?;
                match mode {
                    AddressModeValue::Absolute(target) => {
                        let next_pc = self.current_address.wrapping_add(2);
                        let offset = target as i32 - next_pc as i32;
                        if !(-128..=127).contains(&offset) {
                            return Err(Error::AssemblerBranchOutOfRange(
                                "Branch target too far".to_string(),
                            ));
                        }
                        Ok(Some(Instruction::BCS(offset as i8)))
                    }
                    _ => Err(Error::InvalidAddressingMode("BCS")),
                }
            }
            "BEQ" => {
                self.skip_whitespace();
                let mode = self.parse_branch_target()?;
                match mode {
                    AddressModeValue::Absolute(target) => {
                        let next_pc = self.current_address.wrapping_add(2);
                        let offset = target as i32 - next_pc as i32;
                        if !(-128..=127).contains(&offset) {
                            return Err(Error::AssemblerBranchOutOfRange(
                                "Branch target too far".to_string(),
                            ));
                        }
                        Ok(Some(Instruction::BEQ(offset as i8)))
                    }
                    _ => Err(Error::InvalidAddressingMode("BEQ")),
                }
            }
            "BNE" => {
                self.skip_whitespace();
                let mode = self.parse_branch_target()?;
                match mode {
                    AddressModeValue::Absolute(target) => {
                        let next_pc = self.current_address.wrapping_add(2);
                        let offset = target as i32 - next_pc as i32;
                        if !(-128..=127).contains(&offset) {
                            return Err(Error::AssemblerBranchOutOfRange(
                                "Branch target too far".to_string(),
                            ));
                        }
                        Ok(Some(Instruction::BNE(offset as i8)))
                    }
                    _ => Err(Error::InvalidAddressingMode("BNE")),
                }
            }
            "BMI" => {
                self.skip_whitespace();
                let mode = self.parse_branch_target()?;
                match mode {
                    AddressModeValue::Absolute(target) => {
                        let next_pc = self.current_address.wrapping_add(2);
                        let offset = target as i32 - next_pc as i32;
                        if !(-128..=127).contains(&offset) {
                            return Err(Error::AssemblerBranchOutOfRange(
                                "Branch target too far".to_string(),
                            ));
                        }
                        Ok(Some(Instruction::BMI(offset as i8)))
                    }
                    _ => Err(Error::InvalidAddressingMode("BMI")),
                }
            }
            "BPL" => {
                self.skip_whitespace();
                let mode = self.parse_branch_target()?;
                match mode {
                    AddressModeValue::Absolute(target) => {
                        let next_pc = self.current_address.wrapping_add(2);
                        let offset = target as i32 - next_pc as i32;
                        if !(-128..=127).contains(&offset) {
                            return Err(Error::AssemblerBranchOutOfRange(
                                "Branch target too far".to_string(),
                            ));
                        }
                        Ok(Some(Instruction::BPL(offset as i8)))
                    }
                    _ => Err(Error::InvalidAddressingMode("BPL")),
                }
            }
            "BVC" => {
                self.skip_whitespace();
                let mode = self.parse_branch_target()?;
                match mode {
                    AddressModeValue::Absolute(target) => {
                        let next_pc = self.current_address.wrapping_add(2);
                        let offset = target as i32 - next_pc as i32;
                        if !(-128..=127).contains(&offset) {
                            return Err(Error::AssemblerBranchOutOfRange(
                                "Branch target too far".to_string(),
                            ));
                        }
                        Ok(Some(Instruction::BVC(offset as i8)))
                    }
                    _ => Err(Error::InvalidAddressingMode("BVC")),
                }
            }
            "BVS" => {
                self.skip_whitespace();
                let mode = self.parse_branch_target()?;
                match mode {
                    AddressModeValue::Absolute(target) => {
                        let next_pc = self.current_address.wrapping_add(2);
                        let offset = target as i32 - next_pc as i32;
                        if !(-128..=127).contains(&offset) {
                            return Err(Error::AssemblerBranchOutOfRange(
                                "Branch target too far".to_string(),
                            ));
                        }
                        Ok(Some(Instruction::BVS(offset as i8)))
                    }
                    _ => Err(Error::InvalidAddressingMode("BVS")),
                }
            }

            // 점프 명령어들
            "JMP" => {
                self.skip_whitespace();
                let mode = self.parse_addressing_mode()?;
                match mode {
                    AddressModeValue::Absolute(addr) => {
                        Ok(Some(Instruction::JMP(AddressModeValue::Absolute(addr))))
                    }
                    AddressModeValue::Indirect(addr) => {
                        Ok(Some(Instruction::JMP(AddressModeValue::Indirect(addr))))
                    }
                    _ => Err(Error::InvalidAddressingMode("JMP")),
                }
            }
            "JSR" => {
                self.skip_whitespace();
                let mode = self.parse_addressing_mode()?;
                match mode {
                    AddressModeValue::Absolute(addr) => {
                        Ok(Some(Instruction::JSR(AddressModeValue::Absolute(addr))))
                    }
                    _ => Err(Error::InvalidAddressingMode("JSR")),
                }
            }

            // 알 수 없는 명령어
            _ => Ok(None),
        }
    }

    fn parse_branch_target(&mut self) -> Result<AddressModeValue> {
        let token = self.peek()?.token.clone();
        match token {
            Token::HexNumber(value) | Token::Number(value) => {
                self.advance();
                Ok(AddressModeValue::Absolute(value))
            }
            Token::Label(label) => {
                self.advance();
                let target = self.addressing_parser.resolve_label(&label)?;
                Ok(AddressModeValue::Absolute(target))
            }
            _ => Err(Error::InvalidAddressingMode("Expected branch target")),
        }
    }

    fn parse_addressing_mode(&mut self) -> Result<AddressModeValue> {
        self.skip_whitespace();

        let token = self.peek()?.token.clone();
        match token {
            Token::Hash => {
                self.advance();
                self.skip_whitespace();
                match self.peek()?.token {
                    Token::HexNumber(value) | Token::Number(value) => {
                        self.advance();
                        Ok(AddressModeValue::Immediate(value as u8))
                    }
                    _ => Err(Error::InvalidAddressingMode("Expected number after #")),
                }
            }
            Token::LeftParen => {
                self.advance();
                self.skip_whitespace();

                // 주소 값 파싱
                let addr = match self.peek()?.token {
                    Token::HexNumber(value) | Token::Number(value) => {
                        self.advance();
                        value as u8
                    }
                    _ => return Err(Error::InvalidAddressingMode("Expected address after (")),
                };

                self.skip_whitespace();

                // 다음 토큰 확인
                match self.peek()?.token {
                    Token::Comma => {
                        // ($addr,X) 형식
                        self.advance();
                        self.skip_whitespace();
                        match self.peek()?.token {
                            Token::Register('X') => {
                                self.advance();
                                self.skip_whitespace();
                                match self.peek()?.token {
                                    Token::RightParen => {
                                        self.advance();
                                        Ok(AddressModeValue::IndirectX(addr))
                                    }
                                    _ => Err(Error::InvalidAddressingMode("Expected ) after X")),
                                }
                            }
                            Token::IndexRegister('X') => {
                                self.advance();
                                self.skip_whitespace();
                                match self.peek()?.token {
                                    Token::RightParen => {
                                        self.advance();
                                        Ok(AddressModeValue::IndirectX(addr))
                                    }
                                    _ => Err(Error::InvalidAddressingMode("Expected ) after X")),
                                }
                            }
                            _ => Err(Error::InvalidAddressingMode("Expected X after comma")),
                        }
                    }
                    Token::RightParen => {
                        // ($addr),Y 형식
                        self.advance();
                        self.skip_whitespace();
                        match self.peek()?.token {
                            Token::Comma => {
                                self.advance();
                                self.skip_whitespace();
                                match self.peek()?.token {
                                    Token::Register('Y') => {
                                        self.advance();
                                        Ok(AddressModeValue::IndirectY(addr))
                                    }
                                    Token::IndexRegister('Y') => {
                                        self.advance();
                                        Ok(AddressModeValue::IndirectY(addr))
                                    }
                                    _ => {
                                        Err(Error::InvalidAddressingMode("Expected Y after comma"))
                                    }
                                }
                            }
                            _ => Ok(AddressModeValue::Indirect(addr as u16)),
                        }
                    }
                    _ => Err(Error::InvalidAddressingMode("Expected , or )")),
                }
            }
            Token::Label(name) => {
                self.advance();
                let addr = self.addressing_parser.resolve_label(&name)?;
                Ok(AddressModeValue::Absolute(addr))
            }
            Token::Register('A') => {
                self.advance();
                Ok(AddressModeValue::Accumulator)
            }
            Token::HexNumber(value) | Token::Number(value) => {
                self.advance();
                self.skip_whitespace();

                let next_token = self.peek().ok().map(|t| t.token.clone());
                match next_token {
                    Some(Token::Comma) => {
                        self.advance();
                        self.skip_whitespace();
                        let reg_token = self.peek()?.token.clone();

                        match reg_token {
                            Token::Register('X') | Token::IndexRegister('X') => {
                                self.advance();
                                if value <= 0xFF {
                                    Ok(AddressModeValue::ZeroPageX(value as u8))
                                } else {
                                    Ok(AddressModeValue::AbsoluteX(value))
                                }
                            }
                            Token::Register('Y') | Token::IndexRegister('Y') => {
                                self.advance();
                                if value <= 0xFF {
                                    Ok(AddressModeValue::ZeroPageY(value as u8))
                                } else {
                                    Ok(AddressModeValue::AbsoluteY(value))
                                }
                            }
                            _ => Err(Error::InvalidAddressingMode("Expected X or Y register")),
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
            }
            Token::EOL | Token::Comment(_) => {
                self.advance();
                Ok(AddressModeValue::Implied)
            }
            _ => {
                // 어큐뮬레이터 모드를 위한 추가 검사
                self.skip_whitespace();
                if let Ok(token_info) = self.peek() {
                    match &token_info.token {
                        Token::Register('A') => {
                            self.advance();
                            return Ok(AddressModeValue::Accumulator);
                        }
                        Token::EOL | Token::Comment(_) => {
                            self.advance();
                            return Ok(AddressModeValue::Implied);
                        }
                        _ => {}
                    }
                }
                Ok(AddressModeValue::Implied)
            }
        }
    }

    // 유틸리티 메소드들
    fn peek(&self) -> Result<&TokenInfo> {
        self.tokens
            .get(self.current)
            .ok_or(Error::AssemblerUnexpectedEndOfInput)
    }

    fn advance(&mut self) {
        self.current += 1;
    }

    fn is_at_end(&self) -> bool {
        self.current >= self.tokens.len()
    }

    fn skip_whitespace(&mut self) {
        while let Ok(TokenInfo {
            token: Token::Whitespace,
            ..
        }) = self.peek()
        {
            self.advance();
        }
    }

    fn reset_for_second_pass(&mut self) {
        self.current = 0;
        self.current_address = self.org;
        self.instruction_parser.set_current_address(self.org);
    }

    fn get_instruction_size(&self, instruction: &Instruction) -> u16 {
        match instruction {
            // 단일 바이트 명령어
            Instruction::TAX
            | Instruction::TXA
            | Instruction::TAY
            | Instruction::TYA
            | Instruction::INX
            | Instruction::INY
            | Instruction::DEX
            | Instruction::DEY => 1,

            // 시프트 명령어
            Instruction::LSR(mode) | Instruction::ASL(mode) => match mode {
                AddressModeValue::Accumulator => 1,
                AddressModeValue::ZeroPage(_) | AddressModeValue::ZeroPageX(_) => 2,
                AddressModeValue::Absolute(_) | AddressModeValue::AbsoluteX(_) => 3,
                _ => 2,
            },

            // 어드레싱 모드에 따른 크기
            Instruction::LDA(mode)
            | Instruction::LDX(mode)
            | Instruction::LDY(mode)
            | Instruction::STA(mode)
            | Instruction::STX(mode)
            | Instruction::STY(mode) => match mode {
                AddressModeValue::Immediate(_)
                | AddressModeValue::ZeroPage(_)
                | AddressModeValue::ZeroPageX(_)
                | AddressModeValue::ZeroPageY(_)
                | AddressModeValue::IndirectX(_)
                | AddressModeValue::IndirectY(_) => 2,

                AddressModeValue::Absolute(_)
                | AddressModeValue::AbsoluteX(_)
                | AddressModeValue::AbsoluteY(_)
                | AddressModeValue::Indirect(_) => 3,

                AddressModeValue::Accumulator | AddressModeValue::Implied => 1,
            },
            _ => 2, // 기본값
        }
    }
}
