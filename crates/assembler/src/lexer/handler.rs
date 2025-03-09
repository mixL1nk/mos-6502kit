use std::iter::Peekable;
use std::str::Chars;

use common::Result;
use error::Error;

use crate::lexer::Lexer;
use crate::lexer::Token;
use crate::lexer::TokenInfo;
use crate::lexer::macros::DIRECTIVES;
use crate::lexer::macros::MNEMONICS;

impl Lexer<'_> {
    pub(crate) fn handle_identifier(&mut self, chars: &mut Peekable<Chars>) -> Result<TokenInfo> {
        let mut identifier = String::new();

        // 첫 글자가 백슬래시인지 확인
        let is_macro_param = chars.peek() == Some(&'\\');

        while let Some(&c) = chars.peek() {
            if c.is_alphanumeric() || c == '_' || c == '.' || c == '\\' {
                identifier.push(c);
                chars.next();
                self.current_column += 1;
            } else {
                break;
            }
        }

        // 매크로 파라미터 우선 처리 (백슬래시로 시작하는 식별자)
        if is_macro_param {
            return Ok(self.create_token(Token::MacroParam(identifier)));
        }

        let upper_id = identifier.to_uppercase();

        Ok(match upper_id.as_str() {
            "X" => self.create_token(Token::IndexRegister('X')),
            "Y" => self.create_token(Token::IndexRegister('Y')),
            ".MACRO" => {
                self.in_macro = true;
                self.create_token(Token::Directive(upper_id))
            }
            ".ENDM" => {
                if !self.in_macro {
                    return Err(Error::UnexpectedEndMacro {
                        line: self.current_line,
                        column: self.current_column,
                    });
                }
                self.in_macro = false;
                self.create_token(Token::Directive(upper_id))
            }
            _ => {
                if chars.peek() == Some(&':') {
                    self.create_token(Token::Label(identifier))
                } else if MNEMONICS.contains_key(upper_id.as_str()) {
                    self.create_token(Token::Mnemonic(upper_id))
                } else if DIRECTIVES.contains_key(upper_id.as_str()) {
                    self.create_token(Token::Directive(upper_id))
                } else if self.in_macro {
                    if identifier.starts_with('\\') {
                        self.create_token(Token::MacroParam(identifier))
                    } else if self.last_token_was_macro {
                        self.create_token(Token::MacroName(identifier))
                    } else {
                        self.create_token(Token::Label(identifier))
                    }
                } else {
                    self.create_token(Token::Label(identifier))
                }
            }
        })
    }

    pub(crate) fn handle_comment(&mut self, chars: &mut Peekable<Chars>) -> Result<TokenInfo> {
        let mut comment = String::new();
        while let Some(&c) = chars.peek() {
            if c == '\n' {
                break;
            }
            comment.push(c);
            chars.next();
            self.current_column += 1;
        }
        Ok(self.create_token(Token::Comment(comment)))
    }

    pub(crate) fn handle_number(&mut self, chars: &mut Peekable<Chars>) -> Result<TokenInfo> {
        let mut number = String::new();
        while let Some(&c) = chars.peek() {
            if c.is_ascii_digit() || c == '$' || c == '%' {
                number.push(c);
                chars.next();
                self.current_column += 1;
            } else {
                break;
            }
        }
        let number = match number.parse() {
            Ok(value) => value,
            Err(_) => {
                return Err(Error::InvalidNumber {
                    line: self.current_line,
                    column: self.current_column,
                });
            }
        };
        Ok(self.create_token(Token::Number(number)))
    }

    pub(crate) fn handle_hex_number(&mut self, chars: &mut Peekable<Chars>) -> Result<TokenInfo> {
        let mut number = String::new();
        while let Some(&c) = chars.peek() {
            if c.is_ascii_hexdigit() {
                number.push(c);
                chars.next();
                self.current_column += 1;
            } else {
                break;
            }
        }
        let number = u16::from_str_radix(&number, 16).map_err(|_| Error::InvalidHexNumber {
            line: self.current_line,
            column: self.current_column,
        })?;
        Ok(self.create_token(Token::HexNumber(number)))
    }

    pub(crate) fn handle_binary_number(
        &mut self,
        chars: &mut Peekable<Chars>,
    ) -> Result<TokenInfo> {
        let mut number = String::new();
        while let Some(&c) = chars.peek() {
            if c == '0' || c == '1' {
                number.push(c);
                chars.next();
                self.current_column += 1;
            } else {
                break;
            }
        }
        let number = u8::from_str_radix(&number, 2).map_err(|_| Error::InvalidBinaryNumber {
            line: self.current_line,
            column: self.current_column,
        })?;
        Ok(self.create_token(Token::BinaryNumber(number)))
    }

    pub(crate) fn handle_expression(&mut self, chars: &mut Peekable<Chars>) -> Result<TokenInfo> {
        let mut expr = String::new();
        let mut paren_count = 1; // 이미 첫 번째 괄호는 읽었다고 가정

        while let Some(&c) = chars.peek() {
            match c {
                '(' => paren_count += 1,
                ')' => {
                    paren_count -= 1;
                    if paren_count == 0 {
                        chars.next();
                        break;
                    }
                }
                _ => {}
            }
            expr.push(c);
            chars.next();
            self.current_column += 1;
        }

        Ok(self.create_token(Token::Expression(expr)))
    }

    pub(crate) fn skip_whitespace(&mut self, chars: &mut Peekable<Chars>) -> Result<()> {
        while let Some(&c) = chars.peek() {
            if !c.is_whitespace() {
                break;
            }
            chars.next();
            self.current_column += 1;
        }
        Ok(())
    }
}
