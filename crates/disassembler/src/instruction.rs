use derive_more::{Display, From};

#[derive(From, Display, Debug)]
#[display("0x{address:04x} {base_instruction} {mnemonic} ")]
pub struct DInstruction {
    pub opcode: u8,
    pub operand: Vec<u8>,
    pub mnemonic: String,
    pub base_instruction: String,
    pub address: u16,
}

impl Default for DInstruction {
    fn default() -> Self {
        Self::new(0, vec![], String::new(), String::new(), 0)
    }
}

impl DInstruction {
    pub fn new(
        opcode: u8,
        operand: Vec<u8>,
        mnemonic: String,
        base_instruction: String,
        address: u16,
    ) -> Self {
        Self {
            opcode,
            operand,
            mnemonic,
            base_instruction,
            address,
        }
    }
}
