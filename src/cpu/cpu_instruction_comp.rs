use std::collections::HashMap;

use crate::cpu::cpu::{Cpu, Xlen};

pub struct CompressedOpecode {
    pub operation: fn(cpu: &Cpu, word: u16) -> Result<&'static CompressedInstruction, ()>,
}

pub struct CompressedInstruction {
    pub mnemonic: &'static str,
    pub decompress: fn(word: u16) -> Result<u32, ()>,
    pub disassemble: fn(cpu: &Cpu, mnemonic: &str, word: u16) -> String,
}

lazy_static! {
static ref COMPRESSED_OPECODES: HashMap<u8, CompressedOpecode> = {
    let mut m = HashMap::new();
    m.insert(0x0, CompressedOpecode {operation: opecode_0});
    m.insert(0x1, CompressedOpecode {operation: opecode_1});
    m.insert(0x2, CompressedOpecode {operation: opecode_2});
    m
};

static ref COMPRESSED_INSTRUCTIONS_GROUP0: HashMap<u8, CompressedInstruction> = {
    let mut m = HashMap::new();
    m.insert(0, CompressedInstruction {
        mnemonic: "c.addi4spn",
        decompress: c_addi4spn,
        disassemble: disassemble_mnemonic,
    });
    m.insert(1, CompressedInstruction {
        mnemonic: "c.fld",
        decompress: c_fld,
        disassemble: disassemble_mnemonic,
    });
    m.insert(2, CompressedInstruction {
        mnemonic: "c.lw",
        decompress: c_lw,
        disassemble: disassemble_mnemonic,
    });
    m.insert(5, CompressedInstruction {
        mnemonic: "c.fsd",
        decompress: c_fsd,
        disassemble: disassemble_mnemonic,
    });
    m.insert(6, CompressedInstruction {
        mnemonic: "c.sw",
        decompress: c_sw,
        disassemble: disassemble_mnemonic,
    });
    m
};
static ref COMPRESSED_INSTRUCTIONS_GROUP0_SUB: HashMap<(u8, u8), CompressedInstruction> = {
    let mut m = HashMap::new();
    m.insert((0, 3), CompressedInstruction { // FV32FC only.
        mnemonic: "c.flw",
        decompress: c_flw,
        disassemble: disassemble_mnemonic,
    });
    m.insert((1, 3), CompressedInstruction { // FC64IC only.
        mnemonic: "c.ld",
        decompress: c_ld,
        disassemble: disassemble_mnemonic,
    });
    m.insert((0, 7), CompressedInstruction { // FV32FC only.
        mnemonic: "c.fsw",
        decompress: c_fsw,
        disassemble: disassemble_mnemonic,
    });
    m.insert((1, 7), CompressedInstruction { // FC64IC only.
        mnemonic: "c.sd",
        decompress: c_sd,
        disassemble: disassemble_mnemonic,
    });
    m
};

static ref COMPRESSED_INSTRUCTIONS_GROUP1: HashMap<u8, CompressedInstruction> = {
    let mut m = HashMap::new();
    m.insert(2, CompressedInstruction {
        mnemonic: "c.li",
        decompress: c_li,
        disassemble: disassemble_mnemonic,
    });
    m.insert(5, CompressedInstruction {
        mnemonic: "c.j",
        decompress: c_j,
        disassemble: disassemble_mnemonic,
    });
    m.insert(6, CompressedInstruction {
        mnemonic: "c.beqz",
        decompress: c_beqz,
        disassemble: disassemble_mnemonic,
    });
    m.insert(7, CompressedInstruction {
        mnemonic: "c.bnez",
        decompress: c_bnez,
        disassemble: disassemble_mnemonic,
    });
    m
};

static ref COMPRESSED_INSTRUCTIONS_GROUP2: HashMap<u8, CompressedInstruction> = {
    let mut m = HashMap::new();
    m.insert(0, CompressedInstruction {
        mnemonic: "c.slli",
        decompress: c_slli,
        disassemble: disassemble_mnemonic,
    });
    m.insert(1, CompressedInstruction {
        mnemonic: "c.fldsp",
        decompress: c_fldsp,
        disassemble: disassemble_mnemonic,
    });
    m.insert(2, CompressedInstruction {
        mnemonic: "c.lwsp",
        decompress: c_lwsp,
        disassemble: disassemble_mnemonic,
    });
    m.insert(5, CompressedInstruction {
        mnemonic: "c.fsdsp",
        decompress: c_fsdsp,
        disassemble: disassemble_mnemonic,
    });
    m.insert(6, CompressedInstruction {
        mnemonic: "c.swsp",
        decompress: c_swsp,
        disassemble: disassemble_mnemonic,
    });
    m
};}

fn opecode_0(cpu: &Cpu, word: u16) -> Result<&'static CompressedInstruction, ()> {
    let funct3 = ((word >> 13) & 0x7) as u8;
    match funct3 {
        3 | 7 => match COMPRESSED_INSTRUCTIONS_GROUP0_SUB.get(&(cpu.xlen.clone() as u8, funct3)) {
            Some(instruction) => Ok(&instruction),
            None => panic!("Not found instruction!"),
        },
        _ => match COMPRESSED_INSTRUCTIONS_GROUP0.get(&funct3) {
            Some(instruction) => Ok(&instruction),
            None => panic!("Not found instruction!"),
        },
    }
}

fn opecode_1(cpu: &Cpu, word: u16) -> Result<&'static CompressedInstruction, ()> {
    let funct3 = ((word >> 13) & 0x7) as u8;
    match funct3 {
        0 => match word == 1 {
            true => Ok(&CompressedInstruction {
                mnemonic: "c.nop",
                decompress: c_nop,
                disassemble: disassemble_mnemonic,
            }),
            false => Ok(&CompressedInstruction {
                mnemonic: "c.addi",
                decompress: c_addi,
                disassemble: disassemble_mnemonic,
            }),
        },
        1 => match cpu.xlen {
            Xlen::X32 => Ok(&CompressedInstruction {
                mnemonic: "c.jal",
                decompress: c_jal,
                disassemble: disassemble_mnemonic,
            }),
            _ => Ok(&CompressedInstruction {
                mnemonic: "c.addiw",
                decompress: c_addiw,
                disassemble: disassemble_mnemonic,
            }),
        },
        3 => match (word >> 7) & 0x1f {
            2 => Ok(&CompressedInstruction {
                mnemonic: "c.addi16sp",
                decompress: c_addi16sp,
                disassemble: disassemble_mnemonic,
            }),
            _ => Ok(&CompressedInstruction {
                mnemonic: "c.lui",
                decompress: c_lui,
                disassemble: disassemble_mnemonic,
            }),
        },
        4 => match (word >> 10) & 0x3 {
            0 => Ok(&CompressedInstruction {
                mnemonic: "c.srli",
                decompress: c_srli,
                disassemble: disassemble_mnemonic,
            }),
            1 => Ok(&CompressedInstruction {
                mnemonic: "c.srai",
                decompress: c_srai,
                disassemble: disassemble_mnemonic,
            }),
            2 => Ok(&CompressedInstruction {
                mnemonic: "c.andi",
                decompress: c_andi,
                disassemble: disassemble_mnemonic,
            }),
            _ => match (word >> 5) & 0x3 {
                0 => match (word >> 12) & 0x1 {
                    0 => Ok(&CompressedInstruction {
                        mnemonic: "c.sub",
                        decompress: c_sub,
                        disassemble: disassemble_mnemonic,
                    }),
                    _ => Ok(&CompressedInstruction {
                        mnemonic: "c.subw",
                        decompress: c_subw,
                        disassemble: disassemble_mnemonic,
                    }),
                },
                1 => match (word >> 12) & 0x1 {
                    0 => Ok(&CompressedInstruction {
                        mnemonic: "c.xor",
                        decompress: c_xor,
                        disassemble: disassemble_mnemonic,
                    }),
                    _ => Ok(&CompressedInstruction {
                        mnemonic: "c.addw",
                        decompress: c_addw,
                        disassemble: disassemble_mnemonic,
                    }),
                },
                2 => Ok(&CompressedInstruction {
                    mnemonic: "c.or",
                    decompress: c_or,
                    disassemble: disassemble_mnemonic,
                }),
                _ => Ok(&CompressedInstruction {
                    mnemonic: "c.and",
                    decompress: c_and,
                    disassemble: disassemble_mnemonic,
                }),
            },
        },
        _ => match COMPRESSED_INSTRUCTIONS_GROUP1.get(&funct3) {
            Some(instruction) => Ok(&instruction),
            None => panic!("Not found instruction!"),
        },
    }
}

fn opecode_2(cpu: &Cpu, word: u16) -> Result<&'static CompressedInstruction, ()> {
    let funct3 = ((word >> 13) & 0x7) as u8;
    match funct3 {
        3 => match cpu.xlen {
            Xlen::X32 => Ok(&CompressedInstruction {
                // RV32FC only.
                mnemonic: "c.flwsp",
                decompress: c_flwsp,
                disassemble: disassemble_mnemonic,
            }),
            _ => Ok(&CompressedInstruction {
                // RV64IC only.
                mnemonic: "c.ldsp",
                decompress: c_ldsp,
                disassemble: disassemble_mnemonic,
            }),
        },
        4 => match (word >> 12) & 0x1 {
            0 => match (word >> 2) & 0x1f {
                0 => Ok(&CompressedInstruction {
                    mnemonic: "c.jr",
                    decompress: c_jr,
                    disassemble: disassemble_mnemonic,
                }),
                _ => Ok(&CompressedInstruction {
                    mnemonic: "c.mv",
                    decompress: c_mv,
                    disassemble: disassemble_mnemonic,
                }),
            },
            _ => match (word >> 2) & 0x3ff {
                0 => Ok(&CompressedInstruction {
                    mnemonic: "c.ebreak",
                    decompress: c_ebreak,
                    disassemble: disassemble_mnemonic,
                }),
                _ => match (word >> 2) & 0x1f {
                    0 => Ok(&CompressedInstruction {
                        mnemonic: "c.jalr",
                        decompress: c_jalr,
                        disassemble: disassemble_mnemonic,
                    }),
                    _ => Ok(&CompressedInstruction {
                        mnemonic: "c.add",
                        decompress: c_add,
                        disassemble: disassemble_mnemonic,
                    }),
                },
            },
        },
        7 => match cpu.xlen {
            Xlen::X32 => Ok(&CompressedInstruction {
                // RV32FC only.
                mnemonic: "c.fswsp",
                decompress: c_fswsp,
                disassemble: disassemble_mnemonic,
            }),
            _ => Ok(&CompressedInstruction {
                // RV64IC only.
                mnemonic: "c.sdsp",
                decompress: c_sdsp,
                disassemble: disassemble_mnemonic,
            }),
        },
        _ => match COMPRESSED_INSTRUCTIONS_GROUP2.get(&funct3) {
            Some(instruction) => Ok(&instruction),
            None => panic!("Not found instruction!"),
        },
    }
}

pub fn instruction_decompress(cpu: &Cpu, instruction_addr: u64, word: u32) -> Result<u32, ()> {
    let compressed_word = (word & 0xffff) as u16;

    let opecodes = match COMPRESSED_OPECODES.get(&((word & 0x3) as u8)) {
        Some(ops) => ops,
        None => panic!("Not found opecode: {:016x}", word),
    };

    match (opecodes.operation)(cpu, compressed_word) {
        Ok(instruction) => (instruction.decompress)(compressed_word),
        Err(()) => panic!("Not found instruction: {:016x}", instruction_addr),
    }
}

fn disassemble_mnemonic(_cpu: &Cpu, mnemonic: &str, _word: u16) -> String {
    let mut s = String::new();
    s += &format!("{}", mnemonic);
    s
}

/// [c.addi4spn rd’,uimm]
fn c_addi4spn(word: u16) -> Result<u32, ()> {
    let rd_ = ((word >> 2) & 0x7) as u32;
    let uimm_ =
        (((word >> 7) & 0x30) | ((word >> 1) & 0x3c0) | ((word >> 4) & 0x4) | ((word >> 2) & 0x8))
            as u32;
    match uimm_ {
        0 => Err(()),
        _ => {
            // addi rd,rs1,imm
            let op = 0x13 as u32;
            let imm = uimm_ << 20;
            let rs1 = (2 << 15) as u32;
            let rd = (rd_ + 8) << 7;
            Ok(imm | rs1 | rd | op)
        }
    }
}

/// [c.fld rd’,uimm(rs1’)]
fn c_fld(_word: u16) -> Result<u32, ()> {
    panic!("TODO");
}

/// [c.lw rd’,uimm(rs1’)]
fn c_lw(word: u16) -> Result<u32, ()> {
    let rd_ = ((word >> 2) & 0x7) as u32;
    let rs1_ = ((word >> 7) & 0x7) as u32;
    let uimm = (((word >> 7) & 0x38) | ((word >> 4) & 0x4) | ((word << 1) & 0x40)) as u32;

    // lw rd,offset(rs1)
    let op = 0x3 as u32;
    let rd = (rd_ + 8) << 7;
    let rs1 = (rs1_ + 8) << 15;
    let offset = uimm << 20;
    Ok(offset | rs1 | 2 << 12 | rd | op)
}

/// [c.flw rd’,uimm(rs1’)]
fn c_flw(_word: u16) -> Result<u32, ()> {
    panic!("TODO");
}

/// [c.ld rd’,uimm(rs1’)]
fn c_ld(word: u16) -> Result<u32, ()> {
    let rd_ = ((word >> 2) & 0x7) as u32;
    let rs1_ = ((word >> 7) & 0x7) as u32;
    let uimm = (((word >> 7) & 0x38) | ((word << 1) & 0xc0)) as u32;

    // ld rd,offset(rs1)
    let op = 0x3 as u32;
    let rd = (rd_ + 8) << 7;
    let rs1 = (rs1_ + 8) << 15;
    let offset = uimm << 20;
    Ok(offset | rs1 | 3 << 12 | rd | op)
}

/// [c.fsd rd’,uimm(rs1’)]
fn c_fsd(_word: u16) -> Result<u32, ()> {
    panic!("TODO");
}

/// [c.sw rd’,uimm(rs1’)]
fn c_sw(word: u16) -> Result<u32, ()> {
    let rs2_ = ((word >> 2) & 0x7) as u32;
    let rs1_ = ((word >> 7) & 0x7) as u32;
    let uimm = (((word >> 7) & 0x38) | ((word >> 4) & 0x4) | ((word << 1) & 0x40)) as u32;

    // sw rs2,offset(rs1)
    let op = 0x23 as u32;
    let rs2 = (rs2_ + 8) << 20;
    let rs1 = (rs1_ + 8) << 15;
    let offset_h = ((uimm >> 5) & 0x7f) << 25;
    let offset_l = (uimm & 0x1f) << 7;
    Ok(offset_h | rs2 | rs1 | 2 << 12 | offset_l | op)
}

/// [c.fsw rd’,uimm(rs1’)]
fn c_fsw(_word: u16) -> Result<u32, ()> {
    panic!("TODO");
}

/// [c.sd rd’,uimm(rs1’)]
fn c_sd(word: u16) -> Result<u32, ()> {
    let rs2_ = ((word >> 2) & 0x7) as u32;
    let rs1_ = ((word >> 7) & 0x7) as u32;
    let uimm = (((word >> 7) & 0x38) | ((word << 1) & 0xc0)) as u32;

    // sd rs2,offset(rs1)
    let op = 0x23 as u32;
    let rs2 = (rs2_ + 8) << 20;
    let rs1 = (rs1_ + 8) << 15;
    let offset_h = ((uimm >> 5) & 0x7f) << 25;
    let offset_l = (uimm & 0x1f) << 7;
     Ok(offset_h | rs2 | rs1 | 3 << 12 | offset_l | op)
}

/// [c.nop]
fn c_nop(_word: u16) -> Result<u32, ()> {
    // addi x0,x0,0
    Ok(0x13)
}

/// [c.addi rd,u[12:12]|u[6:2]]
fn c_addi(word: u16) -> Result<u32, ()> {
    let rd_ = ((word >> 7) & 0x1f) as u32;
    let imm_ = (((word >> 7) & 0x20) | ((word >> 2) & 0x1f)) as u32;

    // addi rd,rs1,imm
    let op = 0x13 as u32;
    let imm = (match word & 0x1000 {
        0 => 0,
        _ => 0xffffffc0,
    } | imm_)
        << 20;
    let rd = rd_ << 7;
    let rs1 = rd_ << 15;
    Ok(imm | rs1 | rd | op)
}

/// [c.jal offset]
fn c_jal(word: u16) -> Result<u32, ()> {
    let offset = match word & 0x1000 {
        0 => 0,
        _ => 0xfffff000,
    } | (((word >> 1) & 0x800)
        | ((word >> 7) & 0x10)
        | ((word >> 1) & 0x300)
        | ((word << 2) & 0x400)
        | ((word >> 1) & 0x40)
        | ((word << 1) & 0x80)
        | ((word >> 2) & 0xe)
        | ((word << 3) & 0x20)) as u32;

    // jal rd,offset
    let op = 0x6f as u32;
    let rd = 1/* x1 */ << 7;
    let imm = (((offset >> 1) & 0x80000)
        | ((offset << 8) & 0x7fe00)
        | ((offset >> 3) & 0x100)
        | ((offset >> 12) & 0xff))
        << 12;
    Ok(imm | rd | op)
}

/// [c.addiw rd,imm]
fn c_addiw(word: u16) -> Result<u32, ()> {
    let rd_ = ((word >> 7) & 0x1f) as u32;
    match rd_ {
        0 => Err(()),
        _ => {
            let imm_ = (((word >> 7) & 0x20) | ((word >> 2) & 0x1f)) as u32;

            // addiw rd,rs1,imm
            let op = 0x1b as u32;
            let imm = (match word & 0x1000 {
                0 => 0,
                _ => 0xffffffc0,
            } | imm_)
                << 20;
            let rd = rd_ << 7;
            let rs1 = rd_ << 15;
            Ok(imm | rs1 | rd | op)
        }
    }
}

/// [c.li rd,uimm]
fn c_li(word: u16) -> Result<u32, ()> {
    let rd_ = ((word >> 7) & 0x1f) as u32;
    let uimm = (((word >> 7) & 0x20) | ((word >> 2) & 0x1f)) as u32;

    // addi rd,rs1,imm
    let op = 0x13 as u32;
    let imm = (match word & 0x1000 {
        0 => 0,
        _ => 0xffffffc0,
    } | uimm)
        << 20;
    let rd = rd_ << 7;
    Ok(imm | rd | op)
}

/// [c.addi16sp imm]
fn c_addi16sp(word: u16) -> Result<u32, ()> {
    let rd_ = 2;
    let imm_ = (((word >> 3) & 0x200)
        | ((word >> 2) & 0x10)
        | ((word << 1) & 0x40)
        | ((word << 4) & 0x180)
        | ((word << 3) & 0x20)) as u32;
    match imm_ {
        0 => Err(()),
        _ => {
            // addi rd,rs1,imm
            let op = 0x13 as u32;
            let imm = (match word & 0x1000 {
                0 => 0,
                _ => 0xfffffc00,
            } | imm_)
                << 20;
            let rs1 = rd_ << 15;
            let rd = rd_ << 7;
            Ok(imm | rs1 | rd | op)
        }
    }
}

/// [c.lui rd,uimm]
fn c_lui(word: u16) -> Result<u32, ()> {
    let rd_ = ((word >> 7) & 0x1f) as u32;
    let imm_ = ((((word as u32) << 5) & 0x20000) | (((word as u32) << 10) & 0x1f000)) as u32;
    match imm_ {
        0 => Err(()),
        _ => {
            // lui rd,imm
            let op = 0x37 as u32;
            let imm = match word & 0x1000 {
                0 => 0,
                _ => 0xfffc0000,
            } | imm_;
            let rd = rd_ << 7;
            Ok(imm | rd | op)
        }
    }
}

/// [c.srli rd’,uimm]
fn c_srli(word: u16) -> Result<u32, ()> {
    let rd_ = ((word >> 7) & 0x7) as u32;
    let uimm = (((word >> 7) & 0x20) | ((word >> 2) & 0x1f)) as u32;

    // srli rd,rs1,shamt
    let op = 0x13 as u32;
    let shamt = uimm << 20;
    let rs1 = (rd_ + 8) << 15;
    let rd = (rd_ + 8) << 7;
    Ok(shamt | rs1 | 5 << 12 | rd | op)
}

/// [c.srai rd’,uimm]
fn c_srai(word: u16) -> Result<u32, ()> {
    let rd_ = ((word >> 7) & 0x7) as u32;
    let uimm = (((word >> 7) & 0x20) | ((word >> 2) & 0x1f)) as u32;

    // srai rd,rs1,shamt
    let op = 0x13 as u32;
    let shamt = uimm << 20;
    let rs1 = (rd_ + 8) << 15;
    let rd = (rd_ + 8) << 7;
    Ok(0x20 << 25 | shamt | rs1 | 5 << 12 | rd | op)
}

/// [c.andi rd’,uimm]
fn c_andi(word: u16) -> Result<u32, ()> {
    let rd_ = ((word >> 7) & 0x7) as u32;
    let imm_ = (((word >> 7) & 0x20) | ((word >> 2) & 0x1f)) as u32;

    // andi rd,rs1,imm
    let op = 0x13 as u32;
    let imm = (match word & 0x1000 {
        0 => 0,
        _ => 0xffffffc0,
    } | imm_)
        << 20;
    let rd = (rd_ + 8) << 7;
    let rs1 = (rd_ + 8) << 15;
    Ok(imm | rs1 | 7 << 12 | rd | op)
}

/// [c.sub rd’,rd’]
fn c_sub(word: u16) -> Result<u32, ()> {
    let rd_ = ((word >> 7) & 0x7) as u32;
    let rs2_ = ((word >> 2) & 0x7) as u32;

    // sub rd,rs1,rs2
    let op = 0x33 as u32;
    let rd = (rd_ + 8) << 7;
    let rs1 = (rd_ + 8) << 15;
    let rs2 = (rs2_ + 8) << 20;
    Ok(0x20 << 25 | rs2 | rs1 | rd | op)
}

/// [c.xor rd’,rd’]
fn c_xor(word: u16) -> Result<u32, ()> {
    let rd_ = ((word >> 7) & 0x7) as u32;
    let rs2_ = ((word >> 2) & 0x7) as u32;

    // xor rd,rs1,rs2
    let op = 0x33 as u32;
    let rd = (rd_ + 8) << 7;
    let rs1 = (rd_ + 8) << 15;
    let rs2 = (rs2_ + 8) << 20;
    Ok(rs2 | rs1 | 4 << 12 | rd | op)
}

/// [c.or rd’,rd’]
fn c_or(word: u16) -> Result<u32, ()> {
    let rd_ = ((word >> 7) & 0x7) as u32;
    let rs2_ = ((word >> 2) & 0x7) as u32;

    // or rd,rs1,rs2
    let op = 0x33 as u32;
    let rd = (rd_ + 8) << 7;
    let rs1 = (rd_ + 8) << 15;
    let rs2 = (rs2_ + 8) << 20;
    Ok(rs2 | rs1 | 6 << 12 | rd | op)
}

/// [c.and rd’,rd’]
fn c_and(word: u16) -> Result<u32, ()> {
    let rd_ = ((word >> 7) & 0x7) as u32;
    let rs2_ = ((word >> 2) & 0x7) as u32;

    // xor rd,rs1,rs2
    let op = 0x33 as u32;
    let rd = (rd_ + 8) << 7;
    let rs1 = (rd_ + 8) << 15;
    let rs2 = (rs2_ + 8) << 20;
    Ok(rs2 | rs1 | 7 << 12 | rd | op)
}

/// [c.subw rd’,rs2’]
fn c_subw(word: u16) -> Result<u32, ()> {
    let rd_ = ((word >> 7) & 0x7) as u32;
    let rs2_ = ((word >> 2) & 0x7) as u32;

    // subw rd,rs1,rs2
    let op = 0x3b as u32;
    let rd = (rd_ + 8) << 7;
    let rs1 = (rd_ + 8) << 15;
    let rs2 = (rs2_ + 8) << 20;
    Ok(0x20 << 25 | rs2 | rs1 | rd | op)
}

/// [c.addw rd’,rs2’]
fn c_addw(word: u16) -> Result<u32, ()> {
    let rd_ = ((word >> 7) & 0x7) as u32;
    let rs2_ = ((word >> 2) & 0x7) as u32;

    // addw rd,rs1,rs2
    let op = 0x3b as u32;
    let rd = (rd_ + 8) << 7;
    let rs1 = (rd_ + 8) << 15;
    let rs2 = (rs2_ + 8) << 20;
    Ok(rs2 | rs1 | rd | op)
}

/// [c.j offset]
fn c_j(word: u16) -> Result<u32, ()> {
    let offset = match word & 0x1000 {
        0 => 0,
        _ => 0xfffff000,
    } | (((word >> 1) & 0x800)
        | ((word >> 7) & 0x10)
        | ((word >> 1) & 0x300)
        | ((word << 2) & 0x400)
        | ((word >> 1) & 0x40)
        | ((word << 1) & 0x80)
        | ((word >> 2) & 0xe)
        | ((word << 3) & 0x20)) as u32;

    // jal rd,offset
    let op = 0x6f as u32;
    let rd = 0/* x0 */ << 7;
    let imm = (((offset >> 1) & 0x80000)
        | ((offset << 8) & 0x7fe00)
        | ((offset >> 3) & 0x100)
        | ((offset >> 12) & 0xff))
        << 12;
    Ok(imm | rd | op)
}

/// [c.beqz rs1’,offset]
fn c_beqz(word: u16) -> Result<u32, ()> {
    let rs1_ = ((word >> 7) & 0x7) as u32;
    let offset = match word & 0x1000 {
        0 => 0,
        _ => 0xfffffe00,
    } | (((word >> 4) & 0x100)
        | ((word >> 7) & 0x18)
        | ((word << 1) & 0xc0)
        | ((word >> 2) & 0x6)
        | ((word << 3) & 0x20)) as u32;

    // beq rs1,rs2,offset
    let op = 0x63 as u32;
    let rs1 = 0; // x0
    let rs2 = (rs1_ + 8) << 20;
    let offset_h = (((offset >> 6) & 0x40) | ((offset >> 5) & 0x3f)) << 25;
    let offset_l = ((offset & 0x1e) | ((offset >> 11) & 0x1)) << 7;
    Ok(offset_h | rs2 | rs1 /*| (0 << 12)*/ | offset_l | op)
}

/// [c.bnez rs1’,offset]
fn c_bnez(word: u16) -> Result<u32, ()> {
    let rs1_ = ((word >> 7) & 0x7) as u32;
    let offset = match word & 0x1000 {
        0 => 0,
        _ => 0xfffffe00,
    } | (((word >> 4) & 0x100)
        | ((word >> 7) & 0x18)
        | ((word << 1) & 0xc0)
        | ((word >> 2) & 0x6)
        | ((word << 3) & 0x20)) as u32;

    // bne rs1,rs2,offset
    let op = 0x63 as u32;
    let rs1 = 0; // x0
    let rs2 = (rs1_ + 8) << 20;
    let offset_h = (((offset >> 6) & 0x40) | ((offset >> 5) & 0x3f)) << 25;
    let offset_l = ((offset & 0x1e) | ((offset >> 11) & 0x1)) << 7;
    Ok(offset_h | rs2 | rs1 | (1 << 12) | offset_l | op)
}

/// [c.slli rd,uimm]
fn c_slli(word: u16) -> Result<u32, ()> {
    let rd_ = ((word >> 7) & 0x1f) as u32;
    let uimm = (((word >> 7) & 0x20) | ((word >> 2) & 0x1f)) as u32;

    // slli rd,rs1,shamt
    let op = 0x13 as u32;
    let shamt = uimm << 20;
    let rs1 = rd_ << 15;
    let rd = rd_ << 7;
    Ok(shamt | rs1 | 1 << 12 | rd | op)
}

/// [c.fldsp rd,uimm(x2)]
fn c_fldsp(_word: u16) -> Result<u32, ()> {
    panic!("TODO");
}

/// [c.lwsp rd,uimm(x2)]
fn c_lwsp(word: u16) -> Result<u32, ()> {
    let rd_ = ((word >> 7) & 0x1f) as u32;
    match rd_ {
        0 => Err(()),
        _ => {
            let uimm = (((word >> 7) & 0x20) |
            ((word >> 2) & 0x1c) |
            ((word << 4) & 0xc0)) as u32;
        
            // lw rd,offset(rs1)
            let op = 0x3 as u32;
            let rd = rd_ << 7;
            let rs1 = 2/* x2 */ << 15;
            let offset = uimm << 20;
            Ok(offset | rs1 | 2 << 12 | rd | op)
        }
    }
}

/// [c.flwsp rd,uimm(x2)]
fn c_flwsp(_word: u16) -> Result<u32, ()> {
    panic!("TODO");
}

/// [c.ldsp rd,uimm(x2)]
fn c_ldsp(word: u16) -> Result<u32, ()> {
    let rd_ = ((word >> 7) & 0x1f) as u32;
    match rd_ {
        0 => Err(()),
        _ => {
            let uimm = (((word >> 7) & 0x20) |
            ((word >> 2) & 0x18) |
            ((word << 4) & 0x1c0)) as u32;

            // ld rd,offset(rs1)
            let op = 0x3 as u32;
            let rd = rd_ << 7;
            let rs1 = 2/* x2 */ << 15;
            let offset = uimm << 20;
            Ok(offset | rs1 | 3 << 12 | rd | op)
        }
    }
}

/// [c.jr rs1]
fn c_jr(word: u16) -> Result<u32, ()> {
    let rs1_ = ((word >> 7) & 0x1f) as u32;
    match rs1_ {
        0 => Err(()),
        _ => {
            // jalr rd,rs1,offset
            let op = 0x67 as u32;
            let offset = 0 << 20;
            let rs1 = rs1_ << 15;
            let rd = 0 << 7; // x0
            Ok(offset | rs1 | 0 << 12 | rd | op)
        }
    }
}

/// [c.mv rd,rs2’]
fn c_mv(word: u16) -> Result<u32, ()> {
    let rd_ = ((word >> 7) & 0x1f) as u32;
    let rs2_ = ((word >> 2) & 0x1f) as u32;

    match rs2_ == 0 {
        true => Err(()),
        _ => {
            // add rd,rs1,rs2
            let op = 0x33 as u32;
            let rd = rd_ << 7;
            let rs1 = 0/* x0 */ << 15;
            let rs2 = rs2_ << 20;
            Ok(rs2 | rs1 | rd | op)
        }
    }
}

/// [c.ebreak]
fn c_ebreak(_word: u16) -> Result<u32, ()> {
    Ok(0x00100073)
}

/// [c.jalr rd]
fn c_jalr(word: u16) -> Result<u32, ()> {
    let rs1_ = ((word >> 7) & 0x1f) as u32;
    match rs1_ {
        0 => Err(()),
        _ => {
            // jalr rd,rs1,offset
            let op = 0x67 as u32;
            let offset = 0 << 20;
            let rs1 = rs1_ << 15;
            let rd = 1 << 7; // x1
            Ok(offset | rs1 | 0 << 12 | rd | op)
        }
    }
}

/// [c.add rd,rs2’]
fn c_add(word: u16) -> Result<u32, ()> {
    let rd_ = ((word >> 7) & 0x1f) as u32;
    let rs2_ = ((word >> 2) & 0x1f) as u32;

    match rd_ == 0 || rs2_ == 0 {
        true => Err(()),
        _ => {
            // add rd,rs1,rs2
            let op = 0x33 as u32;
            let rd = rd_ << 7;
            let rs1 = rd_ << 15;
            let rs2 = rs2_ << 20;
            Ok(rs2 | rs1 | rd | op)
        }
    }
}

/// [c.fsdsp rs2,uimm(x2)]
fn c_fsdsp(_word: u16) -> Result<u32, ()> {
    panic!("TODO");
}

/// [c.swsp rs2,uimm(x2)]
fn c_swsp(word: u16) -> Result<u32, ()> {
    let rs2_ = ((word >> 2) & 0x1f) as u32;
    let uimm = (((word >> 7) & 0x3c) | ((word >> 1) & 0xc0)) as u32;

    // sw rs2,offset(rs1)
    let op = 0x23 as u32;
    let rs2 = rs2_ << 20;
    let rs1 = 2/* x2 */ << 15;
    let offset_h = ((uimm >> 5) & 0x7f) << 25;
    let offset_l = (uimm & 0x1f) << 7;
    Ok(offset_h | rs2 | rs1 | 2 << 12 | offset_l | op)
}

/// [c.fswsp rs2,uimm(rs2)]
fn c_fswsp(_word: u16) -> Result<u32, ()> {
    panic!("TODO");
}

/// [c.sdsp rs2,uimm(x2)]
fn c_sdsp(word: u16) -> Result<u32, ()> {
    let rs2_ = ((word >> 2) & 0x1f) as u32;
    let uimm = (((word >> 7) & 0x38) | ((word >> 1) & 0x1c0)) as u32;

    // sd rs2,offset(rs1)
    let op = 0x23 as u32;
    let rs1 = 2 << 15; // x2
    let rs2 = rs2_ << 20;
    let offset_h = ((uimm >> 5) & 0x3f) << 25;
    let offset_l = (uimm & 0x1f) << 7;
    Ok(offset_h | rs2 | rs1 | 3 << 12 | offset_l | op)
}
