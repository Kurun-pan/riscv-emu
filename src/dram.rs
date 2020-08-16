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

    pub fn initialize(&mut self, data: Vec<u8>) {
        self.code_size = data.len() as u64;
        self.dram.splice(..data.len(), data.iter().cloned());
    }

    pub fn write8(&mut self, addr: u64, data: u8) {
        self.dram[addr as usize] = data;
    }

    pub fn write16(&mut self, addr: u64, data: u16) {
        let index = addr as usize;
        for i in 0..2 {
            self.dram[index + i] = ((data >> (i * 8)) & 0xff) as u8;
        }
    }

    pub fn write32(&mut self, addr: u64, data: u32) {
        let index = addr as usize;
        for i in 0..4 {
            self.dram[index + i] = ((data >> (i * 8)) & 0xff) as u8;
        }
    }

    pub fn write64(&mut self, addr: u64, data: u64) {
        let index = addr as usize;
        for i in 0..8 {
            self.dram[index + i] = ((data >> (i * 8)) & 0xff) as u8;
        }
    }

    pub fn read8(&self, addr: u64) -> u8 {
        let index = addr as usize;
        self.dram[index]
    }

    pub fn read16(&self, addr: u64) -> u16 {
        let index = addr as usize;
        let mut data = 0 as u16;
        for i in 0..2 {
            data |= (self.dram[index + i] as u16) << (i * 8);
        }
        data
    }

    pub fn read32(&self, addr: u64) -> u32 {
        let index = addr as usize;
        let mut data = 0 as u32;
        for i in 0..4 {
            data |= (self.dram[index + i] as u32) << (i * 8);
        }
        data
    }

    pub fn read64(&self, addr: u64) -> u64 {
        let index = addr as usize;
        let mut data = 0 as u64;
        for i in 0..8 {
            data |= (self.dram[index + i] as u64) << (i * 8);
        }
        data
    }
}
