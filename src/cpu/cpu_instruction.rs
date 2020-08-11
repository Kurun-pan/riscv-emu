use std::collections::HashMap;

use crate::cpu::cpu::{Cpu, Privilege, Xlen};
use crate::cpu::cpu_csr::*;
use crate::cpu::trap::*;

pub struct Opecode {
    pub operation: fn(cpu: &Cpu, addr: u64, word: u32) -> Result<&Instruction, ()>,
}

pub struct Instruction {
    pub mnemonic: &'static str,
    pub operation: fn(cpu: &mut Cpu, addr: u64, word: u32) -> Result<(), Trap>,
    pub disassemble: fn(cpu: &Cpu, mnemonic: &str, word: u32) -> String,
}

struct InstructionTypeB {
    rs1: u8,
    rs2: u8,
    imm: u64,
}

struct InstructionTypeR {
    rd: u8,
    rs1: u8,
    rs2: u8,
}

struct InstructionTypeI {
    rd: u8,
    rs1: u8,
    imm: i64,
}

struct InstructionTypeJ {
    rd: u8,
    imm: u64,
}

struct InstructionTypeS {
    rs1: u8,
    rs2: u8,
    imm: i64,
}

struct InstructionTypeU {
    rd: u8,
    imm: u64,
}

struct InstructionTypeCSR {
    rd: u8,
    rs1: u8,
    csr: u16,
}

lazy_static! {
    // ABI name
    static ref REGISTERS: HashMap<u8, &'static str> = {
        let mut m = HashMap::new();
        m.insert(0, "zero");  // Hard-wired zero
        m.insert(1, "ra");    // Return address
        m.insert(2, "sp");    // Stack pointer
        m.insert(3, "gp");    // Global pointer
        m.insert(4, "tp");    // Thread pointer
        m.insert(5, "t0");    // Temporary registers
        m.insert(6, "t1");    // Temporary registers
        m.insert(7, "t2");    // Temporary registers
        m.insert(8, "s0");    // Saved register/frame pointer
        m.insert(9, "s1");    // Saved register
        m.insert(10, "a0");   // Function arguments/return values
        m.insert(11, "a1");   // Function arguments/return values
        m.insert(12, "a2");   // Function arguments
        m.insert(13, "a3");   // Function arguments
        m.insert(14, "a4");   // Function arguments
        m.insert(15, "a5");   // Function arguments
        m.insert(16, "a6");   // Function arguments
        m.insert(17, "a7");   // Function arguments
        m.insert(18, "s2");   // Saved registers
        m.insert(19, "s3");   // Saved registers
        m.insert(20, "s4");   // Saved registers
        m.insert(21, "s5");   // Saved registers
        m.insert(22, "s6");   // Saved registers
        m.insert(23, "s7");   // Saved registers
        m.insert(24, "s8");   // Saved registers
        m.insert(25, "s9");   // Saved registers
        m.insert(26, "s10");  // Saved registers
        m.insert(27, "s11");  // Saved registers
        m.insert(28, "t3");   // Temporary registers
        m.insert(29, "t4");   // Temporary registers
        m.insert(30, "t5");   // Temporary registers
        m.insert(31, "t6");   // Temporary registers
        m
    };

    pub static ref OPECODES: HashMap<u8, Opecode> = {
        let mut m = HashMap::new();
        m.insert(0x03, Opecode {operation: opecode_03});
        m.insert(0x07, Opecode {operation: opecode_07});
        m.insert(0x0f, Opecode {operation: opecode_0f});
        m.insert(0x13, Opecode {operation: opecode_13});
        m.insert(0x17, Opecode {operation: opecode_17});
        m.insert(0x1b, Opecode {operation: opecode_1b});
        m.insert(0x23, Opecode {operation: opecode_23});
        m.insert(0x27, Opecode {operation: opecode_27});
        m.insert(0x33, Opecode {operation: opecode_33});
        m.insert(0x37, Opecode {operation: opecode_37});
        m.insert(0x3b, Opecode {operation: opecode_3b});
        m.insert(0x63, Opecode {operation: opecode_63});
        m.insert(0x67, Opecode {operation: opecode_67});
        m.insert(0x6F, Opecode {operation: opecode_6f});
        m.insert(0x73, Opecode {operation: opecode_73});
        m
    };

    // RV32I/RV64I Load Instructions.
    pub static ref INSTRUCTIONS_GROUP03: HashMap<u8, Instruction> = {
        let mut m = HashMap::new();
        m.insert(0, Instruction{
            mnemonic: "lb",
            operation: lb,
            disassemble: disassemble_i,
        });
        m.insert(1, Instruction{
            mnemonic: "lh",
            operation: lh,
            disassemble: disassemble_i,
        });
        m.insert(2, Instruction{
            mnemonic: "lw",
            operation: lw,
            disassemble: disassemble_i,
        });
        m.insert(3, Instruction{
            mnemonic: "ld",
            operation: ld,
            disassemble: disassemble_i,
        });
        m.insert(4, Instruction{
            mnemonic: "lbu",
            operation: lbu,
            disassemble: disassemble_i,
        });
        m.insert(5, Instruction{
            mnemonic: "lhu",
            operation: lhu,
            disassemble: disassemble_i,
        });
        m.insert(6, Instruction{
            mnemonic: "lwu",
            operation: lwu,
            disassemble: disassemble_i,
        });
        m
    };

    // RV32F/RV64F Single/Double-Precision Load Instructions.
    pub static ref INSTRUCTIONS_GROUP07: HashMap<u8, Instruction> = {
        let mut m = HashMap::new();
        m.insert(0, Instruction{
            mnemonic: "flw",
            operation: flw,
            disassemble: disassemble_precision_load,
        });
        m.insert(3, Instruction{
            mnemonic: "fld",
            operation: fld,
            disassemble: disassemble_precision_load,
        });
        /* TODO: support 128-bit
        m.insert(4, Instruction{
            mnemonic: "flq",
            operation: flq,
            disassemble: ,
        });
        */
        m
    };
    pub static ref INSTRUCTIONS_GROUP27: HashMap<u8, Instruction> = {
        let mut m = HashMap::new();
        m.insert(2, Instruction{
            mnemonic: "fsw",
            operation: fsw,
            disassemble: disassemble_store,
        });
        m.insert(3, Instruction{
            mnemonic: "fsd",
            operation: fsd,
            disassemble: disassemble_store,
        });
        m
    };

    // Memory Ordering Instructions.
    pub static ref INSTRUCTIONS_GROUP0F: HashMap<u8, Instruction> = {
        let mut m = HashMap::new();
        m.insert(0, Instruction{
            mnemonic: "fence",
            operation: fence,
            disassemble: disassemble_fence,
        });
        m.insert(1, Instruction{
            mnemonic: "fence.i",
            operation: fence,
            disassemble: disassemble_fence,
        });
        m
    };

    // RV32I/RV64I Integer Register-Immediate Instructions.
    pub static ref INSTRUCTIONS_GROUP13: HashMap<u8, Instruction> = {
        let mut m = HashMap::new();
        m.insert(0, Instruction{
            mnemonic: "addi",
            operation: addi,
            disassemble: disassemble_computation,
        });
        m.insert(1, Instruction{
            mnemonic: "slli",
            operation: slli,
            disassemble: disassemble_computation_shamt,
        });
        m.insert(2, Instruction{
            mnemonic: "slti",
            operation: slti,
            disassemble: disassemble_computation,
        });
        m.insert(3, Instruction{
            mnemonic: "sltiu",
            operation: sltiu,
            disassemble: disassemble_computation,
        });
        m.insert(4, Instruction{
            mnemonic: "xori",
            operation: xori,
            disassemble: disassemble_computation,
        });
        m.insert(6, Instruction{
            mnemonic: "ori",
            operation: ori,
            disassemble: disassemble_computation,
        });
        m.insert(7, Instruction{
            mnemonic: "andi",
            operation: andi,
            disassemble: disassemble_computation,
        });
        m
    };
    pub static ref INSTRUCTIONS_GROUP13_SUB: HashMap<(u8, u8), Instruction> = {
        let mut m = HashMap::new();
        m.insert((0, 5), Instruction{
            mnemonic: "srli",
            operation: srli,
            disassemble: disassemble_computation_shamt,
        });
        m.insert((32, 5), Instruction{
            mnemonic: "srai",
            operation: srai,
            disassemble: disassemble_computation_shamt,
        });
        m
    };

    pub static ref INSTRUCTIONS_GROUP17: HashMap<u8, Instruction> = {
        let mut m = HashMap::new();
        m.insert(0, Instruction{
            mnemonic: "auipc",
            operation: auipc,
            disassemble: disassemble_u,
        });
        m
    };

    // RV64I Integer Register-Immediate Instructions.
    pub static ref INSTRUCTIONS_GROUP1B: HashMap<u8, Instruction> = {
        let mut m = HashMap::new();
        m.insert(0, Instruction{
            mnemonic: "addiw",
            operation: addiw,
            disassemble: disassemble_computation,
        });
        m.insert(1, Instruction{
            mnemonic: "slliw",
            operation: slliw,
            disassemble: disassemble_computation,
        });
        m
    };

    pub static ref INSTRUCTIONS_GROUP1B_SUB: HashMap<(u8, u8), Instruction> = {
        let mut m = HashMap::new();
        m.insert((0, 5), Instruction{
            mnemonic: "srliw",
            operation: srliw,
            disassemble: disassemble_computation_shamt,
        });
        m.insert((32, 5), Instruction{
            mnemonic: "sraiw",
            operation: sraiw,
            disassemble: disassemble_computation_shamt,
        });
        m
    };
    // RV32I/RV64I Store Instructions.
    pub static ref INSTRUCTIONS_GROUP23: HashMap<u8, Instruction> = {
        let mut m = HashMap::new();
        m.insert(0, Instruction{
            mnemonic: "sb",
            operation: sb,
            disassemble: disassemble_store,
        });
        m.insert(1, Instruction{
            mnemonic: "sh",
            operation: sh,
            disassemble: disassemble_store,
        });
        m.insert(2, Instruction{
            mnemonic: "sw",
            operation: sw,
            disassemble: disassemble_store,
        });
        m.insert(3, Instruction{
            mnemonic: "sd",
            operation: sd,
            disassemble: disassemble_store,
        });
        m
    };

    pub static ref INSTRUCTIONS_GROUP33: HashMap<(u8, u8), Instruction> = {
        let mut m = HashMap::new();
        m.insert((0, 0), Instruction{
            mnemonic: "add",
            operation: add,
            disassemble: disassemble_r,
        });
        m.insert((1, 0), Instruction{
            mnemonic: "mul",
            operation: mul,
            disassemble: disassemble_r,
        });
        m.insert((32, 0), Instruction{
            mnemonic: "sub",
            operation: sub,
            disassemble: disassemble_r,
        });
        m.insert((0, 1), Instruction{
            mnemonic: "sll",
            operation: sll,
            disassemble: disassemble_r,
        });
        m.insert((1, 1), Instruction{
            mnemonic: "mulh",
            operation: mulh,
            disassemble: disassemble_r,
        });
        m.insert((0, 2), Instruction{
            mnemonic: "slt",
            operation: slt,
            disassemble: disassemble_r,
        });
        m.insert((1, 2), Instruction{
            mnemonic: "mulhsu",
            operation: mulhsu,
            disassemble: disassemble_r,
        });
        m.insert((0, 3), Instruction{
            mnemonic: "sltu",
            operation: sltu,
            disassemble: disassemble_r,
        });
        m.insert((1, 3), Instruction{
            mnemonic: "mulhu",
            operation: mulhu,
            disassemble: disassemble_r,
        });
        m.insert((0, 4), Instruction{
            mnemonic: "xor",
            operation: xor,
            disassemble: disassemble_r,
        });
        m.insert((1, 4), Instruction{
            mnemonic: "div",
            operation: div,
            disassemble: disassemble_r,
        });
        m.insert((0, 5), Instruction{
            mnemonic: "srl",
            operation: srl,
            disassemble: disassemble_r,
        });
        m.insert((1, 5), Instruction{
            mnemonic: "divu",
            operation: divu,
            disassemble: disassemble_r,
        });
        m.insert((32, 5), Instruction{
            mnemonic: "sra",
            operation: sra,
            disassemble: disassemble_r,
        });
        m.insert((0, 6), Instruction{
            mnemonic: "or",
            operation: or,
            disassemble: disassemble_r,
        });
        m.insert((1, 6), Instruction{
            mnemonic: "rem",
            operation: rem,
            disassemble: disassemble_r,
        });
        m.insert((0, 7), Instruction{
            mnemonic: "and",
            operation: and,
            disassemble: disassemble_r,
        });
        m.insert((1, 7), Instruction{
            mnemonic: "remu",
            operation: remu,
            disassemble: disassemble_r,
        });
        m
    };

    pub static ref INSTRUCTIONS_GROUP3B: HashMap<(u8, u8), Instruction> = {
        let mut m = HashMap::new();
        m.insert((0, 0), Instruction{
            mnemonic: "addw",
            operation: addw,
            disassemble: disassemble_r,
        });
        m.insert((32, 0), Instruction{
            mnemonic: "subw",
            operation: subw,
            disassemble: disassemble_r,
        });
        m.insert((0, 1), Instruction{
            mnemonic: "sllw",
            operation: sllw,
            disassemble: disassemble_r,
        });
        m.insert((0, 5), Instruction{
            mnemonic: "srlw",
            operation: srlw,
            disassemble: disassemble_r,
        });
        m.insert((32, 5), Instruction{
            mnemonic: "sraw",
            operation: sraw,
            disassemble: disassemble_r,
        });
        m
    };

    // Conditional Branches.
    pub static ref INSTRUCTIONS_GROUP63: HashMap<u8, Instruction> = {
        let mut m = HashMap::new();
        m.insert(0, Instruction{
            mnemonic: "beq",
            operation: beq,
            disassemble: disassemble_b,
        });
        m.insert(1, Instruction{
            mnemonic: "bne",
            operation: bne,
            disassemble: disassemble_b,
        });
        m.insert(4, Instruction{
            mnemonic: "blt",
            operation: blt,
            disassemble: disassemble_b,
        });
        m.insert(5, Instruction{
            mnemonic: "bge",
            operation: bge,
            disassemble: disassemble_b,
        });
        m.insert(6, Instruction{
            mnemonic: "bltu",
            operation: bltu,
            disassemble: disassemble_b,
        });
        m.insert(7, Instruction{
            mnemonic: "bgeu",
            operation: bgeu,
            disassemble: disassemble_b,
        });
        m
    };

    // Control and Status Register (CSR) Instructions.
    pub static ref INSTRUCTIONS_GROUP73: HashMap<u8, Instruction> = {
        let mut m = HashMap::new();
        m.insert(1, Instruction{
            mnemonic: "csrrw",
            operation: csrrw,
            disassemble: disassemble_csr,
        });
        m.insert(2, Instruction{
            mnemonic: "csrrs",
            operation: csrrs,
            disassemble: disassemble_csr,
        });
        m.insert(3, Instruction{
            mnemonic: "csrrc",
            operation: csrrc,
            disassemble: disassemble_csr,
        });
        m.insert(5, Instruction{
            mnemonic: "csrrwi",
            operation: csrrwi,
            disassemble: disassemble_csr,
        });
        m.insert(6, Instruction{
            mnemonic: "csrrsi",
            operation: csrrsi,
            disassemble: disassemble_csr,
        });
        m.insert(7, Instruction{
            mnemonic: "csrrci",
            operation: csrrci,
            disassemble: disassemble_csr,
        });
        m
    };
    pub static ref INSTRUCTIONS_GROUP73_EXTEND: HashMap<u16, Instruction> = {
        let mut m = HashMap::new();
        m.insert(0x000, Instruction{
            mnemonic: "ecall",
            operation: ecall,
            disassemble: disassemble_mnemonic,
        });
        m.insert(0x001, Instruction{
            mnemonic: "ebreak",
            operation: ebreak,
            disassemble: disassemble_mnemonic,
        });
        m.insert(0x002, Instruction{
            mnemonic: "uret",
            operation: uret,
            disassemble: disassemble_mnemonic,
        });
        m.insert(0x102, Instruction{
            mnemonic: "sret",
            operation: sret,
            disassemble: disassemble_mnemonic,
        });
        m.insert(0x302, Instruction{
            mnemonic: "mret",
            operation: mret,
            disassemble: disassemble_mnemonic,
        });
        m.insert(0x105, Instruction{
            mnemonic: "wfi",
            operation: wfi,
            disassemble: disassemble_mnemonic,
        });
        m
    };
}

fn opecode_03(_cpu: &Cpu, _addr: u64, word: u32) -> Result<&Instruction, ()> {
    let funct3 = ((word & 0x00007000) >> 12) as u8;
    match INSTRUCTIONS_GROUP03.get(&funct3) {
        Some(instruction) => Ok(&instruction),
        None => panic!("Not found instruction!"),
    }
}

fn opecode_07(_cpu: &Cpu, _addr: u64, word: u32) -> Result<&Instruction, ()> {
    let funct3 = ((word & 0x00007000) >> 12) as u8;
    match INSTRUCTIONS_GROUP07.get(&funct3) {
        Some(instruction) => Ok(&instruction),
        None => panic!("Not found instruction!"),
    }
}

fn opecode_0f(_cpu: &Cpu, _addr: u64, word: u32) -> Result<&Instruction, ()> {
    let funct3 = ((word & 0x00007000) >> 12) as u8;
    match INSTRUCTIONS_GROUP0F.get(&funct3) {
        Some(instruction) => Ok(&instruction),
        None => panic!("Not found instruction!"),
    }
}

fn opecode_13(_cpu: &Cpu, _addr: u64, word: u32) -> Result<&Instruction, ()> {
    let funct3 = ((word & 0x00007000) >> 12) as u8;
    match funct3 {
        5 => {
            let funct7 = ((word & 0xfe000000) >> 25) as u8;
            match INSTRUCTIONS_GROUP13_SUB.get(&(funct7, funct3)) {
                Some(instruction) => Ok(&instruction),
                None => panic!("Not found instruction!"),
            }
        }
        _ => match INSTRUCTIONS_GROUP13.get(&funct3) {
            Some(instruction) => Ok(&instruction),
            None => panic!("Not found instruction!"),
        },
    }
}

fn opecode_17(_cpu: &Cpu, _addr: u64, _word: u32) -> Result<&Instruction, ()> {
    let idx = 0;
    match INSTRUCTIONS_GROUP17.get(&idx) {
        Some(instruction) => Ok(&instruction),
        None => panic!("Not found instruction!"),
    }
}

fn opecode_1b(_cpu: &Cpu, _addr: u64, word: u32) -> Result<&Instruction, ()> {
    let funct3 = ((word & 0x00007000) >> 12) as u8;
    match funct3 {
        5 => {
            let funct7 = ((word & 0xfe000000) >> 25) as u8;
            match INSTRUCTIONS_GROUP1B_SUB.get(&(funct7, funct3)) {
                Some(instruction) => Ok(&instruction),
                None => panic!("Not found instruction!"),
            }
        }
        _ => match INSTRUCTIONS_GROUP1B.get(&funct3) {
            Some(instruction) => Ok(&instruction),
            None => panic!("Not found instruction!"),
        },
    }
}

fn opecode_23(_cpu: &Cpu, _addr: u64, word: u32) -> Result<&Instruction, ()> {
    let funct3 = ((word & 0x00007000) >> 12) as u8;
    match INSTRUCTIONS_GROUP23.get(&funct3) {
        Some(instruction) => Ok(&instruction),
        None => panic!("Not found instruction!"),
    }
}

fn opecode_27(_cpu: &Cpu, _addr: u64, word: u32) -> Result<&Instruction, ()> {
    let funct3 = ((word & 0x00007000) >> 12) as u8;
    match INSTRUCTIONS_GROUP27.get(&funct3) {
        Some(instruction) => Ok(&instruction),
        None => panic!("Not found instruction!"),
    }
}

fn opecode_33(_cpu: &Cpu, _addr: u64, word: u32) -> Result<&Instruction, ()> {
    let funct3 = ((word & 0x00007000) >> 12) as u8;
    let funct7 = ((word & 0xfe000000) >> 25) as u8;
    match INSTRUCTIONS_GROUP33.get(&(funct7, funct3)) {
        Some(instruction) => Ok(&instruction),
        None => panic!("Not found instruction!"),
    }
}

fn opecode_37(_cpu: &Cpu, _addr: u64, _word: u32) -> Result<&Instruction, ()> {
    Ok(&Instruction {
        mnemonic: "lui",
        operation: lui,
        disassemble: disassemble_u,
    })
}

fn opecode_3b(_cpu: &Cpu, _addr: u64, word: u32) -> Result<&Instruction, ()> {
    let funct3 = ((word & 0x00007000) >> 12) as u8;
    let funct7 = ((word & 0xfe000000) >> 25) as u8;
    match INSTRUCTIONS_GROUP3B.get(&(funct7, funct3)) {
        Some(instruction) => Ok(&instruction),
        None => panic!("Not found instruction!"),
    }
}

fn opecode_63(_cpu: &Cpu, _addr: u64, word: u32) -> Result<&Instruction, ()> {
    let funct3 = ((word & 0x00007000) >> 12) as u8;
    match INSTRUCTIONS_GROUP63.get(&funct3) {
        Some(instruction) => Ok(&instruction),
        None => panic!("Not found instruction!"),
    }
}

fn opecode_67(_cpu: &Cpu, _addr: u64, _word: u32) -> Result<&Instruction, ()> {
    Ok(&Instruction {
        mnemonic: "jalr",
        operation: jalr,
        disassemble: disassemble_i,
    })
}

fn opecode_6f(_cpu: &Cpu, _addr: u64, _word: u32) -> Result<&Instruction, ()> {
    Ok(&Instruction {
        mnemonic: "jal",
        operation: jal,
        disassemble: disassemble_j,
    })
}

fn opecode_73(_cpu: &Cpu, _addr: u64, word: u32) -> Result<&Instruction, ()> {
    let funct3 = ((word & 0x00007000) >> 12) as u8;
    match funct3 {
        0 => {
            let funct12 = ((word & 0xfff00000) >> 20) as u16;
            match funct12 & 0x120 {
                0x120 => Ok(&Instruction {
                    mnemonic: "sfence.vma",
                    operation: sfence,
                    disassemble: disassemble_mnemonic,
                }),
                _ => match INSTRUCTIONS_GROUP73_EXTEND.get(&funct12) {
                    Some(instruction) => Ok(&instruction),
                    None => panic!("Not found instruction!"),
                },
            }
        }
        _ => match INSTRUCTIONS_GROUP73.get(&funct3) {
            Some(instruction) => Ok(&instruction),
            None => panic!("Not found instruction!"),
        },
    }
}

fn parse_type_i(word: u32) -> InstructionTypeI {
    InstructionTypeI {
        rd: ((word & 0x00000f80) >> 7) as u8,
        rs1: ((word & 0x000f8000) >> 15) as u8,
        imm: (match word & 0x80000000 > 0 {
            // MSB sign-extended
            true => 0xfffff800,
            false => 0,
        } | ((word & 0x7ff00000) >> 20)) as i32 as i64,
    }
}

fn parse_type_j(word: u32) -> InstructionTypeJ {
    InstructionTypeJ {
        rd: ((word & 0x00000f80) >> 7) as u8,
        imm: (match word & 0x80000000 > 0 {
            // MSB sign-extended
            true => 0xfff00000,
            false => 0,
        } | ((word & 0x7fe00000) >> 20)
            | ((word & 0x00100000) >> 9)
            | (word & 0x000ff000)) as i32 as i64 as u64,
    }
}

fn parse_type_b(word: u32) -> InstructionTypeB {
    InstructionTypeB {
        rs1: ((word & 0x000f8000) >> 15) as u8,
        rs2: ((word & 0x01f00000) >> 20) as u8,
        imm: (match word & 0x80000000 > 0 {
            // MSB sign-extended
            true => 0xfffff000,
            false => 0,
        } | ((word & 0x7e000000) >> 20)
            | ((word & 0x00000080) << 4)
            | ((word & 0x00000f00) >> 7)) as i32 as i64 as u64,
    }
}

fn parse_type_s(word: u32) -> InstructionTypeS {
    InstructionTypeS {
        rs1: ((word & 0x000f8000) >> 15) as u8,
        rs2: ((word & 0x01f00000) >> 20) as u8,
        imm: (((word & 0xfe000000) as i32 >> 20) | ((word & 0x00000f80) as i32 >> 7)) as i64,
    }
}

fn parse_type_u(word: u32) -> InstructionTypeU {
    InstructionTypeU {
        rd: ((word & 0x00000f80) >> 7) as u8,
        imm: ((word & 0xfffff000) as i32 as i64 as u64),
    }
}

fn parse_type_r(word: u32) -> InstructionTypeR {
    InstructionTypeR {
        rd: ((word & 0x00000f80) >> 7) as u8,
        rs1: ((word & 0x000f8000) >> 15) as u8,
        rs2: ((word & 0x01f00000) >> 20) as u8,
    }
}

fn parse_type_csr(word: u32) -> InstructionTypeCSR {
    InstructionTypeCSR {
        rd: ((word & 0x00000f80) >> 7) as u8,
        rs1: ((word & 0x000f8000) >> 15) as u8,
        csr: ((word & 0xfff00000) >> 20) as u16,
    }
}

/// unsigned extension.
pub fn unsigned(cpu: &Cpu, data: i64) -> u64 {
    match cpu.xlen {
        Xlen::X32 => (data as u64) & 0x00000000_ffffffff,
        Xlen::X64 => (data as u64) & 0xffffffff_ffffffff,
    }
}

/// sign extension.
pub fn signed(cpu: &Cpu, data: i64) -> i64 {
    match cpu.xlen {
        Xlen::X32 => data as i32 as i64,
        Xlen::X64 => data,
    }
}

//==============================================================================
// Load Instructions (RV32I/RV64I)
//==============================================================================
// The LW instruction loads a 32-bit value from memory into rd. LH loads a 16-bit value from memory,
// then sign-extends to 32-bits before storing in rd. LHU loads a 16-bit value from memory but then
// zero extends to 32-bits before storing in rd. LB and LBU are defined analogously for 8-bit values.
// The SW, SH, and SB instructions store 32-bit, 16-bit, and 8-bit values from the low bits of register
// rs2 to memory.

/// lb rd,offset(rs1)
fn lb(cpu: &mut Cpu, _addr: u64, word: u32) -> Result<(), Trap> {
    let o = parse_type_i(word);
    let data = match cpu
        .mmu
        .read8(cpu.x[o.rs1 as usize].wrapping_add(o.imm) as u64)
    {
        Ok(d) => d as i8 as i64,
        Err(e) => return Err(e),
    };
    cpu.x[o.rd as usize] = data;
    Ok(())
}

/// lh rd,offset(rs1)
fn lh(cpu: &mut Cpu, _addr: u64, word: u32) -> Result<(), Trap> {
    let o = parse_type_i(word);
    let data = match cpu
        .mmu
        .read16(cpu.x[o.rs1 as usize].wrapping_add(o.imm) as u64)
    {
        Ok(d) => d as i16 as i64,
        Err(e) => return Err(e),
    };
    cpu.x[o.rd as usize] = data;
    Ok(())
}

/// lw rd,offset(rs1)
fn lw(cpu: &mut Cpu, _addr: u64, word: u32) -> Result<(), Trap> {
    let o = parse_type_i(word);
    let data = match cpu
        .mmu
        .read32(cpu.x[o.rs1 as usize].wrapping_add(o.imm) as u64)
    {
        Ok(d) => d as i32 as i64,
        Err(e) => return Err(e),
    };
    cpu.x[o.rd as usize] = data;
    Ok(())
}

/// ld rd,offset(rs1)
fn ld(cpu: &mut Cpu, _addr: u64, word: u32) -> Result<(), Trap> {
    let o = parse_type_i(word);
    let data = match cpu
        .mmu
        .read64(cpu.x[o.rs1 as usize].wrapping_add(o.imm) as u64)
    {
        Ok(d) => d as i64,
        Err(e) => return Err(e),
    };
    cpu.x[o.rd as usize] = data;
    Ok(())
}

/// lbu rd,offset(rs1)
fn lbu(cpu: &mut Cpu, _addr: u64, word: u32) -> Result<(), Trap> {
    let o = parse_type_i(word);
    let data = match cpu
        .mmu
        .read8(cpu.x[o.rs1 as usize].wrapping_add(o.imm) as u64)
    {
        Ok(d) => d as i64,
        Err(e) => return Err(e),
    };
    cpu.x[o.rd as usize] = data;
    Ok(())
}

/// lhu rd,offset(rs1)
fn lhu(cpu: &mut Cpu, _addr: u64, word: u32) -> Result<(), Trap> {
    let o = parse_type_i(word);
    let data = match cpu
        .mmu
        .read16(cpu.x[o.rs1 as usize].wrapping_add(o.imm) as u64)
    {
        Ok(d) => d as i64,
        Err(e) => return Err(e),
    };
    cpu.x[o.rd as usize] = data;
    Ok(())
}

/// lwu rd,offset(rs1)
fn lwu(cpu: &mut Cpu, _addr: u64, word: u32) -> Result<(), Trap> {
    let o = parse_type_i(word);
    let data = match cpu
        .mmu
        .read32(cpu.x[o.rs1 as usize].wrapping_add(o.imm) as u64)
    {
        Ok(d) => d as i64,
        Err(e) => return Err(e),
    };
    cpu.x[o.rd as usize] = data;
    Ok(())
}

/// [lui rd,imm]
fn lui(cpu: &mut Cpu, _addr: u64, word: u32) -> Result<(), Trap> {
    let o = parse_type_u(word);
    cpu.x[o.rd as usize] = o.imm as i64;
    Ok(())
}

fn disassemble_i(_cpu: &Cpu, mnemonic: &str, word: u32) -> String {
    let o = parse_type_i(word);
    let mut s = String::new();
    s += &format!(
        "{0: <10} {1: },{2:x}({3:})",
        mnemonic,
        REGISTERS.get(&o.rd).unwrap(),
        o.imm,
        REGISTERS.get(&o.rs1).unwrap()
    );
    s
}

//==============================================================================
// Store Instructions (RV32I/RV64I)
//==============================================================================
// The SW, SH, and SB instructions store 32-bit, 16-bit, and 8-bit values
// from the low bits of register rs2 to memory.

/// [sb rs2,offset(rs1)]
fn sb(cpu: &mut Cpu, _addr: u64, word: u32) -> Result<(), Trap> {
    let o = parse_type_s(word);
    let addr = unsigned(cpu, cpu.x[o.rs1 as usize].wrapping_add(o.imm));
    let data = cpu.x[o.rs2 as usize] as u8;
    cpu.mmu.write8(addr, data)
}

/// [sh rs2,offset(rs1)]
fn sh(cpu: &mut Cpu, _addr: u64, word: u32) -> Result<(), Trap> {
    let o = parse_type_s(word);
    let addr = unsigned(cpu, cpu.x[o.rs1 as usize].wrapping_add(o.imm));
    let data = cpu.x[o.rs2 as usize] as u16;
    cpu.mmu.write16(addr, data)
}

/// [sw rs2,offset(rs1)]
fn sw(cpu: &mut Cpu, _addr: u64, word: u32) -> Result<(), Trap> {
    let o = parse_type_s(word);
    let addr = unsigned(cpu, cpu.x[o.rs1 as usize].wrapping_add(o.imm));
    let data = cpu.x[o.rs2 as usize] as u32;
    cpu.mmu.write32(addr, data)
}

/// [sd rs2,offset(rs1)]
fn sd(cpu: &mut Cpu, _addr: u64, word: u32) -> Result<(), Trap> {
    let o = parse_type_s(word);
    let addr = unsigned(cpu, cpu.x[o.rs1 as usize].wrapping_add(o.imm));
    let data = cpu.x[o.rs2 as usize] as u64;
    cpu.mmu.write64(addr, data)
}

fn disassemble_store(_cpu: &Cpu, mnemonic: &str, word: u32) -> String {
    let o = parse_type_s(word);
    let mut s = String::new();
    s += &format!(
        "{0: <10} {1:},{2:x}({3:})",
        mnemonic,
        REGISTERS.get(&o.rs2).unwrap(),
        o.imm,
        REGISTERS.get(&o.rs1).unwrap()
    );
    s
}

//==============================================================================
// Single, Double-Precision Load Store Instructions.
//==============================================================================
// Floating-point loads and stores use the same base+offset addressing mode as the integer base ISA,
// with a base address in register rs1 and a 12-bit signed byte offset

/// [flw rd,offset(rs1)]
/// The FLW instruction loads a single-precision floating-point value
/// from memory into floating-point register rd.
fn flw(cpu: &mut Cpu, _addr: u64, word: u32) -> Result<(), Trap> {
    let o = parse_type_i(word);
    let data = match cpu
        .mmu
        .read32(cpu.x[o.rs1 as usize].wrapping_add(o.imm) as u64)
    {
        Ok(d) => f64::from_bits(d as i32 as i64 as u64),
        Err(e) => return Err(e),
    };
    cpu.f[o.rd as usize] = data;
    Ok(())
}

/// [fld rd,rs1,offset]
/// The FLD instruction loads a double-precision floating-point value
/// from memory into floating-point register rd.
fn fld(cpu: &mut Cpu, _addr: u64, word: u32) -> Result<(), Trap> {
    let o = parse_type_i(word);
    let data = match cpu
        .mmu
        .read64(cpu.x[o.rs1 as usize].wrapping_add(o.imm) as u64)
    {
        Ok(d) => f64::from_bits(d),
        Err(e) => return Err(e),
    };
    cpu.f[o.rd as usize] = data;
    Ok(())
}

/// [fsw rs2,offset(rs1)]
fn fsw(cpu: &mut Cpu, _addr: u64, word: u32) -> Result<(), Trap> {
    let o = parse_type_s(word);
    let addr = cpu.x[o.rs1 as usize].wrapping_add(o.imm) as u64;
    cpu.mmu
        .write32(addr, cpu.f[o.rs2 as usize].to_bits() as u32)
}

/// [fsd rs2,offset(rs1)]
fn fsd(cpu: &mut Cpu, _addr: u64, word: u32) -> Result<(), Trap> {
    let o = parse_type_s(word);
    let addr = unsigned(cpu, cpu.x[o.rs1 as usize].wrapping_add(o.imm));
    cpu.mmu.write64(addr, cpu.f[o.rs2 as usize].to_bits())
}

fn disassemble_precision_load(_cpu: &Cpu, mnemonic: &str, word: u32) -> String {
    let o = parse_type_i(word);
    let mut s = String::new();
    s += &format!(
        "{0: <10} {1:},{2:},{3:x}",
        mnemonic,
        REGISTERS.get(&o.rd).unwrap(),
        REGISTERS.get(&o.rs1).unwrap(),
        o.imm
    );
    s
}

//==============================================================================
// Memory Ordering Instructions
//==============================================================================
/// [fence pred, succ], [fence.i]
/// The FENCE instruction is used to order device I/O and memory accesses
/// as viewed by other RISC- V harts and external devices or coprocessors.
fn fence(_cpu: &mut Cpu, _addr: u64, _word: u32) -> Result<(), Trap> {
    // do nothing.
    Ok(())
}

fn disassemble_fence(_cpu: &Cpu, mnemonic: &str, _word: u32) -> String {
    let mut s = String::new();
    s += &format!("{}", mnemonic);
    s
}

//==============================================================================
// Integer Register-Immediate Instructions (RV32I/RV64I)
//==============================================================================
/// [addi rd,rs1,imm]
/// ADDI adds the sign-extended 12-bit immediate to register rs1. Arithmetic overfl ow is ignored and
/// the result is simply the low XLEN bits of the result. ADDI rd, rs1, 0 is used to implement the MV
/// rd, rs1 assembler pseudoinstruction.
fn addi(cpu: &mut Cpu, _addr: u64, word: u32) -> Result<(), Trap> {
    let o = parse_type_i(word);
    cpu.x[o.rd as usize] = signed(cpu, cpu.x[o.rs1 as usize].wrapping_add(o.imm));
    Ok(())
}

/// [slli rd,rs1,shamt]
fn slli(cpu: &mut Cpu, _addr: u64, word: u32) -> Result<(), Trap> {
    let o = parse_type_i(word);
    let shamt = match cpu.xlen {
        Xlen::X64 => (word >> 20) & 0x3f,
        Xlen::X32 => (word >> 20) & 0x1f,
    };
    cpu.x[o.rd as usize] = signed(cpu, cpu.x[o.rs1 as usize] << shamt);
    Ok(())
}

/// [slti rd,rs1,imm]
fn slti(cpu: &mut Cpu, _addr: u64, word: u32) -> Result<(), Trap> {
    let o = parse_type_i(word);
    cpu.x[o.rd as usize] = match cpu.x[o.rs1 as usize] < o.imm {
        true => 1,
        false => 0,
    };
    Ok(())
}

/// [sltiu rd,rs1,imm]
fn sltiu(cpu: &mut Cpu, _addr: u64, word: u32) -> Result<(), Trap> {
    let o = parse_type_i(word);
    cpu.x[o.rd as usize] = match unsigned(cpu, cpu.x[o.rs1 as usize]) < unsigned(cpu, o.imm) {
        true => 1,
        false => 0,
    };
    Ok(())
}

/// [xori rd,rs1,imm]
fn xori(cpu: &mut Cpu, _addr: u64, word: u32) -> Result<(), Trap> {
    let o = parse_type_i(word);
    cpu.x[o.rd as usize] = cpu.x[o.rs1 as usize] ^ o.imm;
    Ok(())
}

/// [srli rd,rs1,shamt]
fn srli(cpu: &mut Cpu, _addr: u64, word: u32) -> Result<(), Trap> {
    let o = parse_type_i(word);
    let shamt = match cpu.xlen {
        Xlen::X64 => (word >> 20) & 0x3f,
        Xlen::X32 => (word >> 20) & 0x1f,
    };
    cpu.x[o.rd as usize] = signed(cpu, (unsigned(cpu, cpu.x[o.rs1 as usize]) >> shamt) as i64);
    Ok(())
}

/// [srai rd,rs1,shamt]
fn srai(cpu: &mut Cpu, _addr: u64, word: u32) -> Result<(), Trap> {
    let o = parse_type_i(word);
    let shamt = match cpu.xlen {
        Xlen::X64 => (word >> 20) & 0x3f,
        Xlen::X32 => (word >> 20) & 0x1f,
    };
    cpu.x[o.rd as usize] = signed(cpu, (cpu.x[o.rs1 as usize] >> shamt) as i64);
    Ok(())
}

/// [ori rd,rs1,imm]
fn ori(cpu: &mut Cpu, _addr: u64, word: u32) -> Result<(), Trap> {
    let o = parse_type_i(word);
    cpu.x[o.rd as usize] = cpu.x[o.rs1 as usize] | o.imm;
    Ok(())
}

/// [andi rd,rs1,imm]
fn andi(cpu: &mut Cpu, _addr: u64, word: u32) -> Result<(), Trap> {
    let o = parse_type_i(word);
    cpu.x[o.rd as usize] = cpu.x[o.rs1 as usize] & o.imm;
    Ok(())
}

/// [auipc rd,imm]
/// AUIPC (add upper immediate to pc) is used to build pc-relative
/// addresses and uses the U-type format.
fn auipc(cpu: &mut Cpu, addr: u64, word: u32) -> Result<(), Trap> {
    let o = parse_type_u(word);
    cpu.x[o.rd as usize] = signed(cpu, addr.wrapping_add(o.imm) as i64);
    Ok(())
}

/// [add rd,rs1,rs2]
fn add(cpu: &mut Cpu, _addr: u64, word: u32) -> Result<(), Trap> {
    let o = parse_type_r(word);
    cpu.x[o.rd as usize] = signed(
        cpu,
        cpu.x[o.rs1 as usize].wrapping_add(cpu.x[o.rs2 as usize]),
    );
    Ok(())
}

/// [sub rd,rs1,rs2]
fn sub(cpu: &mut Cpu, _addr: u64, word: u32) -> Result<(), Trap> {
    let o = parse_type_r(word);
    cpu.x[o.rd as usize] = signed(
        cpu,
        cpu.x[o.rs1 as usize].wrapping_sub(cpu.x[o.rs2 as usize]),
    );
    Ok(())
}

/// [sll rd,rs1,rs2]
/// SLL, SRL, and SRA perform logical left, logical right, and arithmetic right shifts on the value in
/// register rs1 by the shift amount held in the lower 5 bits of register rs2.
fn sll(cpu: &mut Cpu, _addr: u64, word: u32) -> Result<(), Trap> {
    let o = parse_type_r(word);
    let shamt = match cpu.xlen {
        Xlen::X64 => cpu.x[o.rs2 as usize] & 0x3f,
        Xlen::X32 => cpu.x[o.rs2 as usize] & 0x1f,
    };
    cpu.x[o.rd as usize] = signed(cpu, cpu.x[o.rs1 as usize] << shamt);
    Ok(())
}

/// [slt rd,rs1,rs2]
fn slt(cpu: &mut Cpu, _addr: u64, word: u32) -> Result<(), Trap> {
    let o = parse_type_r(word);
    cpu.x[o.rd as usize] = match cpu.x[o.rs1 as usize] < cpu.x[o.rs2 as usize] {
        true => 1,
        false => 0,
    };
    Ok(())
}

/// [sltu rd,rs1,rs2]
fn sltu(cpu: &mut Cpu, _addr: u64, word: u32) -> Result<(), Trap> {
    let o = parse_type_r(word);
    cpu.x[o.rd as usize] =
        match unsigned(cpu, cpu.x[o.rs1 as usize]) < unsigned(cpu, cpu.x[o.rs2 as usize]) {
            true => 1,
            false => 0,
        };
    Ok(())
}

/// [xor rd,rs1,rs2]
fn xor(cpu: &mut Cpu, _addr: u64, word: u32) -> Result<(), Trap> {
    let o = parse_type_r(word);
    cpu.x[o.rd as usize] = cpu.x[o.rs1 as usize] ^ cpu.x[o.rs2 as usize];
    Ok(())
}

/// [srl rd,rs1,rs2]
fn srl(cpu: &mut Cpu, _addr: u64, word: u32) -> Result<(), Trap> {
    let o = parse_type_r(word);
    let shamt = match cpu.xlen {
        Xlen::X64 => cpu.x[o.rs2 as usize] & 0x3f,
        Xlen::X32 => cpu.x[o.rs2 as usize] & 0x1f,
    };
    cpu.x[o.rd as usize] = signed(cpu, (unsigned(cpu, cpu.x[o.rs1 as usize]) >> shamt) as i64);
    Ok(())
}

/// [sra rd,rs1,rs2]
fn sra(cpu: &mut Cpu, _addr: u64, word: u32) -> Result<(), Trap> {
    let o = parse_type_r(word);
    let shamt = match cpu.xlen {
        Xlen::X64 => cpu.x[o.rs2 as usize] & 0x3f,
        Xlen::X32 => cpu.x[o.rs2 as usize] & 0x1f,
    };
    cpu.x[o.rd as usize] = signed(cpu, (signed(cpu, cpu.x[o.rs1 as usize]) >> shamt) as i64);
    Ok(())
}

/// [or rd,rs1,rs2]
fn or(cpu: &mut Cpu, _addr: u64, word: u32) -> Result<(), Trap> {
    let o = parse_type_r(word);
    cpu.x[o.rd as usize] = cpu.x[o.rs1 as usize] | cpu.x[o.rs2 as usize];
    Ok(())
}

/// [and rd,rs1,rs2]
fn and(cpu: &mut Cpu, _addr: u64, word: u32) -> Result<(), Trap> {
    let o = parse_type_r(word);
    cpu.x[o.rd as usize] = cpu.x[o.rs1 as usize] & cpu.x[o.rs2 as usize];
    Ok(())
}

/// [addw rd,rs1,rs2]
fn addw(cpu: &mut Cpu, _addr: u64, word: u32) -> Result<(), Trap> {
    let o = parse_type_r(word);
    cpu.x[o.rd as usize] = signed(
        cpu,
        cpu.x[o.rs1 as usize].wrapping_add(cpu.x[o.rs2 as usize]),
    ) as i32 as i64;
    Ok(())
}

/// [subw rd,rs1,rs2]
fn subw(cpu: &mut Cpu, _addr: u64, word: u32) -> Result<(), Trap> {
    let o = parse_type_r(word);
    cpu.x[o.rd as usize] = signed(
        cpu,
        cpu.x[o.rs1 as usize].wrapping_sub(cpu.x[o.rs2 as usize]),
    ) as i32 as i64;
    Ok(())
}

/// [sllw rd,rs1,rs2]
fn sllw(cpu: &mut Cpu, _addr: u64, word: u32) -> Result<(), Trap> {
    let o = parse_type_r(word);
    cpu.x[o.rd as usize] =
        signed(cpu, cpu.x[o.rs1 as usize] << (cpu.x[o.rs2 as usize] & 0x1f)) as i32 as i64;
    Ok(())
}

/// [srlw rd,rs1,rs2]
fn srlw(cpu: &mut Cpu, _addr: u64, word: u32) -> Result<(), Trap> {
    let o = parse_type_r(word);
    cpu.x[o.rd as usize] = signed(
        cpu,
        (cpu.x[o.rs1 as usize] as u32 >> (cpu.x[o.rs2 as usize] & 0x1f)) as i64,
    ) as i32 as i64;
    Ok(())
}

/// [sraw rd,rs1,rs2]
fn sraw(cpu: &mut Cpu, _addr: u64, word: u32) -> Result<(), Trap> {
    let o = parse_type_r(word);
    cpu.x[o.rd as usize] = signed(
        cpu,
        (cpu.x[o.rs1 as usize] as i32 >> (cpu.x[o.rs2 as usize] & 0x1f)) as i64,
    ) as i32 as i64;
    Ok(())
}

fn disassemble_computation(_cpu: &Cpu, mnemonic: &str, word: u32) -> String {
    let o = parse_type_i(word);
    let mut s = String::new();
    s += &format!(
        "{0: <10} {1:},{2:},{3:x}",
        mnemonic,
        REGISTERS.get(&o.rd).unwrap(),
        REGISTERS.get(&o.rs1).unwrap(),
        o.imm
    );
    s
}

fn disassemble_computation_shamt(cpu: &Cpu, mnemonic: &str, word: u32) -> String {
    let o = parse_type_i(word);
    let shamt = match cpu.xlen {
        Xlen::X64 => (word >> 20) & 0x3f,
        Xlen::X32 => (word >> 20) & 0x1f,
    };
    let mut s = String::new();
    s += &format!(
        "{0: <10} {1:},{2:},{3:}",
        mnemonic,
        REGISTERS.get(&o.rd).unwrap(),
        REGISTERS.get(&o.rs1).unwrap(),
        shamt
    );
    s
}

fn disassemble_u(_cpu: &Cpu, mnemonic: &str, word: u32) -> String {
    let o = parse_type_u(word);
    let mut s = String::new();
    s += &format!(
        "{0: <10} {1:},{2:x}",
        mnemonic,
        REGISTERS.get(&o.rd).unwrap(),
        o.imm
    );
    s
}

fn disassemble_r(_cpu: &Cpu, mnemonic: &str, word: u32) -> String {
    let o = parse_type_r(word);
    let mut s = String::new();
    s += &format!(
        "{0: <10} {1:},{2:},{3:}",
        mnemonic,
        REGISTERS.get(&o.rd).unwrap(),
        REGISTERS.get(&o.rs1).unwrap(),
        REGISTERS.get(&o.rs2).unwrap()
    );
    s
}

//==============================================================================
// Integer Register-Immediate Instructions (RV64I)
//==============================================================================
/// [addiw, rd,rs1,imm]
/// ADDIW is an RV64I instruction that adds the sign-extended 12-bit immediate
/// to register rs1 and produces the proper sign-extension of a 32-bit result
/// in rd. Overflows are ignored and the result is the low 32 bits of the result
/// sign-extended to 64 bits
fn addiw(cpu: &mut Cpu, _addr: u64, word: u32) -> Result<(), Trap> {
    let o = parse_type_i(word);
    cpu.x[o.rd as usize] = cpu.x[o.rs1 as usize].wrapping_add(o.imm) as i32 as i64;
    Ok(())
}

/// [slliw rd,rs1,shamt]
/// SLLIW, SRLIW, and SRAIW are RV64I-only instructions that are analogously defined
/// but operate on 32-bit values and produce signed 32-bit results. SLLIW, SRLIW, and
/// SRAIW encodings with imm[5] ̸= 0 are reserved.
fn slliw(cpu: &mut Cpu, _addr: u64, word: u32) -> Result<(), Trap> {
    let o = parse_type_i(word);
    let shamt = (word >> 20) & 0x3f;
    cpu.x[o.rd as usize] = (cpu.x[o.rs1 as usize] << shamt) as i32 as i64;
    Ok(())
}

/// [srliw rd,rs1,shamt]
fn srliw(cpu: &mut Cpu, _addr: u64, word: u32) -> Result<(), Trap> {
    let o = parse_type_i(word);
    let shamt = (word >> 20) & 0x3f;
    cpu.x[o.rd as usize] = ((cpu.x[o.rs1 as usize] as u32) >> shamt) as i32 as i64;
    Ok(())
}

/// [sraiw rd,rs1,shamt]
fn sraiw(cpu: &mut Cpu, _addr: u64, word: u32) -> Result<(), Trap> {
    let o = parse_type_i(word);
    let shamt = (word >> 20) & 0x1f;
    cpu.x[o.rd as usize] = ((cpu.x[o.rs1 as usize] as i32) >> shamt) as i32 as i64;
    Ok(())
}

//==============================================================================
// Control Transfer Instructions
//==============================================================================
/// [jal rd,offset]
/// JAL stores the address of the instruction following the jump (pc+4) into register rd.
/// The standard software calling convention uses x1 as the return address register and
/// x5 as an alternate link register.
fn jal(cpu: &mut Cpu, addr: u64, word: u32) -> Result<(), Trap> {
    let o = parse_type_j(word);
    cpu.x[o.rd as usize] = signed(cpu, (addr + 4) as i64);
    cpu.pc = addr.wrapping_add(o.imm);
    Ok(())
}

/// [jalr rd,rs1,offset]
fn jalr(cpu: &mut Cpu, addr: u64, word: u32) -> Result<(), Trap> {
    let o = parse_type_i(word);
    let temp = signed(cpu, (addr + 4) as i64);
    cpu.pc = (cpu.x[o.rs1 as usize] as u64).wrapping_add(o.imm as u64);
    cpu.x[o.rd as usize] = temp;
    Ok(())
}

/// [beq rs1,rs2,offset]
/// BEQ and BNE take the branch if registers rs1 and rs2 are equal or unequal respectively.
fn beq(cpu: &mut Cpu, addr: u64, word: u32) -> Result<(), Trap> {
    let o = parse_type_b(word);
    match cpu.x[o.rs1 as usize] == cpu.x[o.rs2 as usize] {
        true => cpu.pc = addr.wrapping_add(o.imm),
        _ => {}
    }
    Ok(())
}

/// [bne rs1,rs2,offset]
fn bne(cpu: &mut Cpu, addr: u64, word: u32) -> Result<(), Trap> {
    let o = parse_type_b(word);
    match cpu.x[o.rs1 as usize] != cpu.x[o.rs2 as usize] {
        true => cpu.pc = addr.wrapping_add(o.imm),
        _ => {}
    }
    Ok(())
}

/// [blt rs1,rs2,offset]
/// BLT and BLTU take the branch if rs1 is less than rs2, using signed and unsigned
/// comparison respectively.
fn blt(cpu: &mut Cpu, addr: u64, word: u32) -> Result<(), Trap> {
    let o = parse_type_b(word);
    match signed(&cpu, cpu.x[o.rs1 as usize]) < signed(&cpu, cpu.x[o.rs2 as usize]) {
        true => cpu.pc = addr.wrapping_add(o.imm),
        _ => {}
    }
    Ok(())
}

/// [bltu rs1,rs2,offset]
fn bltu(cpu: &mut Cpu, addr: u64, word: u32) -> Result<(), Trap> {
    let o = parse_type_b(word);
    match unsigned(cpu, cpu.x[o.rs1 as usize]) < unsigned(cpu, cpu.x[o.rs2 as usize]) {
        true => cpu.pc = addr.wrapping_add(o.imm),
        _ => {}
    }
    Ok(())
}

/// [bge rs1,rs2,offset]
/// BGE and BGEU take the branch if rs1 is greater than or equal to rs2,
/// using signed and unsigned comparison respectively.
fn bge(cpu: &mut Cpu, addr: u64, word: u32) -> Result<(), Trap> {
    let o = parse_type_b(word);
    match signed(&cpu, cpu.x[o.rs1 as usize]) >= signed(&cpu, cpu.x[o.rs2 as usize]) {
        true => cpu.pc = addr.wrapping_add(o.imm),
        _ => {}
    }
    Ok(())
}

/// [bgeu rs1,rs2,offset]
fn bgeu(cpu: &mut Cpu, addr: u64, word: u32) -> Result<(), Trap> {
    let o = parse_type_b(word);
    match unsigned(cpu, cpu.x[o.rs1 as usize]) >= unsigned(cpu, cpu.x[o.rs2 as usize]) {
        true => cpu.pc = addr.wrapping_add(o.imm),
        _ => {}
    }
    Ok(())
}

fn disassemble_j(_cpu: &Cpu, mnemonic: &str, word: u32) -> String {
    let o = parse_type_j(word);
    let mut s = String::new();
    s += &format!(
        "{0: <10} {1:},{2:}",
        mnemonic,
        REGISTERS.get(&o.rd).unwrap(),
        o.imm
    );
    s
}

fn disassemble_b(_cpu: &Cpu, mnemonic: &str, word: u32) -> String {
    let o = parse_type_b(word);
    let mut s = String::new();
    s += &format!(
        "{0: <10} {1:},{2:},{3:x}",
        mnemonic,
        REGISTERS.get(&o.rs1).unwrap(),
        REGISTERS.get(&o.rs2).unwrap(),
        o.imm
    );
    s
}

//==============================================================================
// Control and Status Register (CSR) Instructions.
//==============================================================================
/// [csrrw rd,offset,rs1]
/// The CSRRW (Atomic Read/Write CSR) instruction atomically swaps values in the
/// CSRs and integer registers. CSRRW reads the old value of the CSR, zero-extends
/// the value to XLEN bits, then writes it to integer register rd. The initial
/// value in rs1 is written to the CSR. If rd=x0, then the instruction shall not
/// read the CSR and shall not cause any of the side effects that might occur on a CSR read.
fn csrrw(cpu: &mut Cpu, addr: u64, word: u32) -> Result<(), Trap> {
    let o = parse_type_csr(word);
    let temp = cpu.x[o.rs1 as usize] as u64;
    match cpu.x[o.rd as usize] {
        0 /*cpu.x[0]*/ => {},
        _ => {
            match cpu.csr.read(o.csr, addr, &cpu.privilege) {
                Ok(data) => cpu.x[o.rd as usize] = data as i64,
                Err(e) => return Err(e)
            };
        }
    }
    match cpu.csr.write(o.csr, temp, addr, &cpu.privilege) {
        Ok(()) => Ok(()),
        Err(e) => Err(e),
    }
}

/// [csrrwi rd,offset,uimm]
fn csrrwi(cpu: &mut Cpu, addr: u64, word: u32) -> Result<(), Trap> {
    let o = parse_type_csr(word);
    let temp = o.rs1 as u64; // uimm field
    match cpu.x[o.rd as usize] {
        0 /*cpu.x[0]*/ => {},
        _ => {
            match cpu.csr.read(o.csr, addr, &cpu.privilege) {
                Ok(data) => cpu.x[o.rd as usize] = data as i64,
                Err(e) => return Err(e)
            };
        }
    }
    match cpu.csr.write(o.csr, temp, addr, &cpu.privilege) {
        Ok(()) => Ok(()),
        Err(e) => Err(e),
    }
}

/// [csrrs rd,offset,rs1]
/// The CSRRS (Atomic Read and Set Bits in CSR) instruction reads the value of the CSR,
/// zero- extends the value to XLEN bits, and writes it to integer register rd.
/// The initial value in integer register rs1 is treated as a bit mask that specifies
/// bit positions to be set in the CSR. Any bit that is high in rs1 will cause
/// the corresponding bit to be set in the CSR, if that CSR bit is writable.
/// Other bits in the CSR are unaffected (though CSRs might have side effects when written).
fn csrrs(cpu: &mut Cpu, addr: u64, word: u32) -> Result<(), Trap> {
    let o = parse_type_csr(word);
    let temp = cpu.x[o.rs1 as usize] as u64;
    match cpu.csr.read(o.csr, addr, &cpu.privilege) {
        Ok(data) => cpu.x[o.rd as usize] = data as i64,
        Err(e) => return Err(e),
    };
    match cpu.x[o.rd as usize] {
        0 /*cpu.x[0]*/ => Ok(()),
        _ => {
            match cpu.csr.write(o.csr, cpu.x[o.rd as usize] as u64 | temp, addr, &cpu.privilege) {
                Ok(()) => Ok(()),
                Err(e) => Err(e),
            }
        }
    }
}

/// [csrrsi rd,offset,uimm]
fn csrrsi(cpu: &mut Cpu, addr: u64, word: u32) -> Result<(), Trap> {
    let o = parse_type_csr(word);
    let temp = o.rs1 as u64; // uimm field
    match cpu.csr.read(o.csr, addr, &cpu.privilege) {
        Ok(data) => cpu.x[o.rd as usize] = data as i64,
        Err(e) => return Err(e),
    };
    match cpu.x[o.rd as usize] {
        0 /*cpu.x[0]*/ => Ok(()),
        _ => {
            match cpu.csr.write(o.csr, cpu.x[o.rd as usize] as u64 | temp,  addr, &cpu.privilege) {
                Ok(()) => Ok(()),
                Err(e) => Err(e),
            }
        }
    }
}

/// [csrrc rd,offset,rs1]
/// The CSRRC (Atomic Read and Clear Bits in CSR) instruction reads the value of the CSR,
/// zero- extends the value to XLEN bits, and writes it to integer register rd. The initial
/// value in integer register rs1 is treated as a bit mask that specifies bit positions to
/// be cleared in the CSR. Any bit that is high in rs1 will cause the corresponding bit to
/// be cleared in the CSR, if that CSR bit is writable. Other bits in the CSR are unaffected.
fn csrrc(cpu: &mut Cpu, addr: u64, word: u32) -> Result<(), Trap> {
    let o = parse_type_csr(word);
    let temp = cpu.x[o.rs1 as usize] as u64;
    match cpu.csr.read(o.csr, addr, &cpu.privilege) {
        Ok(data) => cpu.x[o.rd as usize] = data as i64,
        Err(e) => return Err(e),
    };
    match cpu.x[o.rd as usize] {
        0 /*cpu.x[0]*/ => Ok(()),
        _ => {
            match cpu.csr.write(o.csr,  cpu.x[o.rd as usize] as u64 | !temp, addr, &cpu.privilege) {
                Ok(()) => Ok(()),
                Err(e) => Err(e),
            }
        }
    }
}

/// [csrrci rd,offset,uimm]
fn csrrci(cpu: &mut Cpu, addr: u64, word: u32) -> Result<(), Trap> {
    let o = parse_type_csr(word);
    let temp = o.rs1 as u64; // uimm field
    match cpu.csr.read(o.csr, addr, &cpu.privilege) {
        Ok(data) => cpu.x[o.rd as usize] = data as i64,
        Err(e) => return Err(e),
    };
    match cpu.x[o.rd as usize] {
        0 /*cpu.x[0]*/ => Ok(()),
        _ => {
            match cpu.csr.write(o.csr,  cpu.x[o.rd as usize] as u64 | !temp, addr, &cpu.privilege) {
                Ok(()) => Ok(()),
                Err(e) => Err(e),
            }
        }
    }
}

fn disassemble_csr(_cpu: &Cpu, mnemonic: &str, word: u32) -> String {
    let o = parse_type_csr(word);
    let mut s = String::new();
    s += &format!(
        "{0: <10} {1:},{2:x},{3:}",
        mnemonic,
        REGISTERS.get(&o.rd).unwrap(),
        o.csr,
        REGISTERS.get(&o.rs1).unwrap()
    );
    s
}

//==============================================================================
// Environment Call and Breakpoints
//==============================================================================
/// [ecall]
fn ecall(cpu: &mut Cpu, addr: u64, _word: u32) -> Result<(), Trap> {
    Err(Trap {
        exception: match cpu.privilege {
            Privilege::User => Exception::EnvironmentCallFromUMode,
            Privilege::Supervisor => Exception::EnvironmentCallFromSMode,
            Privilege::Hypervisor => panic!("Hypervisor is not supported!"),
            Privilege::Machine => Exception::EnvironmentCallFromMMode,
        },
        value: addr,
    })
}

/// [ebreak]
fn ebreak(_cpu: &mut Cpu, addr: u64, _word: u32) -> Result<(), Trap> {
    Err(Trap {
        exception: Exception::Breakpoint,
        value: addr,
    })
}

//==============================================================================
// Trap-Return Instructions
//==============================================================================
/// [uret]
fn uret(_cpu: &mut Cpu, _addr: u64, _word: u32) -> Result<(), Trap> {
    panic!("TODO!!");
}

/// [sret]
fn sret(_cpu: &mut Cpu, _addr: u64, _word: u32) -> Result<(), Trap> {
    panic!("TODO!!");
}

/// [mret]
fn mret(cpu: &mut Cpu, addr: u64, _word: u32) -> Result<(), Trap> {
    cpu.pc = match cpu.csr.read(CSR_MEPC, addr, &cpu.privilege) {
        Ok(data) => unsigned(cpu, data as i64),
        Err(e) => return Err(e),
    };

    // update MSTATUS register.
    let mstatus = cpu.csr.read_direct(CSR_MSTATUS);
    let mpp = (mstatus >> 11) & 0x3;
    let mpie = (mstatus >> 7) & 1;
    cpu.csr.write_direct(
        CSR_MSTATUS,
        (mstatus & !0x1800) | // set 0 to MPP.
              (mpie << 3) |         // set MPIE to MIE.
              (1 << 7),
    ); // set 1 to MPIE

    // update privilege by MPP.
    // TODO: refactoring.
    cpu.privilege = match mpp {
        0 => Privilege::User,
        1 => Privilege::Supervisor,
        2 => Privilege::Hypervisor,
        3 => Privilege::Machine,
        _ => panic!("Unexpected Error!!"),
    };
    cpu.mmu.set_privilege(&cpu.privilege);
    Ok(())
}

/// [wfi]
/// The Wait for Interrupt instruction (WFI) provides a hint to the implementation that the current
/// hart can be stalled until an interrupt might need servicing. Execution of the WFI instruction
/// can also be used to inform the hardware platform that suitable interrupts should preferentially
/// be routed to this hart. WFI is available in all of the supported S and M privilege modes, and
/// optionally available to U-mode for implementations that support U-mode interrupts.
fn wfi(cpu: &mut Cpu, _addr: u64, _word: u32) -> Result<(), Trap> {
    cpu.wfi = true;
    Ok(())
}

/// [sfence.vma]
fn sfence(_cpu: &mut Cpu, _addr: u64, _word: u32) -> Result<(), Trap> {
    Ok(())
}

fn disassemble_mnemonic(_cpu: &Cpu, mnemonic: &str, _word: u32) -> String {
    let mut s = String::new();
    s += &format!("{}", mnemonic);
    s
}

//==============================================================================
// Multiplication Instructions (RV32M/RV64M)
//==============================================================================
/// [mul rd,rs1,rs2]
/// MUL performs an XLEN-bit×XLEN-bit multiplication of rs1 by rs2 and places
/// the lower XLEN bits in the destination register.
fn mul(cpu: &mut Cpu, _addr: u64, word: u32) -> Result<(), Trap> {
    let o = parse_type_r(word);
    cpu.x[o.rd as usize] = signed(
        cpu,
        cpu.x[o.rs1 as usize].wrapping_mul(cpu.x[o.rs2 as usize]),
    );
    Ok(())
}

/// [mulh rd,rs1,rs2]
fn mulh(cpu: &mut Cpu, _addr: u64, word: u32) -> Result<(), Trap> {
    let o = parse_type_r(word);
    let t = cpu.x[o.rs1 as usize].wrapping_mul(cpu.x[o.rs2 as usize]);
    cpu.x[o.rd as usize] = signed(
        cpu,
        match cpu.xlen {
            Xlen::X64 => t,
            _ => signed(cpu, t >> 32),
        },
    );
    Ok(())
}

/// [mulhsu rd,rs1,rs2]
fn mulhsu(cpu: &mut Cpu, _addr: u64, word: u32) -> Result<(), Trap> {
    let o = parse_type_r(word);
    cpu.x[o.rd as usize] = signed(
        cpu,
        match cpu.xlen {
            Xlen::X64 => {
                ((cpu.x[o.rs1 as usize] as i128 * (cpu.x[o.rs2 as usize] as u64 as i128)) >> 64)
                    as i64
            }
            _ => {
                (((cpu.x[o.rs1 as usize] as i32 as i64) * (cpu.x[o.rs2 as usize] as u32 as i64))
                    >> 32) as i64
            }
        },
    );
    Ok(())
}

/// [mulhu rd,rs1,rs2]
fn mulhu(cpu: &mut Cpu, _addr: u64, word: u32) -> Result<(), Trap> {
    let o = parse_type_r(word);
    cpu.x[o.rd as usize] = signed(
        cpu,
        match cpu.xlen {
            Xlen::X64 => {
                ((cpu.x[o.rs1 as usize] as u128).wrapping_mul(cpu.x[o.rs2 as usize] as u128) >> 64)
                    as i64
            }
            _ => {
                ((cpu.x[o.rs1 as usize] as u32 as u64)
                    .wrapping_mul(cpu.x[o.rs2 as usize] as u32 as u64)
                    >> 32) as i64
            }
        },
    );
    Ok(())
}

/// [div rd,rs1,rs2]
/// DIV and DIVU perform an XLEN bits by XLEN bits signed and unsigned integer
/// division of rs1 by rs2, rounding towards zero.
fn div(cpu: &mut Cpu, _addr: u64, word: u32) -> Result<(), Trap> {
    let o = parse_type_r(word);
    cpu.x[o.rd as usize] = match cpu.x[o.rs2 as usize] {
        0 => -1,
        _ => signed(
            cpu,
            cpu.x[o.rs1 as usize].wrapping_div(cpu.x[o.rs2 as usize]),
        ),
    };
    Ok(())
}

/// [divu rd,rs1,rs2]
fn divu(cpu: &mut Cpu, _addr: u64, word: u32) -> Result<(), Trap> {
    let o = parse_type_r(word);
    let numerator = unsigned(cpu, cpu.x[o.rs1 as usize]);
    let denominator = unsigned(cpu, cpu.x[o.rs2 as usize]);
    cpu.x[o.rd as usize] = match denominator {
        0 => -1,
        _ => signed(cpu, numerator.wrapping_div(denominator) as i64),
    };
    Ok(())
}

/// [rem rd,rs1,rs2]
/// REM and REMU provide the remainder of the corresponding division operation.
/// For REM, the sign of the result equals the sign of the dividend.
fn rem(cpu: &mut Cpu, _addr: u64, word: u32) -> Result<(), Trap> {
    let o = parse_type_r(word);
    cpu.x[o.rd as usize] = match cpu.x[o.rs2 as usize] {
        0 => cpu.x[o.rs1 as usize],
        _ => signed(cpu, cpu.x[o.rs1 as usize] % cpu.x[o.rs2 as usize]),
    };
    Ok(())
}

/// [remu rd,rs1,rs2]
fn remu(cpu: &mut Cpu, _addr: u64, word: u32) -> Result<(), Trap> {
    let o = parse_type_r(word);
    let numerator = unsigned(cpu, cpu.x[o.rs1 as usize]);
    let denominator = unsigned(cpu, cpu.x[o.rs2 as usize]);
    cpu.x[o.rd as usize] = signed(
        cpu,
        match denominator {
            0 => numerator,
            _ => numerator % denominator,
        } as i64,
    );
    Ok(())
}
