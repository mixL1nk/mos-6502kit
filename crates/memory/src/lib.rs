//! Memory 모듈
//! 6502 CPU의 메모리 기능을 구현한 모듈

pub mod memory;

pub use memory::Memory;

pub fn add(left: u64, right: u64) -> u64 {
    left + right
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        let result = add(2, 2);
        assert_eq!(result, 4);
    }
}
