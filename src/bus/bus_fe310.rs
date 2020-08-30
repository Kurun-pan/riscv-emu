// FE310 SoC for NuttX

use crate::bus::bus::*;
use crate::memory::Memory;
use crate::peripherals::fe310_g002::fe310_uart::Fe310Uart;
use crate::peripherals::fe310_g002::gpio::Gpio;
use crate::peripherals::fe310_g002::prci::Prci;
use crate::peripherals::fu540_c000::clint::Clint;
use crate::peripherals::fu540_c000::plic::Plic;
use crate::peripherals::intc::Intc;
use crate::peripherals::timer::Timer;
use crate::console::*;

// TODO: implement Debug
const _DEBUG_ADDRESS_START: u64 = 0x0000_0000;
const _DEBUG_ADDRESS_END: u64 = 0x0000_0FFF;

const TIMER_ADDRESS_START: u64 = 0x0200_0000;
const TIMER_ADDRESS_END: u64 = 0x0200_FFFF;

const INTC_ADDRESS_START: u64 = 0x0C00_0000;
const INTC_ADDRESS_END: u64 = 0x0FFF_FFFF;

const PRCI_ADDRESS_START: u64 = 0x1000_8000;
const PRCI_ADDRESS_END: u64 = 0x1000_8FFF;

const UART0_ADDRESS_START: u64 = 0x1001_3000;
const UART0_ADDRESS_END: u64 = 0x1001_3FFF;

const GPIO_ADDRESS_START: u64 = 0x1001_2000;
const GPIO_ADDRESS_END: u64 = 0x1001_2FFF;

const UART1_ADDRESS_START: u64 = 0x1002_3000;
const UART1_ADDRESS_END: u64 = 0x1002_3FFF;

const SPIFLASH_ADDRESS_START: u64 = 0x2000_0000;
const SPIFLASH_ADDRESS_END: u64 = 0x3FFF_FFFF;

// SRAM for .bss
const DTIM_ADDRESS_START: u64 = 0x8000_0000;
const DTIM_ADDRESS_END: u64 = 0x8000_3FFF;

const DTIM_SIZE: usize = 0x4000;
const FLASH_SIZE: usize = 1024 * 1024 * 512;

pub struct BusFe310 {
    clock: u64,
    dtim: Memory,
    flash: Memory,
    timer: Box<dyn Timer>,
    intc: Box<dyn Intc>,
    prci: Prci,
    uart0: Fe310Uart,
    uart1: Fe310Uart,
    gpio: Gpio,
}

impl BusFe310 {
    pub fn new(console: Box<dyn Console>) -> Self {
        Self {
            clock: 0,
            dtim: Memory::new(DTIM_SIZE),
            flash: Memory::new(FLASH_SIZE),
            timer: Box::new(Clint::new()),
            intc: Box::new(Plic::new()),
            uart0: Fe310Uart::new(console),
            uart1: Fe310Uart::new(Box::new(TtyDummy::new())),
            prci: Prci::new(),
            gpio: Gpio::new(),
        }
    }
}

impl Bus for BusFe310 {
    fn set_device_data(&mut self, device: Device, data: Vec<u8>) {
        match device {
            Device::SpiFlash => {
                self.flash.initialize(data);
            }
            _ => panic!("Unexpected device: {:?}", device),
        }
    }

    fn tick(&mut self) -> Vec<bool> {
        self.clock = self.clock.wrapping_add(1);

        self.timer.tick();
        self.prci.tick();
        self.gpio.tick();
        self.uart0.tick();
        self.uart1.tick();

        let mut interrupts: Vec<usize> = Vec::new();
        if self.uart0.is_irq() {
            interrupts.push(3); // Interrupt ID for UART0
        }
        if self.uart1.is_irq() {
            interrupts.push(4); // Interrupt ID for UART1
        }
        self.intc.tick(0, interrupts)
    }

    fn is_pending_software_interrupt(&mut self, core: usize) -> bool {
        self.timer.is_pending_software_interrupt(core)
    }

    fn is_pending_timer_interrupt(&mut self, core: usize) -> bool {
        self.timer.is_pending_timer_interrupt(core)
    }

    fn get_base_address(&mut self, device: Device) -> u64 {
        match device {
            Device::SpiFlash => SPIFLASH_ADDRESS_START,
            _ => panic!("Unexpected device: {:?}", device),
        }
    }

    fn read8(&mut self, addr: u64) -> Result<u8, ()> {
        match addr {
            TIMER_ADDRESS_START..=TIMER_ADDRESS_END => panic!("Unexpected size access."),
            INTC_ADDRESS_START..=INTC_ADDRESS_END => panic!("Unexpected size access."),
            PRCI_ADDRESS_START..=PRCI_ADDRESS_END => panic!("Unexpected size access."),
            GPIO_ADDRESS_START..=GPIO_ADDRESS_END => panic!("Unexpected size access."),
            UART0_ADDRESS_START..=UART0_ADDRESS_END => panic!("Unexpected size access."),
            UART1_ADDRESS_START..=UART1_ADDRESS_END => panic!("Unexpected size access."),
            SPIFLASH_ADDRESS_START..=SPIFLASH_ADDRESS_END => {
                Ok(self.flash.read8(addr - SPIFLASH_ADDRESS_START))
            }
            DTIM_ADDRESS_START..=DTIM_ADDRESS_END => Ok(self.dtim.read8(addr - DTIM_ADDRESS_START)),
            _ => Err(()),
        }
    }

    fn read16(&mut self, addr: u64) -> Result<u16, ()> {
        match addr {
            TIMER_ADDRESS_START..=TIMER_ADDRESS_END => panic!("Unexpected size access."),
            INTC_ADDRESS_START..=INTC_ADDRESS_END => panic!("Unexpected size access."),
            PRCI_ADDRESS_START..=PRCI_ADDRESS_END => panic!("Unexpected size access."),
            GPIO_ADDRESS_START..=GPIO_ADDRESS_END => panic!("Unexpected size access."),
            UART0_ADDRESS_START..=UART0_ADDRESS_END => panic!("Unexpected size access."),
            UART1_ADDRESS_START..=UART1_ADDRESS_END => panic!("Unexpected size access."),
            SPIFLASH_ADDRESS_START..=SPIFLASH_ADDRESS_END => {
                Ok(self.flash.read16(addr - SPIFLASH_ADDRESS_START))
            }
            DTIM_ADDRESS_START..=DTIM_ADDRESS_END => {
                Ok(self.dtim.read16(addr - DTIM_ADDRESS_START))
            }
            _ => Err(()),
        }
    }

    fn read32(&mut self, addr: u64) -> Result<u32, ()> {
        match addr {
            TIMER_ADDRESS_START..=TIMER_ADDRESS_END => {
                Ok(self.timer.read(addr - TIMER_ADDRESS_START))
            }
            INTC_ADDRESS_START..=INTC_ADDRESS_END => Ok(self.intc.read(addr - INTC_ADDRESS_START)),
            PRCI_ADDRESS_START..=PRCI_ADDRESS_END => Ok(self.prci.read(addr - PRCI_ADDRESS_START)),
            GPIO_ADDRESS_START..=GPIO_ADDRESS_END => Ok(self.gpio.read(addr - GPIO_ADDRESS_START)),
            UART0_ADDRESS_START..=UART0_ADDRESS_END => {
                Ok(self.uart0.read(addr - UART0_ADDRESS_START))
            }
            UART1_ADDRESS_START..=UART1_ADDRESS_END => {
                Ok(self.uart1.read(addr - UART1_ADDRESS_START))
            }
            SPIFLASH_ADDRESS_START..=SPIFLASH_ADDRESS_END => {
                Ok(self.flash.read32(addr - SPIFLASH_ADDRESS_START))
            }
            DTIM_ADDRESS_START..=DTIM_ADDRESS_END => {
                Ok(self.dtim.read32(addr - DTIM_ADDRESS_START))
            }
            _ => Err(()),
        }
    }

    fn read64(&mut self, addr: u64) -> Result<u64, ()> {
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
            UART0_ADDRESS_START..=UART0_ADDRESS_END => {
                let uart0_addr = addr - UART0_ADDRESS_START;
                let data = self.uart0.read(uart0_addr) as u64
                    | ((self.uart0.read(uart0_addr.wrapping_add(4)) as u64) << 32);
                Ok(data)
            }
            UART1_ADDRESS_START..=UART1_ADDRESS_END => {
                let uart1_addr = addr - UART1_ADDRESS_START;
                let data = self.uart1.read(uart1_addr) as u64
                    | ((self.uart1.read(uart1_addr.wrapping_add(4)) as u64) << 32);
                Ok(data)
            }
            SPIFLASH_ADDRESS_START..=SPIFLASH_ADDRESS_END => {
                Ok(self.flash.read64(addr - SPIFLASH_ADDRESS_START))
            }
            DTIM_ADDRESS_START..=DTIM_ADDRESS_END => {
                Ok(self.dtim.read64(addr - DTIM_ADDRESS_START))
            }
            _ => Err(()),
        }
    }

    fn write8(&mut self, addr: u64, data: u8) -> Result<(), ()> {
        match addr {
            TIMER_ADDRESS_START..=TIMER_ADDRESS_END => panic!("Unexpected size access."),
            INTC_ADDRESS_START..=INTC_ADDRESS_END => panic!("Unexpected size access."),
            PRCI_ADDRESS_START..=PRCI_ADDRESS_END => panic!("Unexpected size access."),
            GPIO_ADDRESS_START..=GPIO_ADDRESS_END => panic!("Unexpected size access."),
            UART0_ADDRESS_START..=UART0_ADDRESS_END => panic!("Unexpected size access."),
            UART1_ADDRESS_START..=UART1_ADDRESS_END => panic!("Unexpected size access."),
            SPIFLASH_ADDRESS_START..=SPIFLASH_ADDRESS_END => {
                Ok(self.flash.write8(addr - SPIFLASH_ADDRESS_START, data))
            }
            DTIM_ADDRESS_START..=DTIM_ADDRESS_END => {
                Ok(self.dtim.write8(addr - DTIM_ADDRESS_START, data))
            }
            _ => Err(()),
        }
    }

    fn write16(&mut self, addr: u64, data: u16) -> Result<(), ()> {
        match addr {
            TIMER_ADDRESS_START..=TIMER_ADDRESS_END => panic!("Unexpected size access."),
            INTC_ADDRESS_START..=INTC_ADDRESS_END => panic!("Unexpected size access."),
            PRCI_ADDRESS_START..=PRCI_ADDRESS_END => panic!("Unexpected size access."),
            GPIO_ADDRESS_START..=GPIO_ADDRESS_END => panic!("Unexpected size access."),
            UART0_ADDRESS_START..=UART0_ADDRESS_END => panic!("Unexpected size access."),
            UART1_ADDRESS_START..=UART1_ADDRESS_END => panic!("Unexpected size access."),
            SPIFLASH_ADDRESS_START..=SPIFLASH_ADDRESS_END => {
                Ok(self.flash.write16(addr - SPIFLASH_ADDRESS_START, data))
            }
            DTIM_ADDRESS_START..=DTIM_ADDRESS_END => {
                Ok(self.dtim.write16(addr - DTIM_ADDRESS_START, data))
            }
            _ => Err(()),
        }
    }

    fn write32(&mut self, addr: u64, data: u32) -> Result<(), ()> {
        match addr {
            TIMER_ADDRESS_START..=TIMER_ADDRESS_END => {
                Ok(self.timer.write(addr - TIMER_ADDRESS_START, data))
            }
            INTC_ADDRESS_START..=INTC_ADDRESS_END => {
                Ok(self.intc.write(addr - INTC_ADDRESS_START, data))
            }
            PRCI_ADDRESS_START..=PRCI_ADDRESS_END => {
                Ok(self.prci.write(addr - PRCI_ADDRESS_START, data))
            }
            GPIO_ADDRESS_START..=GPIO_ADDRESS_END => {
                Ok(self.gpio.write(addr - GPIO_ADDRESS_START, data))
            }
            UART0_ADDRESS_START..=UART0_ADDRESS_END => {
                Ok(self.uart0.write(addr - UART0_ADDRESS_START, data))
            }
            UART1_ADDRESS_START..=UART1_ADDRESS_END => {
                Ok(self.uart1.write(addr - UART1_ADDRESS_START, data))
            }
            SPIFLASH_ADDRESS_START..=SPIFLASH_ADDRESS_END => {
                Ok(self.flash.write32(addr - SPIFLASH_ADDRESS_START, data))
            }
            DTIM_ADDRESS_START..=DTIM_ADDRESS_END => {
                Ok(self.dtim.write32(addr - DTIM_ADDRESS_START, data))
            }
            _ => Err(()),
        }
    }

    fn write64(&mut self, addr: u64, data: u64) -> Result<(), ()> {
        match addr {
            TIMER_ADDRESS_START..=TIMER_ADDRESS_END => {
                let timer_addr = addr - TIMER_ADDRESS_START;
                self.timer.write(timer_addr, data as u32);
                self.timer.write(
                    timer_addr.wrapping_add(4),
                    ((data >> 32) & 0xffffffff) as u32,
                );
                Ok(())
            }
            INTC_ADDRESS_START..=INTC_ADDRESS_END => {
                let intc_addr = addr - INTC_ADDRESS_START;
                self.intc.write(intc_addr, data as u32);
                self.intc.write(
                    intc_addr.wrapping_add(4),
                    ((data >> 32) & 0xffffffff) as u32,
                );
                Ok(())
            }
            PRCI_ADDRESS_START..=PRCI_ADDRESS_END => {
                let prci_addr = addr - PRCI_ADDRESS_START;
                self.prci.write(prci_addr, data as u32);
                self.prci.write(
                    prci_addr.wrapping_add(4),
                    ((data >> 32) & 0xffffffff) as u32,
                );
                Ok(())
            }
            GPIO_ADDRESS_START..=GPIO_ADDRESS_END => {
                let gpio_addr = addr - GPIO_ADDRESS_START;
                self.gpio.write(gpio_addr, data as u32);
                self.gpio.write(
                    gpio_addr.wrapping_add(4),
                    ((data >> 32) & 0xffffffff) as u32,
                );
                Ok(())
            }
            UART0_ADDRESS_START..=UART0_ADDRESS_END => {
                let uart0_addr = addr - UART0_ADDRESS_START;
                self.uart0.write(uart0_addr, data as u32);
                self.uart0.write(
                    uart0_addr.wrapping_add(4),
                    ((data >> 32) & 0xffffffff) as u32,
                );
                Ok(())
            }
            UART1_ADDRESS_START..=UART1_ADDRESS_END => {
                let uart1_addr = addr - UART1_ADDRESS_START;
                self.uart1.write(uart1_addr, data as u32);
                self.uart1.write(
                    uart1_addr.wrapping_add(4),
                    ((data >> 32) & 0xffffffff) as u32,
                );
                Ok(())
            }
            SPIFLASH_ADDRESS_START..=SPIFLASH_ADDRESS_END => {
                Ok(self.flash.write64(addr - SPIFLASH_ADDRESS_START, data))
            }
            DTIM_ADDRESS_START..=DTIM_ADDRESS_END => {
                Ok(self.dtim.write64(addr - DTIM_ADDRESS_START, data))
            }
            _ => Err(()),
        }
    }
}
