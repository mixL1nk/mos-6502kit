use common::MemoryBus;
use cpu::cpu::CPU;
use cpu::register::{RegisterData, RegisterType};
use std::sync::{Arc, Mutex};

// 테스트용 메모리 구현
#[derive(Debug)]
struct MockMemoryBus {
    memory: [u8; 65536],
}

impl MockMemoryBus {
    fn new() -> Self {
        let mut memory = [0; 65536];
        // 테스트 프로그램 설정

        // 0x1000: LDA #$42    ; A9 42 - A 레지스터에 0x42 로드
        memory[0x1000] = 0xA9;
        memory[0x1001] = 0x42;

        // 0x1002: STA $2000   ; 8D 00 20 - A 레지스터 값을 $2000 주소에 저장
        memory[0x1002] = 0x8D;
        memory[0x1003] = 0x00;
        memory[0x1004] = 0x20;

        // 0x1005: LDA #$FF    ; A9 FF - A 레지스터에 0xFF 로드
        memory[0x1005] = 0xA9;
        memory[0x1006] = 0xFF;

        // 0x1007: NOP         ; EA - 아무 작업 안함
        memory[0x1007] = 0xEA;

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
fn test_cpu_execute_instructions() {
    // CPU 생성
    let mut cpu = CPU::new();

    // PC 레지스터 초기화 (테스트 프로그램 시작 주소)
    cpu.set_value(RegisterType::PC, RegisterData::Bit16(0x1000));

    // 메모리 버스 모의 객체 생성 및 연결
    let memory_bus = Arc::new(Mutex::new(MockMemoryBus::new()));
    cpu.set_memory_bus(memory_bus.clone());

    // 첫 번째 명령어 실행: LDA #$42
    cpu.step().expect("Failed to execute first instruction");

    // A 레지스터 확인
    assert_eq!(cpu.get_value(RegisterType::A), RegisterData::Bit8(0x42));
    assert_eq!(cpu.get_value(RegisterType::PC), RegisterData::Bit16(0x1002));

    // 두 번째 명령어 실행: STA $2000
    cpu.step().expect("Failed to execute second instruction");

    // 메모리 확인
    let memory_value = memory_bus.lock().unwrap().read(0x2000);
    assert_eq!(memory_value, 0x42);
    assert_eq!(cpu.get_value(RegisterType::PC), RegisterData::Bit16(0x1005));

    // 세 번째 명령어 실행: LDA #$FF
    cpu.step().expect("Failed to execute third instruction");

    // A 레지스터 확인
    assert_eq!(cpu.get_value(RegisterType::A), RegisterData::Bit8(0xFF));

    // 네 번째 명령어 실행: NOP
    cpu.step().expect("Failed to execute fourth instruction");

    // CPU run Continuously before halt
    cpu.set_value(RegisterType::PC, RegisterData::Bit16(0x1000));
    cpu.run().expect("Failed to run CPU");
}
