#[allow(dead_code)]
#[derive(Debug)]
pub enum Device {
    Dram = 0,
    SpiFlash = 1,
    Disk = 2,
}

pub trait Bus {
    fn set_device_data(&mut self, device: Device, data: Vec<u8>);
    fn get_base_address(&mut self, device: Device) -> u64;
    fn tick(&mut self) -> Vec<bool>;
    fn is_pending_software_interrupt(&mut self, core: usize) -> bool;
    fn is_pending_timer_interrupt(&mut self, core: usize) -> bool;
    fn read8(&mut self, addr: u64) -> Result<u8, ()>;
    fn read16(&mut self, addr: u64) -> Result<u16, ()>;
    fn read32(&mut self, addr: u64) -> Result<u32, ()>;
    fn read64(&mut self, addr: u64) -> Result<u64, ()>;
    fn write8(&mut self, addr: u64, data: u8) -> Result<(), ()>;
    fn write16(&mut self, addr: u64, data: u16) -> Result<(), ()>;
    fn write32(&mut self, addr: u64, data: u32) -> Result<(), ()>;
    fn write64(&mut self, addr: u64, data: u64) -> Result<(), ()>;
}
