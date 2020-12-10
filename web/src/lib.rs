extern crate riscv_emu;
extern crate wasm_bindgen;

pub mod tty_web;
use tty_web::TtyWeb;

use wasm_bindgen::prelude::*;

use riscv_emu::bus::bus::Device;
use riscv_emu::emulator::Emulator;
use riscv_emu::machine::Machine;

#[wasm_bindgen]
pub struct RiscvEmu {
    core: Emulator,
}

#[wasm_bindgen]
impl RiscvEmu {
    pub fn new() -> Self {
        RiscvEmu {
            core: Emulator::new(Machine::QemuVirt, Box::new(TtyWeb::new()), false),
        }
    }

    pub fn load_program(&mut self, data: Vec<u8>) {
        self.core.load_program_from_binary(data);
    }

    pub fn load_disk_image(&mut self, data: Vec<u8>) {
        self.core.set_data_from_binary(Device::Disk, data);
    }

    pub fn load_dtb(&mut self, data: Vec<u8>) {
        self.core.set_data_from_binary(Device::DTB, data);
    }

    pub fn run(&mut self) {
        let result = match self.core.run() {
            Ok(ret) => ret,
            Err(ret) => ret,
        };
        println!("Result: {}", result);
    }

    pub fn get_console_output(&mut self) -> u8 {
        self.core.get_console().get_output()
    }

    pub fn set_console_input(&mut self, data: u8) {
        self.core.get_console().set_input(data)
    }
}
