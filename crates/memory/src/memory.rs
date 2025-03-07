use common::bus::{BusInterface, BusOperationType, BusTransaction};
use common::Result;
/// 메모리 구조체
#[derive(Debug)]
pub struct Memory {
    /// 메모리 데이터 (64KB)
    pub data: [u8; 65536],
}

impl Default for Memory {
    fn default() -> Self {
        Self::new()
    }
}

impl Memory {
    /// 새로운 메모리 인스턴스 생성
    pub fn new() -> Self {
        Self { data: [0; 65536] }
    }

    /// 메모리 내용 가져오기
    pub fn get_memory_content(&self, address: u16) -> u8 {
        self.data[address as usize]
    }

    /// 메모리 내용 설정
    pub fn set_memory_content(&mut self, address: u16, value: u8) {
        self.data[address as usize] = value;
    }

    pub fn get_id(&self) -> &str {
        "Memory"
    }

    pub fn dump(&self, addr: u16, size: usize) -> String {
        let mut result = String::new();

        for i in (0..size).step_by(16) {
            let current_addr = addr.wrapping_add(i as u16);

            // 주소 출력
            result.push_str(&format!("{:04x}: ", current_addr));

            // 16진수 값 출력
            for j in 0..16 {
                if i + j < size {
                    let value = self.data[current_addr.wrapping_add(j as u16) as usize];
                    result.push_str(&format!("{:02x} ", value));
                } else {
                    result.push_str("   ");
                }
            }

            // ASCII 문자 출력
            result.push_str(" |");
            for j in 0..16 {
                if i + j < size {
                    let value = self.data[current_addr.wrapping_add(j as u16) as usize];
                    // 출력 가능한 ASCII 문자만 표시
                    let ch = if value >= 0x20 && value <= 0x7e {
                        value as char
                    } else {
                        '.'
                    };
                    result.push(ch);
                }
            }
            result.push_str("|\n");
        }

        result
    }
}

// 버스 인터페이스 구현
impl BusInterface for Memory {
    fn process_bus_transaction(&mut self, transaction: &mut BusTransaction) -> Result<()> {
        match transaction.operation_type {
            BusOperationType::Read => {
                transaction.data = self.data[transaction.address as usize];
                println!(
                    "[BUS] Memory ACTUAL READ: addr=0x{:04x}, data=0x{:02x}",
                    transaction.address, transaction.data
                );
                Ok(())
            }
            BusOperationType::Write => {
                let old_value = self.data[transaction.address as usize];
                self.data[transaction.address as usize] = transaction.data;
                println!(
                    "[BUS] Memory ACTUAL WRITE: addr=0x{:04x}, data=0x{:02x} (was: 0x{:02x})",
                    transaction.address, transaction.data, old_value
                );
                Ok(())
            }
            _ => Err("Invalid bus operation".into()),
        }
    }

    fn begin_transaction(
        &mut self,
        _transaction: BusTransaction,
    ) -> Result<BusTransaction> {
        Err("Memory cannot initiate transactions".into())
    }

    fn respond_to_transaction(&mut self, transaction: &mut BusTransaction) -> Result<()> {
        self.process_bus_transaction(transaction)
    }
}
