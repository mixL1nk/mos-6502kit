use crate::{CPU, RegisterData, RegisterType};

impl CPU {
    /// Update N and Z flags based on result
    pub(crate) fn update_nz_flags(&mut self, result: u8) {
        let mut status = self.get(RegisterType::P).as_u8();
        status &= 0x7D; // Clear N,Z flags
        if result == 0 {
            status |= 0x02;
        } // Zero
        if (result & 0x80) != 0 {
            status |= 0x80;
        } // Negative
        self.set(RegisterType::P, RegisterData::Bit8(status));
    }

    /// Update all arithmetic flags (N,V,Z,C) for binary operations
    pub(crate) fn update_arithmetic_flags(&mut self, result: u8, carry: bool, overflow: bool) {
        let mut status = self.get(RegisterType::P).as_u8();
        status &= 0x3C; // Clear N,V,Z,C flags
        if carry {
            status |= 0x01;
        } // Carry
        if result == 0 {
            status |= 0x02;
        } // Zero
        if (result & 0x80) != 0 {
            status |= 0x80;
        } // Negative
        if overflow {
            status |= 0x40;
        } // Overflow
        self.set(RegisterType::P, RegisterData::Bit8(status));
    }

    /// Update flags for BCD operations (N,Z,C flags only, V is unaffected)
    pub(crate) fn update_bcd_flags(&mut self, result: u8, carry: bool) {
        let mut status = self.get(RegisterType::P).as_u8();
        status &= 0x3C; // Clear N,V,Z,C flags
        if carry {
            status |= 0x01;
        } // Carry
        if result == 0 {
            status |= 0x02;
        } // Zero
        if (result & 0x80) != 0 {
            status |= 0x80;
        } // Negative
        self.set(RegisterType::P, RegisterData::Bit8(status));
    }
}
