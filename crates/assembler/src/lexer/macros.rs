use std::collections::HashMap;
use std::sync::LazyLock;

// 명령어 세트 정의
pub static MNEMONICS: LazyLock<HashMap<&'static str, ()>> = LazyLock::new(|| {
    let mut map = HashMap::new();
    for &mnemonic in &[
        "ADC", "AND", "ASL", "BCC", "BCS", "BEQ", "BIT", "BMI",
        "BNE", "BPL", "BRK", "BVC", "BVS", "CLC", "CLD", "CLI",
        "CLV", "CMP", "CPX", "CPY", "DEC", "DEX", "DEY", "EOR",
        "INC", "INX", "INY", "JMP", "JSR", "LDA", "LDX", "LDY",
        "LSR", "NOP", "ORA", "PHA", "PHP", "PLA", "PLP", "ROL",
        "ROR", "RTI", "RTS", "SBC", "SEC", "SED", "SEI", "STA",
        "STX", "STY", "TAX", "TAY", "TSX", "TXA", "TXS", "TYA",
    ] {
        map.insert(mnemonic, ());
    }
    map
});

// 디렉티브 정의
pub static DIRECTIVES: LazyLock<HashMap<&'static str, ()>> = LazyLock::new(|| {
    let mut map = HashMap::new();
    for &directive in &[
        ".ORG", ".BYTE", ".WORD", ".DB", ".DW", ".EQU", ".INCLUDE", 
        ".MACRO", ".ENDM", ".IF", ".ELSE", ".ENDIF", "DEFINE"
    ] {
        map.insert(directive, ());
    }
    map
});
