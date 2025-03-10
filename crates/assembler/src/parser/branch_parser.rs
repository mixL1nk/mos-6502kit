use crate::lexer::Token;
use crate::parser::addressing_mode_parser::AddressingModeParser;
use common::Result;
use error::Error;
use types::AddressModeValue;

/// Branch instruction parser
pub struct BranchParser {
    current_address: u16,
}

impl Default for BranchParser {
    fn default() -> Self {
        Self::new()
    }
}

impl BranchParser {
    pub fn new() -> Self {
        Self { current_address: 0 }
    }

    pub fn set_current_address(&mut self, address: u16) {
        self.current_address = address;
    }

    pub fn parse_branch_target(
        &self,
        token: &Token,
        addressing_parser: &AddressingModeParser,
    ) -> Result<AddressModeValue> {
        match token {
            Token::HexNumber(value) | Token::Number(value) => {
                Ok(AddressModeValue::Absolute(*value))
            }
            Token::Label(label) => {
                let target = addressing_parser.resolve_label(label)?;
                println!(
                    "[DEBUG] Resolved label '{}' to address ${:04X}",
                    label, target
                );
                Ok(AddressModeValue::Absolute(target))
            }
            _ => Err(Error::InvalidAddressingMode("Expected branch target")),
        }
    }

    pub fn calculate_branch_offset(&self, target_address: u16) -> Result<i8> {
        let next_pc = self.current_address.wrapping_add(2);
        let offset = target_address as i32 - next_pc as i32;

        println!("[DEBUG] Branch offset calculation:");
        println!("[DEBUG]   Current address: ${:04X}", self.current_address);
        println!("[DEBUG]   Next PC: ${:04X}", next_pc);
        println!("[DEBUG]   Target address: ${:04X}", target_address);
        println!(
            "[DEBUG]   Offset: {} (0x{:02X})",
            offset,
            (offset as i8) as u8
        );

        if !(-128..=127).contains(&offset) {
            return Err(Error::AssemblerBranchOutOfRange(
                "Branch target too far".to_string(),
            ));
        }

        Ok(offset as i8)
    }
}
