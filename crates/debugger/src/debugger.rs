use common::Result;
use cpu::{CPU, CPUEvent, InterruptType};
use std::collections::HashMap;
use std::sync::mpsc::{self, Sender};

#[derive(PartialEq, Eq, Hash, Debug, Clone)]
pub enum AccessType {
    Access, // read + write
    Read,
    Write,
}

#[derive(PartialEq, Eq, Hash, Debug, Clone)]
struct BreakPoint {
    address: u16,
    enabled: bool,
    access_type: AccessType,
}

impl BreakPoint {
    fn new(address: u16, enabled: bool, access_type: AccessType) -> Self {
        Self {
            address,
            enabled,
            access_type,
        }
    }
}

#[derive(Debug, Clone)]
pub struct Debugger {
    cpu: Option<CPU>,
    breakpoints: HashMap<u16, BreakPoint>,
    // 인터럽트 전송용 채널
    interrupt_sender: Option<Sender<InterruptType>>,
}

impl Default for Debugger {
    fn default() -> Self {
        Self::new()
    }
}

impl Debugger {
    pub fn new() -> Self {
        Self {
            cpu: None,
            breakpoints: HashMap::new(),
            interrupt_sender: None,
        }
    }

    pub fn get_cpu_mut(&mut self) -> Result<&mut CPU> {
        self.cpu.as_mut().ok_or(error::Error::DebuggerNoCPU)
    }

    pub fn get_cpu(&self) -> Result<&CPU> {
        self.cpu.as_ref().ok_or(error::Error::DebuggerNoCPU)
    }

    pub fn run_cpu(&mut self) -> Result<()> {
        if let Some(cpu) = &mut self.cpu {
            cpu.run()
        } else {
            Err(error::Error::DebuggerNoCPU)
        }
    }

    pub fn attach(&mut self, cpu: CPU) {
        self.cpu = Some(cpu);
    }

    pub fn detach(&mut self) {
        self.cpu = None;
    }

    pub fn enabled(&mut self) -> Result<()> {
        // CPU 디버깅 모드 활성화
        self.get_cpu_mut()?.enabled_debug(true);

        // 인터럽트 채널 생성
        let (tx, rx) = mpsc::channel();
        self.interrupt_sender = Some(tx.clone());

        // 이벤트 핸들러 등록
        // 클로저에서 사용할 브레이크포인트 복사
        let breakpoints = self.breakpoints.clone();
        let tx_clone = tx.clone();

        self.get_cpu_mut()?.register_event_handler(Box::new(move |event| {
            match event {
                CPUEvent::InstructionExecuted { pc, opcode, cycles, operand } => {
                    println!(
                        "[Debugger] InstructionExecuted: pc: {:#X}, opcode: {:#X}, cycles: {:#X}, operand: {:#X}",
                        pc, opcode, cycles, operand
                    );

                    // pc에 브레이크포인트가 활성화되어 있는지 확인
                    let has_active_breakpoint = breakpoints
                        .get(pc)
                        .filter(|bp| bp.enabled && bp.access_type == AccessType::Access)
                        .is_some();

                    println!(
                        "[Debugger] Breakpoint check: {:#X}, {:#?}",
                        pc, has_active_breakpoint
                    );

                    // 활성화된 브레이크포인트가 있으면 CPU 중단
                    if has_active_breakpoint {
                        println!("[Debugger] Breakpoint hit at {:#X}", pc);
                        let _ = tx_clone.send(InterruptType::BRK);
                    }
                }
                CPUEvent::MemoryWrite { address, value } => {
                    println!(
                        "[Debugger] MemoryWrite: address: {:#X}, value: {:#X}",
                        address, value
                    );
                    
                    // 메모리 쓰기 브레이크포인트 확인
                    let has_write_breakpoint = breakpoints
                        .get(address)
                        .filter(|bp| bp.enabled && 
                               (bp.access_type == AccessType::Write || 
                                bp.access_type == AccessType::Access))
                        .is_some();
                        
                    if has_write_breakpoint {
                        println!("[Debugger] Write breakpoint hit at {:#X}", address);
                        let _ = tx_clone.send(InterruptType::BRK);
                    }
                }
                CPUEvent::MemoryRead { address, value } => {
                    println!(
                        "[Debugger] MemoryRead: address: {:#X}, value: {:#X}",
                        address, value
                    );
                    
                    // 메모리 읽기 브레이크포인트 확인
                    let has_read_breakpoint = breakpoints
                        .get(address)
                        .filter(|bp| bp.enabled && 
                               (bp.access_type == AccessType::Read || 
                                bp.access_type == AccessType::Access))
                        .is_some();
                        
                    if has_read_breakpoint {
                        println!("[Debugger] Read breakpoint hit at {:#X}", address);
                        let _ = tx_clone.send(InterruptType::BRK);
                    }
                }
                CPUEvent::FlagChanged { flag, value, old_value } => {
                    println!(
                        "[Debugger] FlagChanged: {} changed from {} to {}",
                        flag, old_value, value
                    );
                }
                CPUEvent::StateChanged { state } => {
                    println!("[Debugger] StateChanged: {:?}", state);
                }
                CPUEvent::RegisterChanged { register, value, old_value } => {
                    println!(
                        "[Debugger] RegisterChanged: {} changed from {:#X} to {:#X}",
                        register, old_value, value
                    );
                }
            }
        }));

        // CPU에 수신자 등록
        self.get_cpu_mut()?.set_interrupt_channel(rx);

        Ok(())
    }

    pub fn disabled(&mut self) -> Result<()> {
        self.get_cpu_mut()?.enabled_debug(false);
        self.get_cpu_mut()?.clear_event_handler();
        self.interrupt_sender = None;
        Ok(())
    }

    pub fn read(&self, address: u16) -> Result<u8> {
        self.get_cpu()?.read_memory(address)
    }
    
    pub fn write(&mut self, address: u16, value: u8) -> Result<()> {
        self.get_cpu_mut()?.write_memory(address, value)
    }

    pub fn add_breakpoint(&mut self, address: u16, access_type: AccessType) {
        self.breakpoints
            .insert(address, BreakPoint::new(address, true, access_type));
    }

    pub fn toggle_breakpoint(&mut self, address: u16) {
        if let Some(bp) = self.breakpoints.get_mut(&address) {
            bp.enabled = !bp.enabled;
        }
    }

    pub fn remove_breakpoint(&mut self, address: u16) {
        self.breakpoints.remove(&address);
    }

    // CPU에 인터럽트 신호 보내기
    pub fn send_interrupt(&self, interrupt_type: InterruptType) -> Result<()> {
        if let Some(sender) = &self.interrupt_sender {
            sender.send(interrupt_type)
                .map_err(|_| error::Error::Internal("Failed to send interrupt".to_string()))?;
            Ok(())
        } else {
            Err(error::Error::Internal("Interrupt channel not initialized".to_string()))
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use common::MemoryBus;
    use std::sync::{Arc, Mutex};

    #[derive(Debug)]
    struct MockMemoryBus {
        memory: [u8; 65536],
    }

    impl MockMemoryBus {
        fn new() -> Self {
            let mut memory = [0; 65536];
            // LDA #$42
            memory[0x1000] = 0xA9;
            memory[0x1001] = 0x42;
            memory[0x1002] = 0xe8; // INX
            memory[0x1003] = 0xe8; // INX
            Self { memory }
        }
    }

    impl MemoryBus for MockMemoryBus {
        fn read(&self, address: u16) -> u8 {
            self.memory[address as usize]
        }

        fn write(&mut self, address: u16, value: u8) {
            self.memory[address as usize] = value;
        }
    }

    #[test]
    fn test_debugger_attach() {
        let mut cpu = CPU::new();
        let mem = Arc::new(Mutex::new(MockMemoryBus::new()));
        cpu.set_pc(0x1000);
        cpu.set_memory_bus(mem);

        let mut debugger = Debugger::new();
        debugger.attach(cpu);
        assert!(debugger.cpu.is_some());
        debugger
            .enabled()
            .expect("an error occurred while enabling the debugger");
        debugger
            .run_cpu()
            .expect("an error occurred while running the CPU");
    }

    #[test]
    fn test_breakpoint_hit() {
        let mut cpu = CPU::new();
        let mem = Arc::new(Mutex::new(MockMemoryBus::new()));
        cpu.set_pc(0x1000);
        cpu.set_memory_bus(mem);

        let mut debugger = Debugger::new();
        debugger.attach(cpu);
        debugger.add_breakpoint(0x1002, AccessType::Access);

        debugger
            .enabled()
            .expect("an error occurred while enabling the debugger");

        debugger
            .run_cpu()
            .expect("an error occurred while running the CPU");
    }
}
