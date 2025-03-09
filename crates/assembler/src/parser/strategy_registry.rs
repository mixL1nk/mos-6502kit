use crate::parser::instruction::{
    ADCStrategy, ANDStrategy, ASLStrategy, BNEStrategy, CMPStrategy, CPXStrategy, CPYStrategy,
    EORStrategy, InstructionStrategy, LDAStrategy, LDXStrategy, LDYStrategy, LSRStrategy,
    ORAStrategy, STAStrategy, STXStrategy, STYStrategy, SingleByteStrategy,
};
use std::collections::HashMap;

/// Strategy registry for instruction parsing
pub struct StrategyRegistry {
    strategies: HashMap<&'static str, Box<dyn InstructionStrategy>>,
}

impl StrategyRegistry {
    pub fn new() -> Self {
        let mut registry = Self {
            strategies: HashMap::new(),
        };
        registry.register_all_strategies();
        registry
    }

    pub fn get_strategy(&self, mnemonic: &str) -> Option<&Box<dyn InstructionStrategy>> {
        self.strategies.get(mnemonic)
    }

    pub fn has_strategy(&self, mnemonic: &str) -> bool {
        self.strategies.contains_key(mnemonic)
    }

    fn register_all_strategies(&mut self) {
        // Load instructions
        self.strategies.insert("LDA", Box::new(LDAStrategy));
        self.strategies.insert("LDX", Box::new(LDXStrategy));
        self.strategies.insert("LDY", Box::new(LDYStrategy));

        // Store instructions
        self.strategies.insert("STA", Box::new(STAStrategy));
        self.strategies.insert("STX", Box::new(STXStrategy));
        self.strategies.insert("STY", Box::new(STYStrategy));

        // Shift instructions
        self.strategies.insert("LSR", Box::new(LSRStrategy));
        self.strategies.insert("ASL", Box::new(ASLStrategy));

        // Arithmetic instructions
        self.strategies.insert("ADC", Box::new(ADCStrategy));

        // Logical instructions
        self.strategies.insert("AND", Box::new(ANDStrategy));
        self.strategies.insert("ORA", Box::new(ORAStrategy));
        self.strategies.insert("EOR", Box::new(EORStrategy));

        // Compare instructions
        self.strategies.insert("CMP", Box::new(CMPStrategy));
        self.strategies.insert("CPY", Box::new(CPYStrategy));
        self.strategies.insert("CPX", Box::new(CPXStrategy));

        // Branch instructions
        self.strategies.insert("BNE", Box::new(BNEStrategy));

        // Single byte instructions
        self.register_single_byte_instructions();
    }

    fn register_single_byte_instructions(&mut self) {
        // Register transfer instructions
        self.strategies.insert("TAX", Box::new(SingleByteStrategy("TAX")));
        self.strategies.insert("TXA", Box::new(SingleByteStrategy("TXA")));
        self.strategies.insert("TAY", Box::new(SingleByteStrategy("TAY")));
        self.strategies.insert("TYA", Box::new(SingleByteStrategy("TYA")));
        
        // Increment/decrement instructions
        self.strategies.insert("INX", Box::new(SingleByteStrategy("INX")));
        self.strategies.insert("INY", Box::new(SingleByteStrategy("INY")));
        self.strategies.insert("DEX", Box::new(SingleByteStrategy("DEX")));
        self.strategies.insert("DEY", Box::new(SingleByteStrategy("DEY")));
        
        // Flag instructions
        self.strategies.insert("CLC", Box::new(SingleByteStrategy("CLC")));
        self.strategies.insert("SEC", Box::new(SingleByteStrategy("SEC")));
        self.strategies.insert("CLI", Box::new(SingleByteStrategy("CLI")));
        self.strategies.insert("SEI", Box::new(SingleByteStrategy("SEI")));
        self.strategies.insert("CLV", Box::new(SingleByteStrategy("CLV")));
        self.strategies.insert("CLD", Box::new(SingleByteStrategy("CLD")));
        self.strategies.insert("SED", Box::new(SingleByteStrategy("SED")));
        
        // System instructions
        self.strategies.insert("NOP", Box::new(SingleByteStrategy("NOP")));
        self.strategies.insert("BRK", Box::new(SingleByteStrategy("BRK")));
        self.strategies.insert("RTI", Box::new(SingleByteStrategy("RTI")));
        self.strategies.insert("RTS", Box::new(SingleByteStrategy("RTS")));
        
        // Stack instructions
        self.strategies.insert("PHA", Box::new(SingleByteStrategy("PHA")));
        self.strategies.insert("PLA", Box::new(SingleByteStrategy("PLA")));
        self.strategies.insert("PHP", Box::new(SingleByteStrategy("PHP")));
        self.strategies.insert("PLP", Box::new(SingleByteStrategy("PLP")));
        self.strategies.insert("TSX", Box::new(SingleByteStrategy("TSX")));
        self.strategies.insert("TXS", Box::new(SingleByteStrategy("TXS")));
    }
} 