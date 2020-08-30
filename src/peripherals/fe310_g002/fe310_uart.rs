// FE310 UART Device
// https://static.dev.sifive.com/FE310-G000.pdf

use crate::tty::Tty;

const UART_TXEN: u32 = 0x1;
const UART_RXEN: u32 = 0x1;

const UART_TXWM: u32 = 0x1;
const UART_RXWM: u32 = 0x2;

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
    /// Reciever FIFO
    r_fifo: Vec<u8>,
    /// transmitter FIFO
    t_fifo: Vec<u8>,
    /// Terminal for serial console.
    tty: Box<dyn Tty>,
    /// current clock cycle.
    cycle: u64,
}

impl Fe310Uart {
    pub fn new(tty_: Box<dyn Tty>) -> Self {
        Fe310Uart {
            txdata: 0,
            rxdata: 0x8000_0000,
            txctrl: 0,
            rxctrl: 0x01,
            ie: 0,
            ip: 0,
            div: 0,
            r_fifo: Vec::new(),
            t_fifo: Vec::new(),
            tty: tty_,
            cycle: 0,
        }
    }

    pub fn tick(&mut self) {
        self.cycle = self.cycle.wrapping_add(1);

        // TODO: Correctly care for the clock frequency.
        // The current settings have no reason.

        // receiver
        if (self.cycle % 0xffff) == 0 {
            if self.rxctrl & UART_RXEN > 0 {
                match self.tty.getchar() {
                    0 => {}
                    c => self.r_fifo.push(c),
                }
            }
            self.update_recieve_interrupt_status();
        }

        // transmitter
        if (self.cycle % 0xf) == 0 && (self.txctrl & UART_TXEN > 0) && self.t_fifo.len() > 0 {
            self.tty.putchar(self.t_fifo[0] as u8);
            self.t_fifo.remove(0);
            self.update_transmit_interrupt_status();
        }
    }

    pub fn read(&mut self, addr: u64) -> u32 {
        match addr & 0xff {
            0x00 => self.txdata,
            0x04 => {
                match self.r_fifo.len() {
                    0 => self.rxdata = 0x8000_0000,
                    _ => {
                        self.rxdata = self.r_fifo[0] as u32;
                        self.r_fifo.remove(0);
                    }
                };
                self.update_recieve_interrupt_status();
                self.rxdata
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
            0x00 => {
                let push_data = (data & 0xff) as u8;
                self.t_fifo.push(push_data);
                self.txdata = push_data as u32;
            }
            0x08 => self.txctrl = data & 0x7_0003,
            0x0C => self.rxctrl = data & 0x7_0001,
            0x10 => self.ie = data & 0x3,
            0x18 => self.div = data & 0xffff,
            n => panic!("Write reserved address: {:x}", n),
        }
    }

    pub fn is_irq(&mut self) -> bool {
        if self.ie & UART_RXWM > 0 && self.ip & UART_RXWM > 0 {
            return true;
        }
        if self.ie & UART_TXWM > 0 && self.ip & UART_TXWM > 0 {
            return true;
        }
        false
    }

    fn update_recieve_interrupt_status(&mut self) {
        if self.r_fifo.len() != 0 && self.r_fifo.len() >= ((self.rxctrl >> 16) & 0x7) as usize {
            if (self.ie & UART_RXWM) > 0 {
                self.ip |= UART_RXWM;
            }
        } else {
            self.ip &= !UART_RXWM;
        }
    }

    fn update_transmit_interrupt_status(&mut self) {
        if self.t_fifo.len() != 0 && self.t_fifo.len() >= ((self.txctrl >> 16) & 0x7) as usize {
            if (self.ie & UART_TXWM) > 0 {
                self.ip |= UART_TXWM;
            }
        } else {
            self.ip &= !UART_TXWM;
        }
    }
}
