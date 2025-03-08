use crate::{CPU, instruction::DecodedInstruction};
use crate::{RegisterData, RegisterType};
use common::Result;
use error::Error;
use types::{AddressMode, Instruction};

impl CPU {
    pub(super) fn execute_transfer(&mut self, decoded: DecodedInstruction) -> Result<()> {
        println!(
            "[CPU] Executing transfer instruction: {:?}",
            decoded.info.instruction
        );
        match decoded.info.instruction {
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

    fn lda(&mut self, _mode: AddressMode, decode: DecodedInstruction) -> Result<()> {
        println!("[CPU] Executing LDA with operand: 0x{:04X}", decode.operand);
        let value = decode.operand as u8;
        self.set_value(RegisterType::A, RegisterData::Bit8(value));
        self.update_nz_flags(value);
        Ok(())
    }

    fn ldx(&mut self, _mode: AddressMode, decode: DecodedInstruction) -> Result<()> {
        println!("[CPU] Executing LDX with operand: 0x{:04X}", decode.operand);
        let value = decode.operand as u8;
        self.set_value(RegisterType::X, RegisterData::Bit8(value));
        self.update_nz_flags(value);
        Ok(())
    }

    fn ldy(&mut self, _mode: AddressMode, decode: DecodedInstruction) -> Result<()> {
        println!("[CPU] Executing LDY with operand: 0x{:04X}", decode.operand);
        let value = decode.operand as u8;
        self.set_value(RegisterType::Y, RegisterData::Bit8(value));
        self.update_nz_flags(value);
        Ok(())
    }

    fn sta(&mut self, _mode: AddressMode, decode: DecodedInstruction) -> Result<()> {
        println!("[CPU] Executing STA with operand: 0x{:04X}", decode.operand);
        let addr = decode.operand;
        let value = self.get_value(RegisterType::A).as_u8();
        self.write_memory(addr, value)?;
        Ok(())
    }

    fn stx(&mut self, _mode: AddressMode, decode: DecodedInstruction) -> Result<()> {
        println!("[CPU] Executing STX with operand: 0x{:04X}", decode.operand);
        let addr = decode.operand;
        let value = self.get_value(RegisterType::X).as_u8();
        self.write_memory(addr, value)?;
        Ok(())
    }

    fn sty(&mut self, _mode: AddressMode, decode: DecodedInstruction) -> Result<()> {
        println!("[CPU] Executing STY with operand: 0x{:04X}", decode.operand);
        let addr = decode.operand;
        let value = self.get_value(RegisterType::Y).as_u8();
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
        // Note: TXS does not affect any flags
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
