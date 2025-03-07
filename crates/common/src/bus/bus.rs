use crate::Result;
/// 버스 작업 유형 정의
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum BusOperationType {
    Read,
    Write,
    None,
}

/// 버스 트랜잭션 구조체
#[derive(Debug, Clone)]
pub struct BusTransaction {
    /// 주소 버스 상태
    pub address: u16,
    /// 데이터 버스 상태
    pub data: u8,
    /// 작업 유형
    pub operation_type: BusOperationType,
    /// 트랜잭션 ID
    pub id: u64,
}

impl BusTransaction {
    pub fn gen_id() -> u64 {
        std::time::SystemTime::now()
            .duration_since(std::time::UNIX_EPOCH)
            .unwrap()
            .as_nanos() as u64
    }
    /// 새 읽기 트랜잭션 생성
    pub fn new_read(address: u16) -> Self {
        Self {
            address,
            data: 0,
            operation_type: BusOperationType::Read,
            id: std::time::SystemTime::now()
                .duration_since(std::time::UNIX_EPOCH)
                .unwrap()
                .as_nanos() as u64,
        }
    }

    /// 새 쓰기 트랜잭션 생성
    pub fn new_write(address: u16, data: u8) -> Self {
        Self {
            address,
            data,
            operation_type: BusOperationType::Write,
            id: std::time::SystemTime::now()
                .duration_since(std::time::UNIX_EPOCH)
                .unwrap()
                .as_nanos() as u64,
        }
    }
}

/// 버스 인터페이스 - CPU와 메모리가 구현해야 함
pub trait BusInterface {
    /// 버스 트랜잭션 처리
    fn process_bus_transaction(&mut self, transaction: &mut BusTransaction) -> Result<()>;

    /// 트랜잭션 시작 (송신측)
    fn begin_transaction(&mut self, transaction: BusTransaction) -> Result<BusTransaction>;

    /// 트랜잭션 응답 (수신측)
    fn respond_to_transaction(&mut self, transaction: &mut BusTransaction) -> Result<()>;
}
