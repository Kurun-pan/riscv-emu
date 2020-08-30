extern crate pancurses;

use self::pancurses::*;
use std::str;

pub trait Console {
    fn putchar(&mut self, c: u8);
    fn getchar(&mut self) -> u8;
}

pub struct Tty {
    window: Window,
    in_esc_sequences: bool,
}

impl Tty {
    pub fn new() -> Self {
        let w = initscr();
        w.keypad(true);
        w.scrollok(true);
        w.nodelay(true);
        curs_set(0);
        noecho();
        Tty {
            window: w,
            in_esc_sequences: false,
        }
    }
}

impl Console for Tty {
    fn putchar(&mut self, c: u8) {
        let str = vec![c];

        // TODO: support ANSI Escape sequences.
        // http://ascii-table.com/ansi-escape-sequences.php
        match c {
            0x0d => {
                // TODO: support CR.
                return;
            }
            0x1b => {
                // ESC
                self.in_esc_sequences = true;
                return;
            }
            0x5b => {
                // [
                if self.in_esc_sequences {
                    return;
                }
            }
            _ => {
                if self.in_esc_sequences {
                    self.in_esc_sequences = false;
                    return;
                }
            }
        }

        match str::from_utf8(&str) {
            Ok(s) => {
                self.window.printw(s);
                self.window.refresh();
            }
            Err(_e) => {}
        }
    }

    fn getchar(&mut self) -> u8 {
        match self.window.getch() {
            Some(Input::Character(c)) => c as u8,
            _ => 0,
        }
    }
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
}
