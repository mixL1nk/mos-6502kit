use cpu::CPU;
use memory::Memory;
use std::collections::HashMap;
use std::sync::{Arc, Mutex};
use error::BreadBoardError;
use crate::bus_manager::BusManager;
use common::bus::{BusInterface, BusTransaction};
use common::Result;

pub enum ComponentType {
    Cpu(CPU),
    Memory(Memory),
    // 필요에 따라 다른 컴포넌트 추가
}

impl ComponentType {
    pub fn get_id(&self) -> &str {
        match self {
            ComponentType::Cpu(cpu) => cpu.get_id(),
            ComponentType::Memory(mem) => mem.get_id(),
        }
    }

    // // BusInterface 트레이트 접근
    pub fn as_bus_interface_mut(&mut self) -> &mut dyn BusInterface {
        match self {
            ComponentType::Cpu(cpu) => cpu,
            ComponentType::Memory(mem) => mem,
        }
    }

    // CPU 특정 메서드 접근을 위한 헬퍼 메서드
    pub fn as_cpu(&self) -> Option<&CPU> {
        match self {
            ComponentType::Cpu(cpu) => Some(cpu),
            _ => None,
        }
    }

    pub fn as_cpu_mut(&mut self) -> Option<&mut CPU> {
        match self {
            ComponentType::Cpu(cpu) => Some(cpu),
            _ => None,
        }
    }

    // Memory 특정 메서드 접근을 위한 헬퍼 메서드
    pub fn as_memory(&self) -> Option<&Memory> {
        match self {
            ComponentType::Memory(mem) => Some(mem),
            _ => None,
        }
    }

    pub fn as_memory_mut(&mut self) -> Option<&mut Memory> {
        match self {
            ComponentType::Memory(mem) => Some(mem),
            _ => None,
        }
    }
}

/// 브레드보드 구조체
pub struct BreadBoard {
    /// 컴포넌트 목록
    components: Arc<Mutex<Vec<ComponentType>>>,
    /// 컴포넌트 ID와 인덱스 매핑
    component_indices: HashMap<String, usize>,
    /// 버스 관리자
    bus_manager: Arc<Mutex<BusManager>>,
}

impl Default for BreadBoard {
    fn default() -> Self {
        Self::new()
    }
}

impl BreadBoard {
    /// 새로운 브레드보드 생성
    pub fn new() -> Self {
        Self {
            components: Arc::new(Mutex::new(Vec::new())),
            component_indices: HashMap::new(),
            bus_manager: Arc::new(Mutex::new(BusManager::new())),
        }
    }

    /// 컴포넌트 추가 및 버스에 연결
    pub fn add_component(
        &mut self,
        component: impl Into<ComponentType>,
    ) -> Result<()> {
        let component = component.into();
        let id = component.get_id().to_string();

        if self.component_indices.contains_key(&id) {
            return Err(BreadBoardError::DuplicateComponentId(id).into());
        }

        // 컴포넌트 인덱스 저장
        let index = self.components.lock().unwrap().len();
        self.components.lock().unwrap().push(component);
        self.component_indices.insert(id.clone(), index);

        // 컴포넌트 인터페이스 버스 매니저에 등록
        self.register_component_interface(&id)?;

        Ok(())
    }

    /// 컴포넌트 인터페이스를 버스 매니저에 등록
    fn register_component_interface(&self, component_id: &str) -> Result<()> {
        // 각 컴포넌트에 대한 BusInterface 구현 래핑
        let interface = self.create_component_interface(component_id)?;

        // 버스 매니저에 등록
        self.bus_manager
            .lock()
            .unwrap()
            .register_component_interface(component_id, interface);

        Ok(())
    }

    /// 컴포넌트 인터페이스 생성 - 실제 컴포넌트와 인터페이스 연결
    fn create_component_interface(
        &self,
        component_id: &str,
    ) -> Result<Arc<Mutex<dyn BusInterface + Send>>> {
        // 각 컴포넌트 타입에 맞는 인터페이스 구현
        let components_clone = self.components.clone();
        let indices = self.component_indices.clone();

        // 해당 컴포넌트 인덱스 찾기
        let index = indices
            .get(component_id)
            .ok_or::<BreadBoardError>(BreadBoardError::ComponentNotFound(component_id.to_string()))?;

        // CPU나 Memory 구현체 대신, 버스 인터페이스 구현 위임
        let interface = Arc::new(Mutex::new(ComponentInterfaceAdapter {
            components: components_clone.clone(),
            component_index: *index,
        }));

        Ok(interface)
    }

    pub fn with_component_mut<F, R>(&self, id: &str, f: F) -> Option<R>
    where
        F: FnOnce(&mut ComponentType) -> R,
    {
        let mut components = self.components.lock().unwrap();
        let index = *self.component_indices.get(id)?;
        Some(f(&mut components[index]))
    }

    /// CPU 메모리 읽기 작업 (버스 트랜잭션 사용)
    pub fn bus_cpu_memory_read(&self, address: u16) -> Result<u8> {
        println!("[BUS] CPU read from memory at address 0x{:04x}", address);

        // 읽기 트랜잭션 생성
        let transaction = BusTransaction::new_read(address);

        // 버스 트랜잭션 수행
        match self
            .bus_manager
            .lock()
            .unwrap()
            .execute_transaction("CPU", "Memory", transaction)
        {
            Ok(response) => {
                println!("[BUS] Read response: data=0x{:02x}", response.data);
                Ok(response.data)
            }
            Err(e) => Err(e),
        }
    }

    /// CPU 메모리 쓰기 작업 (버스 트랜잭션 사용)
    pub fn bus_cpu_memory_write(&self, address: u16, value: u8) -> Result<()> {
        println!(
            "[BUS] CPU write to memory at address 0x{:04x}, value 0x{:02x}",
            address, value
        );

        // 쓰기 트랜잭션 생성
        let transaction = BusTransaction::new_write(address, value);

        // 버스 트랜잭션 수행
        match self
            .bus_manager
            .lock()
            .unwrap()
            .execute_transaction("CPU", "Memory", transaction)
        {
            Ok(_) => {
                println!("[BUS] Write completed successfully");
                Ok(())
            }
            Err(e) => Err(e),
        }
    }

    /// CPU 버스 루프 실행
    pub fn run_cpu_bus_cycle(&self) -> Result<()> {
        // 클럭 사이클 시뮬레이션
        println!("[BUS] Running CPU bus cycle");

        // 명령어 페치 단계
        let instruction = self.bus_cpu_memory_read(0x1000)?;
        println!("[BUS] Fetched instruction: 0x{:02x}", instruction);

        Ok(())
    }
}

// CPU와 Memory에 대한 Into<ComponentType> 구현
impl From<CPU> for ComponentType {
    fn from(cpu: CPU) -> Self {
        ComponentType::Cpu(cpu)
    }
}

impl From<Memory> for ComponentType {
    fn from(memory: Memory) -> Self {
        ComponentType::Memory(memory)
    }
}

/// 컴포넌트 버스 인터페이스 어댑터 - 실제 컴포넌트 인터페이스 호출 위임
struct ComponentInterfaceAdapter {
    components: Arc<Mutex<Vec<ComponentType>>>,
    component_index: usize,
}

impl BusInterface for ComponentInterfaceAdapter {
    fn process_bus_transaction(&mut self, transaction: &mut BusTransaction) -> Result<()> {
        let mut components = self.components.lock().unwrap();
        if self.component_index >= components.len() {
            return Err("Component index out of bounds".into());
        }

        // 실제 컴포넌트의 인터페이스 메서드 호출
        components[self.component_index]
            .as_bus_interface_mut()
            .process_bus_transaction(transaction)
    }

    fn begin_transaction(&mut self, transaction: BusTransaction) -> Result<BusTransaction> {
        let mut components = self.components.lock().unwrap();
        if self.component_index >= components.len() {
            return Err("Component index out of bounds".into());
        }

        components[self.component_index]
            .as_bus_interface_mut()
            .begin_transaction(transaction)
    }

    fn respond_to_transaction(&mut self, transaction: &mut BusTransaction) -> Result<()> {
        let mut components = self.components.lock().unwrap();
        if self.component_index >= components.len() {
            return Err("Component index out of bounds".into());
        }

        components[self.component_index]
            .as_bus_interface_mut()
            .respond_to_transaction(transaction)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_breadboard_creation() {
        let board = BreadBoard::new();
        assert_eq!(board.components.lock().unwrap().len(), 0);
        assert_eq!(board.component_indices.len(), 0);
    }
}
