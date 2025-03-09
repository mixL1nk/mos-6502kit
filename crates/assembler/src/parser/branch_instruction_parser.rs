use crate::lexer::Token;
use crate::parser::addressing_mode_parser::AddressingModeParser;
use crate::parser::branch_parser::BranchParser;
use crate::parser::token_parser::TokenParser;
use common::Result;
use error::Error;
use types::{AddressModeValue, Instruction};

/// Branch instruction parser
pub struct BranchInstructionParser {
    branch_parser: BranchParser,
}

impl BranchInstructionParser {
    pub fn new() -> Self {
        Self {
            branch_parser: BranchParser::new(),
        }
    }

    pub fn set_current_address(&mut self, address: u16) {
        self.branch_parser.set_current_address(address);
    }

    pub fn parse_branch_instruction(
        &self,
        mnemonic: &str,
        token_parser: &mut TokenParser,
        addressing_parser: &AddressingModeParser,
    ) -> Result<Option<Instruction>> {
        token_parser.skip_whitespace();
        
        let token = token_parser.peek()?.token.clone();
        let mode = match token {
            Token::HexNumber(value) | Token::Number(value) => {
                token_parser.advance();
                AddressModeValue::Absolute(value)
            }
            Token::Label(label) => {
                token_parser.advance();
                let target = addressing_parser.resolve_label(&label)?;
                AddressModeValue::Absolute(target)
            }
            _ => return Err(Error::InvalidAddressingMode("Expected branch target")),
        };
        
        match mode {
            AddressModeValue::Absolute(target) => {
                let offset = self.branch_parser.calculate_branch_offset(target)?;
                
                match mnemonic {
                    "BCC" => Ok(Some(Instruction::BCC(offset))),
                    "BCS" => Ok(Some(Instruction::BCS(offset))),
                    "BEQ" => Ok(Some(Instruction::BEQ(offset))),
                    "BNE" => Ok(Some(Instruction::BNE(offset))),
                    "BMI" => Ok(Some(Instruction::BMI(offset))),
                    "BPL" => Ok(Some(Instruction::BPL(offset))),
                    "BVC" => Ok(Some(Instruction::BVC(offset))),
                    "BVS" => Ok(Some(Instruction::BVS(offset))),
                    _ => Err(Error::InvalidAddressingMode(
                        "Unsupported branch instruction",
                    )),
                }
            }
            _ => Err(Error::InvalidAddressingMode("Invalid branch target")),
        }
    }
} 