extern crate pancurses;

use std::str;
use self::pancurses::*;

pub trait Tty {
    fn putchar(&mut self, c: u8);
    fn getchar(&mut self) -> u8;
}

pub struct Tty0 {
    window: Window,
}

impl Tty0 {
    pub fn new() -> Self {
        let w = initscr();
        w.keypad(true);
        w.scrollok(true);
        w.nodelay(true);
        curs_set(0);
        noecho();
        Tty0 {
            window: w,
        }
    }
}

impl Tty for Tty0 {
    fn putchar(&mut self, c: u8) {
        let str = vec![c];
        self.window.printw(str::from_utf8(&str).unwrap());
        self.window.refresh();
    }

    fn getchar(&mut self) -> u8 {
        match self.window.getch() {
            Some(Input::Character(c)) => c as u8,
            _ => 0
        }
    }
}

pub struct TtyDummy {
}

impl TtyDummy {
    pub fn new() -> Self {
        TtyDummy {
        }
    }
}

impl Tty for TtyDummy {
    fn putchar(&mut self, _c: u8) {
    }

    fn getchar(&mut self) -> u8 {
        0
    }
}
