use derive_more::From;
use std::io;

#[derive(Debug, From)]
pub enum Error {
    // -- CPU errors
    InvalidOpcode(u8),
    InvalidAddressMode(u8),
    InvalidOperand(u8),
    InvalidRegister(u8),
    InvalidInstruction {
        inst_type: &'static str,
    },
    FailedToLockMemoryBus,
    MemoryBusConnectionFailed,
    Internal(String),
    // -- Memory errors
    InvalidMemoryAddress(u16),
    InvalidMemoryRange(u16),
    // -- Breadboard errors
    DuplicateComponentId(String),
    ComponentNotFound(String),
    FailedToLockComponent(String),
    // -- Assembler errors
    InvalidNumber {
        line: usize,
        column: usize,
    },
    InvalidHexNumber {
        line: usize,
        column: usize,
    },
    InvalidBinaryNumber {
        line: usize,
        column: usize,
    },
    UnexpectedEndMacro {
        line: usize,
        column: usize,
    },
    #[from]
    Io(io::Error),
    // -- External errors
    // not yet implemented
}

impl core::fmt::Display for Error {
    fn fmt(&self, fmt: &mut core::fmt::Formatter) -> core::result::Result<(), core::fmt::Error> {
        write!(fmt, "{self:?}")
    }
}

impl std::error::Error for Error {}

pub type Result<T> = std::result::Result<T, Error>;
