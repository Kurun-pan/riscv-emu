// UART Device
// https://sifive.cdn.prismic.io/sifive%2F9ecbb623-7c7f-4acc-966f-9bb10ecdb62e_fe310-g002.pdf

use crate::tty::Tty;

const BIT_RXEN: u32 = 0x1;
const BIT_TXEN: u32 = 0x1;

const BIT_TXWM: u32 = 0x1;
const BIT_RXWM: u32 = 0x2;

pub struct Fe310Uart {
    // /Transmit data register
    txdata: u32,
    /// Receive data register (RO)
    rxdata: u32,
    /// Transmit control register
    txctrl: u32,
    /// Receive control register
    rxctrl: u32,
    /// UART interrupt enable
    ie: u32,
    /// UART interrupt pending
    ip: u32,
    /// Baud rate divisor
    div: u32,
    /// Terminal for serial console.
    tty: Box<dyn Tty>,
    /// current clock cycle.
    cycle: u64,
}

impl Fe310Uart {
    pub fn new(tty_: Box<dyn Tty>) -> Self {
        Fe310Uart {
            txdata: 0,
            rxdata: 0,
            txctrl: 0,
            rxctrl: 0x01,
            ie: 0,
            ip: 0,
            div: 0,
            tty: tty_,
            cycle: 0,
        }
    }

    pub fn tick(&mut self) {
        self.cycle = self.cycle.wrapping_add(1);

        // TODO: Correctly care for the clock frequency.
        // The current settings have no reason.

        // receiver
        if (self.cycle & 0xffff) == 0 && (self.rxctrl & BIT_RXEN > 0) {
            match self.tty.getchar() {
                0 => {}
                c => {
                    self.rxdata = (c as u32) | 0x8000_0000;
                    if (self.ie & BIT_RXWM) > 0 {
                        self.ip |= BIT_RXWM;
                    }
                }
            }
        }
        // transmitter
        if (self.cycle & 0xf) == 0 && (self.txctrl & BIT_TXEN > 0) && (self.txdata & 0x8000_0000 > 0) {
            self.tty.putchar((self.txdata & 0xff) as u8);
            if (self.ie & BIT_TXWM) > 0 {
                self.ip |= BIT_TXWM;
            }
            self.txdata = 0;
        }
    }

    pub fn read(&mut self, addr: u64) -> u32 {
        match addr & 0xff {
            0x00 => self.txdata,
            0x04 => {
                let data = self.rxdata;
                self.rxdata &= !8000_0000;
                data
            }
            0x08 => self.txctrl,
            0x0C => self.rxctrl,
            0x10 => self.ie,
            0x14 => self.ip,
            0x18 => self.div,
            n => panic!("Read reserved address: {:x}", n),
        }
    }

    pub fn write(&mut self, addr: u64, data: u32) {
        match addr & 0xff {
            0x00 => self.txdata = data & 0xff | 0x8000_0000,
            0x08 => self.txctrl = data & 0x7_0003,
            0x0C => self.rxctrl = data & 0x7_0001,
            0x10 => self.ie = data & 0x3,
            0x18 => self.div = data & 0xffff,
            n => panic!("Write reserved address: {:x}", n),
        }
    }

    pub fn is_irq(&mut self) -> bool {
        if self.ie & BIT_RXWM > 0 && self.ip & BIT_RXWM > 0 {
            return true;
        }
        if self.ie & BIT_TXWM > 0 && self.ip & BIT_TXWM > 0 {
            return true;
        }
        false
    }
}
