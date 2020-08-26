// GPIO
// https://sifive.cdn.prismic.io/sifive%2F9ecbb623-7c7f-4acc-966f-9bb10ecdb62e_fe310-g002.pdf
// https://bitbucket.org/nuttx/nuttx/src/master/arch/risc-v/src/fe310/fe310_gpio.c

pub struct Gpio {
    /// Pin value
    input_val: u32,
    /// Pin input enable
    input_en: u32,
    /// Pin output enable
    output_en: u32,
    /// Output value
    output_val: u32,
    /// Internal pull-up enable
    pue: u32,
    /// Pin drive strength
    ds: u32,
    /// Rise interrupt enable
    rise_ie: u32,
    /// Rise interrupt pending
    rise_ip: u32,
    /// Fall interrupt enable
    fall_ie: u32,
    /// Fall interrupt pending
    fall_ip: u32,
    /// High interrupt enable
    high_ie: u32,
    /// High interrupt pending
    high_ip: u32,
    /// Low interrupt enable
    low_ie: u32,
    /// Low interrupt pending
    low_ip: u32,
    /// 
    iof_en: u32,
    /// 
    iof_sel: u32,
    /// Output XOR (invert)
    out_xor: u32,
}

impl Gpio {
    pub fn new() -> Self {
        Gpio {
            input_val: 0,
            input_en: 0,
            output_en: 0,
            output_val: 0,
            pue: 0,
            ds: 0,
            rise_ie: 0,
            rise_ip: 0,
            fall_ie: 0,
            fall_ip: 0,
            high_ie: 0,
            high_ip: 0,
            low_ie: 0,
            low_ip: 0,
            iof_en: 0,
            iof_sel: 0,
            out_xor: 0,
        }
    }

    pub fn tick(&mut self) {
        // do nothing.
    }

    pub fn is_irq(&mut self) -> bool {
        self.rise_ip != 0 || self.fall_ip != 0 || self.high_ip != 0 || self.low_ip != 0
    }

    pub fn read(&mut self, addr: u64) -> u32 {
        match addr & 0xff {
            0x00 => self.input_val,
            0x04 => self.input_en,
            0x08 => self.output_en,
            0x0c => self.output_val,
            0x10 => self.pue,
            0x14 => self.ds,
            0x18 => self.rise_ie,
            0x1c => self.rise_ip,
            0x20 => self.fall_ie,
            0x24 => self.fall_ip,
            0x28 => self.high_ie,
            0x2c => self.high_ip,
            0x30 => self.low_ie,
            0x34 => self.low_ip,
            0x38 => self.iof_en,
            0x3c => self.iof_sel,
            0x40 => self.out_xor,
            n => panic!("Read reserved address: {:x}", n),
        }
    }

    pub fn write(&mut self, addr: u64, data: u32) {
        match addr & 0xff {
            0x00 => self.input_val = data,
            0x04 => self.input_en = data,
            0x08 => self.output_en = data,
            0x0c => self.output_val = data,
            0x10 => self.pue = data,
            0x14 => self.ds = data,
            0x18 => self.rise_ie = data,
            0x1c => self.rise_ip &= !data, // clear interrupt when 1 is written
            0x20 => self.fall_ie = data,
            0x24 => self.fall_ip &= !data, // clear interrupt when 1 is written
            0x28 => self.high_ie = data,
            0x2c => self.high_ip &= !data, // clear interrupt when 1 is written
            0x30 => self.low_ie = data,
            0x34 => self.low_ip &= !data, // clear interrupt when 1 is written
            0x38 => self.iof_en = data,
            0x3c => self.iof_sel = data,
            0x40 => self.out_xor = data,
            n => panic!("Write reserved address: {:x}", n),
        }
    }
}
