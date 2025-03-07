//! 브레드보드 시뮬레이션 라이브러리
//!
//! 이 라이브러리는 전자 회로의 브레드보드를 시뮬레이션하기 위한 기능을 제공합니다.
//! 주요 컴포넌트, 와이어 연결, 신호 전파 등의 기능을 포함합니다.

// 핵심 모듈 정의
pub mod breadboard;
pub mod bus_manager;

// 주요 타입 재내보내기(re-export)
pub use breadboard::BreadBoard;
