use crate::lexer::Token;
use crate::parser::addressing_mode_parser::AddressingModeParser;
use crate::parser::instruction_size::InstructionSizeCalculator;
use crate::parser::token_parser::TokenParser;
use common::Result;
use error::Error;

/// Label collector for first pass
pub struct LabelCollector {
    current_address: u16,
    org: u16,
}

impl LabelCollector {
    pub fn new(org: u16) -> Self {
        Self {
            current_address: org,
            org,
        }
    }

    pub fn set_org(&mut self, org: u16) {
        self.org = org;
        self.current_address = org;
    }

    pub fn collect_labels(
        &mut self,
        token_parser: &mut TokenParser,
        addressing_parser: &mut AddressingModeParser,
    ) -> Result<()> {
        token_parser.reset();
        self.current_address = self.org;

        while !token_parser.is_at_end() {
            let token = token_parser.peek()?.token.clone();
            match token {
                Token::Directive(dir) if dir == ".ORG" => {
                    token_parser.advance();
                    token_parser.skip_whitespace();

                    match token_parser.peek()?.token {
                        Token::HexNumber(addr) | Token::Number(addr) => {
                            println!("[DEBUG] Found .ORG directive: ${:04X}", addr);
                            self.set_org(addr);
                            token_parser.advance();
                        }
                        _ => return Err(Error::AssemblerInvalidDirective(".ORG")),
                    }
                }
                Token::Label(name) => {
                    println!(
                        "[DEBUG] Found label '{}' at ${:04X}",
                        name, self.current_address
                    );
                    addressing_parser.add_label(name.clone(), self.current_address);
                    token_parser.advance();
                }
                Token::Mnemonic(ref m) => {
                    let size = InstructionSizeCalculator::estimate_instruction_size(m);
                    println!(
                        "[DEBUG] Estimating size for mnemonic '{}' at ${:04X}: {} bytes",
                        m, self.current_address, size
                    );
                    self.current_address = self.current_address.wrapping_add(size);

                    // Skip instruction operands
                    token_parser.advance();
                    while !token_parser.is_at_end() {
                        match token_parser.peek()?.token {
                            Token::EOL | Token::Comment(_) => break,
                            _ => token_parser.advance(),
                        }
                    }
                }
                Token::EOL | Token::Comment(_) | Token::Whitespace => {
                    token_parser.advance();
                }
                _ => {
                    token_parser.advance();
                }
            }
        }

        // Debug output: all collected labels
        println!(
            "[DEBUG] Collected labels: {:?}",
            addressing_parser.get_labels()
        );

        Ok(())
    }
}
