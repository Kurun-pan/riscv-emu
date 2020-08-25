// PRCI (Power, Reset, Clock, Interrupt)
// https://sifive.cdn.prismic.io/sifive%2F9ecbb623-7c7f-4acc-966f-9bb10ecdb62e_fe310-g002.pdf
// https://bitbucket.org/nuttx/nuttx/src/master/arch/risc-v/src/fe310/fe310_clockconfig.c

pub struct Prci {
    hfrosccfg: u32,
    hfxosccfg: u32,
    pllcfg: u32,
    plloutdiv: u32,
    procmoncfg: u32,
}

impl Prci {
    pub fn new() -> Self {
        Prci {
            hfrosccfg: 0,
            hfxosccfg: 0,
            pllcfg: 0,
            plloutdiv: 0,
            procmoncfg: 0,
        }
    }

    pub fn tick(&mut self) {
        // do nothing.
    }

    pub fn read(&mut self, addr: u64) -> u32 {
        match addr & 0xff {
            0x00 => self.hfrosccfg | 0x8000_0000 /* OSC ready */,
            0x04 => self.hfxosccfg | 0x8000_0000 /* OSC ready */,
            0x08 => self.pllcfg | 0x8000_0000 /* PLL locked */,
            0x0c => self.plloutdiv,
            0xF0 => self.procmoncfg,
            n => panic!("Read reserved address: {:x}", n),
        }
    }

    pub fn write(&mut self, addr: u64, data: u32) {
        match addr & 0xff {
            0x00 => self.hfrosccfg = data & 0x7fff_ffff,
            0x04 => self.hfxosccfg = data & 0x7fff_ffff,
            0x08 => self.pllcfg = data & 0x7fff_ffff,
            0x0c => self.plloutdiv = data,
            0xF0 => self.procmoncfg = data,
            n => panic!("Write reserved address: {:x}", n),
        }
    }
}
