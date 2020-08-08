use std::collections::HashMap;

use crate::cpu::cpu::{Cpu, Xlen};
use crate::trap::Trap;

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
    imm: i64,
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
        m.insert(0x63, Opecode {operation: opecode_63});
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
            disassemble: disassemble_load,
        });
        m.insert(1, Instruction{
            mnemonic: "lh",
            operation: lh,
            disassemble: disassemble_load,
        });
        m.insert(2, Instruction{
            mnemonic: "lw",
            operation: lw,
            disassemble: disassemble_load,
        });
        m.insert(3, Instruction{
            mnemonic: "ld",
            operation: ld,
            disassemble: disassemble_load,
        });
        m.insert(4, Instruction{
            mnemonic: "lbu",
            operation: lbu,
            disassemble: disassemble_load,
        });
        m.insert(5, Instruction{
            mnemonic: "lhu",
            operation: lhu,
            disassemble: disassemble_load,
        });
        m.insert(6, Instruction{
            mnemonic: "lwu",
            operation: lwu,
            disassemble: disassemble_load,
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
        m.insert(5, Instruction{
            mnemonic: "srli",
            operation: srli,
            disassemble: disassemble_computation_shamt,
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

    pub static ref INSTRUCTIONS_GROUP17: HashMap<u8, Instruction> = {
        let mut m = HashMap::new();
        m.insert(0, Instruction{
            mnemonic: "auipc",
            operation: auipc,
            disassemble: disassemble_auipc,
        });
        m
    };

    // RV64I Integer Register-Immediate Instructions.
    pub static ref INSTRUCTIONS_GROUP1B: HashMap<(u8, u8), Instruction> = {
        let mut m = HashMap::new();
        m.insert((0, 0), Instruction{
            mnemonic: "addiw",
            operation: addiw,
            disassemble: disassemble_computation,
        });
        m.insert((0, 1), Instruction{
            mnemonic: "slliw",
            operation: slliw,
            disassemble: disassemble_computation,
        });
        m.insert((0, 5), Instruction{
            mnemonic: "srliw",
            operation: srliw,
            disassemble: disassemble_computation_shamt,
        });
        m.insert((8, 5), Instruction{
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
    match INSTRUCTIONS_GROUP13.get(&funct3) {
        Some(instruction) => Ok(&instruction),
        None => panic!("Not found instruction!"),
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
    let funct7 = ((word & 0xe0000000) >> 25) as u8;
    match INSTRUCTIONS_GROUP1B.get(&(funct7, funct3)) {
        Some(instruction) => Ok(&instruction),
        None => panic!("Not found instruction!"),
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

fn opecode_63(_cpu: &Cpu, _addr: u64, word: u32) -> Result<&Instruction, ()> {
    let funct3 = ((word & 0x00007000) >> 12) as u8;
    match INSTRUCTIONS_GROUP63.get(&funct3) {
        Some(instruction) => Ok(&instruction),
        None => panic!("Not found instruction!"),
    }
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
    match INSTRUCTIONS_GROUP73.get(&funct3) {
        Some(instruction) => Ok(&instruction),
        None => panic!("Not found instruction!"),
    }
}

fn parse_type_i(word: u32) -> InstructionTypeI {
    InstructionTypeI {
        rd: ((word & 0x00000f80) >> 7) as u8,
        rs1: ((word & 0x000f8000) >> 15) as u8,
        imm: ((word & 0xfff00000) as i32 as i64 >> 20),
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
            | (word & 0x000ff000)) as u64,
    }
}

fn parse_type_b(word: u32) -> InstructionTypeB {
    InstructionTypeB {
        rs1: ((word & 0x000f8000) >> 15) as u8,
        rs2: ((word & 0x01f00000) >> 20) as u8,
        imm: (match word & 0x80000000 > 0 {
            // MSB sign-extended
            true => 0xfff00000,
            false => 0,
        } | ((word & 0x7e000000) >> 20)
            | ((word & 0x00000080) << 4)
            | ((word & 0x00000f00) >> 7)) as u64,
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
        imm: ((word & 0xfffff000) as i32 as i64 >> 12),
    }
}

fn parse_type_csr(word: u32) -> InstructionTypeCSR {
    InstructionTypeCSR {
        rd: ((word & 0x00000f80) >> 7) as u8,
        rs1: ((word & 0x000f8000) >> 15) as u8,
        csr: ((word & 0xfff00000) >> 20) as u16,
    }
}

fn to_unsigned(cpu: &mut Cpu, data: i64) -> u64 {
    match cpu.xlen {
        Xlen::X32 => (data as u64) & 0x00000000_ffffffff,
        Xlen::X64 => (data as u64) & 0xffffffff_ffffffff,
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

fn disassemble_load(_cpu: &Cpu, mnemonic: &str, word: u32) -> String {
    let o = parse_type_i(word);
    let mut s = String::new();
    s += &format!(
        "{} {},{:016x}({})",
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
    let addr = cpu.x[o.rs1 as usize].wrapping_add(o.imm) as u64;
    let data = cpu.x[o.rs2 as usize] as u8;
    cpu.mmu.write8(addr, data)
}

/// [sh rs2,offset(rs1)]
fn sh(cpu: &mut Cpu, _addr: u64, word: u32) -> Result<(), Trap> {
    let o = parse_type_s(word);
    let addr = cpu.x[o.rs1 as usize].wrapping_add(o.imm) as u64;
    let data = cpu.x[o.rs2 as usize] as u16;
    cpu.mmu.write16(addr, data)
}

/// [sw rs2,offset(rs1)]
fn sw(cpu: &mut Cpu, _addr: u64, word: u32) -> Result<(), Trap> {
    let o = parse_type_s(word);
    let addr = cpu.x[o.rs1 as usize].wrapping_add(o.imm) as u64;
    let data = cpu.x[o.rs2 as usize] as u32;
    cpu.mmu.write32(addr, data)
}

/// [sd rs2,offset(rs1)]
fn sd(cpu: &mut Cpu, _addr: u64, word: u32) -> Result<(), Trap> {
    let o = parse_type_s(word);
    let addr = cpu.x[o.rs1 as usize].wrapping_add(o.imm) as u64;
    let data = cpu.x[o.rs2 as usize] as u64;
    cpu.mmu.write64(addr, data)
}

fn disassemble_store(_cpu: &Cpu, mnemonic: &str, word: u32) -> String {
    let o = parse_type_s(word);
    let mut s = String::new();
    s += &format!(
        "{} {},{:016x}({})",
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
    let addr = cpu.x[o.rs1 as usize].wrapping_add(o.imm) as u64;
    cpu.mmu.write64(addr, cpu.f[o.rs2 as usize].to_bits())
}

fn disassemble_precision_load(_cpu: &Cpu, mnemonic: &str, word: u32) -> String {
    let o = parse_type_i(word);
    let mut s = String::new();
    s += &format!(
        "{} {},{},{:016x}",
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
    cpu.x[o.rd as usize] = cpu.x[o.rs1 as usize].wrapping_add(o.imm);
    Ok(())
}

/// [slli rd,rs1,shamt]
fn slli(cpu: &mut Cpu, _addr: u64, word: u32) -> Result<(), Trap> {
    let o = parse_type_i(word);
    let shamt = match cpu.xlen {
        Xlen::X64 => (word >> 20) & 0x1f,
        Xlen::X32 => (word >> 20) & 0x3f,
    };
    cpu.x[o.rd as usize] = cpu.x[o.rs1 as usize] << shamt;
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
    cpu.x[o.rd as usize] = match to_unsigned(cpu, cpu.x[o.rs1 as usize]) < to_unsigned(cpu, o.imm) {
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
        Xlen::X64 => (word >> 20) & 0x1f,
        Xlen::X32 => (word >> 20) & 0x3f,
    };
    cpu.x[o.rd as usize] = cpu.x[o.rs1 as usize] >> shamt;
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
fn auipc(cpu: &mut Cpu, _addr: u64, word: u32) -> Result<(), Trap> {
    let o = parse_type_u(word);
    cpu.x[o.rd as usize] = to_unsigned(cpu, cpu.pc.wrapping_add(o.imm as u64) as i64) as i64;
    Ok(())
}

fn disassemble_computation(_cpu: &Cpu, mnemonic: &str, word: u32) -> String {
    let o = parse_type_i(word);
    let mut s = String::new();
    s += &format!(
        "{} {},{},{}",
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
        Xlen::X64 => (word >> 20) & 0x1f,
        Xlen::X32 => (word >> 20) & 0x3f,
    };
    let mut s = String::new();
    s += &format!(
        "{} {},{},{}",
        mnemonic,
        REGISTERS.get(&o.rd).unwrap(),
        REGISTERS.get(&o.rs1).unwrap(),
        shamt
    );
    s
}

fn disassemble_auipc(_cpu: &Cpu, mnemonic: &str, word: u32) -> String {
    let o = parse_type_u(word);
    let mut s = String::new();
    s += &format!("{} {},{}", mnemonic, REGISTERS.get(&o.rd).unwrap(), o.imm);
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
    let shamt = (word >> 20) & 0x1f;
    cpu.x[o.rd as usize] = (cpu.x[o.rs1 as usize] << shamt) as i32 as i64;
    Ok(())
}

/// [srliw rd,rs1,shamt]
fn srliw(cpu: &mut Cpu, _addr: u64, word: u32) -> Result<(), Trap> {
    let o = parse_type_i(word);
    let shamt = (word >> 20) & 0x1f;
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
    cpu.x[o.rd as usize] = (cpu.pc + 4) as i64;
    cpu.pc = addr.wrapping_add(o.imm);
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
    match cpu.x[o.rs1 as usize] < cpu.x[o.rs2 as usize] {
        true => cpu.pc = addr.wrapping_add(o.imm),
        _ => {}
    }
    Ok(())
}

/// [bltu rs1,rs2,offset]
fn bltu(cpu: &mut Cpu, addr: u64, word: u32) -> Result<(), Trap> {
    let o = parse_type_b(word);
    match to_unsigned(cpu, cpu.x[o.rs1 as usize]) > to_unsigned(cpu, cpu.x[o.rs2 as usize]) {
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
    match cpu.x[o.rs1 as usize] >= cpu.x[o.rs2 as usize] {
        true => cpu.pc = addr.wrapping_add(o.imm),
        _ => {}
    }
    Ok(())
}

/// [bgeu rs1,rs2,offset]
fn bgeu(cpu: &mut Cpu, addr: u64, word: u32) -> Result<(), Trap> {
    let o = parse_type_b(word);
    match to_unsigned(cpu, cpu.x[o.rs1 as usize]) >= to_unsigned(cpu, cpu.x[o.rs2 as usize]) {
        true => cpu.pc = addr.wrapping_add(o.imm),
        _ => {}
    }
    Ok(())
}

fn disassemble_j(_cpu: &Cpu, mnemonic: &str, word: u32) -> String {
    let o = parse_type_j(word);
    let mut s = String::new();
    s += &format!("{} {},{}", mnemonic, REGISTERS.get(&o.rd).unwrap(), o.imm);
    s
}

fn disassemble_b(_cpu: &Cpu, mnemonic: &str, word: u32) -> String {
    let o = parse_type_b(word);
    let mut s = String::new();
    s += &format!(
        "{} {},{},{}",
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
            match cpu.read_csr(o.csr, addr) {
                Ok(data) => cpu.x[o.rd as usize] = data as i64,
                Err(e) => return Err(e)
            };
        }
    }
    match cpu.write_csr(o.csr, temp, addr) {
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
            match cpu.read_csr(o.csr, addr) {
                Ok(data) => cpu.x[o.rd as usize] = data as i64,
                Err(e) => return Err(e)
            };
        }
    }
    match cpu.write_csr(o.csr, temp, addr) {
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
    match cpu.read_csr(o.csr, addr) {
        Ok(data) => cpu.x[o.rd as usize] = data as i64,
        Err(e) => return Err(e),
    };
    match cpu.x[o.rd as usize] {
        0 /*cpu.x[0]*/ => Ok(()),
        _ => {
            match cpu.write_csr(o.csr,
                         cpu.x[o.rd as usize] as u64 | temp,
                          addr) {
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
    match cpu.read_csr(o.csr, addr) {
        Ok(data) => cpu.x[o.rd as usize] = data as i64,
        Err(e) => return Err(e),
    };
    match cpu.x[o.rd as usize] {
        0 /*cpu.x[0]*/ => Ok(()),
        _ => {
            match cpu.write_csr(o.csr, cpu.x[o.rd as usize] as u64 | temp,  addr) {
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
    match cpu.read_csr(o.csr, addr) {
        Ok(data) => cpu.x[o.rd as usize] = data as i64,
        Err(e) => return Err(e),
    };
    match cpu.x[o.rd as usize] {
        0 /*cpu.x[0]*/ => Ok(()),
        _ => {
            match cpu.write_csr(o.csr,  cpu.x[o.rd as usize] as u64 | !temp, addr) {
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
    match cpu.read_csr(o.csr, addr) {
        Ok(data) => cpu.x[o.rd as usize] = data as i64,
        Err(e) => return Err(e),
    };
    match cpu.x[o.rd as usize] {
        0 /*cpu.x[0]*/ => Ok(()),
        _ => {
            match cpu.write_csr(o.csr,  cpu.x[o.rd as usize] as u64 | !temp, addr) {
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
        "{} {},{},{}",
        mnemonic,
        REGISTERS.get(&o.rd).unwrap(),
        o.csr,
        REGISTERS.get(&o.rs1).unwrap()
    );
    s
}
