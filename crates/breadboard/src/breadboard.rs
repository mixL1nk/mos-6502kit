use cpu::CPU;
use memory::Memory;
use common::Result;
use common::bus::memory_bus::MemoryBus;
use std::sync::{Arc, Mutex, RwLock};

/// 브레드보드 - CPU와 메모리를 연결하는 간단한 구현
pub struct BreadBoard {
    /// CPU 인스턴스
    pub cpu: CPU,
    /// 메모리 인스턴스 - 공유 참조로 사용
    memory: Arc<RwLock<Memory>>,
}

/// 메모리 버스 구현체 - 공유 메모리 참조 사용
#[derive(Debug)]
struct SharedMemoryBus {
    /// 메모리 공유 참조
    memory: Arc<RwLock<Memory>>,
}

impl MemoryBus for SharedMemoryBus {
    fn read(&self, address: u16) -> u8 {
        // 읽기 락 사용
        let memory = self.memory.read().unwrap();
        match memory.get(address) {
            Ok(value) => {
                println!("[MemoryBus] Read: addr=0x{:04x}, data=0x{:02x}", address, value);
                value
            },
            Err(_) => {
                println!("[MemoryBus] Read error at 0x{:04x}, returning 0", address);
                0
            }
        }
    }

    fn write(&mut self, address: u16, value: u8) {
        // 쓰기 락 사용
        let mut memory = self.memory.write().unwrap();
        if let Err(e) = memory.set(address, value) {
            println!("[MemoryBus] Write error at 0x{:04x}: {}", address, e);
        } else {
            println!("[MemoryBus] Write: addr=0x{:04x}, data=0x{:02x}", address, value);
        }
    }
}

impl BreadBoard {
    /// 새로운 브레드보드 생성
    pub fn new() -> Self {
        // 메모리 생성 및 공유
        let memory = Arc::new(RwLock::new(Memory::new()));
        
        // 메모리 버스 생성
        let memory_bus = Arc::new(Mutex::new(SharedMemoryBus { 
            memory: memory.clone() 
        }));
        
        // CPU 생성 및 메모리 버스 연결
        let mut cpu = CPU::new();
        cpu.set_memory_bus(memory_bus.clone());
        
        Self {
            cpu,
            memory,
        }
    }
    
    /// 메모리 직접 접근 (임시)
    pub fn memory(&self) -> Arc<RwLock<Memory>> {
        self.memory.clone()
    }
    
    /// 메모리에 프로그램 로드
    pub fn load_program(&self, address: u16, program: &[u8]) -> Result<()> {
        let mut memory = self.memory.write().unwrap();
        for (i, &byte) in program.iter().enumerate() {
            memory.set(address.wrapping_add(i as u16), byte)?;
        }
        Ok(())
    }
    
    /// CPU PC 설정
    pub fn set_pc(&mut self, address: u16) {
        self.cpu.set(cpu::register::RegisterType::PC, 
                    cpu::register::RegisterData::Bit16(address));
    }
    
    /// CPU 실행 사이클
    pub fn run_cpu_cycle(&mut self) -> Result<()> {
        self.cpu.execute_cycle()
    }
    
    /// 메모리 내용 덤프
    pub fn dump_memory(&self, start: u16, length: usize) -> String {
        let memory = self.memory.read().unwrap();
        memory.dump(start, length)
    }
    
    /// CPU 상태 덤프
    pub fn dump_cpu(&self) -> String {
        format!(
            "PC: ${:04X}, A: ${:02X}, X: ${:02X}, Y: ${:02X}, S: ${:02X}, P: ${:02X}",
            self.cpu.get(cpu::register::RegisterType::PC).as_u16(),
            self.cpu.get(cpu::register::RegisterType::A).as_u8(),
            self.cpu.get(cpu::register::RegisterType::X).as_u8(),
            self.cpu.get(cpu::register::RegisterType::Y).as_u8(),
            self.cpu.get(cpu::register::RegisterType::S).as_u8(),
            self.cpu.get(cpu::register::RegisterType::P).as_u8()
        )
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    
    #[test]
    fn test_breadboard_creation() {
        // 브레드보드 생성
        let board = BreadBoard::new();
        assert_eq!(board.cpu.get_id(), "CPU");
    }
    
    #[test]
    fn test_memory_access() {
        // 브레드보드 생성
        let mut board = BreadBoard::new();
        
        // 메모리에 값 설정
        board.memory.write().unwrap().set(0x1000, 0x42).unwrap();
        
        // CPU를 통해 메모리 읽기
        board.set_pc(0x1000);
        let opcode = board.cpu.read_memory(0x1000).unwrap();
        assert_eq!(opcode, 0x42);
        
        // CPU를 통해 메모리 쓰기
        board.cpu.write_memory(0x2000, 0x55).unwrap();
        let value = board.memory.read().unwrap().get(0x2000).unwrap();
        assert_eq!(value, 0x55);
    }
    
    #[test]
    fn test_load_program() {
        // 브레드보드 생성 있을 수 있음 - 메모리에 값을 설정한 후,
        let mut board = BreadBoard::new();
        // 프로그램 로드
        let program = [0xA9, 0x42, 0x85, 0x10]; // LDA #$42, STA $10
        board.load_program(0x0200, &program).unwrap();
        board.set_pc(0x0200);

        
        // 메모리 확인요할 수 있음
        let memory = board.memory.read().unwrap();
        assert_eq!(memory.get(0x0200).unwrap(), 0xA9);
        assert_eq!(memory.get(0x0201).unwrap(), 0x42);
        assert_eq!(memory.get(0x0202).unwrap(), 0x85);   
        assert_eq!(memory.get(0x0203).unwrap(), 0x10);
    }
}
 
