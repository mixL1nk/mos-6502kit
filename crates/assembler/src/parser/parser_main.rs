use crate::lexer::{Token, TokenInfo};
use crate::parser::addressing_mode_parser::AddressingModeParser;
use crate::parser::branch_instruction_parser::BranchInstructionParser;
use crate::parser::instruction::InstructionParser;
use crate::parser::instruction_size::InstructionSizeCalculator;
use crate::parser::jump_instruction_parser::JumpInstructionParser;
use crate::parser::label_collector::LabelCollector;
use crate::parser::strategy_registry::StrategyRegistry;
use crate::parser::token_parser::TokenParser;
use common::Result;
use error::Error;
use std::collections::HashMap;
use types::{AddressModeValue, Instruction};

pub struct Parser<'a> {
    token_parser: TokenParser,
    instruction_parser: InstructionParser,
    addressing_parser: AddressingModeParser,
    branch_instruction_parser: BranchInstructionParser,
    jump_instruction_parser: JumpInstructionParser,
    label_collector: LabelCollector,
    org: u16,
    current_address: u16,
    strategy_registry: StrategyRegistry<'a>,
}

impl Parser<'_> {
    pub fn new(tokens: Vec<TokenInfo>) -> Self {
        let labels = HashMap::new();
        let addressing_parser = AddressingModeParser::new(labels);
        let instruction_parser = InstructionParser::new();
        let token_parser = TokenParser::new(tokens);
        let branch_instruction_parser = BranchInstructionParser::new();
        let jump_instruction_parser = JumpInstructionParser::new();
        let strategy_registry = StrategyRegistry::new();

        let default_org = 0x8000;
        let label_collector = LabelCollector::new(default_org);

        Self {
            token_parser,
            instruction_parser,
            addressing_parser,
            branch_instruction_parser,
            jump_instruction_parser,
            label_collector,
            org: default_org,
            current_address: default_org,
            strategy_registry,
        }
    }

    pub fn set_org(&mut self, org: u16) {
        self.org = org;
        self.current_address = org;
        self.instruction_parser.set_current_address(org);
        self.branch_instruction_parser.set_current_address(org);
        self.label_collector.set_org(org);
    }

    pub fn get_org(&self) -> u16 {
        self.org
    }

    pub fn parse(&mut self) -> Result<Vec<Instruction>> {
        // First pass: collect labels and .ORG
        self.label_collector
            .collect_labels(&mut self.token_parser, &mut self.addressing_parser)?;

        // Second pass: generate instructions
        self.reset_for_second_pass();

        let mut instructions = Vec::new();
        while !self.token_parser.is_at_end() {
            let token = self.token_parser.peek()?.token.clone();
            match token {
                Token::Directive(dir) if dir == ".ORG" => {
                    self.token_parser.advance();
                    self.token_parser.skip_whitespace();

                    match self.token_parser.peek()?.token {
                        Token::HexNumber(addr) | Token::Number(addr) => {
                            self.set_org(addr);
                            self.token_parser.advance();
                        }
                        _ => return Err(Error::AssemblerInvalidDirective(".ORG")),
                    }
                }
                Token::Label(_) | Token::Whitespace | Token::EOL | Token::Comment(_) => {
                    self.token_parser.advance();
                }
                Token::Mnemonic(_) => {
                    if let Some(instruction) = self.parse_next_instruction()? {
                        instructions.push(instruction);
                    }
                }
                _ => {
                    self.token_parser.advance();
                }
            }
        }

        Ok(instructions)
    }

    fn parse_next_instruction(&mut self) -> Result<Option<Instruction>> {
        let token = self.token_parser.peek()?.token.clone();
        match token {
            Token::Directive(dir) if dir == ".ORG" => {
                self.token_parser.advance();
                self.token_parser.skip_whitespace();
                match self.token_parser.peek()?.token {
                    Token::HexNumber(addr) | Token::Number(addr) => {
                        self.set_org(addr);
                        self.token_parser.advance();
                    }
                    _ => return Err(Error::AssemblerInvalidDirective(".ORG")),
                }
                Ok(None)
            }
            Token::Label(name) => {
                self.token_parser.advance();
                self.addressing_parser.add_label(name, self.current_address);
                Ok(None)
            }
            Token::Mnemonic(m) => {
                let instruction_start = self.current_address;
                self.token_parser.advance();

                // Handle single byte instructions
                if self.is_single_byte_instruction(&m) {
                    self.token_parser.skip_whitespace();
                    let mode = AddressModeValue::Implied;
                    let strategy = self
                        .strategy_registry
                        .get_strategy(&m)
                        .expect("Strategy should exist for single byte instruction");
                    let instruction = strategy.parse(&self.instruction_parser, mode)?;
                    self.current_address = instruction_start.wrapping_add(1);
                    self.instruction_parser
                        .set_current_address(self.current_address);
                    self.branch_instruction_parser
                        .set_current_address(self.current_address);
                    return Ok(Some(instruction));
                }

                // Handle branch instructions
                if self.is_branch_instruction(&m) {
                    let instruction = self.branch_instruction_parser.parse_branch_instruction(
                        &m,
                        &mut self.token_parser,
                        &self.addressing_parser,
                    )?;

                    if let Some(ref instr) = instruction {
                        let size = InstructionSizeCalculator::get_instruction_size(instr);
                        self.current_address = instruction_start.wrapping_add(size);
                        self.instruction_parser
                            .set_current_address(self.current_address);
                        self.branch_instruction_parser
                            .set_current_address(self.current_address);
                    }

                    return Ok(instruction);
                }

                // Handle jump instructions
                if self.is_jump_instruction(&m) {
                    let instruction = self.jump_instruction_parser.parse_jump_instruction(
                        &m,
                        &mut self.token_parser,
                        &self.addressing_parser,
                    )?;

                    if let Some(ref instr) = instruction {
                        let size = InstructionSizeCalculator::get_instruction_size(instr);
                        self.current_address = instruction_start.wrapping_add(size);
                        self.instruction_parser
                            .set_current_address(self.current_address);
                        self.branch_instruction_parser
                            .set_current_address(self.current_address);
                    }

                    return Ok(instruction);
                }

                // Handle other instructions with strategies
                if self.strategy_registry.has_strategy(&m) {
                    let mode = self
                        .addressing_parser
                        .parse_addressing_mode(&mut self.token_parser)?;
                    let strategy = self
                        .strategy_registry
                        .get_strategy(&m)
                        .expect("Strategy was just checked");
                    let instruction = strategy.parse(&self.instruction_parser, mode)?;
                    let size = InstructionSizeCalculator::get_instruction_size(&instruction);
                    self.current_address = instruction_start.wrapping_add(size);
                    self.instruction_parser
                        .set_current_address(self.current_address);
                    self.branch_instruction_parser
                        .set_current_address(self.current_address);
                    Ok(Some(instruction))
                } else {
                    // Fallback for instructions without strategies
                    Err(Error::AssemblerInvalidInstruction(m))
                }
            }
            _ => {
                self.token_parser.advance();
                Ok(None)
            }
        }
    }

    fn is_single_byte_instruction(&self, mnemonic: &str) -> bool {
        matches!(
            mnemonic,
            "INX"
                | "INY"
                | "DEX"
                | "DEY"
                | "TAX"
                | "TXA"
                | "TAY"
                | "TYA"
                | "CLC"
                | "SEC"
                | "CLI"
                | "SEI"
                | "CLV"
                | "CLD"
                | "SED"
                | "NOP"
                | "BRK"
                | "RTI"
                | "RTS"
                | "PHA"
                | "PLA"
                | "PHP"
                | "PLP"
                | "TSX"
                | "TXS"
        )
    }

    fn is_branch_instruction(&self, mnemonic: &str) -> bool {
        matches!(
            mnemonic,
            "BCC" | "BCS" | "BEQ" | "BNE" | "BMI" | "BPL" | "BVC" | "BVS"
        )
    }

    fn is_jump_instruction(&self, mnemonic: &str) -> bool {
        matches!(mnemonic, "JMP" | "JSR")
    }

    fn reset_for_second_pass(&mut self) {
        self.token_parser.reset();
        self.current_address = self.org;
        self.instruction_parser.set_current_address(self.org);
        self.branch_instruction_parser.set_current_address(self.org);
    }
}
