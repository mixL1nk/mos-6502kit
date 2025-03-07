use thiserror::Error;

#[derive(Error, Debug)]
pub enum SystemError {
    #[error("CPU Error: {0}")]
    CPU(#[from] CPUError),
    #[error("Memory Error: {0}")]
    Memory(#[from] MemoryError),
    #[error("BreadBoard Error: {0}")]
    BreadBoard(#[from] BreadBoardError),

    #[error("General Error: {0}")]
    General(String),
}

// String 및 &str에서 SystemError로의 자동 변환
impl From<String> for SystemError {
    fn from(s: String) -> Self {
        SystemError::General(s)
    }
}

impl From<&str> for SystemError {
    fn from(s: &str) -> Self {
        SystemError::General(s.to_string())
    }
}

#[derive(Error, Debug)]
pub enum CPUError {
    #[error("Invalid opcode: {0}")]
    InvalidOpcode(u8),
    #[error("Invalid address mode: {0}")]
    InvalidAddressMode(u8),
    #[error("Invalid operand")]
    InvalidOperand(u8),
    #[error("Invalid register: {0}")]
    InvalidRegister(u8),
    #[error("Decode error: {0}")]
    Decode(String),
    #[error("Internal error: {0}")]
    Internal(String),
}

#[derive(Error, Debug)]
pub enum MemoryError {
    #[error("Invalid memory address: {0}")]
    InvalidMemoryAddress(u16),
    #[error("Invalid memory range: {0}")]
    InvalidMemoryRange(u16),
}

#[derive(Error, Debug)]
pub enum BreadBoardError {
    #[error("Duplicaed Component id: {0}")]
    DuplicateComponentId(String),
    #[error("Component not found: {0}")]
    ComponentNotFound(String),
    #[error("Failed to Process transaction")]
    FailedToProcessTransaction,
    #[error("Failed to send transaction")]
    FailedToSendTransaction,
    #[error("Failed to receive transaction")]
    FailedToReceiveTransaction,
    #[error("Failed to lock component: {0}")]
    FailedToLockComponent(String),
    #[error("Invalid Bus Operation")]
    InvalidBusOperation
}

pub type Result<T> = std::result::Result<T, SystemError>;
