#[macro_use]
extern crate lazy_static;
extern crate memmap;

pub mod emulator;
pub mod console;
pub mod cpu;
pub mod bus;
pub mod elf_loader;
pub mod peripherals;