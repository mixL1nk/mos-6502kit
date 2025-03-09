use crate::{
    cpu::CPU,
    instruction::DecodedInstruction,
    register::{RegisterData, RegisterType},
};
use common::Result;
use error::Error;
use types::{AddressModeValue, Instruction};

/// 데이터 전송 명령어 구현
pub trait TransferOperation {
    /// LDA - Load Accumulator
    fn lda(&mut self, mode: AddressModeValue, decode: DecodedInstruction) -> Result<()>;

    /// LDX - Load X Register
    fn ldx(&mut self, mode: AddressModeValue, decode: DecodedInstruction) -> Result<()>;

    /// LDY - Load Y Register
    fn ldy(&mut self, mode: AddressModeValue, decode: DecodedInstruction) -> Result<()>;

    /// STA - Store Accumulator
    fn sta(&mut self, mode: AddressModeValue, decode: DecodedInstruction) -> Result<()>;

    /// STX - Store X Register
    fn stx(&mut self, mode: AddressModeValue, decode: DecodedInstruction) -> Result<()>;

    /// STY - Store Y Register
    fn sty(&mut self, mode: AddressModeValue, decode: DecodedInstruction) -> Result<()>;

    /// TAX - Transfer Accumulator to X
    fn tax(&mut self) -> Result<()>;

    /// TAY - Transfer Accumulator to Y
    fn tay(&mut self) -> Result<()>;

    /// TSX - Transfer Stack Pointer to X
    fn tsx(&mut self) -> Result<()>;

    /// TXA - Transfer X to Accumulator
    fn txa(&mut self) -> Result<()>;

    /// TXS - Transfer X to Stack Pointer
    fn txs(&mut self) -> Result<()>;

    /// TYA - Transfer Y to Accumulator
    fn tya(&mut self) -> Result<()>;
}

impl TransferOperation for CPU {
    fn lda(&mut self, mode: AddressModeValue, _decode: DecodedInstruction) -> Result<()> {
        println!("[CPU] Executing LDA with mode: {:?}", mode);

        let value = match mode {
            AddressModeValue::Immediate(val) => val,
            AddressModeValue::ZeroPage(addr) => self.read_memory(addr as u16)?,
            AddressModeValue::ZeroPageX(addr) => {
                let effective_addr =
                    addr.wrapping_add(self.get_value(RegisterType::X).as_u8()) as u16;
                self.read_memory(effective_addr)?
            }
            AddressModeValue::Absolute(addr) => self.read_memory(addr)?,
            AddressModeValue::AbsoluteX(addr) => {
                let x = self.get_value(RegisterType::X).as_u8() as u16;
                let effective_addr = addr.wrapping_add(x);
                self.read_memory(effective_addr)?
            }
            AddressModeValue::AbsoluteY(addr) => {
                let y = self.get_value(RegisterType::Y).as_u8() as u16;
                let effective_addr = addr.wrapping_add(y);
                self.read_memory(effective_addr)?
            }
            AddressModeValue::IndirectX(addr) => {
                let x = self.get_value(RegisterType::X).as_u8();
                let ptr = addr.wrapping_add(x) as u16;
                let low = self.read_memory(ptr)? as u16;
                let high = self.read_memory(ptr.wrapping_add(1))? as u16;
                let effective_addr = (high << 8) | low;
                self.read_memory(effective_addr)?
            }
            AddressModeValue::IndirectY(addr) => {
                let ptr = addr as u16;
                let low = self.read_memory(ptr)? as u16;
                let high = self.read_memory(ptr.wrapping_add(1))? as u16;
                let base_addr = (high << 8) | low;
                let y = self.get_value(RegisterType::Y).as_u8() as u16;
                let effective_addr = base_addr.wrapping_add(y);
                self.read_memory(effective_addr)?
            }
            _ => return Err(Error::InvalidAddressingMode("LDA")),
        };
        println!("[CPU] Loaded value: 0x{:02X}", value);

        self.set_value(RegisterType::A, RegisterData::Bit8(value));
        self.update_nz_flags(value);
        Ok(())
    }

    fn ldx(&mut self, mode: AddressModeValue, _decode: DecodedInstruction) -> Result<()> {
        println!("[CPU] Executing LDX with mode: {:?}", mode);

        let value = match mode {
            AddressModeValue::Immediate(val) => val,
            AddressModeValue::ZeroPage(addr) => self.read_memory(addr as u16)?,
            AddressModeValue::ZeroPageY(addr) => {
                let effective_addr =
                    addr.wrapping_add(self.get_value(RegisterType::Y).as_u8()) as u16;
                self.read_memory(effective_addr)?
            }
            AddressModeValue::Absolute(addr) => self.read_memory(addr)?,
            AddressModeValue::AbsoluteY(addr) => {
                let y = self.get_value(RegisterType::Y).as_u8() as u16;
                let effective_addr = addr.wrapping_add(y);
                self.read_memory(effective_addr)?
            }
            _ => return Err(Error::InvalidAddressingMode("LDX")),
        };

        self.set_value(RegisterType::X, RegisterData::Bit8(value));
        self.update_nz_flags(value);

        Ok(())
    }

    fn ldy(&mut self, mode: AddressModeValue, _decode: DecodedInstruction) -> Result<()> {
        println!("[CPU] Executing LDY with mode: {:?}", mode);

        let value = match mode {
            AddressModeValue::Immediate(val) => val,
            AddressModeValue::ZeroPage(addr) => self.read_memory(addr as u16)?,
            AddressModeValue::ZeroPageX(addr) => {
                let effective_addr =
                    addr.wrapping_add(self.get_value(RegisterType::X).as_u8()) as u16;
                self.read_memory(effective_addr)?
            }
            AddressModeValue::Absolute(addr) => self.read_memory(addr)?,
            AddressModeValue::AbsoluteX(addr) => {
                let x = self.get_value(RegisterType::X).as_u8() as u16;
                let effective_addr = addr.wrapping_add(x);
                self.read_memory(effective_addr)?
            }
            _ => return Err(Error::InvalidAddressingMode("LDY")),
        };

        self.set_value(RegisterType::Y, RegisterData::Bit8(value));
        self.update_nz_flags(value);

        Ok(())
    }

    fn sta(&mut self, mode: AddressModeValue, _decode: DecodedInstruction) -> Result<()> {
        println!("[CPU] Executing STA with mode: {:?}", mode);
        let value = self.get_value(RegisterType::A).as_u8();

        let addr = match mode {
            AddressModeValue::ZeroPage(addr) => addr as u16,
            AddressModeValue::ZeroPageX(addr) => {
                let effective_addr = addr.wrapping_add(self.get_value(RegisterType::X).as_u8());
                effective_addr as u16
            }
            AddressModeValue::Absolute(addr) => addr,
            AddressModeValue::AbsoluteX(addr) => {
                let x = self.get_value(RegisterType::X).as_u8() as u16;
                addr.wrapping_add(x)
            }
            AddressModeValue::AbsoluteY(addr) => {
                let y = self.get_value(RegisterType::Y).as_u8() as u16;
                addr.wrapping_add(y)
            }
            AddressModeValue::IndirectX(addr) => {
                let x = self.get_value(RegisterType::X).as_u8();
                let ptr = addr.wrapping_add(x) as u16;
                let low = self.read_memory(ptr)? as u16;
                let high = self.read_memory(ptr.wrapping_add(1))? as u16;
                (high << 8) | low
            }
            AddressModeValue::IndirectY(addr) => {
                let ptr = addr as u16;
                let low = self.read_memory(ptr)? as u16;
                let high = self.read_memory(ptr.wrapping_add(1))? as u16;
                let base_addr = (high << 8) | low;
                let y = self.get_value(RegisterType::Y).as_u8() as u16;
                base_addr.wrapping_add(y)
            }
            _ => return Err(Error::InvalidAddressingMode("STA")),
        };

        self.write_memory(addr, value)?;
        Ok(())
    }

    fn stx(&mut self, mode: AddressModeValue, _decode: DecodedInstruction) -> Result<()> {
        println!("[CPU] Executing STX with mode: {:?}", mode);
        let value = self.get_value(RegisterType::X).as_u8();

        let addr = match mode {
            AddressModeValue::ZeroPage(addr) => addr as u16,
            AddressModeValue::ZeroPageY(addr) => {
                let effective_addr = addr.wrapping_add(self.get_value(RegisterType::Y).as_u8());
                effective_addr as u16
            }
            AddressModeValue::Absolute(addr) => addr,
            _ => return Err(Error::InvalidAddressingMode("STX")),
        };

        self.write_memory(addr, value)?;
        Ok(())
    }

    fn sty(&mut self, mode: AddressModeValue, _decode: DecodedInstruction) -> Result<()> {
        println!("[CPU] Executing STY with mode: {:?}", mode);
        let value = self.get_value(RegisterType::Y).as_u8();

        let addr = match mode {
            AddressModeValue::ZeroPage(addr) => addr as u16,
            AddressModeValue::ZeroPageX(addr) => {
                let effective_addr = addr.wrapping_add(self.get_value(RegisterType::X).as_u8());
                effective_addr as u16
            }
            AddressModeValue::Absolute(addr) => addr,
            _ => return Err(Error::InvalidAddressingMode("STY")),
        };

        self.write_memory(addr, value)?;
        Ok(())
    }

    fn tax(&mut self) -> Result<()> {
        println!("[CPU] Executing TAX");
        let value = self.get_value(RegisterType::A).as_u8();
        self.set_value(RegisterType::X, RegisterData::Bit8(value));
        self.update_nz_flags(value);
        Ok(())
    }

    fn tay(&mut self) -> Result<()> {
        println!("[CPU] Executing TAY");
        let value = self.get_value(RegisterType::A).as_u8();
        self.set_value(RegisterType::Y, RegisterData::Bit8(value));
        self.update_nz_flags(value);
        Ok(())
    }

    fn tsx(&mut self) -> Result<()> {
        println!("[CPU] Executing TSX");
        let value = self.get_value(RegisterType::S).as_u8();
        self.set_value(RegisterType::X, RegisterData::Bit8(value));
        self.update_nz_flags(value);
        Ok(())
    }

    fn txa(&mut self) -> Result<()> {
        println!("[CPU] Executing TXA");
        let value = self.get_value(RegisterType::X).as_u8();
        self.set_value(RegisterType::A, RegisterData::Bit8(value));
        self.update_nz_flags(value);
        Ok(())
    }

    fn txs(&mut self) -> Result<()> {
        println!("[CPU] Executing TXS");
        let value = self.get_value(RegisterType::X).as_u8();
        self.set_value(RegisterType::S, RegisterData::Bit8(value));
        Ok(())
    }

    fn tya(&mut self) -> Result<()> {
        println!("[CPU] Executing TYA");
        let value = self.get_value(RegisterType::Y).as_u8();
        self.set_value(RegisterType::A, RegisterData::Bit8(value));
        self.update_nz_flags(value);
        Ok(())
    }
}

impl CPU {
    pub(super) fn execute_transfer(&mut self, decoded: DecodedInstruction) -> Result<()> {
        println!(
            "[CPU] Executing transfer instruction: {:?}",
            decoded.instruction
        );
        match decoded.instruction {
            Instruction::LDA(mode) => self.lda(mode, decoded),
            Instruction::LDX(mode) => self.ldx(mode, decoded),
            Instruction::LDY(mode) => self.ldy(mode, decoded),
            Instruction::STA(mode) => self.sta(mode, decoded),
            Instruction::STX(mode) => self.stx(mode, decoded),
            Instruction::STY(mode) => self.sty(mode, decoded),
            Instruction::TAX => self.tax(),
            Instruction::TAY => self.tay(),
            Instruction::TSX => self.tsx(),
            Instruction::TXA => self.txa(),
            Instruction::TXS => self.txs(),
            Instruction::TYA => self.tya(),
            _ => Err(Error::InvalidInstruction {
                inst_type: "transfer",
            }),
        }
    }
}
