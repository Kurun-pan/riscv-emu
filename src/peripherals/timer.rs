use crate::peripherals::fu540_c000::clint::Clint;

pub struct Timer {
    clint: Clint
}

impl Timer {
    pub fn new() -> Self {
        Timer {
            clint: Clint::new(),
        }
    }

    pub fn tick(&mut self) {
        self.clint.tick()
    }

    pub fn is_pending_software_interrupt(&mut self, core: usize) -> bool {
        self.clint.is_pending_software_interrupt(core)
    }

    pub fn is_pending_timer_interrupt(&mut self, core: usize) -> bool {
        self.clint.is_pending_timer_interrupt(core)
    }

    pub fn read(&mut self, addr: u64) -> u32 {
        self.clint.read(addr)
    }

    pub fn write(&mut self, addr: u64, data: u32) {
        self.clint.write(addr, data)
    }
}
