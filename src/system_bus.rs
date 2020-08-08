use crate::cpu::trap::{Trap, Traps};
use crate::dram::Dram;

pub const DRAM_BASE_ADDRESS: u64 = 0x8000_0000;

pub struct SystemBus {
    pub dram: Dram,
}

impl SystemBus {
    pub fn new() -> Self {
        Self {
            dram: Dram::new(),
        }
    }

    pub fn read8(&mut self, addr: u64) -> Result<u8, Trap> {
        if DRAM_BASE_ADDRESS <= addr {
            return Ok(self.dram.read8(addr - DRAM_BASE_ADDRESS));
        }
        Err(Trap {
            factor: Traps::InstructionAccessFault,
            value: addr
        })
    }

    pub fn read16(&self, addr: u64) -> Result<u16, Trap> {
        if DRAM_BASE_ADDRESS <= addr {
            return Ok(self.dram.read16(addr - DRAM_BASE_ADDRESS));
        }
        Err(Trap {
            factor: Traps::InstructionAccessFault,
            value: addr
        })
    }

    pub fn read32(&self, addr: u64) -> Result<u32, Trap> {
        if DRAM_BASE_ADDRESS <= addr {
            return Ok(self.dram.read32(addr - DRAM_BASE_ADDRESS));
        }
        Err(Trap {
            factor: Traps::InstructionAccessFault,
            value: addr
        })
    }

    pub fn read64(&self, addr: u64) -> Result<u64, Trap> {
        if DRAM_BASE_ADDRESS <= addr {
            return Ok(self.dram.read64(addr - DRAM_BASE_ADDRESS));
        }
        Err(Trap {
            factor: Traps::InstructionAccessFault,
            value: addr
        })
    }

    pub fn write8(&mut self, addr: u64, val: u8) -> Result<(), Trap> {
        if DRAM_BASE_ADDRESS <= addr {
            return Ok(self.dram.write8(addr - DRAM_BASE_ADDRESS, val));
        }
        Err(Trap {
            factor: Traps::InstructionAccessFault,
            value: addr
        })
    }

    pub fn write16(&mut self, addr: u64, val: u16) -> Result<(), Trap> {
        if DRAM_BASE_ADDRESS <= addr {
            return Ok(self.dram.write16(addr - DRAM_BASE_ADDRESS, val));
        }
        Err(Trap {
            factor: Traps::InstructionAccessFault,
            value: addr
        })
    }

    pub fn write32(&mut self, addr: u64, val: u32) -> Result<(), Trap> {
        if DRAM_BASE_ADDRESS <= addr {
            return Ok(self.dram.write32(addr - DRAM_BASE_ADDRESS, val));
        }
        Err(Trap {
            factor: Traps::InstructionAccessFault,
            value: addr
        })
    }

    pub fn write64(&mut self, addr: u64, val: u64) -> Result<(), Trap> {
        if DRAM_BASE_ADDRESS <= addr {
            return Ok(self.dram.write64(addr - DRAM_BASE_ADDRESS, val));
        }
        Err(Trap {
            factor: Traps::InstructionAccessFault,
            value: addr
        })
    }
}
