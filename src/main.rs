extern crate riscv_emu;

use riscv_emu::emulator::Emulator;
use riscv_emu::system_bus::DRAM_BASE_ADDRESS;

fn main() {
    let mut emu = Emulator::new();

    let data = vec![
        0x13, 0x85, 0x87, 0xfd // addi a0,a5,-40
    ];
    emu.load_dram_data(data);
    emu.set_pc(DRAM_BASE_ADDRESS);
    emu.run();
}
