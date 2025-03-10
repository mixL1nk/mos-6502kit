use crate::CPU;
use crate::register::StatusRegister;
impl CPU {
    /// Update N and Z flags based on result
    pub fn update_nz_flags(&mut self, result: u8) {
        self.set_flag(StatusRegister::ZERO, result == 0);
        self.set_flag(StatusRegister::NEGATIVE, (result & 0x80) != 0);
    }

    /// Update all arithmetic flags (N,V,Z,C) for binary operations
    pub fn update_flags_arithmetic(&mut self, result: u8, carry: bool, overflow: bool) {
        self.update_nz_flags(result);
        self.set_flag(StatusRegister::CARRY, carry);
        self.set_flag(StatusRegister::OVERFLOW, overflow);
    }

    //TODO: BCD 플래그 테스트 해야함 불필요하면 제거 예정
    /// Update flags for BCD operations (N,Z,C flags only, V is unaffected)
    pub(crate) fn _update_bcd_flags(&mut self, result: u8, carry: bool) {
        self.set_flag(StatusRegister::CARRY, carry);
        self.update_nz_flags(result);
    }
}
