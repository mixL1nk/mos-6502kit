//! CPU 에 대한 기본 정보
use crate::instruction::{Fetch, InstructionDecoder};
use crate::register::{RegisterData, RegisterType, Registers, SpecialRegister8, StatusRegister};
use common::MemoryBus;
use common::Result;
use error::Error;
use std::sync::{Arc, Mutex};
use types::Instruction;

/// CPU 인터럽트 타입
#[derive(Debug, Clone, Copy, PartialEq)]
pub enum InterruptType {
    /// 리셋 인터럽트
    Reset,
    /// 비마스크에블 인터럽트 요청
    IRQ,
    /// 마스크불가능 인터럽트 요청
    NMI,
    /// 소프트웨어 인터럽트 (BRK 명령)
    BRK,
    /// 잘못된 명령어로 인한 정지
    IllegalOpcode,
}

#[derive(Debug, Clone, Copy, PartialEq)]
pub enum CPUState {
    /// CPU가 정상적으로 실행 중
    Running,
    /// CPU가 정지됨 (BRK, 잘못된 명령어 등으로 인해)
    Halted(InterruptType),
    /// CPU가 대기 중 (실행 준비는 되어있으나 아직 시작하지 않음)
    Ready,
}

/// CPU 구조체
#[derive(Debug)]
pub struct CPU {
    /// 레지스터 값들
    registers: Registers,
    /// 메모리 버스 접근자 (옵션 - 설정되지 않았을 수 있음)
    memory_bus: Option<Arc<Mutex<dyn MemoryBus>>>,
    instruction: InstructionDecoder,
    state: CPUState,
    /// 현재 명령어의 사이클 정보
    cycles: u8,
}

impl Default for CPU {
    fn default() -> Self {
        Self::new()
    }
}

impl CPU {
    /// 새로운 CPU 인스턴스 생성
    pub fn new() -> Self {
        Self {
            registers: Registers::default(),
            memory_bus: None,
            instruction: InstructionDecoder::new(),
            state: CPUState::Ready,
            cycles: 0,
        }
    }

    /// CPU를 특정 이유로 정지시킴
    pub fn halt_with_reason(&mut self, reason: InterruptType) {
        self.state = CPUState::Halted(reason);
        match reason {
            InterruptType::Reset => println!("[CPU] CPU halted: Reset requested"),
            InterruptType::IRQ => println!("[CPU] CPU halted: IRQ received"),
            InterruptType::NMI => println!("[CPU] CPU halted: NMI received"),
            InterruptType::BRK => println!("[CPU] CPU halted: BRK instruction executed"),
            InterruptType::IllegalOpcode => {
                println!("[CPU] CPU halted: Illegal opcode encountered")
            }
        }
    }

    /// CPU 리셋 - 상태도 초기화
    pub fn reset(&mut self) {
        self.registers = Registers::default();
        self.state = CPUState::Ready;
    }

    /// 레지스터 값 가져오기
    pub fn get_value(&self, reg: RegisterType) -> RegisterData {
        self.registers.get_value(reg)
    }

    /// 레지스터 값 설정
    pub fn set_value(&mut self, reg: RegisterType, value: RegisterData) {
        self.registers.set_value(reg, value);
    }

    /// CPU 클럭 실행
    pub fn clock(&mut self) {
        println!("[+] CPU clock cycle");
    }

    /// CPU ID 가져오기
    pub fn get_id(&self) -> &str {
        "CPU"
    }

    /// 메모리 버스 설정 - 외부 매니저에서 호출
    pub fn set_memory_bus(&mut self, bus: Arc<Mutex<dyn MemoryBus>>) {
        self.memory_bus = Some(bus);
    }

    /// 상태 레지스터 직접 가져오기
    pub fn status_flag(&self) -> StatusRegister {
        match self.registers.p {
            SpecialRegister8::P(status) => status,
            _ => unreachable!("Status register has unexpected type"),
        }
    }

    /// 상태 레지스터 직접 설정하기
    pub fn set_status(&mut self, status: StatusRegister) {
        match &mut self.registers.p {
            SpecialRegister8::P(s) => *s = status,
            _ => unreachable!("Status register has unexpected type"),
        }
    }

    /// 플래그 설정하기
    pub fn set_flag(&mut self, flag: StatusRegister, value: bool) {
        let mut status = self.status_flag();
        status.set(flag, value);
        self.set_status(status);
    }

    /// 플래그 확인하기
    pub fn get_flag(&self, flag: StatusRegister) -> bool {
        self.status_flag().contains(flag)
    }

    /// PC 레지스터 값 가져오기
    pub(crate) fn get_pc(&self) -> u16 {
        self.registers.get_value(RegisterType::PC).as_u16()
    }

    /// PC 레지스터 값 설정
    pub fn set_pc(&mut self, value: u16) {
        self.registers
            .set_value(RegisterType::PC, RegisterData::Bit16(value));
    }

    /// PC 레지스터 증가
    fn increment_pc(&mut self, increment: u16) {
        let pc = self.get_pc();
        self.set_pc(pc.wrapping_add(increment));
    }

    /// opcode 가져오기
    fn fetch_opcode(&mut self) -> Result<u8> {
        let pc = self.get_pc();
        let opcode = self.read_memory(pc)?;
        self.increment_pc(1);
        Ok(opcode)
    }

    /// 메모리에서 PC 위치의 데이터 읽기
    fn fetch(&mut self) -> Result<Fetch> {
        // PC 레지스터에서 주소 가져오기
        let opcode = self.fetch_opcode()?;
        println!("[DEBUG] Fetched opcode: 0x{:02X}", opcode);

        let ins = self.instruction.get_instruction_info(opcode);

        // 유효하지 않은 opcode 처리
        if ins.is_none() {
            println!("[CPU] Invalid opcode: 0x{:02X}", opcode);
            self.halt_with_reason(InterruptType::IllegalOpcode);
            return Err(Error::InvalidOpcode(opcode));
        }

        let ins = ins.unwrap();
        println!("[DEBUG] Instruction info: {:?}", ins);

        let operand = self.fetch_operand(&ins)?;
        Ok(Fetch::new(ins, operand))
    }

    /// 명령어의 피연산자(operand) 가져오기
    fn fetch_operand(&mut self, ins: &types::InstructionInfo) -> Result<Vec<u8>> {
        let need = ins.get_operand_size();
        println!("[DEBUG] Need {} bytes for operand", need);

        match need {
            0 => Ok(vec![]),
            1 => self.fetch_one_byte_operand(ins),
            2 => self.fetch_two_byte_operand(),
            _ => {
                println!("[DEBUG] Unexpected operand size: {}", need);
                Ok(vec![])
            }
        }
    }

    /// 1바이트 피연산자 가져오기 (분기 명령어와 일반 명령어 구분)
    fn fetch_one_byte_operand(&mut self, ins: &types::InstructionInfo) -> Result<Vec<u8>> {
        let pc = self.get_pc();

        // 분기 명령어인지 확인
        let is_branch = matches!(
            ins.instruction,
            Instruction::BCC(_)
                | Instruction::BCS(_)
                | Instruction::BEQ(_)
                | Instruction::BNE(_)
                | Instruction::BMI(_)
                | Instruction::BPL(_)
                | Instruction::BVC(_)
                | Instruction::BVS(_)
        );

        let operand = self.read_memory(pc)?;
        self.increment_pc(1);

        if is_branch {
            // 분기 명령어인 경우 signed byte로 표시
            let signed_operand = operand as i8;
            println!("[DEBUG] Fetched 1-byte branch offset: {:+}", signed_operand);
        } else {
            println!("[DEBUG] Fetched 1-byte operand: 0x{:02X}", operand);
        }

        Ok(vec![operand])
    }

    /// 2바이트 피연산자 가져오기
    fn fetch_two_byte_operand(&mut self) -> Result<Vec<u8>> {
        let pc = self.get_pc();
        let low_byte = self.read_memory(pc)?;
        self.increment_pc(1);

        let high_byte = self.read_memory(self.get_pc())?;
        self.increment_pc(1);

        println!(
            "[DEBUG] Fetched 2-byte operand: 0x{:02X}{:02X}",
            high_byte, low_byte
        );

        Ok(vec![low_byte, high_byte])
    }

    /// 메모리 읽기 (MemoryBus 사용)
    pub fn read_memory(&self, address: u16) -> Result<u8> {
        if let Some(bus) = &self.memory_bus {
            let value = bus
                .lock()
                .map_err(|_| Error::FailedToLockMemoryBus)?
                .read(address);
            Ok(value)
        } else {
            Err(Error::MemoryBusConnectionFailed)
        }
    }

    /// 메모리 쓰기 (MemoryBus 사용)
    pub fn write_memory(&self, address: u16, value: u8) -> Result<()> {
        if let Some(bus) = &self.memory_bus {
            bus.lock()
                .map_err(|_| Error::FailedToLockMemoryBus)?
                .write(address, value);
            Ok(())
        } else {
            Err(Error::MemoryBusConnectionFailed)
        }
    }

    pub fn run(&mut self) -> Result<()> {
        self.state = CPUState::Running;
        while self.state == CPUState::Running {
            self.step()?;
        }
        Ok(())
    }

    /// 명령어 실행
    pub fn step(&mut self) -> Result<()> {
        match self.state {
            CPUState::Halted(reason) => {
                println!("[CPU] CPU is halted: {:?}", reason);
                Ok(())
            }
            CPUState::Ready => {
                self.state = CPUState::Running;
                self.execute_cycle()
            }
            CPUState::Running => self.execute_cycle(),
        }
    }

    /// 명령어 실행 사이클
    pub(crate) fn execute_cycle(&mut self) -> Result<()> {
        if self.state != CPUState::Running {
            return Ok(());
        }

        // Check for pending interrupts
        if let Some(interrupt) = self.check_interrupts() {
            self.handle_interrupt(interrupt)?;
            return Ok(());
        }

        // 1. 명령어 가져오기
        let fetch = self.fetch()?;
        let decode = self.instruction.decode(fetch)?;

        // 2. 사이클 설정
        self.cycles = decode.cycles;

        // 3. 명령어 실행
        self.execute(decode)?;

        println!("[DEBUG] PC: {:?}", self.get_pc());
        println!("[DEBUG] P: {:?}", self.status_flag());
        println!("[DEBUG] A: {:?}", self.get_value(RegisterType::A));
        println!("[DEBUG] X: {:?}", self.get_value(RegisterType::X));
        println!("[DEBUG] Y: {:?}", self.get_value(RegisterType::Y));
        // println!("{}", self.dump_flag());
        println!("[CPU] Instruction completed in {} cycles", self.cycles);
        Ok(())
    }

    /// 인터럽트 체크
    fn check_interrupts(&self) -> Option<InterruptType> {
        // 실제 하드웨어에서는 여기서 외부 인터럽트 핀의 상태를 체크합니다
        // 현재는 예시로 구현

        // NMI가 최우선
        if self.check_nmi_pin() {
            return Some(InterruptType::NMI);
        }

        // IRQ는 인터럽트 비활성화 플래그가 설정되어 있지 않을 때만
        if !self.get_flag(StatusRegister::INTERRUPT_DISABLE) && self.check_irq_pin() {
            return Some(InterruptType::IRQ);
        }

        None
    }

    /// NMI 핀 상태 체크 (하드웨어 구현 필요)
    fn check_nmi_pin(&self) -> bool {
        // TODO: 실제 하드웨어 NMI 핀 상태 체크 구현
        false
    }

    /// IRQ 핀 상태 체크 (하드웨어 구현 필요)
    fn check_irq_pin(&self) -> bool {
        // TODO: 실제 하드웨어 IRQ 핀 상태 체크 구현
        false
    }

    /// 스택 포인터의 실제 메모리 주소 계산
    fn get_stack_address(&self, offset: u8) -> u16 {
        0x0100 | (offset as u16)
    }

    /// 스택에 푸시
    pub(crate) fn stack_push(&mut self, value: u8) -> Result<()> {
        let sp = self.get_value(RegisterType::S).as_u8();
        self.write_memory(self.get_stack_address(sp), value)?;
        self.set_value(RegisterType::S, RegisterData::Bit8(sp.wrapping_sub(1)));
        Ok(())
    }

    /// 스택에서 풀
    pub(crate) fn stack_pull(&mut self) -> Result<u8> {
        let sp = self.get_value(RegisterType::S).as_u8();
        let new_sp = sp.wrapping_add(1);
        self.set_value(RegisterType::S, RegisterData::Bit8(new_sp));
        self.read_memory(self.get_stack_address(new_sp))
    }

    /// 16비트 값을 스택에 푸시 (상위 바이트 먼저)
    pub(crate) fn stack_push_u16(&mut self, value: u16) -> Result<()> {
        let high = (value >> 8) as u8;
        let low = value as u8;
        self.stack_push(high)?;
        self.stack_push(low)
    }

    /// 스택에서 16비트 값을 풀 (하위 바이트 먼저)
    pub(crate) fn stack_pull_u16(&mut self) -> Result<u16> {
        let low = self.stack_pull()? as u16;
        let high = self.stack_pull()? as u16;
        Ok((high << 8) | low)
    }

    /// 인터럽트 처리
    pub fn handle_interrupt(&mut self, interrupt: InterruptType) -> Result<()> {
        match interrupt {
            InterruptType::Reset => {
                self.reset();
                // Reset vector at 0xFFFC-0xFFFD
                let low = self.read_memory(0xFFFC)?;
                let high = self.read_memory(0xFFFD)?;
                let reset_vector = ((high as u16) << 8) | (low as u16);
                self.set_pc(reset_vector);
            }
            InterruptType::NMI => {
                // NMI vector at 0xFFFA-0xFFFB
                self.stack_push_u16(self.get_pc())?;
                self.stack_push(self.get_value(RegisterType::P).as_u8())?;
                let low = self.read_memory(0xFFFA)?;
                let high = self.read_memory(0xFFFB)?;
                let nmi_vector = ((high as u16) << 8) | (low as u16);
                self.set_pc(nmi_vector);
            }
            InterruptType::IRQ => {
                // IRQ vector at 0xFFFE-0xFFFF (same as BRK)
                if !self.get_flag(StatusRegister::INTERRUPT_DISABLE) {
                    self.stack_push_u16(self.get_pc())?;
                    self.stack_push(self.get_value(RegisterType::P).as_u8())?;
                    let low = self.read_memory(0xFFFE)?;
                    let high = self.read_memory(0xFFFF)?;
                    let irq_vector = ((high as u16) << 8) | (low as u16);
                    self.set_pc(irq_vector);
                }
            }
            _ => self.halt_with_reason(interrupt),
        }
        Ok(())
    }

    /// 사이클 추가
    pub(crate) fn add_cycles(&mut self, cycles: u8) {
        self.cycles += cycles;
    }

    /// 현재 사이클 수 가져오기
    pub fn get_cycles(&self) -> u8 {
        self.cycles
    }

    /// 플래그 레지스터(P)의 상태를 읽기 쉽게 출력합니다
    pub fn dump_flag(&self) -> String {
        let flags = self.registers.get_value(RegisterType::P).as_u8();
        let n = (flags >> 7) & 1;
        let v = (flags >> 6) & 1;
        let u = (flags >> 5) & 1; // 미사용 플래그 (항상 1)
        let b = (flags >> 4) & 1;
        let d = (flags >> 3) & 1;
        let i = (flags >> 2) & 1;
        let z = (flags >> 1) & 1;
        let c = flags & 1;

        let binary = format!("{:08b}", flags);

        let mut result = format!("{}\n", binary);
        result.push_str("||||||\\- Carry (C) = ");
        result.push_str(&format!("{} {}\n", c, if c == 1 { "✓" } else { "✗" }));

        result.push_str("|||||\\-- Zero (Z) = ");
        result.push_str(&format!("{} {}\n", z, if z == 1 { "✓" } else { "✗" }));

        result.push_str("||||\\--- Interrupt (I) = ");
        result.push_str(&format!(
            "{} {}\n",
            i,
            if i == 1 {
                "✓ (IRQ 비활성화)"
            } else {
                "✗ (IRQ 활성화)"
            }
        ));

        result.push_str("|||\\---- Decimal (D) = ");
        result.push_str(&format!("{} {}\n", d, if d == 1 { "✓" } else { "✗" }));

        result.push_str("||\\----- Break (B) = ");
        result.push_str(&format!("{} {}\n", b, if b == 1 { "✓" } else { "✗" }));

        result.push_str("|\\------ Unused = ");
        result.push_str(&format!(
            "{} {}\n",
            u,
            if u == 1 {
                "✓ (항상 1)"
            } else {
                "✗ (오류!)"
            }
        ));

        result.push_str("\\------- Overflow (V) = ");
        result.push_str(&format!("{} {}\n", v, if v == 1 { "✓" } else { "✗" }));

        result.push_str("-------- Negative (N) = ");
        result.push_str(&format!("{} {}", n, if n == 1 { "✓" } else { "✗" }));

        result
    }

    /// 플래그 레지스터의 상태를 콘솔에 출력합니다
    pub fn print_flags(&self) {
        println!("{}", self.dump_flag());
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::register::StatusRegister;

    #[test]
    fn test_cpu_reset() {
        let mut cpu = CPU::new();
        cpu.reset();

        assert_eq!(cpu.get_value(RegisterType::A), RegisterData::Bit8(0));
        assert_eq!(cpu.get_value(RegisterType::X), RegisterData::Bit8(0));
        assert_eq!(cpu.get_value(RegisterType::Y), RegisterData::Bit8(0));
        assert_eq!(
            cpu.get_value(RegisterType::P),
            RegisterData::Bit8(StatusRegister::UNUSED.bits())
        );
        assert_eq!(cpu.get_value(RegisterType::S), RegisterData::Bit8(0xFD));
        assert_eq!(cpu.get_value(RegisterType::PC), RegisterData::Bit16(0));

        cpu.set_value(RegisterType::A, RegisterData::Bit8(0x10));
        cpu.set_value(RegisterType::X, RegisterData::Bit8(0x20));
        cpu.set_value(RegisterType::Y, RegisterData::Bit8(0x30));
        cpu.set_value(
            RegisterType::P,
            RegisterData::Bit8(0x40 | StatusRegister::UNUSED.bits()),
        );
        cpu.set_value(RegisterType::S, RegisterData::Bit8(0x50));
        cpu.set_value(RegisterType::PC, RegisterData::Bit16(0x60));

        assert_eq!(cpu.get_value(RegisterType::A), RegisterData::Bit8(0x10));
        assert_eq!(cpu.get_value(RegisterType::X), RegisterData::Bit8(0x20));
        assert_eq!(cpu.get_value(RegisterType::Y), RegisterData::Bit8(0x30));
        assert_eq!(
            cpu.get_value(RegisterType::P),
            RegisterData::Bit8(0x40 | StatusRegister::UNUSED.bits())
        );
        assert_eq!(cpu.get_value(RegisterType::S), RegisterData::Bit8(0x50));
        assert_eq!(cpu.get_value(RegisterType::PC), RegisterData::Bit16(0x60));
    }

    #[test]
    fn test_register() {
        let cpu = CPU::new();
        let mut status = cpu.status_flag();
        println!("{:?}", status);
        status.set(StatusRegister::CARRY, true);
        println!("{:?}", status);
        assert!(status.contains(StatusRegister::CARRY));
        assert!(!status.contains(StatusRegister::ZERO));
        assert!(!status.contains(StatusRegister::INTERRUPT_DISABLE));
        assert!(!status.contains(StatusRegister::DECIMAL));
        assert!(!status.contains(StatusRegister::BREAK));
    }
}
