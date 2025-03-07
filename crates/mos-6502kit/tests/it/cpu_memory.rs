use breadboard::BreadBoard;
use cpu::CPU;
use memory::Memory;

#[test]
fn test_cpu_memory() {
    let mut board = BreadBoard::new();

    // 1. CPU와 메모리 추가
    println!("[TEST] Adding components");
    board.add_component(CPU::new()).unwrap();
    board.add_component(Memory::new()).unwrap();

    // 초기화 대기
    println!("[TEST] Waiting for initialization...");

    // ========== 테스트 3: 버스 인터페이스 ==========
    println!("\n[TEST] ========== TEST 3: BUS INTERFACE ==========");
    let test_addr3 = 0x4040;
    let test_value3 = 0x41;

    // 메모리 초기화
    board.with_component_mut("Memory", |component| {
        let memory = component.as_memory_mut().unwrap();
        memory.set_memory_content(test_addr3, 0);
    });

    // 버스 인터페이스 쓰기
    println!(
        "[TEST] Bus interface write: addr=0x{:04x}, value=0x{:02x}",
        test_addr3, test_value3
    );
    match board.bus_cpu_memory_write(test_addr3, test_value3) {
        Ok(_) => println!("[TEST] Bus write successful"),
        Err(e) => {
            println!(
                "[TEST] Bus write failed: {:?}, using direct write as fallback",
                e
            );
        }
    };

    // 버스 인터페이스 읽기
    println!("[TEST] Bus interface read: addr=0x{:04x}", test_addr3);
    let bus_read_value = match board.bus_cpu_memory_read(test_addr3) {
        Ok(value) => {
            println!("[TEST] Bus read successful: 0x{:02x}", value);
            value
        }
        Err(e) => {
            panic!("[TEST] Bus read failed: {:?}", e);
        }
    };

    // 최종 검증
    let memory_dump = board
        .with_component_mut("Memory", |component| {
            let memory = component.as_memory_mut().unwrap();
            memory.dump(0x4000, 0x50)
        })
        .unwrap();

    println!("\n[TEST] Final memory dump:\n{}", memory_dump);

    println!("\n[TEST] ========== TEST RESULTS ==========");

    assert_eq!(
        bus_read_value, test_value3,
        "Test : Bus interface access failed!"
    );
}

#[test]
fn test_bus_communication() {
    let mut board = BreadBoard::new();

    // 1. 컴포넌트 추가
    println!("[TEST] Adding components");
    board.add_component(CPU::new()).unwrap();
    board.add_component(Memory::new()).unwrap();

    // 메모리 테스트 영역 설정
    let test_addresses = [0x2020, 0x3030, 0x4040];
    let test_values = [0x42, 0x55, 0xAA];

    // 메모리에 값 쓰기
    println!("\n[TEST] ========== BUS WRITE TEST ==========");
    for (&addr, &value) in test_addresses.iter().zip(test_values.iter()) {
        println!("[TEST] Writing to address 0x{:04x}: 0x{:02x}", addr, value);
        let res = board.bus_cpu_memory_write(addr, value);
        match res {
            Ok(_) => println!("[TEST] Write successful"),
            Err(e) => {
                panic!("[TEST] Write failed: {:?}", e);
            }
        }
    }

    // 메모리에서 값 읽기
    println!("\n[TEST] ========== BUS READ TEST ==========");
    for (&addr, &expected) in test_addresses.iter().zip(test_values.iter()) {
        println!("[TEST] Reading from address 0x{:04x}", addr);
        let value = board.bus_cpu_memory_read(addr).unwrap();
        println!(
            "[TEST] Read value: 0x{:02x} (expected: 0x{:02x})",
            value, expected
        );
        assert_eq!(
            value, expected,
            "Bus read test failed for address 0x{:04x}",
            addr
        );
    }

    println!("[TEST] All bus communication tests passed!");
}
