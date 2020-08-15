use crate::tty::Tty;
use crate::dram::Dram;
use crate::peripherals::timer::Timer;
use crate::peripherals::uart::Uart;
use crate::peripherals::intc::Intc;

pub const TIMER_ADDRESS_START: u64 = 0x0200_0000;
pub const TIMER_ADDRESS_END:   u64 = 0x0200_FFFF;

pub const INTC_ADDRESS_START:  u64 = 0x0C00_0000;
pub const INTC_ADDRESS_END:    u64 = 0x0FFF_FFFF;

pub const UART_ADDRESS_START:  u64 = 0x1000_0000;
pub const UART_ADDRESS_END:    u64 = 0x1000_FFFF;

pub const DRAM_ADDRESS_START:  u64 = 0x8000_0000;

// Physical memory layout
// -------------------------------------------------
// 00001000 -- boot ROM, provided by qemu
// 02000000 -- CLINT
// 0C000000 -- PLIC
// 10000000 -- uart0 
// 10001000 -- virtio disk 
// 80000000 -- boot ROM jumps here in machine mode
//             -kernel loads the kernel here
// unused RAM after 80000000.
// -------------------------------------------------

pub struct SystemBus {
    clock: u64,
    pub dram: Dram,
    timer: Timer,
    intc: Intc,
    uart: Uart,
}

impl SystemBus {
    pub fn new(tty: Box<dyn Tty>) -> Self {
        Self {
            clock: 0,
            dram: Dram::new(),
            timer: Timer::new(),
            intc: Intc::new(),
            uart: Uart::new(tty),
        }
    }

    pub fn tick(&mut self) -> Vec<bool> {
        self.clock = self.clock.wrapping_add(1);

        // TODO: care 1MHz clock (RTCCLK).
        if self.clock & 0xf == 0 {
            self.timer.tick();
        }

        // TODO: care ???Hz clock
        let mut interrupt_uart = false;
        if self.clock & 0xf == 0 {
            self.uart.tick();
            interrupt_uart = self.uart.is_irq();
        }

        let mut interrupts: Vec<usize> = Vec::new();
        if interrupt_uart {
             interrupts.push(10); // Interrupt ID for UART0
        }
        self.intc.tick(0, interrupts)
    }

    pub fn read8(&mut self, addr: u64) -> Result<u8, ()> {
        if DRAM_ADDRESS_START <= addr {
            return Ok(self.dram.read8(addr - DRAM_ADDRESS_START));
        }
        match addr {
            TIMER_ADDRESS_START..=TIMER_ADDRESS_END => panic!("Unexpected size access."),
            INTC_ADDRESS_START..=INTC_ADDRESS_START => panic!("Unexpected size access."),
            UART_ADDRESS_START..=UART_ADDRESS_END => Ok(self.uart.read(addr - UART_ADDRESS_START)),
            _ => Err(()),
        }
    }

    pub fn read16(&mut self, addr: u64) -> Result<u16, ()> {
        if DRAM_ADDRESS_START <= addr {
            return Ok(self.dram.read16(addr - DRAM_ADDRESS_START));
        }
        match addr {
            TIMER_ADDRESS_START..=TIMER_ADDRESS_END => panic!("Unexpected size access."),
            INTC_ADDRESS_START..=INTC_ADDRESS_START => panic!("Unexpected size access."),
            UART_ADDRESS_START..=UART_ADDRESS_END => {
                let addr_ = addr - UART_ADDRESS_START;
                let data = self.uart.read(addr_) as u16
                    | ((self.uart.read(addr_.wrapping_add(1)) as u16) << 8);
                Ok(data)
            }
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
            INTC_ADDRESS_START..=INTC_ADDRESS_START => {
                Ok(self.intc.read(addr - INTC_ADDRESS_START))
            }
            UART_ADDRESS_START..=UART_ADDRESS_END => {
                let addr_ = addr - UART_ADDRESS_START;
                let data = self.uart.read(addr_) as u32
                    | ((self.uart.read(addr_.wrapping_add(1)) as u32) << 8)
                    | ((self.uart.read(addr_.wrapping_add(2)) as u32) << 16)
                    | ((self.uart.read(addr_.wrapping_add(3)) as u32) << 24);
                    Ok(data)
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
            INTC_ADDRESS_START..=INTC_ADDRESS_END => {
                let intc_addr = addr - INTC_ADDRESS_START;
                let data = self.intc.read(intc_addr) as u64
                    | ((self.intc.read(intc_addr.wrapping_add(4)) as u64) << 32);
                Ok(data)
            }
            UART_ADDRESS_START..=UART_ADDRESS_END => {
                let addr_ = addr - UART_ADDRESS_START;
                let data = self.uart.read(addr_) as u64
                    | ((self.uart.read(addr_.wrapping_add(1)) as u64) << 8)
                    | ((self.uart.read(addr_.wrapping_add(2)) as u64) << 16)
                    | ((self.uart.read(addr_.wrapping_add(3)) as u64) << 24)
                    | ((self.uart.read(addr_.wrapping_add(4)) as u64) << 32)
                    | ((self.uart.read(addr_.wrapping_add(5)) as u64) << 40)
                    | ((self.uart.read(addr_.wrapping_add(6)) as u64) << 48)
                    | ((self.uart.read(addr_.wrapping_add(7)) as u64) << 56);
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
            INTC_ADDRESS_START..=INTC_ADDRESS_END => panic!("Unexpected size access."),
            UART_ADDRESS_START..=UART_ADDRESS_END => Ok(self.uart.write(addr - UART_ADDRESS_START, val)),
            _ => Err(()),
        }
    }

    pub fn write16(&mut self, addr: u64, val: u16) -> Result<(), ()> {
        if DRAM_ADDRESS_START <= addr {
            return Ok(self.dram.write16(addr - DRAM_ADDRESS_START, val));
        }
        match addr {
            TIMER_ADDRESS_START..=TIMER_ADDRESS_END => panic!("Unexpected size access."),
            INTC_ADDRESS_START..=INTC_ADDRESS_END => panic!("Unexpected size access."),
            UART_ADDRESS_START..=UART_ADDRESS_END => {
                let addr_ = addr - UART_ADDRESS_START;
                self.uart.write(addr_, (val & 0xff) as u8);
                self.uart.write(addr_.wrapping_add(1), ((val >> 8) & 0xff) as u8);
                Ok(())
            }
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
            INTC_ADDRESS_START..=INTC_ADDRESS_END => {
                Ok(self.intc.write(addr - INTC_ADDRESS_START, val))
            }
            UART_ADDRESS_START..=UART_ADDRESS_END => {
                let addr_ = addr - UART_ADDRESS_START;
                self.uart.write(addr_, (val & 0xff) as u8);
                self.uart.write(addr_.wrapping_add(1), ((val >> 8) & 0xff) as u8);
                self.uart.write(addr_.wrapping_add(2), ((val >> 16) & 0xff) as u8);
                self.uart.write(addr_.wrapping_add(3), ((val >> 24) & 0xff) as u8);
                Ok(())
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
            INTC_ADDRESS_START..=INTC_ADDRESS_END => {
                let intc_addr = addr - INTC_ADDRESS_START;
                self.intc.write(intc_addr, val as u32);
                self.intc
                    .write(intc_addr.wrapping_add(4), (val >> 32 & 0xffff) as u32);
                Ok(())
            }
            UART_ADDRESS_START..=UART_ADDRESS_END => {
                let addr_ = addr - UART_ADDRESS_START;
                self.uart.write(addr_, (val & 0xff) as u8);
                self.uart.write(addr_.wrapping_add(1), ((val >> 8) & 0xff) as u8);
                self.uart.write(addr_.wrapping_add(2), ((val >> 16) & 0xff) as u8);
                self.uart.write(addr_.wrapping_add(3), ((val >> 24) & 0xff) as u8);
                self.uart.write(addr_.wrapping_add(4), ((val >> 32) & 0xff) as u8);
                self.uart.write(addr_.wrapping_add(5), ((val >> 40) & 0xff) as u8);                
                self.uart.write(addr_.wrapping_add(6), ((val >> 48) & 0xff) as u8);
                self.uart.write(addr_.wrapping_add(7), ((val >> 56) & 0xff) as u8);
                Ok(())
            }
            _ => Err(()),
        }
    }
}
