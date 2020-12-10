use riscv_emu::console::Console;
use std::collections::VecDeque;

pub struct TtyWeb {
    queue_i: VecDeque<u8>,
    queue_o: VecDeque<u8>,
}

impl TtyWeb {
    pub fn new() -> Self {
        TtyWeb {
            queue_i: VecDeque::new(),
            queue_o: VecDeque::new(),
        }
    }
}

impl Console for TtyWeb {
    fn putchar(&mut self, c: u8) {
        self.queue_o.push_back(c);
    }

    fn getchar(&mut self) -> u8 {
        match self.queue_i.len() > 0 {
            true => self.queue_i.pop_front().unwrap(),
            false => 0,
        }
    }

    fn set_input(&mut self, c: u8) {
        self.queue_i.push_back(c);
    }

    fn get_output(&mut self) -> u8 {
        match self.queue_o.len() > 0 {
            true => self.queue_o.pop_front().unwrap(),
            false => 0,
        }
    }
}
