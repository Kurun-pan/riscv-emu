use crate::dram::Dram;
use crate::peripherals::timer::Timer;

pub const TIMER_ADDRESS_START: u64 = 0x0200_0000;
pub const TIMER_ADDRESS_END: u64 = 0x0200_FFFF;

pub const DRAM_ADDRESS_START: u64 = 0x8000_0000;

pub struct SystemBus {
    clock: u128,
    pub dram: Dram,
    pub timer: Timer,
}

impl SystemBus {
    pub fn new() -> Self {
        Self {
            clock: 0,
            dram: Dram::new(),
            timer: Timer::new(),
        }
    }

    pub fn tick(&mut self) {
        self.clock = self.clock.wrapping_add(1);
        if self.clock & 0xf == 0 {
            // TODO: care 1MHz clock (RTCCLK).
            self.timer.tick();
        }
    }

    pub fn read8(&mut self, addr: u64) -> Result<u8, ()> {
        if DRAM_ADDRESS_START <= addr {
            return Ok(self.dram.read8(addr - DRAM_ADDRESS_START));
        }
        match addr {
            TIMER_ADDRESS_START..=TIMER_ADDRESS_END => panic!("Unexpected size access."),
            _ => Err(()),
        }
    }

    pub fn read16(&mut self, addr: u64) -> Result<u16, ()> {
        if DRAM_ADDRESS_START <= addr {
            return Ok(self.dram.read16(addr - DRAM_ADDRESS_START));
        }
        match addr {
            TIMER_ADDRESS_START..=TIMER_ADDRESS_END => panic!("Unexpected size access."),
            _ => Err(()),
        }
    }

    pub fn read32(&mut self, addr: u64) -> Result<u32, ()> {
        if DRAM_ADDRESS_START <= addr {
            return Ok(self.dram.read32(addr - DRAM_ADDRESS_START));
        }
        match addr {
            TIMER_ADDRESS_START..=TIMER_ADDRESS_END => {
                Ok(self.timer.read(addr - TIMER_ADDRESS_START))
            }
            _ => Err(()),
        }
    }

    pub fn read64(&mut self, addr: u64) -> Result<u64, ()> {
        if DRAM_ADDRESS_START <= addr {
            return Ok(self.dram.read64(addr - DRAM_ADDRESS_START));
        }
        match addr {
            TIMER_ADDRESS_START..=TIMER_ADDRESS_END => {
                let timer_addr = addr - TIMER_ADDRESS_START;
                let data = self.timer.read(timer_addr) as u64
                    | ((self.timer.read(timer_addr.wrapping_add(4)) as u64) << 32);
                Ok(data)
            }
            _ => Err(()),
        }
    }

    pub fn write8(&mut self, addr: u64, val: u8) -> Result<(), ()> {
        if DRAM_ADDRESS_START <= addr {
            return Ok(self.dram.write8(addr - DRAM_ADDRESS_START, val));
        }
        match addr {
            TIMER_ADDRESS_START..=TIMER_ADDRESS_END => panic!("Unexpected size access."),
            _ => Err(()),
        }
    }

    pub fn write16(&mut self, addr: u64, val: u16) -> Result<(), ()> {
        if DRAM_ADDRESS_START <= addr {
            return Ok(self.dram.write16(addr - DRAM_ADDRESS_START, val));
        }
        match addr {
            TIMER_ADDRESS_START..=TIMER_ADDRESS_END => panic!("Unexpected size access."),
            _ => Err(()),
        }
    }

    pub fn write32(&mut self, addr: u64, val: u32) -> Result<(), ()> {
        if DRAM_ADDRESS_START <= addr {
            return Ok(self.dram.write32(addr - DRAM_ADDRESS_START, val));
        }
        match addr {
            TIMER_ADDRESS_START..=TIMER_ADDRESS_END => {
                Ok(self.timer.write(addr - TIMER_ADDRESS_START, val))
            }
            _ => Err(()),
        }
    }

    pub fn write64(&mut self, addr: u64, val: u64) -> Result<(), ()> {
        if DRAM_ADDRESS_START <= addr {
            return Ok(self.dram.write64(addr - DRAM_ADDRESS_START, val));
        }
        match addr {
            TIMER_ADDRESS_START..=TIMER_ADDRESS_END => {
                let timer_addr = addr - TIMER_ADDRESS_START;
                self.timer.write(timer_addr, val as u32);
                self.timer
                    .write(timer_addr.wrapping_add(4), (val >> 32 & 0xffff) as u32);
                Ok(())
            }
            _ => Err(()),
        }
    }
}
