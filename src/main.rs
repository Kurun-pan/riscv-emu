extern crate riscv_emu;

use riscv_emu::emulator::Emulator;
use riscv_emu::tty::Tty0;

use std::path::PathBuf;

fn main() {
    let testmode = false;
    let tty = Box::new(Tty0::new());
    let mut emu = Emulator::new(tty, testmode);

    /*
    let data = vec![
        0x13, 0x85, 0x87, 0xfd // addi a0,a5,-40
    ];
    emu.load_dram_data(data);
    emu.set_pc(DRAM_BASE_ADDRESS);
    emu.run();
    */

    let mut root = PathBuf::from(env!("CARGO_MANIFEST_DIR"));
    //root.push("tests/bin/rv64uc-v-rvc");
    root.push("artifacts/xv6/kernel");

    // run test program.
    emu.load_program(root.as_path());
    let result = match emu.run() {
        Ok(ret) => ret,
        Err(ret) => ret,
    };
    println!("Result: {}", result);
}
