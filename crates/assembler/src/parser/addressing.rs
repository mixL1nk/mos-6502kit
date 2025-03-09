use common::Result;
use error::Error;
use std::collections::HashMap;
use types::AddressModeValue;

/// 어드레싱 모드 파서
#[derive(Clone)]
pub struct AddressingModeParser {
    labels: HashMap<String, u16>,
}

impl AddressingModeParser {
    pub fn new(labels: HashMap<String, u16>) -> Self {
        Self { labels }
    }

    pub fn add_label(&mut self, name: String, address: u16) {
        self.labels.insert(name, address);
    }

    pub fn parse_immediate(&self, value: u8) -> Result<AddressModeValue> {
        Ok(AddressModeValue::Immediate(value))
    }

    pub fn parse_zero_page(&self, addr: u8) -> Result<AddressModeValue> {
        Ok(AddressModeValue::ZeroPage(addr))
    }

    pub fn parse_zero_page_x(&self, addr: u8) -> Result<AddressModeValue> {
        Ok(AddressModeValue::ZeroPageX(addr))
    }

    pub fn parse_zero_page_y(&self, addr: u8) -> Result<AddressModeValue> {
        Ok(AddressModeValue::ZeroPageY(addr))
    }

    pub fn parse_absolute(&self, addr: u16) -> Result<AddressModeValue> {
        Ok(AddressModeValue::Absolute(addr))
    }

    pub fn parse_absolute_x(&self, addr: u16) -> Result<AddressModeValue> {
        Ok(AddressModeValue::AbsoluteX(addr))
    }

    pub fn parse_absolute_y(&self, addr: u16) -> Result<AddressModeValue> {
        Ok(AddressModeValue::AbsoluteY(addr))
    }

    pub fn parse_indirect(&self, addr: u16) -> Result<AddressModeValue> {
        Ok(AddressModeValue::Indirect(addr))
    }

    pub fn parse_indirect_x(&self, addr: u8) -> Result<AddressModeValue> {
        Ok(AddressModeValue::IndirectX(addr))
    }

    pub fn parse_indirect_y(&self, addr: u8) -> Result<AddressModeValue> {
        Ok(AddressModeValue::IndirectY(addr))
    }

    pub fn resolve_label(&self, label: &str) -> Result<u16> {
        self.labels
            .get(label)
            .copied()
            .ok_or_else(|| Error::AssemblerUndefinedLabel(label.to_string()))
    }

    pub fn get_labels(&self) -> &HashMap<String, u16> {
        &self.labels
    }
}
