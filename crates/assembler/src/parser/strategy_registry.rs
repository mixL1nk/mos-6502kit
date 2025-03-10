use crate::parser::instruction::{
    ADCStrategy, ANDStrategy, ASLStrategy, BNEStrategy, CMPStrategy, CPXStrategy, CPYStrategy,
    EORStrategy, InstructionStrategy, LDAStrategy, LDXStrategy, LDYStrategy, LSRStrategy,
    ORAStrategy, STAStrategy, STXStrategy, STYStrategy, SingleByteStrategy,
};
use std::collections::HashMap;

/// Strategy registry for instruction parsing
pub struct StrategyRegistry<'a> {
    strategies: HashMap<&'static str, Option<&'a dyn InstructionStrategy>>,
}

impl Default for StrategyRegistry<'_> {
    fn default() -> Self {
        Self::new()
    }
}

impl StrategyRegistry<'_> {
    pub fn new() -> Self {
        let mut registry = Self {
            strategies: HashMap::new(),
        };
        registry.register_all_strategies();
        registry
    }

    pub fn get_strategy(&self, mnemonic: &str) -> Option<&dyn InstructionStrategy> {
        self.strategies.get(mnemonic).and_then(|opt| *opt)
    }

    pub fn has_strategy(&self, mnemonic: &str) -> bool {
        self.strategies.contains_key(mnemonic)
    }

    fn register_all_strategies(&mut self) {
        // Load instructions
        self.strategies.insert("LDA", Some(&LDAStrategy));
        self.strategies.insert("LDX", Some(&LDXStrategy));
        self.strategies.insert("LDY", Some(&LDYStrategy));

        // Store instructions
        self.strategies.insert("STA", Some(&STAStrategy));
        self.strategies.insert("STX", Some(&STXStrategy));
        self.strategies.insert("STY", Some(&STYStrategy));

        // Shift instructions
        self.strategies.insert("LSR", Some(&LSRStrategy));
        self.strategies.insert("ASL", Some(&ASLStrategy));

        // Arithmetic instructions
        self.strategies.insert("ADC", Some(&ADCStrategy));

        // Logical instructions
        self.strategies.insert("AND", Some(&ANDStrategy));
        self.strategies.insert("ORA", Some(&ORAStrategy));
        self.strategies.insert("EOR", Some(&EORStrategy));

        // Compare instructions
        self.strategies.insert("CMP", Some(&CMPStrategy));
        self.strategies.insert("CPY", Some(&CPYStrategy));
        self.strategies.insert("CPX", Some(&CPXStrategy));

        // Branch instructions
        self.strategies.insert("BNE", Some(&BNEStrategy));

        // Single byte instructions
        self.register_single_byte_instructions();
    }

    fn register_single_byte_instructions(&mut self) {
        // Register transfer instructions
        self.strategies
            .insert("TAX", Some(&SingleByteStrategy("TAX")));
        self.strategies
            .insert("TXA", Some(&SingleByteStrategy("TXA")));
        self.strategies
            .insert("TAY", Some(&SingleByteStrategy("TAY")));
        self.strategies
            .insert("TYA", Some(&SingleByteStrategy("TYA")));

        // Increment/decrement instructions
        self.strategies
            .insert("INX", Some(&SingleByteStrategy("INX")));
        self.strategies
            .insert("INY", Some(&SingleByteStrategy("INY")));
        self.strategies
            .insert("DEX", Some(&SingleByteStrategy("DEX")));
        self.strategies
            .insert("DEY", Some(&SingleByteStrategy("DEY")));

        // Flag instructions
        self.strategies
            .insert("CLC", Some(&SingleByteStrategy("CLC")));
        self.strategies
            .insert("SEC", Some(&SingleByteStrategy("SEC")));
        self.strategies
            .insert("CLI", Some(&SingleByteStrategy("CLI")));
        self.strategies
            .insert("SEI", Some(&SingleByteStrategy("SEI")));
        self.strategies
            .insert("CLV", Some(&SingleByteStrategy("CLV")));
        self.strategies
            .insert("CLD", Some(&SingleByteStrategy("CLD")));
        self.strategies
            .insert("SED", Some(&SingleByteStrategy("SED")));

        // System instructions
        self.strategies
            .insert("NOP", Some(&SingleByteStrategy("NOP")));
        self.strategies
            .insert("BRK", Some(&SingleByteStrategy("BRK")));
        self.strategies
            .insert("RTI", Some(&SingleByteStrategy("RTI")));
        self.strategies
            .insert("RTS", Some(&SingleByteStrategy("RTS")));

        // Stack instructions
        self.strategies
            .insert("PHA", Some(&SingleByteStrategy("PHA")));
        self.strategies
            .insert("PLA", Some(&SingleByteStrategy("PLA")));
        self.strategies
            .insert("PHP", Some(&SingleByteStrategy("PHP")));
        self.strategies
            .insert("PLP", Some(&SingleByteStrategy("PLP")));
        self.strategies
            .insert("TSX", Some(&SingleByteStrategy("TSX")));
        self.strategies
            .insert("TXS", Some(&SingleByteStrategy("TXS")));
    }
}
