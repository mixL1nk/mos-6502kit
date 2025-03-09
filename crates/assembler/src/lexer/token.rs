#[derive(Debug, PartialEq, Clone)]
pub struct TokenInfo {
    pub token: Token,
    pub line: usize,
    pub column: usize,
}

#[derive(Debug, PartialEq, Clone)]
pub enum Token {
    // 1. 기본 구성요소
    Mnemonic(String), // LDA, STA, BEQ 등 명령어
    Label(String),    // LOOP:, START: 등의 레이블

    // 2. 주소 지정 모드 관련
    Hash,                // # (즉시 주소 지정)
    Comma,               // , (X,Y 인덱스 구분용)
    LeftParen,           // ( (간접 주소 지정)
    RightParen,          // ) (간접 주소 지정)
    IndexRegister(char), // X 또는 Y 레지스터

    // 3. 수식 관련
    Plus,               // +
    Minus,              // -
    Multiply,           // *
    Divide,             // /
    LeftBracket,        // [ (그룹핑)
    RightBracket,       // ] (그룹핑)
    Expression(String), // 괄호 안의 수식

    // 4. 값 표현
    Number(u16),      // 일반 10진수
    HexNumber(u16),   // $ 접두사로 시작하는 16진수
    BinaryNumber(u8), // % 접두사로 시작하는 2진수

    // 5. 특수 연산자
    HighByte, // > (상위 바이트 추출)
    LowByte,  // < (하위 바이트 추출)

    // 6. 어셈블러 지시자
    Directive(String),  // EQU, ORG 등
    MacroName(String),  // 매크로 이름
    MacroParam(String), // 매크로 파라미터

    // 7. 기타
    Comment(String), // ; 또는 \ 로 시작하는 주석
    Colon,           // : (레이블 구분자)
    EOL,             // 줄 끝
    Whitespace,      // 공백 문자

    // 8. BBC BASIC 모드 관련
    Dot, // . (BBC 모드의 레이블 선언)
    At,  // @ (Acorn Atom 즉시 주소 지정)
}
