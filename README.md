# mos-6502kit

[![Rust](https://github.com/mixL1nk/mos-6502kit/actions/workflows/test-rust.yml/badge.svg)](https://github.com/mixL1nk/mos-6502kit/actions/workflows/test-rust.yml)
[![License: MIT](https://img.shields.io/badge/License-MIT-yellow.svg)](https://opensource.org/licenses/MIT)

🚀 A comprehensive MOS 6502 development toolkit featuring emulation, interactive debugging, and educational tools. Perfect for learning assembly programming, hardware emulation, and retro computing. 🎓💻

## Overview

MOS-6502Kit is a comprehensive development toolkit for learning and working with the MOS 6502 processor. It provides a cycle-accurate CPU emulator, debugger, assembler, and development tools to help understand 6502 assembly programming.

## Features

### Core Components

#### CPU Emulator

- ✅ Complete MOS 6502 instruction set
- ✅ Cycle-accurate execution
- ✅ All addressing modes supported
- ✅ BCD arithmetic support
- ✅ Interrupt handling with debugger integration
  - Hardware interrupts (IRQ, NMI)
  - Software interrupts (BRK)
  - Breakpoint interrupts
- ✅ Event-based monitoring system

#### Memory System

- ✅ Full 64KB address space
- ✅ Memory bus interface
- ✅ Memory access tracking
- ✅ Memory dump functionality

#### Debugger

- ✅ Breakpoint management
  - Execution breakpoints
  - Memory access breakpoints
  - Conditional breakpoints
- ✅ CPU state monitoring
  - Register inspection
  - Flag status tracking
  - Instruction execution tracking
- ✅ Memory inspection
  - Memory dumps
  - Access history

#### Assembler

- ✅ 6502 assembly parsing
- ✅ Machine code generation
- ✅ Label support
- ✅ Error reporting

## Project Structure

```bash
crates/
├── assembler/     # Assembly language processing
├── breadboard/    # System integration
├── common/        # Shared utilities
├── cpu/          # CPU emulation core
├── debugger/     # Debug functionality
├── disassembler/ # Machine code analysis
├── error/        # Error handling
├── memory/       # Memory management
└── types/        # Common types

```

## Usage Example

```rust
use cpu::CPU;
use debugger::Debugger;
use memory::Memory;
use std::sync::{Arc, Mutex};

// Initialize CPU and memory
let mut cpu = CPU::new();
let memory = Arc::new(Mutex::new(Memory::new()));
cpu.set_memory_bus(memory);

// Configure debugger
let mut debugger = Debugger::new();
debugger.attach(cpu);
debugger.add_breakpoint(0x1000, AccessType::Access);
debugger.enabled()?;

// Execute program
debugger.run_cpu()?;
```

## Development Status

### Implemented

- ✅ CPU core with full 6502 instruction set
- ✅ Memory system with bus interface
- ✅ Debugger with breakpoint support
- ✅ Basic assembler functionality

### In Progress

- 🔄 Test coverage expansion
- 🔄 Documentation improvements
- 🔄 GUI integration planning

### Planned

- ⏳ Performance optimizations
- ⏳ Extended debugging features
- ⏳ Additional development tools

## Contributing

Contributions are welcome! Please feel free to submit issues and pull requests.

## License

This project is licensed under the MIT License - see the [LICENSE](LICENSE) file for details.
