use std::fmt::Debug;

/// 메모리 버스 인터페이스 - CPU가 메모리에 접근하는 방법 정의
pub trait MemoryBus: Debug + Send + Sync {
    /// 메모리에서 1바이트 읽기
    fn read(&self, address: u16) -> u8;

    /// 메모리에 1바이트 쓰기
    fn write(&mut self, address: u16, value: u8);

    /// 메모리에서 연속된 바이트 읽기
    fn read_block(&self, address: u16, count: usize) -> Vec<u8> {
        (0..count)
            .map(|i| self.read(address.wrapping_add(i as u16)))
            .collect()
    }

    /// 메모리에 연속된 바이트 쓰기
    fn write_block(&mut self, address: u16, data: &[u8]) {
        for (i, &value) in data.iter().enumerate() {
            self.write(address.wrapping_add(i as u16), value);
        }
    }
}
