extern crate riscv_emu;

use riscv_emu::emulator::Emulator;
use riscv_emu::console::*;

use std::path::PathBuf;

fn main() {
    let testmode = false;
    let tty = Box::new(Tty::new());
    //let testmode = true;
    //let tty = Box::new(TtyDummy::new());
    let mut emu = Emulator::new(tty, testmode);

    /*
    let data = vec![
        0x13, 0x85, 0x87, 0xfd // addi a0,a5,-40
    ];
    emu.set_dram_data(data);
    emu.set_pc(DRAM_BASE_ADDRESS);
    emu.run();
    */

    // download user program to main mermoy.
    {
        let mut kernel = PathBuf::from(env!("CARGO_MANIFEST_DIR"));
        #[cfg(not(feature="nuttx"))]
        kernel.push("artifacts/xv6/kernel");
        #[cfg(feature="nuttx")]
        kernel.push("artifacts/nuttx/nuttx");
        emu.load_program(kernel.as_path());
    }

    // download disk image (Userland rootfs)
    if cfg!(feature="xv6") {
        let mut fs = PathBuf::from(env!("CARGO_MANIFEST_DIR"));
        fs.push("artifacts/xv6/fs.img");
        emu.set_disk_data(fs.as_path());
    }

    // run emulator.
    let result = match emu.run() {
        Ok(ret) => ret,
        Err(ret) => ret,
    };
    println!("Result: {}", result);
}
