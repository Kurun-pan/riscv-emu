#[macro_use]
extern crate lazy_static;
extern crate memmap;

pub mod emulator;
pub mod tty;
pub mod cpu;
pub mod memory;
pub mod mmu;
pub mod bus;
pub mod elf_loader;
pub mod peripherals;