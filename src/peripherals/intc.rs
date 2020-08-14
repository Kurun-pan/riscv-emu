// INTC (Interrupt Controller)

use crate::peripherals::fu540_c000::plic::Plic;

pub struct Intc {
    plic: Plic
}

impl Intc {
    pub fn new() -> Self {
        Intc {
            plic: Plic::new(),
        }
    }

    pub fn tick(&mut self, core: usize, interrupts: Vec<usize>) -> Vec<bool> {
        self.plic.tick(core, interrupts)
    }

    pub fn read(&mut self, addr: u64) -> u32 {
        self.plic.read(addr)
    }

    pub fn write(&mut self, addr: u64, data: u32) {
        self.plic.write(addr, data)
    }
}