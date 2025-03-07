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

    pub fn get(&self, address: u16) -> Result<u8> {
        Ok(self.data[address as usize])
    }

    pub fn set(&mut self, address: u16, value: u8) -> Result<()> {
        self.data[address as usize] = value;
        Ok(())
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
                    let ch = if (0x20..=0x7e).contains(&value) {
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
