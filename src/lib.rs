#[macro_use]
extern crate lazy_static;
extern crate memmap;

pub mod emulator;
pub mod cpu;
pub mod dram;
pub mod mmu;
pub mod system_bus;
pub mod elf_loader;
pub mod peripherals;
