use crate::lexer::{Token, TokenInfo};
use common::Result;
use error::Error;

/// Token parsing utilities
pub struct TokenParser {
    tokens: Vec<TokenInfo>,
    current: usize,
}

impl TokenParser {
    pub fn new(tokens: Vec<TokenInfo>) -> Self {
        Self { tokens, current: 0 }
    }

    pub fn reset(&mut self) {
        self.current = 0;
    }

    pub fn peek(&self) -> Result<&TokenInfo> {
        self.tokens
            .get(self.current)
            .ok_or(Error::AssemblerUnexpectedEndOfInput)
    }

    pub fn advance(&mut self) {
        self.current += 1;
    }

    pub fn is_at_end(&self) -> bool {
        self.current >= self.tokens.len()
    }

    pub fn skip_whitespace(&mut self) {
        while let Ok(TokenInfo {
            token: Token::Whitespace,
            ..
        }) = self.peek()
        {
            self.advance();
        }
    }

    pub fn get_current_position(&self) -> usize {
        self.current
    }

    pub fn set_current_position(&mut self, position: usize) {
        self.current = position;
    }
} 