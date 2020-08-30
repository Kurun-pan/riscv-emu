extern crate riscv_emu;

use std::io;
use std::io::Read;
use std::fs::File;
use std::path::PathBuf;
use std::path::Path;

use riscv_emu::emulator::Emulator;
use riscv_emu::console::TtyDummy;

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
    let testmode = true;
    let tty = Box::new(TtyDummy::new());
    let mut emu = Emulator::new(tty, testmode);
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

#[test]
fn rv32ui_p_sll() { assert_eq!(1, instruction_test("rv32ui-p-sll")); }

#[test]
fn rv32ui_p_slli() { assert_eq!(1, instruction_test("rv32ui-p-slli")); }

#[test]
fn rv32ui_p_slt() { assert_eq!(1, instruction_test("rv32ui-p-slt")); }

#[test]
fn rv32ui_p_slti() { assert_eq!(1, instruction_test("rv32ui-p-slti")); }

#[test]
fn rv32ui_p_sltiu() { assert_eq!(1, instruction_test("rv32ui-p-sltiu")); }

#[test]
fn rv32ui_p_sltu() { assert_eq!(1, instruction_test("rv32ui-p-sltu")); }

#[test]
fn rv32ui_p_sra() { assert_eq!(1, instruction_test("rv32ui-p-sra")); }

#[test]
fn rv32ui_p_srai() { assert_eq!(1, instruction_test("rv32ui-p-srai")); }

#[test]
fn rv32ui_p_srl() { assert_eq!(1, instruction_test("rv32ui-p-srl")); }

#[test]
fn rv32ui_p_srli() { assert_eq!(1, instruction_test("rv32ui-p-srli")); }

#[test]
fn rv32ui_p_sub() { assert_eq!(1, instruction_test("rv32ui-p-sub")); }

#[test]
fn rv32ui_p_sw() { assert_eq!(1, instruction_test("rv32ui-p-sw")); }

#[test]
fn rv32ui_p_xor() { assert_eq!(1, instruction_test("rv32ui-p-xor")); }

#[test]
fn rv32ui_p_xori() { assert_eq!(1, instruction_test("rv32ui-p-xori")); }

//***********************************************************************
// rv32ui (RV32 user-level, integer only), virtual memory is enabled
//***********************************************************************
#[test]
fn rv32ui_v_add() { assert_eq!(1, instruction_test("rv32ui-v-add")); }

#[test]
fn rv32ui_v_addi() { assert_eq!(1, instruction_test("rv32ui-v-addi")); }

#[test]
fn rv32ui_v_and() { assert_eq!(1, instruction_test("rv32ui-v-and")); }

#[test]
fn rv32ui_v_andi() { assert_eq!(1, instruction_test("rv32ui-v-andi")); }

#[test]
fn rv32ui_v_auipc() { assert_eq!(1, instruction_test("rv32ui-v-auipc")); }

#[test]
fn rv32ui_v_beq() { assert_eq!(1, instruction_test("rv32ui-v-beq")); }

#[test]
fn rv32ui_v_bge() { assert_eq!(1, instruction_test("rv32ui-v-bge")); }

#[test]
fn rv32ui_v_bgeu() { assert_eq!(1, instruction_test("rv32ui-v-bgeu")); }

#[test]
fn rv32ui_v_blt() { assert_eq!(1, instruction_test("rv32ui-v-blt")); }

#[test]
fn rv32ui_v_bltu() { assert_eq!(1, instruction_test("rv32ui-v-bltu")); }

#[test]
fn rv32ui_v_bne() { assert_eq!(1, instruction_test("rv32ui-v-bne")); }

#[test]
fn rv32ui_v_fence_i() { assert_eq!(1, instruction_test("rv32ui-v-fence_i")); }

#[test]
fn rv32ui_v_jal() { assert_eq!(1, instruction_test("rv32ui-v-jal")); }

#[test]
fn rv32ui_v_jalr() { assert_eq!(1, instruction_test("rv32ui-v-jalr")); }

#[test]
fn rv32ui_v_lb() { assert_eq!(1, instruction_test("rv32ui-v-lb")); }

#[test]
fn rv32ui_v_lbu() { assert_eq!(1, instruction_test("rv32ui-v-lbu")); }

#[test]
fn rv32ui_v_lh() { assert_eq!(1, instruction_test("rv32ui-v-lh")); }

#[test]
fn rv32ui_v_lhu() { assert_eq!(1, instruction_test("rv32ui-v-lhu")); }

#[test]
fn rv32ui_v_lui() { assert_eq!(1, instruction_test("rv32ui-v-lui")); }

#[test]
fn rv32ui_v_lw() { assert_eq!(1, instruction_test("rv32ui-v-lw")); }

#[test]
fn rv32ui_v_or() { assert_eq!(1, instruction_test("rv32ui-v-or")); }

#[test]
fn rv32ui_v_ori() { assert_eq!(1, instruction_test("rv32ui-v-ori")); }

#[test]
fn rv32ui_v_sb() { assert_eq!(1, instruction_test("rv32ui-v-sb")); }

#[test]
fn rv32ui_v_sh() { assert_eq!(1, instruction_test("rv32ui-v-sh")); }

#[test]
fn rv32ui_v_simple() { assert_eq!(1, instruction_test("rv32ui-v-simple")); }

#[test]
fn rv32ui_v_sll() { assert_eq!(1, instruction_test("rv32ui-v-sll")); }

#[test]
fn rv32ui_v_slli() { assert_eq!(1, instruction_test("rv32ui-v-slli")); }

#[test]
fn rv32ui_v_slt() { assert_eq!(1, instruction_test("rv32ui-v-slt")); }

#[test]
fn rv32ui_v_slti() { assert_eq!(1, instruction_test("rv32ui-v-slti")); }

#[test]
fn rv32ui_v_sltiu() { assert_eq!(1, instruction_test("rv32ui-v-sltiu")); }

#[test]
fn rv32ui_v_sltu() { assert_eq!(1, instruction_test("rv32ui-v-sltu")); }

#[test]
fn rv32ui_v_sra() { assert_eq!(1, instruction_test("rv32ui-v-sra")); }

#[test]
fn rv32ui_v_srai() { assert_eq!(1, instruction_test("rv32ui-v-srai")); }

#[test]
fn rv32ui_v_srl() { assert_eq!(1, instruction_test("rv32ui-v-srl")); }

#[test]
fn rv32ui_v_srli() { assert_eq!(1, instruction_test("rv32ui-v-srli")); }

#[test]
fn rv32ui_v_sub() { assert_eq!(1, instruction_test("rv32ui-v-sub")); }

#[test]
fn rv32ui_v_sw() { assert_eq!(1, instruction_test("rv32ui-v-sw")); }

#[test]
fn rv32ui_v_xor() { assert_eq!(1, instruction_test("rv32ui-v-xor")); }

#[test]
fn rv32ui_v_xori() { assert_eq!(1, instruction_test("rv32ui-v-xori")); }

//***********************************************************************
// rv64ui (RV64 user-level, integer only), virtual memory is disable
//***********************************************************************
#[test]
fn rv64ui_p_add() { assert_eq!(1, instruction_test("rv64ui-p-add")); }

#[test]
fn rv64ui_p_addi() { assert_eq!(1, instruction_test("rv64ui-p-addi")); }

#[test]
fn rv64ui_p_addiw() { assert_eq!(1, instruction_test("rv64ui-p-addiw")); }

#[test]
fn rv64ui_p_addw() { assert_eq!(1, instruction_test("rv64ui-p-addw")); }

#[test]
fn rv64ui_p_and() { assert_eq!(1, instruction_test("rv64ui-p-and")); }

#[test]
fn rv64ui_p_andi() { assert_eq!(1, instruction_test("rv64ui-p-andi")); }

#[test]
fn rv64ui_p_auipc() { assert_eq!(1, instruction_test("rv64ui-p-auipc")); }

#[test]
fn rv64ui_p_beq() { assert_eq!(1, instruction_test("rv64ui-p-beq")); }

#[test]
fn rv64ui_p_bge() { assert_eq!(1, instruction_test("rv64ui-p-bge")); }

#[test]
fn rv64ui_p_bgeu() { assert_eq!(1, instruction_test("rv64ui-p-bgeu")); }

#[test]
fn rv64ui_p_blt() { assert_eq!(1, instruction_test("rv64ui-p-blt")); }

#[test]
fn rv64ui_p_bltu() { assert_eq!(1, instruction_test("rv64ui-p-bltu")); }

#[test]
fn rv64ui_p_bne() { assert_eq!(1, instruction_test("rv64ui-p-bne")); }

#[test]
fn rv64ui_p_fence_i() { assert_eq!(1, instruction_test("rv64ui-p-fence_i")); }

#[test]
fn rv64ui_p_jal() { assert_eq!(1, instruction_test("rv64ui-p-jal")); }

#[test]
fn rv64ui_p_jalr() { assert_eq!(1, instruction_test("rv64ui-p-jalr")); }

#[test]
fn rv64ui_p_lb() { assert_eq!(1, instruction_test("rv64ui-p-lb")); }

#[test]
fn rv64ui_p_lbu() { assert_eq!(1, instruction_test("rv64ui-p-lbu")); }

#[test]
fn rv64ui_p_ld() { assert_eq!(1, instruction_test("rv64ui-p-ld")); }

#[test]
fn rv64ui_p_lh() { assert_eq!(1, instruction_test("rv64ui-p-lh")); }

#[test]
fn rv64ui_p_lhu() { assert_eq!(1, instruction_test("rv64ui-p-lhu")); }

#[test]
fn rv64ui_p_lui() { assert_eq!(1, instruction_test("rv64ui-p-lui")); }

#[test]
fn rv64ui_p_lw() { assert_eq!(1, instruction_test("rv64ui-p-lw")); }

#[test]
fn rv64ui_p_lwu() { assert_eq!(1, instruction_test("rv64ui-p-lwu")); }

#[test]
fn rv64ui_p_or() { assert_eq!(1, instruction_test("rv64ui-p-or")); }

#[test]
fn rv64ui_p_ori() { assert_eq!(1, instruction_test("rv64ui-p-ori")); }

#[test]
fn rv64ui_p_sb() { assert_eq!(1, instruction_test("rv64ui-p-sb")); }

#[test]
fn rv64ui_p_sd() { assert_eq!(1, instruction_test("rv64ui-p-sd")); }

#[test]
fn rv64ui_p_sh() { assert_eq!(1, instruction_test("rv64ui-p-sh")); }

#[test]
fn rv64ui_p_simple() { assert_eq!(1, instruction_test("rv64ui-p-simple")); }

#[test]
fn rv64ui_p_sll() { assert_eq!(1, instruction_test("rv64ui-p-sll")); }

#[test]
fn rv64ui_p_slli() { assert_eq!(1, instruction_test("rv64ui-p-slli")); }

#[test]
fn rv64ui_p_slliw() { assert_eq!(1, instruction_test("rv64ui-p-slliw")); }

#[test]
fn rv64ui_p_sllw() { assert_eq!(1, instruction_test("rv64ui-p-sllw")); }

#[test]
fn rv64ui_p_slt() { assert_eq!(1, instruction_test("rv64ui-p-slt")); }

#[test]
fn rv64ui_p_slti() { assert_eq!(1, instruction_test("rv64ui-p-slti")); }

#[test]
fn rv64ui_p_sltiu() { assert_eq!(1, instruction_test("rv64ui-p-sltiu")); }

#[test]
fn rv64ui_p_sltu() { assert_eq!(1, instruction_test("rv64ui-p-sltu")); }

#[test]
fn rv64ui_p_sra() { assert_eq!(1, instruction_test("rv64ui-p-sra")); }

#[test]
fn rv64ui_p_srai() { assert_eq!(1, instruction_test("rv64ui-p-srai")); }

#[test]
fn rv64ui_p_sraiw() { assert_eq!(1, instruction_test("rv64ui-p-sraiw")); }

#[test]
fn rv64ui_p_sraw() { assert_eq!(1, instruction_test("rv64ui-p-sraw")); }

#[test]
fn rv64ui_p_srl() { assert_eq!(1, instruction_test("rv64ui-p-srl")); }

#[test]
fn rv64ui_p_srli() { assert_eq!(1, instruction_test("rv64ui-p-srli")); }

#[test]
fn rv64ui_p_srliw() { assert_eq!(1, instruction_test("rv64ui-p-srliw")); }

#[test]
fn rv64ui_p_srlw() { assert_eq!(1, instruction_test("rv64ui-p-srlw")); }

#[test]
fn rv64ui_p_sub() { assert_eq!(1, instruction_test("rv64ui-p-sub")); }

#[test]
fn rv64ui_p_subw() { assert_eq!(1, instruction_test("rv64ui-p-subw")); }

#[test]
fn rv64ui_p_sw() { assert_eq!(1, instruction_test("rv64ui-p-sw")); }

#[test]
fn rv64ui_p_xor() { assert_eq!(1, instruction_test("rv64ui-p-xor")); }

#[test]
fn rv64ui_p_xori() { assert_eq!(1, instruction_test("rv64ui-p-xori")); }

//***********************************************************************
// rv64ui (RV64 user-level, integer only), virtual memory is enable
//***********************************************************************
#[test]
fn rv64ui_v_add() { assert_eq!(1, instruction_test("rv64ui-v-add")); }

#[test]
fn rv64ui_v_addi() { assert_eq!(1, instruction_test("rv64ui-v-addi")); }

#[test]
fn rv64ui_v_addiw() { assert_eq!(1, instruction_test("rv64ui-v-addiw")); }

#[test]
fn rv64ui_v_addw() { assert_eq!(1, instruction_test("rv64ui-v-addw")); }

#[test]
fn rv64ui_v_and() { assert_eq!(1, instruction_test("rv64ui-v-and")); }

#[test]
fn rv64ui_v_andi() { assert_eq!(1, instruction_test("rv64ui-v-andi")); }

#[test]
fn rv64ui_v_auipc() { assert_eq!(1, instruction_test("rv64ui-v-auipc")); }

#[test]
fn rv64ui_v_beq() { assert_eq!(1, instruction_test("rv64ui-v-beq")); }

#[test]
fn rv64ui_v_bge() { assert_eq!(1, instruction_test("rv64ui-v-bge")); }

#[test]
fn rv64ui_v_bgeu() { assert_eq!(1, instruction_test("rv64ui-v-bgeu")); }

#[test]
fn rv64ui_v_blt() { assert_eq!(1, instruction_test("rv64ui-v-blt")); }

#[test]
fn rv64ui_v_bltu() { assert_eq!(1, instruction_test("rv64ui-v-bltu")); }

#[test]
fn rv64ui_v_bne() { assert_eq!(1, instruction_test("rv64ui-v-bne")); }

#[test]
fn rv64ui_v_fence_i() { assert_eq!(1, instruction_test("rv64ui-v-fence_i")); }

#[test]
fn rv64ui_v_jal() { assert_eq!(1, instruction_test("rv64ui-v-jal")); }

#[test]
fn rv64ui_v_jalr() { assert_eq!(1, instruction_test("rv64ui-v-jalr")); }

#[test]
fn rv64ui_v_lb() { assert_eq!(1, instruction_test("rv64ui-v-lb")); }

#[test]
fn rv64ui_v_lbu() { assert_eq!(1, instruction_test("rv64ui-v-lbu")); }

#[test]
fn rv64ui_v_ld() { assert_eq!(1, instruction_test("rv64ui-v-ld")); }

#[test]
fn rv64ui_v_lh() { assert_eq!(1, instruction_test("rv64ui-v-lh")); }

#[test]
fn rv64ui_v_lhu() { assert_eq!(1, instruction_test("rv64ui-v-lhu")); }

#[test]
fn rv64ui_v_lui() { assert_eq!(1, instruction_test("rv64ui-v-lui")); }

#[test]
fn rv64ui_v_lw() { assert_eq!(1, instruction_test("rv64ui-v-lw")); }

#[test]
fn rv64ui_v_lwu() { assert_eq!(1, instruction_test("rv64ui-v-lwu")); }

#[test]
fn rv64ui_v_or() { assert_eq!(1, instruction_test("rv64ui-v-or")); }

#[test]
fn rv64ui_v_ori() { assert_eq!(1, instruction_test("rv64ui-v-ori")); }

#[test]
fn rv64ui_v_sb() { assert_eq!(1, instruction_test("rv64ui-v-sb")); }

#[test]
fn rv64ui_v_sd() { assert_eq!(1, instruction_test("rv64ui-v-sd")); }

#[test]
fn rv64ui_v_sh() { assert_eq!(1, instruction_test("rv64ui-v-sh")); }

#[test]
fn rv64ui_v_simple() { assert_eq!(1, instruction_test("rv64ui-v-simple")); }

#[test]
fn rv64ui_v_sll() { assert_eq!(1, instruction_test("rv64ui-v-sll")); }

#[test]
fn rv64ui_v_slli() { assert_eq!(1, instruction_test("rv64ui-v-slli")); }

#[test]
fn rv64ui_v_slliw() { assert_eq!(1, instruction_test("rv64ui-v-slliw")); }

#[test]
fn rv64ui_v_sllw() { assert_eq!(1, instruction_test("rv64ui-v-sllw")); }

#[test]
fn rv64ui_v_slt() { assert_eq!(1, instruction_test("rv64ui-v-slt")); }

#[test]
fn rv64ui_v_slti() { assert_eq!(1, instruction_test("rv64ui-v-slti")); }

#[test]
fn rv64ui_v_sltiu() { assert_eq!(1, instruction_test("rv64ui-v-sltiu")); }

#[test]
fn rv64ui_v_sltu() { assert_eq!(1, instruction_test("rv64ui-v-sltu")); }

#[test]
fn rv64ui_v_sra() { assert_eq!(1, instruction_test("rv64ui-v-sra")); }

#[test]
fn rv64ui_v_srai() { assert_eq!(1, instruction_test("rv64ui-v-srai")); }

#[test]
fn rv64ui_v_sraiw() { assert_eq!(1, instruction_test("rv64ui-v-sraiw")); }

#[test]
fn rv64ui_v_sraw() { assert_eq!(1, instruction_test("rv64ui-v-sraw")); }

#[test]
fn rv64ui_v_srl() { assert_eq!(1, instruction_test("rv64ui-v-srl")); }

#[test]
fn rv64ui_v_srli() { assert_eq!(1, instruction_test("rv64ui-v-srli")); }

#[test]
fn rv64ui_v_srliw() { assert_eq!(1, instruction_test("rv64ui-v-srliw")); }

#[test]
fn rv64ui_v_srlw() { assert_eq!(1, instruction_test("rv64ui-v-srlw")); }

#[test]
fn rv64ui_v_sub() { assert_eq!(1, instruction_test("rv64ui-v-sub")); }

#[test]
fn rv64ui_v_subw() { assert_eq!(1, instruction_test("rv64ui-v-subw")); }

#[test]
fn rv64ui_v_sw() { assert_eq!(1, instruction_test("rv64ui-v-sw")); }

#[test]
fn rv64ui_v_xor() { assert_eq!(1, instruction_test("rv64ui-v-xor")); }

#[test]
fn rv64ui_v_xori() { assert_eq!(1, instruction_test("rv64ui-v-xori")); }

//***********************************************************************
// rv32um (RV32 user-level, Multiplication only), virtual memory is disable
//***********************************************************************
#[test]
fn rv32um_p_div() { assert_eq!(1, instruction_test("rv32um-p-div")); }

#[test]
fn rv32um_p_divu() { assert_eq!(1, instruction_test("rv32um-p-divu")); }

#[test]
fn rv32um_p_mul() { assert_eq!(1, instruction_test("rv32um-p-mul")); }

#[test]
fn rv32um_p_mulh() { assert_eq!(1, instruction_test("rv32um-p-mulh")); }

#[test]
fn rv32um_p_mulhsu() { assert_eq!(1, instruction_test("rv32um-p-mulhsu")); }

#[test]
fn rv32um_p_mulhu() { assert_eq!(1, instruction_test("rv32um-p-mulhu")); }

#[test]
fn rv32um_p_rem() { assert_eq!(1, instruction_test("rv32um-p-rem")); }

#[test]
fn rv32um_p_remu() { assert_eq!(1, instruction_test("rv32um-p-remu")); }

//***********************************************************************
// rv32um (RV32 user-level, Multiplication only), virtual memory is enable
//***********************************************************************
#[test]
fn rv32um_v_div() { assert_eq!(1, instruction_test("rv32um-v-div")); }

#[test]
fn rv32um_v_divu() { assert_eq!(1, instruction_test("rv32um-v-divu")); }

#[test]
fn rv32um_v_mul() { assert_eq!(1, instruction_test("rv32um-v-mul")); }

#[test]
fn rv32um_v_mulh() { assert_eq!(1, instruction_test("rv32um-v-mulh")); }

#[test]
fn rv32um_v_mulhsu() { assert_eq!(1, instruction_test("rv32um-v-mulhsu")); }

#[test]
fn rv32um_v_mulhu() { assert_eq!(1, instruction_test("rv32um-v-mulhu")); }

#[test]
fn rv32um_v_rem() { assert_eq!(1, instruction_test("rv32um-v-rem")); }

#[test]
fn rv32um_v_remu() { assert_eq!(1, instruction_test("rv32um-v-remu")); }

//***********************************************************************
// rv64um (RV64M user-level, Multiplication only), virtual memory is disable
//***********************************************************************
#[test]
fn rv64um_p_div() { assert_eq!(1, instruction_test("rv64um-p-div")); }

#[test]
fn rv64um_p_divu() { assert_eq!(1, instruction_test("rv64um-p-divu")); }

#[test]
fn rv64um_p_divuw() { assert_eq!(1, instruction_test("rv64um-p-divuw")); }

#[test]
fn rv64um_p_divw() { assert_eq!(1, instruction_test("rv64um-p-divw")); }

#[test]
fn rv64um_p_mul() { assert_eq!(1, instruction_test("rv64um-p-mul")); }

#[test]
fn rv64um_p_mulh() { assert_eq!(1, instruction_test("rv64um-p-mulh")); }

#[test]
fn rv64um_p_mulhsu() { assert_eq!(1, instruction_test("rv64um-p-mulhsu")); }

#[test]
fn rv64um_p_mulhu() { assert_eq!(1, instruction_test("rv64um-p-mulhu")); }

#[test]
fn rv64um_p_mulw() { assert_eq!(1, instruction_test("rv64um-p-mulw")); }

#[test]
fn rv64um_p_rem() { assert_eq!(1, instruction_test("rv64um-p-rem")); }

#[test]
fn rv64um_p_remu() { assert_eq!(1, instruction_test("rv64um-p-remu")); }

#[test]
fn rv64um_p_remuw() { assert_eq!(1, instruction_test("rv64um-p-remuw")); }

#[test]
fn rv64um_p_remw() { assert_eq!(1, instruction_test("rv64um-p-remw")); }

//***********************************************************************
// rv64um (RV64M user-level, Multiplication only), virtual memory is enable
//***********************************************************************
#[test]
fn rv64um_v_div() { assert_eq!(1, instruction_test("rv64um-v-div")); }

#[test]
fn rv64um_v_divu() { assert_eq!(1, instruction_test("rv64um-v-divu")); }

#[test]
fn rv64um_v_divuw() { assert_eq!(1, instruction_test("rv64um-v-divuw")); }

#[test]
fn rv64um_v_divw() { assert_eq!(1, instruction_test("rv64um-v-divw")); }

#[test]
fn rv64um_v_mul() { assert_eq!(1, instruction_test("rv64um-v-mul")); }

#[test]
fn rv64um_v_mulh() { assert_eq!(1, instruction_test("rv64um-v-mulh")); }

#[test]
fn rv64um_v_mulhsu() { assert_eq!(1, instruction_test("rv64um-v-mulhsu")); }

#[test]
fn rv64um_v_mulhu() { assert_eq!(1, instruction_test("rv64um-v-mulhu")); }

#[test]
fn rv64um_v_mulw() { assert_eq!(1, instruction_test("rv64um-v-mulw")); }

#[test]
fn rv64um_v_rem() { assert_eq!(1, instruction_test("rv64um-v-rem")); }

#[test]
fn rv64um_v_remu() { assert_eq!(1, instruction_test("rv64um-v-remu")); }

#[test]
fn rv64um_v_remuw() { assert_eq!(1, instruction_test("rv64um-v-remuw")); }

#[test]
fn rv64um_v_remw() { assert_eq!(1, instruction_test("rv64um-v-remw")); }

//***********************************************************************
// rv32ua (RV32A user-level, Atomic only), virtual memory is disable
//***********************************************************************
#[test]
fn rv32ua_p_amoadd_w() { assert_eq!(1, instruction_test("rv32ua-p-amoadd_w")); }

#[test]
fn rv32ua_p_amoand_w() { assert_eq!(1, instruction_test("rv32ua-p-amoand_w")); }

#[test]
fn rv32ua_p_amomax_w() { assert_eq!(1, instruction_test("rv32ua-p-amomax_w")); }

#[test]
fn rv32ua_p_amomaxu_w() { assert_eq!(1, instruction_test("rv32ua-p-amomaxu_w")); }

#[test]
fn rv32ua_p_amomin_w() { assert_eq!(1, instruction_test("rv32ua-p-amomin_w")); }

#[test]
fn rv32ua_p_amominu_w() { assert_eq!(1, instruction_test("rv32ua-p-amominu_w")); }

#[test]
fn rv32ua_p_amoor_w() { assert_eq!(1, instruction_test("rv32ua-p-amoor_w")); }

#[test]
fn rv32ua_p_amoswap_w() { assert_eq!(1, instruction_test("rv32ua-p-amoswap_w")); }

#[test]
fn rv32ua_p_amoxor_w() { assert_eq!(1, instruction_test("rv32ua-p-amoxor_w")); }

#[test]
fn rv32ua_p_lrsc() { assert_eq!(1, instruction_test("rv32ua-p-lrsc")); }


//***********************************************************************
// rv32ua (RV32A user-level, Atomic only), virtual memory is enable
//***********************************************************************
#[test]
fn rv32ua_v_amoadd_w() { assert_eq!(1, instruction_test("rv32ua-v-amoadd_w")); }

#[test]
fn rv32ua_v_amoand_w() { assert_eq!(1, instruction_test("rv32ua-v-amoand_w")); }

#[test]
fn rv32ua_v_amomax_w() { assert_eq!(1, instruction_test("rv32ua-v-amomax_w")); }

#[test]
fn rv32ua_v_amomaxu_w() { assert_eq!(1, instruction_test("rv32ua-v-amomaxu_w")); }

#[test]
fn rv32ua_v_amomin_w() { assert_eq!(1, instruction_test("rv32ua-v-amomin_w")); }

#[test]
fn rv32ua_v_amominu_w() { assert_eq!(1, instruction_test("rv32ua-v-amominu_w")); }

#[test]
fn rv32ua_v_amoor_w() { assert_eq!(1, instruction_test("rv32ua-v-amoor_w")); }

#[test]
fn rv32ua_v_amoswap_w() { assert_eq!(1, instruction_test("rv32ua-v-amoswap_w")); }

#[test]
fn rv32ua_v_amoxor_w() { assert_eq!(1, instruction_test("rv32ua-v-amoxor_w")); }

#[test]
fn rv32ua_v_lrsc() { assert_eq!(1, instruction_test("rv32ua-v-lrsc")); }

//***********************************************************************
// rv64ua (RV64A user-level, Atomic only), virtual memory is disable
//***********************************************************************
#[test]
fn rv64ua_p_amoadd_w() { assert_eq!(1, instruction_test("rv64ua-p-amoadd_w")); }

#[test]
fn rv64ua_p_amoadd_d() { assert_eq!(1, instruction_test("rv64ua-p-amoadd_d")); }

#[test]
fn rv64ua_p_amoand_w() { assert_eq!(1, instruction_test("rv64ua-p-amoand_w")); }

#[test]
fn rv64ua_p_amoand_d() { assert_eq!(1, instruction_test("rv64ua-p-amoand_d")); }

#[test]
fn rv64ua_p_amomax_w() { assert_eq!(1, instruction_test("rv64ua-p-amomax_w")); }

#[test]
fn rv64ua_p_amomax_d() { assert_eq!(1, instruction_test("rv64ua-p-amomax_d")); }

#[test]
fn rv64ua_p_amomaxu_w() { assert_eq!(1, instruction_test("rv64ua-p-amomaxu_w")); }

#[test]
fn rv64ua_p_amomaxu_d() { assert_eq!(1, instruction_test("rv64ua-p-amomaxu_d")); }

#[test]
fn rv64ua_p_amomin_w() { assert_eq!(1, instruction_test("rv64ua-p-amomin_w")); }

#[test]
fn rv64ua_p_amomin_d() { assert_eq!(1, instruction_test("rv64ua-p-amomin_d")); }

#[test]
fn rv64ua_p_amominu_w() { assert_eq!(1, instruction_test("rv64ua-p-amominu_w")); }

#[test]
fn rv64ua_p_amominu_d() { assert_eq!(1, instruction_test("rv64ua-p-amominu_d")); }

#[test]
fn rv64ua_p_amoor_w() { assert_eq!(1, instruction_test("rv64ua-p-amoor_w")); }

#[test]
fn rv64ua_p_amoor_d() { assert_eq!(1, instruction_test("rv64ua-p-amoor_d")); }

#[test]
fn rv64ua_p_amoswap_w() { assert_eq!(1, instruction_test("rv64ua-p-amoswap_w")); }

#[test]
fn rv64ua_p_amoswap_d() { assert_eq!(1, instruction_test("rv64ua-p-amoswap_d")); }

#[test]
fn rv64ua_p_amoxor_w() { assert_eq!(1, instruction_test("rv64ua-p-amoxor_w")); }

#[test]
fn rv64ua_p_amoxor_d() { assert_eq!(1, instruction_test("rv64ua-p-amoxor_d")); }

#[test]
fn rv64ua_p_lrsc() { assert_eq!(1, instruction_test("rv64ua-p-lrsc")); }

//***********************************************************************
// rv64ua (RV64A user-level, Atomic only), virtual memory is enable
//***********************************************************************
#[test]
fn rv64ua_v_amoadd_w() { assert_eq!(1, instruction_test("rv64ua-v-amoadd_w")); }

#[test]
fn rv64ua_v_amoadd_d() { assert_eq!(1, instruction_test("rv64ua-v-amoadd_d")); }

#[test]
fn rv64ua_v_amoand_w() { assert_eq!(1, instruction_test("rv64ua-v-amoand_w")); }

#[test]
fn rv64ua_v_amoand_d() { assert_eq!(1, instruction_test("rv64ua-v-amoand_d")); }

#[test]
fn rv64ua_v_amomax_w() { assert_eq!(1, instruction_test("rv64ua-v-amomax_w")); }

#[test]
fn rv64ua_v_amomax_d() { assert_eq!(1, instruction_test("rv64ua-v-amomax_d")); }

#[test]
fn rv64ua_v_amomaxu_w() { assert_eq!(1, instruction_test("rv64ua-v-amomaxu_w")); }

#[test]
fn rv64ua_v_amomaxu_d() { assert_eq!(1, instruction_test("rv64ua-v-amomaxu_d")); }

#[test]
fn rv64ua_v_amomin_w() { assert_eq!(1, instruction_test("rv64ua-v-amomin_w")); }

#[test]
fn rv64ua_v_amomin_d() { assert_eq!(1, instruction_test("rv64ua-v-amomin_d")); }

#[test]
fn rv64ua_v_amominu_w() { assert_eq!(1, instruction_test("rv64ua-v-amominu_w")); }

#[test]
fn rv64ua_v_amominu_d() { assert_eq!(1, instruction_test("rv64ua-v-amominu_d")); }

#[test]
fn rv64ua_v_amoor_w() { assert_eq!(1, instruction_test("rv64ua-v-amoor_w")); }

#[test]
fn rv64ua_v_amoor_d() { assert_eq!(1, instruction_test("rv64ua-v-amoor_d")); }

#[test]
fn rv64ua_v_amoswap_w() { assert_eq!(1, instruction_test("rv64ua-v-amoswap_w")); }

#[test]
fn rv64ua_v_amoswap_d() { assert_eq!(1, instruction_test("rv64ua-v-amoswap_d")); }

#[test]
fn rv64ua_v_amoxor_w() { assert_eq!(1, instruction_test("rv64ua-v-amoxor_w")); }

#[test]
fn rv64ua_v_amoxor_d() { assert_eq!(1, instruction_test("rv64ua-v-amoxor_d")); }

#[test]
fn rv64ua_v_lrsc() { assert_eq!(1, instruction_test("rv64ua-v-lrsc")); }

//***********************************************************************
// rv32uc (RV32C user-level), virtual memory is disabled
//***********************************************************************
#[test]
fn rv32uc_p_rvc() { assert_eq!(1, instruction_test("rv32uc-p-rvc")); }

//***********************************************************************
// rv32uc (RV32C user-level), virtual memory is enabled
//***********************************************************************
#[test]
fn rv32uc_v_rvc() { assert_eq!(1, instruction_test("rv32uc-v-rvc")); }

//***********************************************************************
// rv64uc (RV64C user-level), virtual memory is disabled
//***********************************************************************
#[test]
fn rv64uc_p_rvc() { assert_eq!(1, instruction_test("rv64uc-p-rvc")); }

//***********************************************************************
// rv64uc (RV32C user-level), virtual memory is enabled
//***********************************************************************
#[test]
fn rv64uc_v_rvc() { assert_eq!(1, instruction_test("rv64uc-v-rvc")); }

//***********************************************************************
// rv32/64si (supervisor-level), integer only
//***********************************************************************
#[test]
fn rv32si_p_csr() { assert_eq!(1, instruction_test("rv32si-p-csr")); }

//#[test]
//fn rv32si_p_dirty() { assert_eq!(1, instruction_test("rv32si-p-dirty")); }

//#[test]
//fn rv32si_p_ma_fetch() { assert_eq!(1, instruction_test("rv32si-p-ma_fetch")); }

//#[test]
//fn rv32si_p_sbreak() { assert_eq!(1, instruction_test("rv32si-p-sbreak")); }

#[test]
fn rv32si_p_scall() { assert_eq!(1, instruction_test("rv32si-p-scall")); }

//#[test]
//fn rv32si_p_wfi() { assert_eq!(1, instruction_test("rv32si-p-wfi")); }

//#[test]
//fn rv64si_p_csr() { assert_eq!(1, instruction_test("rv64si-p-csr")); }

#[test]
fn rv64si_p_icache() { assert_eq!(1, instruction_test("rv64si-p-icache-alias")); }

//#[test]
//fn rv64si_p_sbreak() { assert_eq!(1, instruction_test("rv64si-p-sbreak")); }

//#[test]
//fn rv64si_p_wfi() { assert_eq!(1, instruction_test("rv64si-p-wfi")); }

//#[test]
//fn rv64si_p_dirty() { assert_eq!(1, instruction_test("rv64si-p-dirty")); }

//#[test]
//fn rv64si_p_ma_fetch() { assert_eq!(1, instruction_test("rv64si-p-ma_fetch")); }

#[test]
fn rv64si_p_scall() { assert_eq!(1, instruction_test("rv64si-p-scall")); }

//***********************************************************************
// rv32/64mi (machine-level), integer only
//***********************************************************************
//#[test]
//fn rv32mi_p_breakpoint() { assert_eq!(1, instruction_test("rv32mi-p-breakpoint")); }

//#[test]
//fn rv32mi_p_illegal() { assert_eq!(1, instruction_test("rv32mi-p-illegal")); }

//#[test]
//fn rv32mi_p_ma_fetch() { assert_eq!(1, instruction_test("rv32mi-p-ma_fetch")); }

#[test]
fn rv32mi_p_sbreak() { assert_eq!(1, instruction_test("rv32mi-p-sbreak")); }

//#[test]
//fn rv32mi_p_shamt() { assert_eq!(1, instruction_test("rv32mi-p-shamt")); }

#[test]
fn rv32mi_p_csr() { assert_eq!(1, instruction_test("rv32mi-p-csr")); }

#[test]
fn rv32mi_p_ma_addr() { assert_eq!(1, instruction_test("rv32mi-p-ma_addr")); }

//#[test]
//fn rv32mi_p_mcsr() { assert_eq!(1, instruction_test("rv32mi-p-mcsr")); }

#[test]
fn rv32mi_p_scall() { assert_eq!(1, instruction_test("rv32mi-p-scall")); }

//#[test]
//fn rv64mi_p_access() { assert_eq!(1, instruction_test("rv64mi-p-access")); }

//#[test]
//fn rv64mi_p_csr() { assert_eq!(1, instruction_test("rv64mi-p-csr")); }

#[test]
fn rv64mi_p_ma_addr() { assert_eq!(1, instruction_test("rv64mi-p-ma_addr")); }

//#[test]
//fn rv64mi_p_mcsr() { assert_eq!(1, instruction_test("rv64mi-p-mcsr")); }

#[test]
fn rv64mi_p_scall() { assert_eq!(1, instruction_test("rv64mi-p-scall")); }

//#[test]
//fn rv64mi_p_breakpoint() { assert_eq!(1, instruction_test("rv64mi-p-breakpoint")); }

//#[test]
//fn rv64mi_p_illegal() { assert_eq!(1, instruction_test("rv64mi-p-illegal")); }

//#[test]
//fn rv64mi_p_ma_fetch() { assert_eq!(1, instruction_test("rv64mi-p-ma_fetch")); }

#[test]
fn rv64mi_p_sbreak() { assert_eq!(1, instruction_test("rv64mi-p-sbreak")); }

