pub const MAX_DRAM_SIZE: u64 = 1024 * 1024 * 128;

pub struct Dram {
    pub dram: Vec<u8>,
    code_size: u64,
}

impl Dram {
    pub fn new() -> Self {
        Self {
            dram: vec![0; MAX_DRAM_SIZE as usize],
            code_size: 0,
        }
    }

    pub fn get_size(&self) -> u64 {
        self.code_size
    }

    pub fn initialize(&mut self, data: Vec<u8>) {
        self.code_size = data.len() as u64;
        self.dram.splice(..data.len(), data.iter().cloned());
    }

    pub fn write8(&mut self, addr: u64, val: u8) {
        self.dram[addr as usize] = val;
    }

    pub fn write16(&mut self, addr: u64, val: u16) {
        let index = addr  as usize;
        self.dram[index]     = (val & 0xff) as u8;
        self.dram[index + 1] = ((val >> 8) & 0xff) as u8;
    }

    pub fn write32(&mut self, addr: u64, val: u32) {
        let index = addr as usize;
        self.dram[index]     = (val & 0xff) as u8;
        self.dram[index + 1] = ((val >>  8) & 0xff) as u8;
        self.dram[index + 2] = ((val >> 16) & 0xff) as u8;
        self.dram[index + 3] = ((val >> 24) & 0xff) as u8;
    }

    pub fn write64(&mut self, addr: u64, val: u64) {
        let index = addr as usize;
        self.dram[index]     = (val & 0xff) as u8;
        self.dram[index + 1] = ((val >>  8) & 0xff) as u8;
        self.dram[index + 2] = ((val >> 16) & 0xff) as u8;
        self.dram[index + 3] = ((val >> 24) & 0xff) as u8;
        self.dram[index + 4] = ((val >> 32) & 0xff) as u8;
        self.dram[index + 5] = ((val >> 40) & 0xff) as u8;
        self.dram[index + 6] = ((val >> 48) & 0xff) as u8;
        self.dram[index + 7] = ((val >> 56) & 0xff) as u8;
    }

    pub fn read8(&self, addr: u64) -> u8 {
        let index = addr as usize;
        self.dram[index]
    }

    pub fn read16(&self, addr: u64) -> u16 {
        let index = addr as usize;
        return (self.dram[index] as u16) | ((self.dram[index + 1] as u16) << 8);
    }

    pub fn read32(&self, addr: u64) -> u32 {
        let index = addr as usize;
        return (self.dram[index] as u32)
            | ((self.dram[index + 1] as u32) << 8)
            | ((self.dram[index + 2] as u32) << 16)
            | ((self.dram[index + 3] as u32) << 24);
    }

    pub fn read64(&self, addr: u64) -> u64 {
        let index = addr as usize;
        return (self.dram[index] as u64)
            | ((self.dram[index + 1] as u64) << 8)
            | ((self.dram[index + 2] as u64) << 16)
            | ((self.dram[index + 3] as u64) << 24)
            | ((self.dram[index + 4] as u64) << 32)
            | ((self.dram[index + 5] as u64) << 40)
            | ((self.dram[index + 6] as u64) << 48)
            | ((self.dram[index + 7] as u64) << 56);
    }
}
