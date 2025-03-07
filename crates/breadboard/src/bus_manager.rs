use common::Result;
use common::bus::{BusInterface, BusTransaction};
use error::BreadBoardError;
use std::collections::HashMap;
use std::sync::mpsc::{Receiver, Sender, channel};
use std::sync::{Arc, Mutex};
use std::thread;

/// 버스 관리자 구조체
pub struct BusManager {
    /// 트랜잭션 큐
    transactions: Arc<Mutex<Vec<BusTransaction>>>,
    /// 컴포넌트 통신 채널
    channels: HashMap<String, Sender<BusTransaction>>,
    /// 응답 수신 채널
    response_rx: Receiver<BusTransaction>,
    /// 응답 송신 채널
    response_tx: Sender<BusTransaction>,
    /// 컴포넌트 인터페이스
    components: Arc<Mutex<HashMap<String, Arc<Mutex<dyn BusInterface + Send>>>>>,
}

impl BusManager {
    /// 새 버스 관리자 생성
    pub fn new() -> Self {
        let (response_tx, response_rx) = channel();

        Self {
            transactions: Arc::new(Mutex::new(Vec::new())),
            channels: HashMap::new(),
            response_rx,
            response_tx,
            components: Arc::new(Mutex::new(HashMap::new())),
        }
    }

    /// 컴포넌트 등록 (인터페이스와 함께)
    pub fn register_component_interface(
        &mut self,
        component_id: &str,
        interface: Arc<Mutex<dyn BusInterface + Send>>,
    ) -> Sender<BusTransaction> {
        // 컴포넌트 저장
        self.components
            .lock()
            .unwrap()
            .insert(component_id.to_string(), interface);

        // 통신 채널 설정
        let (tx, rx) = channel();
        self.channels.insert(component_id.to_string(), tx.clone());

        let response_tx = self.response_tx.clone();
        let component_id = component_id.to_string();
        let components = self.components.clone();

        // 컴포넌트의 트랜잭션 처리 스레드 생성
        thread::spawn(move || {
            while let Ok(mut transaction) = rx.recv() {
                println!(
                    "[BUS] Component {} received transaction: {:?}",
                    component_id, transaction
                );

                // 컴포넌트 인터페이스를 통한 실제 트랜잭션 처리
                if let Some(interface) = components.lock().unwrap().get(&component_id) {
                    let mut interface = interface.lock().unwrap();
                    match interface.process_bus_transaction(&mut transaction) {
                        Ok(_) => {
                            println!(
                                "[BUS] Component {} successfully processed transaction",
                                component_id
                            );
                        }
                        Err(e) => {
                            println!(
                                "[BUS] Component {} failed to process transaction: {}",
                                component_id, e
                            );
                        }
                    }
                }

                // 응답 전송
                if let Err(e) = response_tx.send(transaction) {
                    println!("[BUS] Error sending response: {:?}", e);
                    break;
                }
            }
        });

        tx
    }

    // /// 컴포넌트 등록 (레거시 호환용, 실제 처리하지 않음)
    // pub fn register_component(&mut self, component_id: &str) -> Sender<BusTransaction> {
    //     let (tx, _) = channel();
    //     self.channels.insert(component_id.to_string(), tx.clone());
    //     tx
    // }

    /// 트랜잭션 실행 (실제 컴포넌트 인터페이스 사용)
    pub fn execute_transaction_direct(
        &mut self,
        from_id: &str,
        to_id: &str,
        mut transaction: BusTransaction,
    ) -> Result<BusTransaction> {
        println!(
            "[BUS] Direct transaction: {} -> {}, type={:?}, addr=0x{:04x}, data=0x{:02x}",
            from_id, to_id, transaction.operation_type, transaction.address, transaction.data
        );

        let components = self.components.lock().unwrap();

        // 컴포넌트 맵 디버그 출력
        println!("[BUS] Components registered: {}", components.len());
        for key in components.keys() {
            println!("[BUS]   - Component: {}", key);
        }

        // 대상 컴포넌트 확인
        let to_component = components.get(to_id).ok_or_else(|| {
            println!("[BUS] ERROR: Component '{}' not found!", to_id);
            BreadBoardError::ComponentNotFound(to_id.to_string())
        })?;

        // 트랜잭션 처리
        // 트랜잭션 처리
        {
            let mut to_interface = to_component.lock().unwrap();
            to_interface
                .process_bus_transaction(&mut transaction)
                .map_err(|e| {
                    println!("[BUS] ERROR: Failed to process transaction: {}", e);
                    BreadBoardError::FailedToProcessTransaction
                })?;

            println!("[BUS] Transaction processed by {}", to_id);
        }

        // 출발지 컴포넌트 응답 처리
        // 출발지 컴포넌트 응답 처리
        if let Some(from_component) = components.get(from_id) {
            let mut from_interface = from_component.lock().unwrap();
            from_interface
                .respond_to_transaction(&mut transaction)
                .map_err(|e| {
                    // 직접 SystemError 생성자를 사용
                    error::SystemError::BreadBoard(BreadBoardError::ComponentNotFound(
                        from_id.to_string(),
                    ))
                })?;
        }

        Ok(transaction)
    }

    /// 트랜잭션 실행 (이전 채널 기반 방식)
    pub fn execute_transaction(
        &mut self,
        from: &str,
        to: &str,
        transaction: BusTransaction,
    ) -> Result<BusTransaction> {
        // 직접 실행 방식 사용
        if !self.components.lock().unwrap().is_empty() {
            return self.execute_transaction_direct(from, to, transaction);
        }

        // 이전 방식 (테스트용으로 유지)
        self.transactions.lock().unwrap().push(transaction.clone());

        if let Some(tx) = self.channels.get(to) {
            // 단순히 BreadBoardError 반환 (SystemError로 자동 변환됨)
            tx.send(transaction)
                .map_err(|_| BreadBoardError::FailedToSendTransaction)?;

            // 단순히 BreadBoardError 반환 (SystemError로 자동 변환됨)
            let response = self
                .response_rx
                .recv()
                .map_err(|_| BreadBoardError::FailedToReceiveTransaction)?;

            Ok(response)
        } else {
            // 단순히 BreadBoardError 반환 (SystemError로 자동 변환됨)
            Err(BreadBoardError::ComponentNotFound(to.to_string()))?
        }
    }
}
