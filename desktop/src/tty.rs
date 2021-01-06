extern crate pancurses;

use self::pancurses::*;
use std::str;

use riscv_emu::console::Console;

pub struct Tty {
    window: Window,
    in_esc_sequences: bool,
    esc_command_value: u32,
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
            esc_command_value: 0,
        }
    }
}

impl Console for Tty {
    fn putchar(&mut self, c: u8) {
        let str = vec![c];

        // TODO: support ANSI Escape sequences.
        // http://ascii-table.com/ansi-escape-sequences.php
        match c {
            0x0d/* CR */ => {
                // TODO: support CR.
                return;
            }
            0x1b/* ESC */ => {
                self.in_esc_sequences = true;
                return;
            }
            0x5b/* [ */ => {
                if self.in_esc_sequences {
                    return;
                }
            }
            0x3b/* ; */ => {
                // todo: support arguments of commands
                if self.in_esc_sequences {
                    return;
                }
            }
            0x6d/* m */ => {
                // todo: support graphics commands.
                if self.in_esc_sequences {
                    self.in_esc_sequences = false;
                    self.esc_command_value = 0;
                    return;
                }
            }
            0x30/* 0 */ | 0x31/* 1 */ | 0x32/* 2 */ | 0x33/* 3 */ | 0x34/* 4 */ |
            0x35/* 5 */ | 0x36/* 6 */ | 0x37/* 7 */ | 0x38/* 8 */ | 0x39/* 9 */ => {
                if self.in_esc_sequences {
                    self.esc_command_value = self.esc_command_value * 10 + (c - 0x30) as u32;
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

    fn set_input(&mut self, _c: u8) {}

    fn get_output(&mut self) -> u8 {
        0
    }
}
