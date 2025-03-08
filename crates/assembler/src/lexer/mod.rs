mod token;
mod macros;
mod handler;
mod tokenizer;
pub use token::TokenInfo;
pub use token::Token;

pub struct Lexer{
    input: &'static str,
    current_line: usize,
    current_column: usize,
    in_macro: bool,
    last_token_was_macro: bool,
}

impl Lexer {
    pub fn new(input: &'static str) -> Self {
        Lexer {
            input,
            current_line: 1,
            current_column: 1,
            in_macro: false,
            last_token_was_macro: false,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

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
