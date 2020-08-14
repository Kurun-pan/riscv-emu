// UART 16550
// http://caro.su/msx/ocm_de1/16550.pdf

pub struct Uart {
    rhr: u8, // RO
    thr: u8, // WO
    ier: u8, // RW
    isr: u8, // RO
    fcr: u8, // WO
    lcr: u8, // RW
    mcr: u8, // RW
    lsr: u8, // RO
    msr: u8, // RO
    spr: u8, // RW
}

impl Uart {
    pub fn new() -> Self {
        Uart {
            rhr: 0,
            thr: 0,
            ier: 0,
            isr: 0x01,
            fcr: 0,
            lcr: 0,
            mcr: 0,
            lsr: 0x60,
            msr: 0,
            spr: 0,
        }
    }

    pub fn tick(&mut self) {
        // input
        if self.rhr == 0 {
            // TODO: Implement here.

            // assert Data Ready interrupt.
            self.lsr |= 0x1;
        }

        // output
        if self.thr != 0 {
            // TODO: output data to a terminal.
            println!("[UART] {:x}", self.thr);
            self.thr = 0;

            // assert THR Empty interrupt.
            self.lsr |= 0x20;
        }
    }

    pub fn is_irq(&mut self) -> bool {
        if (self.ier & 0x1) > 0 && self.rhr != 0 {
            // set "Received Data Ready" code.
            self.isr = self.isr & 0xf0 | 0x4;
            return true;
        }
        if (self.ier & 0x2) > 0 && self.thr == 0 {
            // set "Transmitter Holding Register Empty" code.
            self.isr = self.isr & 0xf0 | 0x2;
            return true;
        }
        self.isr = self.isr & 0xf0 | 0x1;
        false
    }

    pub fn read(&mut self, addr: u64) -> u8 {
        // TODO: Implement here.
        match addr & 0x7 {
            0 => self.rhr,
            1 => self.ier,
            2 => self.isr,
            3 => self.lcr,
            4 => self.mcr,
            5 => self.lsr,
            6 => self.msr,
            7 => self.spr,
            _ => panic!()
        }
    }

    pub fn write(&mut self, addr: u64, data: u8) {
        // TODO: Implement here.
        match addr & 0x7 {
            0 => self.thr = data,
            1 => self.ier = data,
            2 => self.fcr = data,
            3 => self.lcr = data,
            4 => self.mcr = data,
            5 => self.lsr = data,
            6 => self.msr = data,
            7 => self.spr = data,
            _ => panic!()
        }
    }
}
