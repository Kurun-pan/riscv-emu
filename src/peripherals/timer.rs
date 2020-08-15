pub trait Timer {
    fn tick(&mut self);
    fn is_pending_software_interrupt(&mut self, core: usize) -> bool;
    fn is_pending_timer_interrupt(&mut self, core: usize) -> bool;
    fn read(&mut self, addr: u64) -> u32;
    fn write(&mut self, addr: u64, data: u32);
}