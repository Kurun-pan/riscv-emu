// QEMU Virt Machine

use crate::bus::bus::*;
use crate::console::*;
use crate::peripherals::fu540_c000::clint::Clint;
use crate::peripherals::fu540_c000::plic::Plic;
use crate::peripherals::intc::Intc;
use crate::peripherals::memory::Memory;
use crate::peripherals::timer::Timer;
use crate::peripherals::uart::Uart;
use crate::peripherals::virtio::Virtio;

const DTB_ADDRESS_START: u64 = 0x0000_1020;
const DTB_ADDRESS_END: u64 = 0x0000_1FFF;

const MROM_ADDRESS_START: u64 = 0x0000_1000;
const MROM_ADDRESS_END: u64 = 0x0000_FFFF;

const TIMER_ADDRESS_START: u64 = 0x0200_0000;
const TIMER_ADDRESS_END: u64 = 0x0200_FFFF;

const INTC_ADDRESS_START: u64 = 0x0C00_0000;
const INTC_ADDRESS_END: u64 = 0x0FFF_FFFF;

const UART_ADDRESS_START: u64 = 0x1000_0000;
const UART_ADDRESS_END: u64 = 0x1000_0FFF;

const VIRTIO_ADDRESS_START: u64 = 0x1000_1000;
const VIRTIO_ADDRESS_END: u64 = 0x1000_1FFF;

const DRAM_ADDRESS_START: u64 = 0x8000_0000;

const MROM_SIZE: usize = 0xF000;
pub const DRAM_SIZE: usize = 1024 * 1024 * 256; // todo: support command line to change size.
const DTB_SIZE: usize = 0xfe0;

pub struct BusQemuVirt {
    clock: u64,
    dtb: Vec<u8>,
    mrom: Memory,
    dram: Memory,
    timer: Box<dyn Timer>,
    intc: Box<dyn Intc>,
    uart: Uart,
    virtio: Virtio,
}

impl BusQemuVirt {
    pub fn new(console: Box<dyn Console>) -> Self {
        Self {
            clock: 0,
            dtb: vec![0; DTB_SIZE],
            mrom: Memory::new(MROM_SIZE),
            dram: Memory::new(DRAM_SIZE),
            timer: Box::new(Clint::new()),
            intc: Box::new(Plic::new()),
            uart: Uart::new(console),
            virtio: Virtio::new(DRAM_ADDRESS_START),
        }
    }
}

impl Bus for BusQemuVirt {
    fn set_device_data(&mut self, device: Device, data: Vec<u8>) {
        match device {
            Device::Dram => {
                self.dram.initialize(data);
            }
            Device::Disk => {
                self.virtio.init(data);
            }
            Device::DTB => {
                self.dtb.splice(..data.len(), data.iter().cloned());
            }
            _ => panic!("Unexpected device: {:?}", device),
        }
    }

    fn tick(&mut self) -> Vec<bool> {
        self.clock = self.clock.wrapping_add(1);

        self.virtio.tick(&mut self.dram);
        self.timer.tick();
        self.uart.tick();

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

    fn is_pending_software_interrupt(&mut self, core: usize) -> bool {
        self.timer.is_pending_software_interrupt(core)
    }

    fn is_pending_timer_interrupt(&mut self, core: usize) -> bool {
        self.timer.is_pending_timer_interrupt(core)
    }

    fn get_base_address(&mut self, device: Device) -> u64 {
        match device {
            Device::Dram => DRAM_ADDRESS_START,
            Device::DTB => DTB_ADDRESS_START,
            _ => panic!("Unexpected device: {:?}", device),
        }
    }

    fn read8(&mut self, addr: u64) -> Result<u8, ()> {
        if DRAM_ADDRESS_START <= addr {
            return Ok(self.dram.read8(addr - DRAM_ADDRESS_START));
        }
        match addr {
            DTB_ADDRESS_START..=DTB_ADDRESS_END => {
                Ok(self.dtb[(addr - DTB_ADDRESS_START) as usize])
            }
            MROM_ADDRESS_START..=MROM_ADDRESS_END => Ok(self.mrom.read8(addr - MROM_ADDRESS_START)),
            TIMER_ADDRESS_START..=TIMER_ADDRESS_END => panic!("Unexpected size access."),
            INTC_ADDRESS_START..=INTC_ADDRESS_END => panic!("Unexpected size access."),
            UART_ADDRESS_START..=UART_ADDRESS_END => Ok(self.uart.read(addr - UART_ADDRESS_START)),
            VIRTIO_ADDRESS_START..=VIRTIO_ADDRESS_END => panic!("Unexpected size access."),
            _ => Err(()),
        }
    }

    fn read16(&mut self, addr: u64) -> Result<u16, ()> {
        if DRAM_ADDRESS_START <= addr {
            return Ok(self.dram.read16(addr - DRAM_ADDRESS_START));
        }
        match addr {
            DTB_ADDRESS_START..=DTB_ADDRESS_END => {
                Ok(self.dtb[(addr - DTB_ADDRESS_START) as usize] as u16
                    | ((self.dtb[(addr.wrapping_add(1) - DTB_ADDRESS_START) as usize]) as u16) << 8)
            }
            MROM_ADDRESS_START..=MROM_ADDRESS_END => {
                Ok(self.mrom.read16(addr - MROM_ADDRESS_START))
            }
            TIMER_ADDRESS_START..=TIMER_ADDRESS_END => panic!("Unexpected size access."),
            INTC_ADDRESS_START..=INTC_ADDRESS_END => panic!("Unexpected size access."),
            UART_ADDRESS_START..=UART_ADDRESS_END => {
                let addr_ = addr - UART_ADDRESS_START;
                let data = self.uart.read(addr_) as u16
                    | ((self.uart.read(addr_.wrapping_add(1)) as u16) << 8);
                Ok(data)
            }
            VIRTIO_ADDRESS_START..=VIRTIO_ADDRESS_END => panic!("Unexpected size access."),
            _ => Err(()),
        }
    }

    fn read32(&mut self, addr: u64) -> Result<u32, ()> {
        if DRAM_ADDRESS_START <= addr {
            return Ok(self.dram.read32(addr - DRAM_ADDRESS_START));
        }
        match addr {
            DTB_ADDRESS_START..=DTB_ADDRESS_END => {
                Ok((self.dtb[(addr - DTB_ADDRESS_START) as usize] as u32)
                    | (((self.dtb[(addr.wrapping_add(1) - DTB_ADDRESS_START) as usize]) as u32)
                        << 8)
                    | (((self.dtb[(addr.wrapping_add(2) - DTB_ADDRESS_START) as usize]) as u32)
                        << 16)
                    | (((self.dtb[(addr.wrapping_add(3) - DTB_ADDRESS_START) as usize]) as u32)
                        << 24))
            }
            MROM_ADDRESS_START..=MROM_ADDRESS_END => {
                Ok(self.mrom.read32(addr - MROM_ADDRESS_START))
            }
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
            _ => Err(()),
        }
    }

    fn read64(&mut self, addr: u64) -> Result<u64, ()> {
        if DRAM_ADDRESS_START <= addr {
            return Ok(self.dram.read64(addr - DRAM_ADDRESS_START));
        }
        match addr {
            DTB_ADDRESS_START..=DTB_ADDRESS_END => {
                Ok((self.dtb[(addr - DTB_ADDRESS_START) as usize] as u64)
                    | (((self.dtb[(addr.wrapping_add(1) - DTB_ADDRESS_START) as usize]) as u64)
                        << 8)
                    | (((self.dtb[(addr.wrapping_add(2) - DTB_ADDRESS_START) as usize]) as u64)
                        << 16)
                    | (((self.dtb[(addr.wrapping_add(3) - DTB_ADDRESS_START) as usize]) as u64)
                        << 24)
                    | (((self.dtb[(addr.wrapping_add(4) - DTB_ADDRESS_START) as usize]) as u64)
                        << 32)
                    | (((self.dtb[(addr.wrapping_add(5) - DTB_ADDRESS_START) as usize]) as u64)
                        << 40)
                    | (((self.dtb[(addr.wrapping_add(6) - DTB_ADDRESS_START) as usize]) as u64)
                        << 48)
                    | (((self.dtb[(addr.wrapping_add(7) - DTB_ADDRESS_START) as usize]) as u64)
                        << 56))
            }
            MROM_ADDRESS_START..=MROM_ADDRESS_END => {
                Ok(self.mrom.read64(addr - MROM_ADDRESS_START))
            }
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
            _ => Err(()),
        }
    }

    fn write8(&mut self, addr: u64, data: u8) -> Result<(), ()> {
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
            _ => Err(()),
        }
    }

    fn write16(&mut self, addr: u64, data: u16) -> Result<(), ()> {
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
            _ => Err(()),
        }
    }

    fn write32(&mut self, addr: u64, data: u32) -> Result<(), ()> {
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
            _ => Err(()),
        }
    }

    fn write64(&mut self, addr: u64, data: u64) -> Result<(), ()> {
        if DRAM_ADDRESS_START <= addr {
            return Ok(self.dram.write64(addr - DRAM_ADDRESS_START, data));
        }
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
                self.virtio.write(
                    virtio_addr.wrapping_add(4),
                    ((data >> 32) & 0xffffffff) as u32,
                );
                Ok(())
            }
            _ => Err(()),
        }
    }
}
