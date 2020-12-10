pub trait Console {
    fn putchar(&mut self, c: u8);
    fn getchar(&mut self) -> u8;
    fn set_input(&mut self, c: u8);
    fn get_output(&mut self) -> u8;
}

pub struct TtyDummy {}

impl TtyDummy {
    pub fn new() -> Self {
        TtyDummy {}
    }
}

impl Console for TtyDummy {
    fn putchar(&mut self, _c: u8) {}

    fn getchar(&mut self) -> u8 {
        0
    }

    fn set_input(&mut self, _c: u8) {}

    fn get_output(&mut self) -> u8 {
        0
    }
}
