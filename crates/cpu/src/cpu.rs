//! CPU 에 대한 기본 정보
use crate::executor::InstructionExecutor;
use crate::instruction::{InstructionDecoder, InstructionInfo, OperandSize};
use crate::register::{RegisterData, RegisterType, Registers};
use common::Result;
use common::bus::memory_bus::MemoryBus;
use common::bus::{BusInterface, BusOperationType, BusTransaction};
use std::sync::{Arc, Mutex};

/// CPU 인터럽트 타입
#[derive(Debug, Clone, Copy)]
pub enum InterruptType {
    /// 리셋 인터럽트
    Reset,
    /// 비마스크에블 인터럽트 요청
    IRQ,
    /// 마스크불가능 인터럽트 요청
    NMI,
    /// 소프트웨어 인터럽트 (BRK 명령)
    BRK,
}

/// CPU 구조체
#[derive(Debug)]
pub struct CPU {
    /// 레지스터 값들
    registers: Registers,
    /// 메모리 버스 접근자 (옵션 - 설정되지 않았을 수 있음)
    memory_bus: Option<Arc<Mutex<dyn MemoryBus>>>,
    instruction: InstructionDecoder,
}
pub struct Fetch {
    pub instruction_info: InstructionInfo,
    pub operand: Vec<u8>,
}

impl Fetch {
    fn new(instruction_info: InstructionInfo, operand: Vec<u8>) -> Self {
        Self {
            instruction_info,
            operand,
        }
    }
}

impl Default for CPU {
    fn default() -> Self {
        Self::new()
    }
}

impl CPU {
    /// 새로운 CPU 인스턴스 생성
    pub fn new() -> Self {
        Self {
            registers: Registers::default(),
            memory_bus: None,
            instruction: InstructionDecoder::new(),
        }
    }

    /// CPU 리셋
    pub fn reset(&mut self) {
        self.registers = Registers::default();
    }

    /// 레지스터 값 가져오기
    pub fn get(&self, reg: RegisterType) -> RegisterData {
        self.registers.get(reg)
    }

    /// 레지스터 값 설정
    pub fn set(&mut self, reg: RegisterType, value: RegisterData) {
        self.registers.set(reg, value);
    }

    /// CPU 클럭 실행
    pub fn clock(&mut self) {
        println!("[+] CPU clock cycle");
    }

    /// CPU ID 가져오기
    pub fn get_id(&self) -> &str {
        "CPU"
    }

    /// 메모리 버스 설정 - 외부 매니저에서 호출
    pub fn set_memory_bus(&mut self, bus: Arc<Mutex<dyn MemoryBus>>) {
        self.memory_bus = Some(bus);
    }

    /// PC 레지스터 값 가져오기
    fn get_pc(&self) -> u16 {
        self.registers.get(RegisterType::PC).as_u16()
    }

    /// PC 레지스터 값 설정
    fn set_pc(&mut self, value: u16) {
        self.registers
            .set(RegisterType::PC, RegisterData::Bit16(value));
    }

    /// PC 레지스터 증가
    fn increment_pc(&mut self, increment: u16) {
        let pc = self.get_pc();
        self.set_pc(pc.wrapping_add(increment));
    }

    /// opcode 가져오기
    fn fetch_opcode(&mut self) -> Result<u8> {
        let pc = self.get_pc();
        let opcode = self.read_memory(pc)?;
        self.increment_pc(1);
        Ok(opcode)
    }

    /// 메모리에서 PC 위치의 데이터 읽기
    fn fetch(&mut self) -> Result<Fetch> {
        // PC 레지스터에서 주소 가져오기
        let opcode = self.fetch_opcode()?;
        let ins = self.instruction.get_instruction_info(opcode).unwrap();
        let need = ins.get_operand_size();
        let operand = match need {
            OperandSize::One => {
                let operand = self.read_memory(self.get_pc())?;
                println!("[CPU] Fetched operand: 0x{:02X}", operand);
                self.increment_pc(1);
                vec![operand]
            }
            OperandSize::Two => {
                let low_byte = self.read_memory(self.get_pc())?;
                self.increment_pc(1);
                let high_byte = self.read_memory(self.get_pc())?;
                self.increment_pc(1);
                let operand = u16::from_le_bytes([low_byte, high_byte]);
                println!("[CPU] Fetched operand: 0x{:04X}", operand);
                vec![low_byte, high_byte]
            }
            OperandSize::Zero => {
                vec![]
            }
        };

        Ok(Fetch::new(ins, operand))
    }

    /// 메모리 읽기 (MemoryBus 사용)
    pub fn read_memory(&self, address: u16) -> Result<u8> {
        if let Some(bus) = &self.memory_bus {
            let value = bus
                .lock()
                .map_err(|_| "Failed to lock memory bus".to_string())?
                .read(address);
            Ok(value)
        } else {
            Err("Memory bus not connected".into())
        }
    }

    /// 메모리 쓰기 (MemoryBus 사용)
    pub fn write_memory(&self, address: u16, value: u8) -> Result<()> {
        if let Some(bus) = &self.memory_bus {
            bus.lock()
                .map_err(|_| "Failed to lock memory bus".to_string())?
                .write(address, value);
            Ok(())
        } else {
            Err("Memory bus not connected".into())
        }
    }

    /// 명령어 실행 사이클
    pub fn execute_cycle(&mut self) -> Result<()> {
        // 1. 명령어 가져오기
        let fetch = self.fetch()?;

        let decode = self.instruction.decode(fetch)?;
        // 2. 명령어 디코드 및 실행
        let _e = self.execute(decode);
        Ok(())
    }
}

// CPU에 BusInterface 트레이트 구현
impl BusInterface for CPU {
    fn process_bus_transaction(&mut self, _transaction: &mut BusTransaction) -> Result<()> {
        // CPU는 주로 트랜잭션을 시작하는 쪽이므로 여기선 처리 불필요
        Err("CPU does not process incoming transactions".into())
    }

    fn begin_transaction(&mut self, transaction: BusTransaction) -> Result<BusTransaction> {
        println!(
            "[BUS] CPU initiating transaction: {:?}",
            transaction.operation_type
        );
        // 여기서 CPU 관련 로직 수행 (예: 레지스터 업데이트)
        Ok(transaction)
    }

    fn respond_to_transaction(&mut self, transaction: &mut BusTransaction) -> Result<()> {
        println!(
            "[BUS] CPU received transaction: {:?}",
            transaction.operation_type
        );
        match transaction.operation_type {
            BusOperationType::Read => {
                println!(
                    "[BUS] CPU received read response, data=0x{:02x}",
                    transaction.data
                );
                // 읽은 데이터를 CPU 레지스터에 저장하는 등의 처리
                Ok(())
            }
            BusOperationType::Write => {
                println!("[BUS] CPU received write response");
                // 쓰기 응답 처리
                Ok(())
            }
            _ => Err("Invalid response type for CPU".into()),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::register::StatusRegister;

    #[test]
    fn test_cpu_reset() {
        let mut cpu = CPU::new();
        cpu.reset();

        assert_eq!(cpu.get(RegisterType::A), RegisterData::Bit8(0));
        assert_eq!(cpu.get(RegisterType::X), RegisterData::Bit8(0));
        assert_eq!(cpu.get(RegisterType::Y), RegisterData::Bit8(0));
        assert_eq!(
            cpu.get(RegisterType::P),
            RegisterData::Bit8(StatusRegister::UNUSED.bits())
        );
        assert_eq!(cpu.get(RegisterType::S), RegisterData::Bit8(0xFD));
        assert_eq!(cpu.get(RegisterType::PC), RegisterData::Bit16(0));

        cpu.set(RegisterType::A, RegisterData::Bit8(0x10));
        cpu.set(RegisterType::X, RegisterData::Bit8(0x20));
        cpu.set(RegisterType::Y, RegisterData::Bit8(0x30));
        cpu.set(
            RegisterType::P,
            RegisterData::Bit8(0x40 | StatusRegister::UNUSED.bits()),
        );
        cpu.set(RegisterType::S, RegisterData::Bit8(0x50));
        cpu.set(RegisterType::PC, RegisterData::Bit16(0x60));

        assert_eq!(cpu.get(RegisterType::A), RegisterData::Bit8(0x10));
        assert_eq!(cpu.get(RegisterType::X), RegisterData::Bit8(0x20));
        assert_eq!(cpu.get(RegisterType::Y), RegisterData::Bit8(0x30));
        assert_eq!(
            cpu.get(RegisterType::P),
            RegisterData::Bit8(0x40 | StatusRegister::UNUSED.bits())
        );
        assert_eq!(cpu.get(RegisterType::S), RegisterData::Bit8(0x50));
        assert_eq!(cpu.get(RegisterType::PC), RegisterData::Bit16(0x60));
    }
}
