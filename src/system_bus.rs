use crate::memory::Memory;
use crate::peripherals::fe310_g002::prci::Prci;
use crate::peripherals::fe310_g002::gpio::Gpio;
use crate::peripherals::fu540_c000::clint::Clint;
use crate::peripherals::fu540_c000::plic::Plic;
use crate::peripherals::intc::Intc;
use crate::peripherals::timer::Timer;
use crate::peripherals::uart::Uart;
use crate::peripherals::virtio::Virtio;
use crate::tty::Tty;

// --------------------------------------------------------
// for xv6
// --------------------------------------------------------
pub const TIMER_ADDRESS_START: u64 = 0x0200_0000;
pub const TIMER_ADDRESS_END: u64 = 0x0200_FFFF;

pub const INTC_ADDRESS_START: u64 = 0x0C00_0000;
pub const INTC_ADDRESS_END: u64 = 0x0FFF_FFFF;

pub const UART_ADDRESS_START: u64 = 0x1000_0000;
pub const UART_ADDRESS_END: u64 = 0x1000_0FFF;

pub const VIRTIO_ADDRESS_START: u64 = 0x1000_1000;
pub const VIRTIO_ADDRESS_END: u64 = 0x1000_1FFF;

pub const DRAM_ADDRESS_START: u64 = 0x8000_0000;

// Physical memory layout for xv6-riscv
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

// --------------------------------------------------------
// for nuttx
// --------------------------------------------------------
// https://bitbucket.org/nuttx/nuttx/src/master/arch/risc-v/src/fe310/hardware/fe310_memorymap.h
pub const PRCI_ADDRESS_START: u64 = 0x1000_8000;
pub const PRCI_ADDRESS_END: u64 = 0x1000_8FFF;

pub const GPIO_ADDRESS_START: u64 = 0x1001_2000;
pub const GPIO_ADDRESS_END: u64 = 0x1001_2FFF;

pub const SPIFLASH_ADDRESS_START: u64 = 0x2000_0000;
pub const SPIFLASH_ADDRESS_END: u64 = 0x3FFF_FFFF;

pub const MAX_DRAM_SIZE: usize = 1024 * 1024 * 128;
pub const MAX_FLASH_SIZE: usize = 1024 * 1024 * 512;

pub struct SystemBus {
    clock: u64,
    pub dram: Memory,
    flash: Memory,
    timer: Box<dyn Timer>,
    intc: Box<dyn Intc>,
    uart: Uart,
    virtio: Virtio,
    prci: Prci,
    gpio: Gpio,
}

impl SystemBus {
    pub fn new(tty: Box<dyn Tty>) -> Self {
        Self {
            clock: 0,
            dram: Memory::new(MAX_DRAM_SIZE),
            flash: Memory::new(MAX_FLASH_SIZE),
            timer: Box::new(Clint::new()),
            intc: Box::new(Plic::new()),
            uart: Uart::new(tty),
            virtio: Virtio::new(),
            prci: Prci::new(),
            gpio: Gpio::new(),
        }
    }

    pub fn set_disk_data(&mut self, data: Vec<u8>) {
        self.virtio.init(data);
    }

    pub fn tick(&mut self) -> Vec<bool> {
        self.clock = self.clock.wrapping_add(1);

        self.virtio.tick(&mut self.dram);
        self.timer.tick();
        self.uart.tick();
        self.prci.tick();
        self.gpio.tick();

        // https://github.com/mit-pdos/xv6-riscv/blob/riscv/kernel/memlayout.h
        let mut interrupts: Vec<usize> = Vec::new();
        if self.uart.is_irq() {
            interrupts.push(10); // Interrupt ID for UART0
        }
        if self.virtio.is_irq() {
            interrupts.push(1); // Interrupt ID for Virtio
        }
        self.intc.tick(0, interrupts)
    }

    pub fn is_pending_software_interrupt(&mut self, core: usize) -> bool {
        self.timer.is_pending_software_interrupt(core)
    }

    pub fn is_pending_timer_interrupt(&mut self, core: usize) -> bool {
        self.timer.is_pending_timer_interrupt(core)
    }

    pub fn read8(&mut self, addr: u64) -> Result<u8, ()> {
        if DRAM_ADDRESS_START <= addr {
            return Ok(self.dram.read8(addr - DRAM_ADDRESS_START));
        }
        match addr {
            TIMER_ADDRESS_START..=TIMER_ADDRESS_END => panic!("Unexpected size access."),
            INTC_ADDRESS_START..=INTC_ADDRESS_END => panic!("Unexpected size access."),
            UART_ADDRESS_START..=UART_ADDRESS_END => Ok(self.uart.read(addr - UART_ADDRESS_START)),
            VIRTIO_ADDRESS_START..=VIRTIO_ADDRESS_END => panic!("Unexpected size access."),
            PRCI_ADDRESS_START..=PRCI_ADDRESS_END => panic!("Unexpected size access."),
            GPIO_ADDRESS_START..=GPIO_ADDRESS_END => panic!("Unexpected size access."),
            SPIFLASH_ADDRESS_START..=SPIFLASH_ADDRESS_END => {
                Ok(self.flash.read8(addr - SPIFLASH_ADDRESS_START))
            }
            _ => Err(()),
        }
    }

    pub fn read16(&mut self, addr: u64) -> Result<u16, ()> {
        if DRAM_ADDRESS_START <= addr {
            return Ok(self.dram.read16(addr - DRAM_ADDRESS_START));
        }
        match addr {
            TIMER_ADDRESS_START..=TIMER_ADDRESS_END => panic!("Unexpected size access."),
            INTC_ADDRESS_START..=INTC_ADDRESS_END => panic!("Unexpected size access."),
            UART_ADDRESS_START..=UART_ADDRESS_END => {
                let addr_ = addr - UART_ADDRESS_START;
                let data = self.uart.read(addr_) as u16
                    | ((self.uart.read(addr_.wrapping_add(1)) as u16) << 8);
                Ok(data)
            }
            VIRTIO_ADDRESS_START..=VIRTIO_ADDRESS_END => panic!("Unexpected size access."),
            PRCI_ADDRESS_START..=PRCI_ADDRESS_END => panic!("Unexpected size access."),
            GPIO_ADDRESS_START..=GPIO_ADDRESS_END => panic!("Unexpected size access."),
            SPIFLASH_ADDRESS_START..=SPIFLASH_ADDRESS_END => {
                Ok(self.flash.read16(addr - SPIFLASH_ADDRESS_START))
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
            INTC_ADDRESS_START..=INTC_ADDRESS_END => Ok(self.intc.read(addr - INTC_ADDRESS_START)),
            UART_ADDRESS_START..=UART_ADDRESS_END => {
                let addr_ = addr - UART_ADDRESS_START;
                let data = self.uart.read(addr_) as u32
                    | ((self.uart.read(addr_.wrapping_add(1)) as u32) << 8)
                    | ((self.uart.read(addr_.wrapping_add(2)) as u32) << 16)
                    | ((self.uart.read(addr_.wrapping_add(3)) as u32) << 24);
                Ok(data)
            }
            VIRTIO_ADDRESS_START..=VIRTIO_ADDRESS_END => {
                Ok(self.virtio.read(addr - VIRTIO_ADDRESS_START))
            }
            PRCI_ADDRESS_START..=PRCI_ADDRESS_END => Ok(self.prci.read(addr - PRCI_ADDRESS_START)),
            GPIO_ADDRESS_START..=GPIO_ADDRESS_END => Ok(self.gpio.read(addr - GPIO_ADDRESS_START)),
            SPIFLASH_ADDRESS_START..=SPIFLASH_ADDRESS_END => {
                Ok(self.flash.read32(addr - SPIFLASH_ADDRESS_START))
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
            VIRTIO_ADDRESS_START..=VIRTIO_ADDRESS_END => {
                let virtio_addr = addr - VIRTIO_ADDRESS_START;
                let data = self.virtio.read(virtio_addr) as u64
                    | ((self.virtio.read(virtio_addr.wrapping_add(4)) as u64) << 32);
                Ok(data)
            }
            PRCI_ADDRESS_START..=PRCI_ADDRESS_END => {
                let prci_addr = addr - PRCI_ADDRESS_START;
                let data = self.prci.read(prci_addr) as u64
                    | ((self.prci.read(prci_addr.wrapping_add(4)) as u64) << 32);
                Ok(data)
            }
            GPIO_ADDRESS_START..=GPIO_ADDRESS_END => {
                let gpio_addr = addr - GPIO_ADDRESS_START;
                let data = self.gpio.read(gpio_addr) as u64
                    | ((self.gpio.read(gpio_addr.wrapping_add(4)) as u64) << 32);
                Ok(data)
            }
            SPIFLASH_ADDRESS_START..=SPIFLASH_ADDRESS_END => {
                Ok(self.flash.read64(addr - SPIFLASH_ADDRESS_START))
            }
            _ => Err(()),
        }
    }

    pub fn write8(&mut self, addr: u64, data: u8) -> Result<(), ()> {
        if DRAM_ADDRESS_START <= addr {
            return Ok(self.dram.write8(addr - DRAM_ADDRESS_START, data));
        }
        match addr {
            TIMER_ADDRESS_START..=TIMER_ADDRESS_END => panic!("Unexpected size access."),
            INTC_ADDRESS_START..=INTC_ADDRESS_END => panic!("Unexpected size access."),
            UART_ADDRESS_START..=UART_ADDRESS_END => {
                Ok(self.uart.write(addr - UART_ADDRESS_START, data))
            }
            VIRTIO_ADDRESS_START..=VIRTIO_ADDRESS_END => panic!("Unexpected size access."),
            PRCI_ADDRESS_START..=PRCI_ADDRESS_END => panic!("Unexpected size access."),
            GPIO_ADDRESS_START..=GPIO_ADDRESS_END => panic!("Unexpected size access."),
            SPIFLASH_ADDRESS_START..=SPIFLASH_ADDRESS_END => {
                Ok(self.flash.write8(addr - SPIFLASH_ADDRESS_START, data))
            }
            _ => Err(()),
        }
    }

    pub fn write16(&mut self, addr: u64, data: u16) -> Result<(), ()> {
        if DRAM_ADDRESS_START <= addr {
            return Ok(self.dram.write16(addr - DRAM_ADDRESS_START, data));
        }
        match addr {
            TIMER_ADDRESS_START..=TIMER_ADDRESS_END => panic!("Unexpected size access."),
            INTC_ADDRESS_START..=INTC_ADDRESS_END => panic!("Unexpected size access."),
            UART_ADDRESS_START..=UART_ADDRESS_END => {
                let addr_ = addr - UART_ADDRESS_START;
                self.uart.write(addr_, (data & 0xff) as u8);
                self.uart
                    .write(addr_.wrapping_add(1), ((data >> 8) & 0xff) as u8);
                Ok(())
            }
            VIRTIO_ADDRESS_START..=VIRTIO_ADDRESS_END => panic!("Unexpected size access."),
            PRCI_ADDRESS_START..=PRCI_ADDRESS_END => panic!("Unexpected size access."),
            GPIO_ADDRESS_START..=GPIO_ADDRESS_END => panic!("Unexpected size access."),
            SPIFLASH_ADDRESS_START..=SPIFLASH_ADDRESS_END => {
                Ok(self.flash.write16(addr - SPIFLASH_ADDRESS_START, data))
            }
            _ => Err(()),
        }
    }

    pub fn write32(&mut self, addr: u64, data: u32) -> Result<(), ()> {
        if DRAM_ADDRESS_START <= addr {
            return Ok(self.dram.write32(addr - DRAM_ADDRESS_START, data));
        }
        match addr {
            TIMER_ADDRESS_START..=TIMER_ADDRESS_END => {
                Ok(self.timer.write(addr - TIMER_ADDRESS_START, data))
            }
            INTC_ADDRESS_START..=INTC_ADDRESS_END => {
                Ok(self.intc.write(addr - INTC_ADDRESS_START, data))
            }
            UART_ADDRESS_START..=UART_ADDRESS_END => {
                let addr_ = addr - UART_ADDRESS_START;
                self.uart.write(addr_, (data & 0xff) as u8);
                self.uart
                    .write(addr_.wrapping_add(1), ((data >> 8) & 0xff) as u8);
                self.uart
                    .write(addr_.wrapping_add(2), ((data >> 16) & 0xff) as u8);
                self.uart
                    .write(addr_.wrapping_add(3), ((data >> 24) & 0xff) as u8);
                Ok(())
            }
            VIRTIO_ADDRESS_START..=VIRTIO_ADDRESS_END => {
                Ok(self.virtio.write(addr - VIRTIO_ADDRESS_START, data))
            }
            PRCI_ADDRESS_START..=PRCI_ADDRESS_END => {
                Ok(self.prci.write(addr - PRCI_ADDRESS_START, data))
            }
            GPIO_ADDRESS_START..=GPIO_ADDRESS_END => {
                Ok(self.gpio.write(addr - GPIO_ADDRESS_START, data))
            }
            SPIFLASH_ADDRESS_START..=SPIFLASH_ADDRESS_END => {
                Ok(self.flash.write32(addr - SPIFLASH_ADDRESS_START, data))
            }
            _ => Err(()),
        }
    }

    pub fn write64(&mut self, addr: u64, data: u64) -> Result<(), ()> {
        if DRAM_ADDRESS_START <= addr {
            return Ok(self.dram.write64(addr - DRAM_ADDRESS_START, data));
        }
        match addr {
            TIMER_ADDRESS_START..=TIMER_ADDRESS_END => {
                let timer_addr = addr - TIMER_ADDRESS_START;
                self.timer.write(timer_addr, data as u32);
                self.timer
                    .write(timer_addr.wrapping_add(4), (data >> 32 & 0xffff) as u32);
                Ok(())
            }
            INTC_ADDRESS_START..=INTC_ADDRESS_END => {
                let intc_addr = addr - INTC_ADDRESS_START;
                self.intc.write(intc_addr, data as u32);
                self.intc
                    .write(intc_addr.wrapping_add(4), (data >> 32 & 0xffff) as u32);
                Ok(())
            }
            UART_ADDRESS_START..=UART_ADDRESS_END => {
                let addr_ = addr - UART_ADDRESS_START;
                self.uart.write(addr_, (data & 0xff) as u8);
                self.uart
                    .write(addr_.wrapping_add(1), ((data >> 8) & 0xff) as u8);
                self.uart
                    .write(addr_.wrapping_add(2), ((data >> 16) & 0xff) as u8);
                self.uart
                    .write(addr_.wrapping_add(3), ((data >> 24) & 0xff) as u8);
                self.uart
                    .write(addr_.wrapping_add(4), ((data >> 32) & 0xff) as u8);
                self.uart
                    .write(addr_.wrapping_add(5), ((data >> 40) & 0xff) as u8);
                self.uart
                    .write(addr_.wrapping_add(6), ((data >> 48) & 0xff) as u8);
                self.uart
                    .write(addr_.wrapping_add(7), ((data >> 56) & 0xff) as u8);
                Ok(())
            }
            VIRTIO_ADDRESS_START..=VIRTIO_ADDRESS_END => {
                let virtio_addr = addr - VIRTIO_ADDRESS_START;
                self.virtio.write(virtio_addr, data as u32);
                self.virtio
                    .write(virtio_addr.wrapping_add(4), (data >> 32 & 0xffff) as u32);
                Ok(())
            }
            PRCI_ADDRESS_START..=PRCI_ADDRESS_END => {
                let prci_addr = addr - PRCI_ADDRESS_START;
                self.prci.write(prci_addr, data as u32);
                self.prci
                    .write(prci_addr.wrapping_add(4), (data >> 32 & 0xffff) as u32);
                Ok(())
            }
            GPIO_ADDRESS_START..=GPIO_ADDRESS_END => {
                let gpio_addr = addr - GPIO_ADDRESS_START;
                self.gpio.write(gpio_addr, data as u32);
                self.gpio
                    .write(gpio_addr.wrapping_add(4), (data >> 32 & 0xffff) as u32);
                Ok(())
            }
            SPIFLASH_ADDRESS_START..=SPIFLASH_ADDRESS_END => {
                Ok(self.flash.write64(addr - SPIFLASH_ADDRESS_START, data))
            }
            _ => Err(()),
        }
    }
}
