// INTC (Interrupt Controller)

pub trait Intc {
    fn tick(&mut self, core: usize, interrupts: Vec<usize>) -> Vec<bool>;
    fn read(&mut self, addr: u64) -> u32;
    fn write(&mut self, addr: u64, data: u32);
}