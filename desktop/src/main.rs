extern crate getopts;
extern crate riscv_emu;

use riscv_emu::bus::bus::Device;
use riscv_emu::console::TtyDummy;
use riscv_emu::emulator::Emulator;
use riscv_emu::machine::Machine;

use riscv_emu_desktop::tty::Tty;

use getopts::Options;
use std::path::PathBuf;
use std::{env, process};

fn main() {
    let args: Vec<String> = env::args().collect();
    let program = args[0].clone();

    let mut opts = Options::new();
    opts.optopt("k", "kernel", "Kernel image file", "./artifacts/xv6/kernel");
    opts.optopt(
        "f",
        "filesystem",
        "File system image file",
        "./artifacts/xv6/fs.img",
    );
    opts.optopt(
        "d",
        "dtb",
        "Device tree binary file",
        "./artifacts/linux/qemu_virtio.dtb",
    );
    opts.optopt(
        "m",
        "machine",
        "Target machine (SiFive_e|SiFive_u|Qemu_virt)",
        "SiFive_e",
    );
    opts.optflag("t", "testmode", "Testmode is enabled");
    opts.optflag("h", "help", "Help message");

    if args.len() < 2 {
        print_usage(&program, &opts);
        process::exit(0);
    }

    let matches = opts
        .parse(&args[1..])
        .unwrap_or_else(|f| panic!(f.to_string()));

    if matches.opt_present("h") {
        print_usage(&program, &opts);
    }

    let kernel_path = match matches.opt_str("k") {
        Some(filepath) => filepath,
        None => {
            print_usage(&program, &opts);
            process::exit(0);
        }
    };
    let fs_path = matches.opt_str("f");
    let dtb_path = matches.opt_str("d");
    let testmode = matches.opt_present("t");
    let machine = match matches.opt_str("m") {
        Some(machine_name) => match &*machine_name {
            "Qemu_virt" => Machine::QemuVirt,
            "SiFive_e" => Machine::SiFiveE,
            "SiFive_u" => Machine::SiFiveU,
            _ => Machine::SiFiveU,
        },
        None => Machine::SiFiveU,
    };
    let mut emu;
    if testmode {
        let tty = Box::new(TtyDummy::new());
        emu = Emulator::new(machine, tty, testmode);
    } else {
        let tty = Box::new(Tty::new());
        emu = Emulator::new(machine, tty, testmode);
    }

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
        let kernel = PathBuf::from(kernel_path);
        emu.load_program_from_file(kernel.as_path());
    }

    // download disk image (Userland rootfs)
    match fs_path {
        Some(filepath) => {
            let fs = PathBuf::from(filepath);
            emu.set_data_from_file(Device::Disk, fs.as_path());
        }
        None => {}
    }

    // download dtb image
    match dtb_path {
        Some(filepath) => {
            let fs = PathBuf::from(filepath);
            emu.set_data_from_file(Device::DTB, fs.as_path());
        }
        None => {}
    }

    // run emulator.
    let result = match emu.run() {
        Ok(ret) => ret,
        Err(ret) => ret,
    };
    println!("Result: {}", result);
}

fn print_usage(program: &str, opts: &Options) {
    let brief = format!("Usage: {} FILE [options]", program);
    print!("{}", opts.usage(&brief));
}
