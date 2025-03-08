use std::iter::Peekable;
use std::str::Chars;
use common::Result;

use crate::lexer::{Lexer, Token, TokenInfo};

// 토큰화 결과 열거형 추가
enum TokenizeResult {
    Token(TokenInfo),
    Tokens(Vec<TokenInfo>),
    Skip,
}

impl Lexer {
    pub fn tokenize(mut self) -> Result<Vec<TokenInfo>> {
        let mut tokens = Vec::new();
        let mut chars = self.input.chars().peekable();

        while let Some(c) = chars.peek().cloned() {
            match self.next_token(&mut chars, c, &tokens)? {
                TokenizeResult::Token(token) => {
                    if !matches!(token.token, Token::Whitespace | Token::EOL) {
                        self.last_token_was_macro = false;
                    }
                    tokens.push(token);
                },
                TokenizeResult::Tokens(mut new_tokens) => {
                    for token in &new_tokens {
                        if !matches!(token.token, Token::Whitespace | Token::EOL) {
                            self.last_token_was_macro = false;
                        }
                    }
                    tokens.append(&mut new_tokens);
                },
                TokenizeResult::Skip => continue,
            }
        }

        Ok(tokens)
    }

    fn next_token(&mut self, chars: &mut Peekable<Chars>, c: char, tokens: &[TokenInfo]) -> Result<TokenizeResult> {
        match c {
            'a'..='z' | 'A'..='Z' | '_' | '.' | '\\' => self.process_identifier(chars, tokens),
            '0'..='9' => Ok(TokenizeResult::Token(self.handle_number(chars)?)),
            '$' => {
                chars.next();
                Ok(TokenizeResult::Token(self.handle_hex_number(chars)?))
            },
            '%' => {
                chars.next();
                Ok(TokenizeResult::Token(self.handle_binary_number(chars)?))
            },
            '#' => {
                chars.next();
                Ok(TokenizeResult::Token(self.create_token(Token::Hash)))
            },
            ',' => {
                chars.next();
                Ok(TokenizeResult::Token(self.create_token(Token::Comma)))
            },
            '(' => self.process_left_paren(chars, tokens),
            ')' => {
                chars.next();
                Ok(TokenizeResult::Token(self.create_token(Token::RightParen)))
            },
            '[' => {
                chars.next();
                Ok(TokenizeResult::Token(self.create_token(Token::LeftBracket)))
            },
            ']' => {
                chars.next();
                Ok(TokenizeResult::Token(self.create_token(Token::RightBracket)))
            },
            '+' => {
                chars.next();
                Ok(TokenizeResult::Token(self.create_token(Token::Plus)))
            },
            '-' => {
                chars.next();
                Ok(TokenizeResult::Token(self.create_token(Token::Minus)))
            },
            '*' => {
                chars.next();
                Ok(TokenizeResult::Token(self.create_token(Token::Multiply)))
            },
            '/' => {
                chars.next();
                Ok(TokenizeResult::Token(self.create_token(Token::Divide)))
            },
            '>' => {
                chars.next();
                Ok(TokenizeResult::Token(self.create_token(Token::HighByte)))
            },
            '<' => {
                chars.next();
                Ok(TokenizeResult::Token(self.create_token(Token::LowByte)))
            },
            ':' => {
                chars.next();
                Ok(TokenizeResult::Token(self.create_token(Token::Colon)))
            },
            ';' => Ok(TokenizeResult::Token(self.handle_comment(chars)?)),
            '\n' => {
                chars.next();
                self.current_line += 1;
                self.current_column = 1;
                Ok(TokenizeResult::Token(self.create_token(Token::EOL)))
            },
            ' ' | '\t' | '\r' => {
                chars.next();
                self.current_column += 1;
                Ok(TokenizeResult::Token(self.create_token(Token::Whitespace)))
            },
            // . 처리를 해야할까?
            // '.' => {
            //     chars.next();
            //     self.current_column += 1;
            //     Ok(TokenizeResult::Token(self.create_token(Token::Dot)))
            // },
            _ => {
                // 인식할 수 없는 문자는 건너뛰기
                chars.next();
                self.current_column += 1;
                Ok(TokenizeResult::Skip)
            }
        }
    }

    fn process_identifier(&mut self, chars: &mut Peekable<Chars>, _tokens: &[TokenInfo]) -> Result<TokenizeResult> {
        let token = self.handle_identifier(chars)?;
        
        // .MACRO 디렉티브 처리
        if let Token::Directive(ref d) = token.token {
            if d == ".MACRO" {
                return self.process_macro_directive(chars, token);
            }
        }
        
        // 매크로 이름 처리
        if self.last_token_was_macro {
            if let Token::Label(name) = token.token {
                return Ok(TokenizeResult::Token(TokenInfo {
                    token: Token::MacroName(name),
                    ..token
                }));
            }
        }
        
        Ok(TokenizeResult::Token(token))
    }

    fn process_macro_directive(&mut self, chars: &mut Peekable<Chars>, directive_token: TokenInfo) -> Result<TokenizeResult> {
        self.last_token_was_macro = true;
        
        // 디렉티브 다음의 매크로 이름 추출
        self.skip_whitespace(chars)?;
        
        let mut name = String::new();
        while let Some(&c) = chars.peek() {
            if c.is_whitespace() {
                break;
            }
            name.push(c);
            chars.next();
            self.current_column += 1;
        }
        
        // 매크로 이름 토큰 생성
        let macro_name_token = TokenInfo {
            token: Token::MacroName(name),
            line: self.current_line,
            column: self.current_column,
        };
        
        // 디렉티브와 매크로 이름 토큰 모두 반환
        Ok(TokenizeResult::Tokens(vec![directive_token, macro_name_token]))
    }
    
    fn process_left_paren(&mut self, chars: &mut Peekable<Chars>, tokens: &[TokenInfo]) -> Result<TokenizeResult> {
        chars.next();
        
        // 직전 토큰이 # 인 경우에만 Expression으로 처리
        if tokens.last().map_or(false, |t| matches!(t.token, Token::Hash)) {
            Ok(TokenizeResult::Token(self.handle_expression(chars)?))
        } else {
            Ok(TokenizeResult::Token(self.create_token(Token::LeftParen)))
        }
    }
    

    pub(crate) fn create_token(&self, token: Token) -> TokenInfo {
        TokenInfo {
            token,
            line: self.current_line,
            column: self.current_column,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::lexer::Lexer;

    #[test]
    fn test_tokenize() {
        let input = "
            LDA #$10    ; 즉시 주소 지정
            LDX $1000   ; 직접 주소 지정
            LDY $1000,X ; 간접 주소 지정
            LDA $1000,Y ; 간접 인덱스 주소 지정
        ";
        let lexer = Lexer::new(input);
        let tokens = lexer.tokenize();

        println!("{:?}", tokens);
    }

    #[test]
    fn test_addressing_modes() {
        let input = "
            LDA #($1234 + $10) ; 수식
            LDA ($20,X)        ; 간접 X 인덱스
            LDA ($20),Y        ; 간접 Y 인덱스
        ";
        let lexer = Lexer::new(input);
        let tokens = lexer.tokenize().unwrap();
        
        // 수식 확인
        let expr_tokens: Vec<_> = tokens.iter()
            .filter_map(|t| {
                if let Token::Expression(expr) = &t.token {
                    Some(expr.as_str())
                } else {
                    None
                }
            })
            .collect();
        assert_eq!(expr_tokens, vec!["$1234 + $10"]);
        
        // 간접 주소 지정 확인
        let mut found_indirect_x = false;
        let mut found_indirect_y = false;
        
        for window in tokens.windows(5) {
            match (&window[0].token, &window[1].token, &window[2].token, &window[3].token, &window[4].token) {
                (Token::LeftParen, Token::HexNumber(_), Token::Comma, Token::IndexRegister('X'), Token::RightParen) => {
                    found_indirect_x = true;
                },
                (Token::LeftParen, Token::HexNumber(_), Token::RightParen, Token::Comma, Token::IndexRegister('Y')) => {
                    found_indirect_y = true;
                },
                _ => {}
            }
        }
        
        assert!(found_indirect_x, "간접 X 인덱스 주소 지정이 없음");
        assert!(found_indirect_y, "간접 Y 인덱스 주소 지정이 없음");
    }

    #[test]
    fn test_macro() {
        let input = "
            .MACRO ADD_TWICE param1
                CLC
                ADC \\param1
                ADC \\param1
            .ENDM
        ";
        let lexer = Lexer::new(input);
        let tokens = lexer.tokenize().unwrap();
        
        // 매크로 토큰들 확인
        let mut found_macro_start = false;
        let mut found_macro_param = false;
        let mut found_macro_end = false;
        
        // 디버깅을 위해 토큰들을 출력
        println!("Tokens: {:?}", tokens);
        
        for token in tokens.iter() {
            match &token.token {
                Token::Directive(d) if d == ".MACRO" => found_macro_start = true,
                Token::MacroParam(p) if p == "\\param1" => found_macro_param = true,
                Token::Directive(d) if d == ".ENDM" => found_macro_end = true,
                _ => {}
            }
        }
        
        assert!(found_macro_start, "매크로 시작이 없음");
        assert!(found_macro_param, "매크로 파라미터가 없음");
        assert!(found_macro_end, "매크로 종료가 없음");
    }
}
