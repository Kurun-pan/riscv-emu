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

fn instruction_test(filename: &'static str) -> u32 {
    // load program
    let mut root = PathBuf::from(env!("CARGO_MANIFEST_DIR"));
    root.push("tests/bin");
    root.push(filename);
    //let data = read_file(root.as_path()).unwrap();

    // run test program.
    let mut emu = Emulator::new();
    emu.load_program(root.as_path());
    let result = match emu.run() {
        Ok(ret) => ret,
        Err(ret) => ret,
    };
    println!("instruction test result is {}", result);
    result
}

//***********************************************************************
// rv32ui (RV32 user-level, integer only), virtual memory is disable
//***********************************************************************
#[test]
fn rv32ui_p_add() { assert_eq!(1, instruction_test("rv32ui-p-add")); }

#[test]
fn rv32ui_p_addi() { assert_eq!(1, instruction_test("rv32ui-p-addi")); }

#[test]
fn rv32ui_p_and() { assert_eq!(1, instruction_test("rv32ui-p-and")); }

#[test]
fn rv32ui_p_andi() { assert_eq!(1, instruction_test("rv32ui-p-andi")); }

#[test]
fn rv32ui_p_auipc() { assert_eq!(1, instruction_test("rv32ui-p-auipc")); }

#[test]
fn rv32ui_p_beq() { assert_eq!(1, instruction_test("rv32ui-p-beq")); }

#[test]
fn rv32ui_p_bge() { assert_eq!(1, instruction_test("rv32ui-p-bge")); }

#[test]
fn rv32ui_p_bgeu() { assert_eq!(1, instruction_test("rv32ui-p-bgeu")); }

#[test]
fn rv32ui_p_blt() { assert_eq!(1, instruction_test("rv32ui-p-blt")); }

#[test]
fn rv32ui_p_bltu() { assert_eq!(1, instruction_test("rv32ui-p-bltu")); }

#[test]
fn rv32ui_p_bne() { assert_eq!(1, instruction_test("rv32ui-p-bne")); }

#[test]
fn rv32ui_p_fence_i() { assert_eq!(1, instruction_test("rv32ui-p-fence_i")); }

#[test]
fn rv32ui_p_jal() { assert_eq!(1, instruction_test("rv32ui-p-jal")); }

#[test]
fn rv32ui_p_jalr() { assert_eq!(1, instruction_test("rv32ui-p-jalr")); }

#[test]
fn rv32ui_p_lb() { assert_eq!(1, instruction_test("rv32ui-p-lb")); }

#[test]
fn rv32ui_p_lbu() { assert_eq!(1, instruction_test("rv32ui-p-lbu")); }

#[test]
fn rv32ui_p_lh() { assert_eq!(1, instruction_test("rv32ui-p-lh")); }

#[test]
fn rv32ui_p_lhu() { assert_eq!(1, instruction_test("rv32ui-p-lhu")); }

#[test]
fn rv32ui_p_lui() { assert_eq!(1, instruction_test("rv32ui-p-lui")); }

#[test]
fn rv32ui_p_lw() { assert_eq!(1, instruction_test("rv32ui-p-lw")); }

#[test]
fn rv32ui_p_or() { assert_eq!(1, instruction_test("rv32ui-p-or")); }

#[test]
fn rv32ui_p_ori() { assert_eq!(1, instruction_test("rv32ui-p-ori")); }

#[test]
fn rv32ui_p_sb() { assert_eq!(1, instruction_test("rv32ui-p-sb")); }

#[test]
fn rv32ui_p_sh() { assert_eq!(1, instruction_test("rv32ui-p-sh")); }

#[test]
fn rv32ui_p_simple() { assert_eq!(1, instruction_test("rv32ui-p-simple")); }

/*
#[test]
fn rv32ui_p_sll() { assert_eq!(1, instruction_test("rv32ui-p-sll")); }
*/

#[test]
fn rv32ui_p_slli() { assert_eq!(1, instruction_test("rv32ui-p-slli")); }

/*
#[test]
fn rv32ui_p_slt() { assert_eq!(1, instruction_test("rv32ui-p-slt")); }
*/

#[test]
fn rv32ui_p_slti() { assert_eq!(1, instruction_test("rv32ui-p-slti")); }

#[test]
fn rv32ui_p_sltiu() { assert_eq!(1, instruction_test("rv32ui-p-sltiu")); }

/*
#[test]
fn rv32ui_p_sltu() { assert_eq!(1, instruction_test("rv32ui-p-sltu")); }

#[test]
fn p_sra() { assert_eq!(1, instruction_test("rv32ui-p-sra")); }
*/

#[test]
fn rv32ui_p_srai() { assert_eq!(1, instruction_test("rv32ui-p-srai")); }

/*
#[test]
fn rv32ui_p_srl() { assert_eq!(1, instruction_test("rv32ui-p-srl")); }

#[test]
fn rv32ui_p_srli() { assert_eq!(1, instruction_test("rv32ui-p-srli")); }
*/

#[test]
fn rv32ui_p_sub() { assert_eq!(1, instruction_test("rv32ui-p-sub")); }

#[test]
fn rv32ui_p_sw() { assert_eq!(1, instruction_test("rv32ui-p-sw")); }

#[test]
fn p_xor() { assert_eq!(1, instruction_test("rv32ui-p-xor")); }

#[test]
fn p_xori() { assert_eq!(1, instruction_test("rv32ui-p-xori")); }

//***********************************************************************
// rv32ui (RV32 user-level, integer only), virtual memory is enabled
//***********************************************************************
/*
#[test]
fn v_add() { assert_eq!(1, instruction_test("rv32ui-v-add")); }

#[test]
fn v_addi() { assert_eq!(1, instruction_test("rv32ui-v-addi")); }
*/