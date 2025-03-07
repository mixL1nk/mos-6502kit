/// 위치 정보를 나타내는 구조체
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct Position {
    /// 행 위치
    pub row: u16,
    /// 열 위치
    pub col: u16,
}

impl Position {
    /// 새로운 Position 인스턴스 생성
    pub fn new(row: u16, col: u16) -> Self {
        Self { row, col }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_position_creation() {
        let pos = Position::new(1, 2);
        assert_eq!(pos.row, 1);
        assert_eq!(pos.col, 2);
    }
}
