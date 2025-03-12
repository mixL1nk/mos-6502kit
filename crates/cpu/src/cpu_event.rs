use crate::CPU;
use crate::register::{RegisterValue, Registers};

pub type EventHandler = Box<dyn Fn(&CPUEvent)>;

#[derive(Debug, Clone)]
pub enum CPUEvent {
    InstructionExecuted {
        pc: u16,
        opcode: u8,
        cycles: u8,
        operand: u16,
    },
    RegisterChanged {
        register: String,
        value: u16,
        old_value: u16,
    },
    MemoryRead {
        address: u16,
        value: u8,
    },
    MemoryWrite {
        address: u16,
        value: u8,
    },
    FlagChanged {
        flag: String,
        value: bool,
        old_value: bool,
    },
    StateChanged {
        state: CPUContext,
    },
}

#[derive(Debug, Clone)]
pub struct CPUContext {
    pub a: u8,
    pub x: u8,
    pub y: u8,
    pub s: u8,
    pub p: u8,
    pub pc: u16,
}

impl From<Registers> for CPUContext {
    fn from(regs: Registers) -> Self {
        Self {
            a: regs.a.get(),
            x: regs.x.get(),
            y: regs.y.get(),
            p: regs.p.get(),
            s: regs.s.get(),
            pc: regs.pc.get(),
        }
    }
}

impl CPU {
    pub fn register_event_handler(&mut self, event_handler: EventHandler) {
        self.event_handlers.push(event_handler)
    }

    pub fn clear_event_handler(&mut self) {
        self.event_handlers.clear();
    }

    pub fn enabled_debug(&mut self, enabled: bool) {
        self.debug_enabled = enabled;
    }

    pub(crate) fn emit_event(&self, event: CPUEvent) {
        if self.debug_enabled {
            for handler in &self.event_handlers {
                handler(&event);
            }
        }
    }
}

impl std::fmt::Debug for CPU {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("CPU")
            .field("registers", &self.registers)
            .field("memory_bus", &self.memory_bus)
            .field("instruction", &self.instruction)
            .field("state", &self.state)
            .field("cycles", &self.cycles)
            // event_handlers는 제외
            .field("debug_enabled", &self.debug_enabled)
            .finish()
    }
}

impl std::clone::Clone for CPU {
    fn clone(&self) -> Self {
        Self {
            registers: self.registers.clone(),
            memory_bus: self.memory_bus.clone(),
            instruction: self.instruction.clone(),
            state: self.state,
            cycles: self.cycles,
            event_handlers: vec![],
            debug_enabled: self.debug_enabled,
            interrupt_channel: None,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::register::StatusRegister;
    use std::sync::{Arc, Mutex};

    #[test]
    fn test_cpu_context_from_registers() {
        let regs = Registers::default();
        let context: CPUContext = regs.into();
        println!("{:?}", context);
        assert_eq!(context.a, 0);
        assert_eq!(context.x, 0);
        assert_eq!(context.y, 0);
        assert_eq!(context.s, 0xfd); // default value
        assert_eq!(context.p, StatusRegister::UNUSED.bits()); // default value
        assert_eq!(context.pc, 0);
    }

    #[test]
    fn test_cpu_emit_event() {
        let mut cpu = CPU::default();
        cpu.enabled_debug(true);

        let received_events = Arc::new(Mutex::new(Vec::<CPUEvent>::new()));
        let events_clone = received_events.clone();

        cpu.register_event_handler(Box::new(move |event| {
            let mut events = events_clone.lock().expect("lock failed");
            events.push(event.clone());
        }));

        let event = CPUEvent::InstructionExecuted {
            pc: 0x8000,
            opcode: 0x69,
            cycles: 2,
            operand: 0x42,
        };
        println!("emit event: {:?}", event);
        cpu.emit_event(event);

        let events = received_events.lock().expect("lock failed");
        assert_eq!(events.len(), 1);

        println!("received events: {:?}", events);
    }
}
