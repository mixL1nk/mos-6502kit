use assembler::Assembler;
use breadboard::BreadBoard;
use cpu::{RegisterType, register::StatusRegister};

#[test]
fn test_cpu_memory() {
    let board = BreadBoard::new();

    // 1. CPU와 메모리 추가
    println!("[TEST] Adding components");

    // 초기화 대기
    println!("[TEST] Waiting for initialization...");

    // ========== 테스트 3: 버스 인터페이스 ==========
    println!("\n[TEST] ========== TEST 3: BUS INTERFACE ==========");
    let test_addr3 = 0x4040;
    let test_value3 = 0x41;

    // 버스 인터페이스 쓰기
    println!(
        "[TEST] Bus interface write: addr=0x{:04x}, value=0x{:02x}",
        test_addr3, test_value3
    );
    match board.cpu.write_memory(test_addr3, test_value3) {
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
    let bus_read_value = match board.cpu.read_memory(test_addr3) {
        Ok(value) => {
            println!("[TEST] Bus read successful: 0x{:02x}", value);
            value
        }
        Err(e) => {
            panic!("[TEST] Bus read failed: {:?}", e);
        }
    };

    println!("\n[TEST] ========== TEST RESULTS ==========");

    assert_eq!(
        bus_read_value, test_value3,
        "Test : Bus interface access failed!"
    );
}

#[test]
fn test_bus_communication() {
    let board = BreadBoard::new();

    // 1. 컴포넌트 추가
    println!("[TEST] Adding components");
    // board.add_component(CPU::new()).unwrap();
    // board.add_component(Memory::new()).unwrap();

    // 메모리 테스트 영역 설정
    let test_addresses = [0x2020, 0x3030, 0x4040];
    let test_values = [0x42, 0x55, 0xAA];

    // 메모리에 값 쓰기
    println!("\n[TEST] ========== BUS WRITE TEST ==========");
    for (&addr, &value) in test_addresses.iter().zip(test_values.iter()) {
        println!("[TEST] Writing to address 0x{:04x}: 0x{:02x}", addr, value);
        let res = board.cpu.write_memory(addr, value);
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
        let value = board.cpu.read_memory(addr).unwrap();
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

#[test]
fn test_inx_instruction() {
    // X 레지스터 증가 명령어 테스트
    let mut board = BreadBoard::new();
    let assembler: Assembler = Assembler::default();
    let source = "INX";
    let machine_code = assembler.assemble(source).unwrap();
    println!("[TEST] Assembled machine code:");
    // 메모리에 기계어 쓰기
    for (addr, &code) in machine_code.iter().enumerate() {
        println!("[TEST] Writing to address 0x{:04x}: 0x{:02x}", addr, code);
        board.cpu.write_memory(addr as u16, code).unwrap();
    }

    board.cpu.run().expect("CPU run failed");
    let result = board.cpu.get_value(RegisterType::X).as_u8();
    println!("[TEST] Result: 0x{:02x}", result);
    assert_eq!(result, 1);
}

#[test]
fn test_lda_immediate() {
    // 즉시 주소 지정 모드로 A 레지스터에 값 로드
    let mut board = BreadBoard::new();
    let assembler = Assembler::default();
    let source = "LDA #$42";
    let machine_code = assembler.assemble(source).unwrap();
    // let expected = 0x99;
    // board.cpu.write_memory(0x42, expected).expect("Write failed");
    // 메모리에 기계어 쓰기
    for (addr, &code) in machine_code.iter().enumerate() {
        println!("[TEST] Writing to address 0x{:04x}: 0x{:02x}", addr, code);
        board.cpu.write_memory(addr as u16, code).unwrap();
    }

    board.cpu.run().expect("CPU run failed");
    let result = board.cpu.get_value(RegisterType::A).as_u8();
    println!("[TEST] Result: 0x{:02x}", result);
    assert_eq!(result, 0x42);
}

#[test]
fn test_register_transfers() {
    // 레지스터 간 값 전송 테스트
    let mut board = BreadBoard::new();
    let assembler = Assembler::default();
    let source = "
        LDA #$42
        TAX
        TAY
    ";
    let machine_code = assembler.assemble(source).unwrap();
    let expected = 0x42;
    board
        .cpu
        .write_memory(0x42, expected)
        .expect("Write failed");
    // 메모리에 기계어 쓰기
    for (addr, &code) in machine_code.iter().enumerate() {
        board.cpu.write_memory(addr as u16, code).unwrap();
    }

    board.cpu.run().expect("CPU run failed");

    // 모든 레지스터가 0x42 값을 가져야 함
    assert_eq!(board.cpu.get_value(RegisterType::A).as_u8(), 0x42);
    assert_eq!(board.cpu.get_value(RegisterType::X).as_u8(), 0x42);
    assert_eq!(board.cpu.get_value(RegisterType::Y).as_u8(), 0x42);
}

#[test]
fn test_memory_store_and_load() {
    // 메모리에 값 저장 및 로드 테스트
    let mut board = BreadBoard::new();
    let assembler = Assembler::default();
    let source = "
        LDA #$FF
        STA $80
        LDX #$00
        LDA #$00
        LDA $80
    ";
    let machine_code = assembler.assemble(source).unwrap();

    // 메모리에 기계어 쓰기
    for (addr, &code) in machine_code.iter().enumerate() {
        board.cpu.write_memory(addr as u16, code).unwrap();
    }

    board.cpu.run().expect("CPU run failed");

    // A 레지스터가 메모리에서 다시 로드한 0xFF 값을 가져야 함
    assert_eq!(board.cpu.get_value(RegisterType::A).as_u8(), 0xFF);
    // 메모리 주소 0x80이 0xFF 값을 가져야 함
    assert_eq!(board.cpu.read_memory(0x80).unwrap(), 0xFF);
}

#[test]
fn test_arithmetic_operations() {
    // 산술 연산 테스트 (덧셈)
    let mut board = BreadBoard::new();
    let assembler = Assembler::new(0);
    let source = "
        CLC        ; 캐리 플래그 초기화
        LDA #$38   ; A = 56
        ADC #$25   ; A += 37
    ";
    let machine_code = assembler.assemble(source).unwrap();
    println!("[TEST] Assembled machine code: {:?}", machine_code);
    // 메모리에 기계어 쓰기
    for (addr, &code) in machine_code.iter().enumerate() {
        board.cpu.write_memory(addr as u16, code).unwrap();
    }

    board.cpu.run().expect("CPU run failed");

    // 56 + 37 = 93 (0x5D)
    assert_eq!(board.cpu.get_value(RegisterType::A).as_u8(), 0x5D);
    // 결과에 캐리가 없으므로 캐리 플래그는 0이어야 함
    assert!(!board.cpu.get_flag(StatusRegister::CARRY));
}

#[test]
fn test_logical_operations() {
    // 논리 연산 테스트 (AND, ORA, EOR)
    let mut board = BreadBoard::new();
    let assembler = Assembler::default();
    let source = "
        LDA #$AA   ; A = 10101010
        AND #$0F   ; A = 00001010
        ORA #$30   ; A = 00111010
        EOR #$FF   ; A = 11000101
    ";
    let machine_code = assembler.assemble(source).unwrap();

    println!("[TEST] Assembled machine code:");
    for (i, &code) in machine_code.iter().enumerate() {
        println!("[TEST] ${:04X}: {:02X}", i, code);
    }

    // 메모리에 기계어 쓰기
    for (addr, &code) in machine_code.iter().enumerate() {
        board.cpu.write_memory(addr as u16, code).unwrap();
    }

    // 각 명령어 실행 후 A 레지스터 값 확인
    board.cpu.step().expect("LDA failed"); // LDA #$AA
    println!(
        "[TEST] After LDA #$AA: A = ${:02X}",
        board.cpu.get_value(RegisterType::A).as_u8()
    );

    board.cpu.step().expect("AND failed"); // AND #$0F
    println!(
        "[TEST] After AND #$0F: A = ${:02X}",
        board.cpu.get_value(RegisterType::A).as_u8()
    );

    board.cpu.step().expect("ORA failed"); // ORA #$30
    println!(
        "[TEST] After ORA #$30: A = ${:02X}",
        board.cpu.get_value(RegisterType::A).as_u8()
    );

    board.cpu.step().expect("EOR failed"); // EOR #$FF
    println!(
        "[TEST] After EOR #$FF: A = ${:02X}",
        board.cpu.get_value(RegisterType::A).as_u8()
    );

    // 결과는 0xC5 (11000101)
    let result = board.cpu.get_value(RegisterType::A).as_u8();
    assert_eq!(result, 0xC5, "Expected A = 0xC5, got 0x{:02X}", result);
}

#[test]
fn test_flag_operations() {
    // 플래그 연산 테스트
    let mut board = BreadBoard::new();
    let assembler = Assembler::default();
    let source = "
        CLC        ; 캐리 플래그 클리어
        SEC        ; 캐리 플래그 설정
        LDA #$80   ; 음수 값 로드 (0x80 = -128)
    ";
    let machine_code = assembler.assemble(source).unwrap();

    // 메모리에 기계어 쓰기
    for (addr, &code) in machine_code.iter().enumerate() {
        board.cpu.write_memory(addr as u16, code).unwrap();
    }

    board.cpu.run().expect("CPU run failed");

    // 캐리 플래그가 설정되어 있어야 함
    assert!(board.cpu.get_flag(StatusRegister::CARRY));

    // 음수 값을 로드했으므로 네거티브 플래그가 설정되어 있어야 함
    assert!(board.cpu.get_flag(StatusRegister::NEGATIVE));
}

#[test]
fn test_branch_instructions() {
    // 분기 명령어 테스트
    let mut board = BreadBoard::new();
    let assembler = Assembler::default();
    let source = "
        .org $0600
        LDX #$00   ; X = 0
        LDY #$00   ; Y = 0
    loop:
        INX        ; X++
        INY        ; Y++
        CPY #$05   ; 비교 Y - 5
        BNE loop   ; Z=0이면 loop로 분기 (Y != 5)
        LDA #$42   ; A = 0x42 (루프 종료 후 실행)
    ";
    // 0600: a2 00 a0 00 e8 c8 c0 05 d0 fa a9 42
    let machine_code = assembler.assemble(source).unwrap();
    println!("[TEST] Assembled machine code: {:?}", machine_code);

    // 메모리에 기계어 쓰기
    for (addr, &code) in machine_code.iter().enumerate() {
        board.cpu.write_memory(0x0600 + addr as u16, code).unwrap();
    }
    board.cpu.set_pc(0x0600);
    board.cpu.run().expect("CPU run failed");

    // 루프가 5번 반복되어야 함
    assert_eq!(board.cpu.get_value(RegisterType::X).as_u8(), 5);
    assert_eq!(board.cpu.get_value(RegisterType::Y).as_u8(), 5);
    // 루프 종료 후 A에 0x42가 로드되어야 함
    assert_eq!(board.cpu.get_value(RegisterType::A).as_u8(), 0x42);
}

#[test]
fn test_indexed_addressing() {
    // 인덱스 주소 지정 모드 테스트
    let mut board = BreadBoard::new();
    let assembler = Assembler::default();
    let source = "
        LDX #$02       ; X = 2
        LDA #$11       ; A = 0x11
        STA $80,X      ; mem[0x82] = 0x11
        LDA #$00       ; A = 0
        LDA $80,X      ; A = mem[0x82]
    ";
    let machine_code = assembler.assemble(source).unwrap();
    println!("[TEST] Assembled machine code: {:?}", machine_code);
    // 메모리에 기계어 쓰기
    for (addr, &code) in machine_code.iter().enumerate() {
        board.cpu.write_memory(addr as u16, code).unwrap();
    }

    board.cpu.run().expect("CPU run failed");

    // A가 메모리에서 로드한 값을 가져야 함
    assert_eq!(board.cpu.read_memory(0x82).unwrap(), 0x11);
    assert_eq!(board.cpu.get_value(RegisterType::A).as_u8(), 0x11);
}

#[test]
fn test_stack_operations() {
    // 스택 연산 테스트
    let mut board = BreadBoard::new();
    let assembler = Assembler::default();
    let source = "
        LDA #$42   ; A = 0x42
        PHA        ; 스택에 A 푸시
        LDA #$00   ; A = 0
        PLA        ; 스택에서 A로 풀
    ";
    let machine_code = assembler.assemble(source).unwrap();

    // 메모리에 기계어 쓰기
    for (addr, &code) in machine_code.iter().enumerate() {
        board.cpu.write_memory(addr as u16, code).unwrap();
    }

    board.cpu.run().expect("CPU run failed");

    // 스택에서 풀된 값이 A에 있어야 함
    assert_eq!(board.cpu.get_value(RegisterType::A).as_u8(), 0x42);
}

#[test]
fn test_complex_program() {
    // 복합적인 프로그램 테스트: 1부터 10까지의 합계 계산
    let mut board = BreadBoard::new();
    let assembler = Assembler::default();

    let source = "
        .org $0600     ; 프로그램 시작 주소를 0x0600으로 설정
start:  
        LDA #$00       ; A = 0 (합계 초기화)
        TAX            ; X = 0 (카운터 초기화)
loop:
        INX            ; X++ (카운터 증가)
        STX $80        ; 임시로 X 값을 메모리에 저장
        CLC            ; 캐리 클리어
        ADC $80        ; A += mem[0x80] (X의 값을 더함)
        CPX #$0A       ; X == 10 비교
        BNE loop       ; X != 10이면 계속
        
done:   
        BRK           ; 프로그램 종료
    ";
    let machine_code = assembler.assemble(source).unwrap();
    println!(
        "[TEST] Assembled machine code (hex): {}",
        assembler.format_hex(&machine_code)
    );

    // 메모리에 기계어 쓰기
    for (addr, &code) in machine_code.iter().enumerate() {
        board.cpu.write_memory(0x0600 + addr as u16, code).unwrap();
    }
    board.cpu.set_pc(0x0600);
    board.cpu.run().expect("CPU run failed");

    // 1+2+...+10 = 55 (0x37)
    assert_eq!(board.cpu.get_value(RegisterType::A).as_u8(), 0x37);
}

#[test]
fn test_indirect_addressing() {
    // 간접 주소 지정 모드 테스트
    let mut board = BreadBoard::new();

    // 테스트를 위해 메모리 직접 설정
    board.cpu.write_memory(0x20, 0x80).unwrap(); // 주소의 하위 바이트
    board.cpu.write_memory(0x21, 0x00).unwrap(); // 주소의 상위 바이트
    board.cpu.write_memory(0x80, 0x42).unwrap(); // 대상 데이터

    let assembler = Assembler::default();
    let source = "
        LDY #$00       ; Y = 0
        LDA ($20),Y    ; 간접주소 + Y 모드: A = mem[mem[0x20]+Y] = mem[0x0080] = 0x42
    ";
    //A0 00 B1 20
    let machine_code = assembler.assemble(source).unwrap();
    println!(
        "[TEST] Assembled machine code (hex): {}",
        assembler.format_hex(&machine_code)
    );
    // 메모리에 기계어 쓰기
    for (addr, &code) in machine_code.iter().enumerate() {
        board.cpu.write_memory(addr as u16, code).unwrap();
    }

    board.cpu.run().expect("CPU run failed");

    // 간접 주소를 통해 로드한 값이 A에 있어야 함
    assert_eq!(board.cpu.get_value(RegisterType::A).as_u8(), 0x42);
}

#[test]
fn test_bit_shift_operations() {
    // 비트 시프트 연산 테스트
    let mut board = BreadBoard::new();
    let assembler = Assembler::default();
    let source = "
        LDA #$81   ; A = 10000001 (0x81)
        LSR A      ; A = 01000000 (0x40), 오른쪽으로 1비트 시프트, C = 1 (맨 오른쪽 비트가 캐리로)
        ASL A      ; A = 10000000 (0x80), 왼쪽으로 1비트 시프트, C = 0 (맨 왼쪽 비트가 캐리로)
    ";
    let machine_code = assembler.assemble(source).unwrap();
    println!(
        "[TEST] Assembled machine code (hex): {}",
        assembler.format_hex(&machine_code)
    );

    // 메모리에 기계어 쓰기
    for (addr, &code) in machine_code.iter().enumerate() {
        board.cpu.write_memory(addr as u16, code).unwrap();
    }

    // LSR 실행 후 상태 확인
    board.cpu.step().expect("LDA 실행 실패");
    assert_eq!(board.cpu.get_value(RegisterType::A).as_u8(), 0x81);
    board.cpu.step().expect("LSR 실행 실패");

    // LSR 실행 후 A = 0x40, Carry = 1 (원래 값 0x81의 맨 오른쪽 비트)
    assert_eq!(board.cpu.get_value(RegisterType::A).as_u8(), 0x40);
    assert!(
        board.cpu.get_flag(StatusRegister::CARRY),
        "LSR 후 캐리 플래그는 1이어야 함"
    );
    assert!(
        !board.cpu.get_flag(StatusRegister::NEGATIVE),
        "LSR 후 네거티브 플래그는 0이어야 함"
    );

    // ASL 실행 후 상태 확인
    board.cpu.step().expect("ASL 실행 실패");

    // 최종 결과는 0x80, Carry = 0 (0x40의 맨 왼쪽 비트는 0)
    assert_eq!(
        board.cpu.get_value(RegisterType::A).as_u8(),
        0x80,
        "ASL 후 A 레지스터 값은 0x80이어야 함"
    );
    assert!(
        !board.cpu.get_flag(StatusRegister::CARRY),
        "ASL 후 캐리 플래그는 0이어야 함"
    );
    assert!(
        board.cpu.get_flag(StatusRegister::NEGATIVE),
        "ASL 후 네거티브 플래그는 1이어야 함"
    );
}

#[test]
fn test_compare_operations() {
    // 비교 연산 테스트
    let mut board = BreadBoard::new();
    let assembler = Assembler::default();
    let source = "
        LDA #$40   ; A = 0x40
        CMP #$40   ; A == 0x40 ?
        LDX #$30   ; X = 0x30
        CPX #$20   ; X > 0x20 ?
        LDY #$10   ; Y = 0x10
        CPY #$20   ; Y < 0x20 ?
    ";
    //0000:A9 40 C9 40 A2 30 E0 20
    //0008: A0 10 C0 20
    let machine_code = assembler.assemble(source).unwrap();
    println!(
        "[TEST] Assembled machine code (hex): {}",
        assembler.format_hex(&machine_code)
    );

    // 메모리에 기계어 쓰기
    for (addr, &code) in machine_code.iter().enumerate() {
        board.cpu.write_memory(addr as u16, code).unwrap();
    }

    board.cpu.step().expect("CPU step failed");
    board.cpu.step().expect("CPU step failed");

    // A == 0x40 이므로 Z=1, C=1, N=0
    assert!(board.cpu.get_flag(StatusRegister::ZERO));
    assert!(board.cpu.get_flag(StatusRegister::CARRY));
    assert!(!board.cpu.get_flag(StatusRegister::NEGATIVE));

    board.cpu.step().expect("CPU step failed"); // LDX #$30
    board.cpu.step().expect("CPU step failed"); // CPX #$20

    // X > 0x20 이므로 Z=0, C=1, N=0
    assert!(!board.cpu.get_flag(StatusRegister::ZERO));
    assert!(board.cpu.get_flag(StatusRegister::CARRY));
    assert!(!board.cpu.get_flag(StatusRegister::NEGATIVE));

    board.cpu.step().expect("CPU step failed"); // LDY #$10
    board.cpu.step().expect("CPU step failed"); // CPY #$20

    // Y < 0x20 이므로 Z=0, C=0, N=1
    assert!(!board.cpu.get_flag(StatusRegister::ZERO));
    assert!(!board.cpu.get_flag(StatusRegister::CARRY));
    assert!(board.cpu.get_flag(StatusRegister::NEGATIVE));
}

#[test]
fn test_counter_and_memory() {
    let mut board = BreadBoard::new();
    let assembler = Assembler::new(0x0600);

    // 간단한 카운터 프로그램
    let source = "
        .org $0600     ; 프로그램 시작 주소를 0x0600으로 설정
start:  
        LDA #$00       ; A = 0 초기화
loop:
        CLC            ; 캐리 클리어
        ADC #$01       ; A += 1
        STA $80        ; 현재 값을 메모리에 저장
        CMP #$0A       ; A == 10 비교
        BNE loop       ; A != 10이면 루프 계속
        
        LDX $80        ; X = mem[0x80] (메모리에서 값 읽기)
        
done:   
        BRK           ; 프로그램 종료
    ";

    let machine_code = assembler.assemble(source).unwrap();

    // 메모리에 기계어 쓰기
    for (i, &code) in machine_code.iter().enumerate() {
        board.cpu.write_memory(0x0600 + i as u16, code).unwrap();
    }

    // PC를 시작 주소로 설정
    board.cpu.set_pc(0x0600);

    board.cpu.run().expect("CPU run failed");

    let mem_value = board.cpu.read_memory(0x80).unwrap();
    println!("$80: ${:02X}", mem_value);

    // 결과 확인: 메모리에 저장된 값이 10(0x0A)이어야 함
    assert_eq!(mem_value, 0x0A, "Counter should reach 10");

    // X 레지스터의 값도 10이어야 함
    let x_value = board.cpu.get_value(RegisterType::X).as_u8();
    assert_eq!(x_value, 0x0A, "X register should contain 10");
}
