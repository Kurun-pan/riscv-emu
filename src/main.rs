extern crate riscv_emu;

use riscv_emu::emulator::Emulator;
//use riscv_emu::system_bus::DRAM_BASE_ADDRESS;

use std::path::PathBuf;

fn main() {
    let mut emu = Emulator::new();

    /*
    let data = vec![
        0x13, 0x85, 0x87, 0xfd // addi a0,a5,-40
    ];
    emu.load_dram_data(data);
    emu.set_pc(DRAM_BASE_ADDRESS);
    emu.run();
    */

    let mut root = PathBuf::from(env!("CARGO_MANIFEST_DIR"));
    //root.push("tests/bin/rv32ui-v-add");
    root.push("artifacts/xv6/kernel");

    // run test program.
    emu.load_program(root.as_path());
    let result = match emu.run() {
        Ok(ret) => ret,
        Err(ret) => ret,
    };
    //println!("instruction test result is {}", result);
}
