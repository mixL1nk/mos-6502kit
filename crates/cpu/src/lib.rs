//! CPU 모듈
//! 6502 CPU의 기능을 구현한 모듈

// 내부 모듈 선언
pub mod cpu;
pub mod executor;
pub mod flags;
pub mod instruction;
pub mod opcode_table;
pub mod register;

pub use common::Result;
// 주요 타입 재내보내기
pub use cpu::CPU;
pub use register::{RegisterData, RegisterType};

pub fn add(left: u64, right: u64) -> u64 {
    left + right
}
