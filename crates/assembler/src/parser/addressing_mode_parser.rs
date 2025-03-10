use crate::lexer::Token;
use crate::parser::token_parser::TokenParser;
use common::Result;
use error::Error;
use types::AddressModeValue;

/// Addressing mode parser for instruction operands
pub struct AddressingModeParser {
    labels: std::collections::HashMap<String, u16>,
}

impl AddressingModeParser {
    pub fn new(labels: std::collections::HashMap<String, u16>) -> Self {
        Self { labels }
    }

    pub fn add_label(&mut self, name: String, address: u16) {
        self.labels.insert(name, address);
    }

    pub fn resolve_label(&self, label: &str) -> Result<u16> {
        self.labels
            .get(label)
            .copied()
            .ok_or_else(|| Error::AssemblerUndefinedLabel(label.to_string()))
    }

    pub fn get_labels(&self) -> &std::collections::HashMap<String, u16> {
        &self.labels
    }

    pub fn parse_addressing_mode(
        &self,
        token_parser: &mut TokenParser,
    ) -> Result<AddressModeValue> {
        token_parser.skip_whitespace();

        let token = token_parser.peek()?.token.clone();
        match token {
            Token::Hash => {
                token_parser.advance();
                token_parser.skip_whitespace();
                match token_parser.peek()?.token {
                    Token::HexNumber(value) | Token::Number(value) => {
                        token_parser.advance();
                        Ok(AddressModeValue::Immediate(value as u8))
                    }
                    _ => Err(Error::InvalidAddressingMode("Expected number after #")),
                }
            }
            Token::LeftParen => {
                token_parser.advance();
                token_parser.skip_whitespace();

                // Parse address value
                let addr = match token_parser.peek()?.token {
                    Token::HexNumber(value) | Token::Number(value) => {
                        token_parser.advance();
                        value as u8
                    }
                    _ => return Err(Error::InvalidAddressingMode("Expected address after (")),
                };

                token_parser.skip_whitespace();

                // Check next token
                match token_parser.peek()?.token {
                    Token::Comma => {
                        // ($addr,X) format
                        token_parser.advance();
                        token_parser.skip_whitespace();
                        match token_parser.peek()?.token {
                            Token::Register('X') | Token::IndexRegister('X') => {
                                token_parser.advance();
                                token_parser.skip_whitespace();
                                match token_parser.peek()?.token {
                                    Token::RightParen => {
                                        token_parser.advance();
                                        Ok(AddressModeValue::IndirectX(addr))
                                    }
                                    _ => Err(Error::InvalidAddressingMode("Expected ) after X")),
                                }
                            }
                            _ => Err(Error::InvalidAddressingMode("Expected X after comma")),
                        }
                    }
                    Token::RightParen => {
                        // ($addr),Y format
                        token_parser.advance();
                        token_parser.skip_whitespace();
                        match token_parser.peek()?.token {
                            Token::Comma => {
                                token_parser.advance();
                                token_parser.skip_whitespace();
                                match token_parser.peek()?.token {
                                    Token::Register('Y') | Token::IndexRegister('Y') => {
                                        token_parser.advance();
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
                token_parser.advance();
                let addr = self.resolve_label(&name)?;
                Ok(AddressModeValue::Absolute(addr))
            }
            Token::Register('A') => {
                token_parser.advance();
                Ok(AddressModeValue::Accumulator)
            }
            Token::HexNumber(value) | Token::Number(value) => {
                token_parser.advance();
                token_parser.skip_whitespace();

                let next_token = token_parser.peek().ok().map(|t| t.token.clone());
                match next_token {
                    Some(Token::Comma) => {
                        token_parser.advance();
                        token_parser.skip_whitespace();
                        let reg_token = token_parser.peek()?.token.clone();

                        match reg_token {
                            Token::Register('X') | Token::IndexRegister('X') => {
                                token_parser.advance();
                                if value <= 0xFF {
                                    Ok(AddressModeValue::ZeroPageX(value as u8))
                                } else {
                                    Ok(AddressModeValue::AbsoluteX(value))
                                }
                            }
                            Token::Register('Y') | Token::IndexRegister('Y') => {
                                token_parser.advance();
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
                token_parser.advance();
                Ok(AddressModeValue::Implied)
            }
            _ => {
                // Additional check for accumulator mode
                token_parser.skip_whitespace();
                if let Ok(token_info) = token_parser.peek() {
                    match &token_info.token {
                        Token::Register('A') => {
                            token_parser.advance();
                            return Ok(AddressModeValue::Accumulator);
                        }
                        Token::EOL | Token::Comment(_) => {
                            token_parser.advance();
                            return Ok(AddressModeValue::Implied);
                        }
                        _ => {}
                    }
                }
                Ok(AddressModeValue::Implied)
            }
        }
    }
}
