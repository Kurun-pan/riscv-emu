// 16550a UART Device
// http://byterunner.com/16550.html

use crate::console::Console;

const IER_DATA_READY: u8 = 0x01;
const IER_THR_EMPTY: u8 = 0x02;

const ISR_IDENTIFICATION_CODE_DATA_READY: u8 = 0x4;
const ISR_IDENTIFICATION_CODE_THR_EMPTY: u8 = 0x2;

const LCR_DIVISOR_LATCH_ENABLE: u8 = 0x80;

const LSR_DATA_READY: u8 = 0x01;
const LSR_THR_EMPTY: u8 = 0x20;

pub struct Uart {
    /// Receive Hold Register, RO
    rhr: u8,
    /// Transmit Hold Register, WO
    thr: u8,
    /// Interrupt Enable Register (IER), R/W    
    ier: u8,
    /// Interrupt Status Register (ISR), RO
    isr: u8,
    /// FIFO Control Register (FCR), WO
    fcr: u8,
    /// Line Control Register (LCR), R/W
    lcr: u8,
    /// Modem Control Register (MCR), R/W
    mcr: u8,
    /// Line Status Register (LSR), RO
    lsr: u8,
    /// Modem Status Register (MSR), RO
    msr: u8,
    /// ScratchPad Register (SPR), R/W
    spr: u8,
    /// Terminal for serial console.
    console: Box<dyn Console>,
    /// current clock cycle.
    cycle: u64,
}

impl Uart {
    pub fn new(console_: Box<dyn Console>) -> Self {
        Uart {
            rhr: 0,
            thr: 0,
            ier: 0,
            isr: 0x0e,
            fcr: 0,
            lcr: 0,
            mcr: 0,
            lsr: 0x20,
            msr: 0,
            spr: 0,
            console: console_,
            cycle: 0,
        }
    }

    pub fn tick(&mut self) {
        self.cycle = self.cycle.wrapping_add(1);

        // TODO: Correctly care for the clock frequency (1MHz clock @ RTCCLK).
        // The current settings have no reason.
        // receiver
        if (self.cycle & 0xffff) == 0 && self.rhr == 0 {
            match self.console.getchar() {
                0 => {}
                c => {
                    self.rhr = c;
                    self.lsr |= LSR_DATA_READY;
                }
            }
        }

        // transmitter
        if (self.cycle & 0xf) == 0 && self.thr != 0 {
            self.console.putchar(self.thr);
            self.thr = 0;
            self.lsr |= LSR_THR_EMPTY;
        }
    }

    pub fn read(&mut self, addr: u64) -> u8 {
        match addr & 0x7 {
            0 => {
                let rhr = self.rhr;
                self.rhr = 0;
                self.lsr &= !LSR_DATA_READY;
                rhr
            }
            1 => {
                if self.lcr & LCR_DIVISOR_LATCH_ENABLE == 0 {
                    self.ier
                } else {
                    0
                }
            }
            2 => self.isr,
            3 => self.lcr,
            4 => self.mcr,
            5 => self.lsr,
            6 => self.msr,
            7 => self.spr,
            _ => panic!(),
        }
    }

    pub fn write(&mut self, addr: u64, data: u8) {
        match addr & 0x7 {
            0 => {
                if self.lcr & LCR_DIVISOR_LATCH_ENABLE == 0 {
                    self.thr = data;
                    self.lsr &= !LSR_THR_EMPTY;
                }
            }
            1 => {
                if self.lcr & LCR_DIVISOR_LATCH_ENABLE == 0 {
                    self.ier = data;
                }
            }
            2 => self.fcr = data,
            3 => self.lcr = data,
            4 => self.mcr = data,
            //5 => {} // RO
            //6 => {} // RO
            7 => self.spr = data,
            _ => panic!(),
        }
    }

    pub fn is_irq(&mut self) -> bool {
        let mut irq = false;
        // prioritized interrupt levels: LSR > RXRDY > RXRDY (Timeout) > TXRDY > MSR
        if (self.ier & IER_DATA_READY) != 0 && self.rhr != 0 {
            self.isr = ISR_IDENTIFICATION_CODE_DATA_READY;
            irq = true;
        } else if (self.ier & IER_THR_EMPTY) != 0 && self.thr == 0 {
            self.isr = ISR_IDENTIFICATION_CODE_THR_EMPTY;
            irq = true;
        } else {
            self.isr = 0xe;
        }
        return irq;
    }
}
