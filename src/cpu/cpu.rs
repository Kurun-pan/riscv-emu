use crate::cpu::cpu_instruction::{OPECODES, Opecode};
use crate::trap::Trap;
use crate::mmu::Mmu;

pub enum Xlen {
    X32,
    X64,
}

pub enum PrivilegeMode {
    User,
    Supervisor,
    Hypervisor,
    Machine,
}

pub struct Cpu {
    pub pc: u64,
    pub xlen: Xlen,
    pub privilege_mode: PrivilegeMode,
    pub x: [i64; 32],
    pub f: [f64; 32],
    pub csr: [u64; 4096],
    pub mmu: Mmu,
}

impl Cpu {
    pub fn new() -> Self {
        let mut cpu = Cpu {
            pc: 0,
            xlen: Xlen::X64,
            privilege_mode: PrivilegeMode::Machine,
            x: [0; 32],
            f: [0.0; 32],
            csr: [0; 4096],
            mmu: Mmu::new(Xlen::X64),
        };
        cpu
    }

    pub fn reset(&mut self) {
        self.pc = 0;
        self.privilege_mode = PrivilegeMode::Machine;
    }

    pub fn set_pc(&mut self, pc: u64) {
        self.pc = pc;
    }

    pub fn set_xlen(&mut self, xlen: Xlen) {
        self.xlen = xlen;
    }

    pub fn tick(&mut self) {
        let instruction_addr = self.pc;
        match self.tick_do() {
            Ok(()) => {}
            Err(e) => self.handle_trap(e, instruction_addr),
        }
    }

    fn tick_do(&mut self) -> Result<(), Trap> {
        let instruction_addr = self.pc;
        let word = match self.fetch() {
            Ok(_word) => _word,
            Err(e) => return Err(e),
        };

        // instruction decode.
        print!(" 0x{:016x}: {:08x}    ", instruction_addr, word);
        let instruction = match self.decode(word) {
            Ok(opecode) => match (opecode.operation)(self, instruction_addr, word) {
                Ok(_instruction) => _instruction,
                Err(()) => panic!("Not found instruction!"),
            }
            Err(e) => return Err(e)
        };

        // instruction execute
        println!("{}", (instruction.disassemble)(self, instruction.mnemonic, word));
        match (instruction.operation)(self, instruction_addr, word) {
            Err(e) => return Err(e),
            _ => {}
        }
        self.x[0] = 0; // hardwired zero

        return Ok(())
    }

    fn fetch(&mut self) -> Result<u32, Trap> {
        let fetch_word = match self.mmu.fetch32(self.pc) {
            Ok(word) => word,
            Err(e) => return Err(e),
        };

        match (fetch_word & 0x3) == 0x3 {
            // 32bit instruction
            true => {
                self.pc = self.pc.wrapping_add(4);
                return Ok(fetch_word);
            }
            // 16bit compressed instruction
            false => {
                self.pc = self.pc.wrapping_add(2);
                return self.instruction_uncompress((fetch_word & 0xffff) as u16);
            }
        };
    }

    fn instruction_uncompress(&mut self, word: u16) -> Result<u32, Trap> {
        panic!("Compressed instruction is not implimented now!");
    }

    fn decode(&mut self, word: u32) -> Result<&Opecode, Trap> {
        match OPECODES.get(&((word & 0x7f) as u8)) {
            Some(opecode) => return Ok(&opecode),
            None => panic!("Not found opecode!"),
        }
    }

    fn handle_trap(&mut self, trap: Trap, instruction_addr: u64) {
        // TODO!!
    }
}
