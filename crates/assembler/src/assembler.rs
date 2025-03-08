use common::Result;

use crate::lexer::Lexer;

pub struct Assembler {
    output: Vec<u8>,
}

impl Default for Assembler {
    fn default() -> Self {
        
        Self::new()
    }
}

impl Assembler {
    pub fn new() -> Self {
        Assembler { output: Vec::new() }
    }

    pub fn assemble(&mut self, source: &'static str) -> Result<()> {
        let lexer = Lexer::new(source);
        let tokens = lexer.tokenize()?;
        println!("tokens: {:?}", tokens);
        // let instructions = parse(&tokens)?;
        // self.output = assemble(&instructions)?;
        Ok(())
    }

}

// 렉서를 사용하는 코드 예시
fn process_source_code(source: &'static str) -> Result<()> {
    let lexer = Lexer::new(source);
    let tokens = lexer.tokenize()?;
    
    // 토큰으로 어셈블러 작업 수행
    // ...
    
    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_assemble() {
        let mut assembler = Assembler::new();
        let source = "123";
        let result = assembler.assemble(source);
        assert!(result.is_ok());
    }
}
