pub mod addressing_mode_parser;
pub mod branch_instruction_parser;
pub mod branch_parser;
pub mod instruction;
pub mod instruction_size;
pub mod jump_instruction_parser;
pub mod label_collector;
pub mod parser_main;
pub mod strategy_registry;
pub mod token_parser;

pub use parser_main::Parser;
