extern crate riscv_emu;

use std::io;
use std::io::Read;
use std::fs::File;
use std::path::PathBuf;
use std::path::Path;

use riscv_emu::emulator::Emulator;

fn _read_file(filename: &Path) -> io::Result<Vec<u8>> {
    let mut file = match File::open(&filename) {
        Err(why) => panic!("couldn't open {}: {}", filename.display(), why),
        Ok(file) => file,
    };
    let mut data = Vec::new();
    match file.read_to_end(&mut data) {
        Err(why) => panic!("couldn't read {}: {}", filename.display(), why),
        Ok(_) => Ok(data)
    }
}

fn instruction_test(filename: &'static str) -> i64 {
    // load program
    let mut root = PathBuf::from(env!("CARGO_MANIFEST_DIR"));
    root.push("tests/bin");
    root.push(filename);
    //let data = read_file(root.as_path()).unwrap();
    
    // run test program.
    let mut emu = Emulator::new();
    emu.load_program(root.as_path());
    emu.run();

    emu.cpu.x[10]
}

#[test]
fn addi() {
    assert_eq!(0, instruction_test("rv32ui-p-addi"));
}
