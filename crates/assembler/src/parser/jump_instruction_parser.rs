use crate::parser::addressing_mode_parser::AddressingModeParser;
use crate::parser::token_parser::TokenParser;
use common::Result;
use error::Error;
use types::{AddressModeValue, Instruction};

/// Jump instruction parser
pub struct JumpInstructionParser;

impl JumpInstructionParser {
    pub fn new() -> Self {
        Self
    }

    pub fn parse_jump_instruction(
        &self,
        mnemonic: &str,
        token_parser: &mut TokenParser,
        addressing_parser: &AddressingModeParser,
    ) -> Result<Option<Instruction>> {
        token_parser.skip_whitespace();
        
        let mode = addressing_parser.parse_addressing_mode(token_parser)?;
        
        match mnemonic {
            "JMP" => match mode {
                AddressModeValue::Absolute(addr) => {
                    Ok(Some(Instruction::JMP(AddressModeValue::Absolute(addr))))
                }
                AddressModeValue::Indirect(addr) => {
                    Ok(Some(Instruction::JMP(AddressModeValue::Indirect(addr))))
                }
                _ => Err(Error::InvalidAddressingMode("JMP")),
            },
            "JSR" => match mode {
                AddressModeValue::Absolute(addr) => {
                    Ok(Some(Instruction::JSR(AddressModeValue::Absolute(addr))))
                }
                _ => Err(Error::InvalidAddressingMode("JSR")),
            },
            _ => Err(Error::InvalidAddressingMode("Unsupported jump instruction")),
        }
    }
} 