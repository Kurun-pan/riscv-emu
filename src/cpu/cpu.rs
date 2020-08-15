use crate::tty::Tty;
use crate::cpu::cpu_csr::*;
use crate::cpu::cpu_instruction::{Instruction, Opecode, OPECODES, unsigned};
use crate::cpu::cpu_instruction_comp::*;
use crate::cpu::trap::*;
use crate::mmu::Mmu;

#[derive(Clone)]
pub enum Xlen {
    X32 = 0,
    X64 = 1,
}

#[derive(Clone)]
#[derive(Debug)]
pub enum Privilege {
    User = 0,
    Supervisor = 1,
    Hypervisor = 2,
    Machine = 3,
}

pub struct Cpu {
    pub pc: u64,
    pub wfi: bool,
    pub xlen: Xlen,
    pub privilege: Privilege,
    pub x: [i64; 32],
    pub f: [f64; 32],
    pub csr: Csr,
    pub mmu: Mmu,
    testmode: bool,
}

impl Cpu {
    pub fn new(tty: Box<dyn Tty>, testmode: bool) -> Self {
        let cpu = Cpu {
            pc: 0,
            wfi: false,
            xlen: Xlen::X64,
            privilege: Privilege::Machine,
            x: [0; 32],
            f: [0.0; 32],
            csr: Csr::new(),
            mmu: Mmu::new(Xlen::X64, tty),
            testmode: testmode,
        };
        cpu
    }

    pub fn reset(&mut self) {
        self.pc = 0;
        self.privilege = Privilege::Machine;
    }

    pub fn set_pc(&mut self, pc: u64) {
        self.pc = pc;
    }

    pub fn set_xlen(&mut self, xlen: Xlen) {
        self.xlen = xlen;
        self.mmu.set_xlen(&self.xlen);
    }

    pub fn tick(&mut self) {
        match self.wfi {
            true => return,
            _ => {}
        };

        match self.check_interrupt() {
            Some(interrupt) => self.interrupt_handler(interrupt),
            None => {}
        }

        let instruction_addr = self.pc;
        match self.tick_do() {
            Ok(()) => {}
            Err(e) => self.catch_exception(e, instruction_addr),
        }

        // run peripherals.        
        let bus = self.mmu.get_bus();
        let irqs = bus.tick();
        // set external interrupts to CSR register.
        if irqs[Privilege::Machine as usize] {
            self.csr.read_modify_write_direct(CSR_MIP, 0x800, 0);
        }
        if irqs[Privilege::Hypervisor as usize] {
            self.csr.read_modify_write_direct(CSR_MIP, 0x400, 0);
        }
        if irqs[Privilege::Supervisor as usize] {
            self.csr.read_modify_write_direct(CSR_MIP, 0x200, 0);
            self.csr.read_modify_write_direct(CSR_SIP, 0x200, 0);
        }
        if irqs[Privilege::User as usize] {
            self.csr.read_modify_write_direct(CSR_MIP, 0x100, 0);
            self.csr.read_modify_write_direct(CSR_SIP, 0x100, 0);
            self.csr.read_modify_write_direct(CSR_UIP, 0x100, 0);
        }
    }

    fn tick_do(&mut self) -> Result<(), Trap> {
        let instruction_addr = self.pc;
        let word = match self.fetch() {
            Ok(_word) => _word,
            Err(e) => return Err(e),
        };

        let mut debug_message = String::new();
        if self.testmode {
            debug_message += &format!(" [PC]: {:016x}", instruction_addr);
            debug_message += &format!(" [P]: {:?} |", self.privilege);
            debug_message += &format!("    {:08x}    ", word);
            match self.pc.wrapping_sub(instruction_addr) {
                0x2 => debug_message += "(C)",
                _ => debug_message += "   "
            };
        }

        // instruction decode.
        let instruction = match self.decode(word) {
            Ok(opecode) => match (opecode.operation)(self, instruction_addr, word) {
                Ok(_instruction) => _instruction,
                Err(()) => panic!("Not found instruction: {:016x}", instruction_addr),
            },
            Err(e) => return Err(e),
        };

        // instruction execute.
        if self.testmode {
            let dis = (instruction.disassemble)(self, instruction.mnemonic, word);
            debug_message += &format!("{}", dis);
        }

        match (instruction.operation)(self, instruction_addr, word) {
            Err(e) => return Err(e),
            _ => {}
        }
        self.x[0] = 0; // hardwired zero

        if self.testmode {
            println!("{}", debug_message);
        }

        return Ok(());
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
                return match instruction_decompress(self, self.pc.wrapping_sub(2), fetch_word) {
                    Ok(word) => Ok(word),
                    Err(()) => Err(Trap {
                        exception: Exception::IllegalInstruction,
                        value: self.pc.wrapping_sub(2)
                    }),
                }
            }
        };
    }

    fn decode(&mut self, word: u32) -> Result<&Opecode, Trap> {
        match OPECODES.get(&((word & 0x7f) as u8)) {
            Some(opecode) => return Ok(&opecode),
            None => panic!("Not found opecode: {:016x}", word),
        }
    }

    fn catch_exception(&mut self, trap: Trap, addr: u64) {
        println!("  >> Exception: {:?} ({:?})", trap.exception, self.privilege);
        let exception_code = trap.exception as u64 as u8;

        // update CSR/xSTATUS register.
        match self.privilege {
            Privilege::User => println!("TODO: SSTATUS spec is existing?"),
            Privilege::Supervisor => {
                let sstatus = self.csr.read_direct(CSR_SSTATUS);
                let spie = ((sstatus >> 1) & 0x1) << 5;
                let data = (sstatus & !0x122) | spie | (1 << 8);
                self.csr.write_direct(CSR_SSTATUS, data);
            },
            Privilege::Hypervisor => panic!("TODO: HSTATUS spec is existing?"),
            Privilege::Machine => {
                let mstatus = self.csr.read_direct(CSR_MSTATUS);
                let mpie = ((mstatus >> 3) & 0x1) << 7;
                let data = (mstatus & !0x1888) | mpie | (3 << 11);
                self.csr.write_direct(CSR_MSTATUS, data);
            },
        };

        // change privilege.
        {
            let medeleg = self.csr.read_direct(CSR_MEDELEG);
            //let hedeleg = self.csr.read_direct(CSR_HEDELEG);
            let sedeleg = self.csr.read_direct(CSR_SEDELEG);
            let next_privilege = match ((medeleg >> exception_code) & 1) > 0 {
                //true => match ((hedeleg >> exception_code) & 1) > 0 {
                    true => match ((sedeleg >> exception_code) & 1) > 0 {
                        true => Privilege::User,
                        false => Privilege::Supervisor,
                    },
                //    false => Privilege::Hypervisor
                //},
                false => Privilege::Machine,
            };
            self.privilege = next_privilege;
            self.mmu.set_privilege(&self.privilege);
        }

        // set exeption vectior address to PC.
        self.pc = self.csr.read_direct(match self.privilege {
            Privilege::User => CSR_UTVEC,
            Privilege::Supervisor => CSR_STVEC,
            Privilege::Hypervisor => CSR_HTVEC,
            Privilege::Machine => CSR_MTVEC,
        });
        self.pc = unsigned(self, self.pc as i64);

        // update CSR/xEPC, xCAUSE, xTVAL registers.
        {
            self.csr.write_direct(match self.privilege {
                Privilege::User => CSR_UEPC,
                Privilege::Supervisor => CSR_SEPC,
                Privilege::Hypervisor => CSR_HEPC,
                Privilege::Machine => CSR_MEPC,
            }, addr);

            /*
            let cause = match self.xlen {
                Xlen::X32 => 0x80000000,
                Xlen::X64 => 0x80000000_00000000
            } | exception_code as u64;
            */
            let cause = exception_code as u64;
            self.csr.write_direct(match self.privilege {
                Privilege::User => CSR_UCAUSE,
                Privilege::Supervisor => CSR_SCAUSE,
                Privilege::Hypervisor => CSR_HCAUSE,
                Privilege::Machine => CSR_MCAUSE,
            }, cause);

            self.csr.write_direct(match self.privilege {
                Privilege::User => CSR_UTVAL,
                Privilege::Supervisor => CSR_STVAL,
                Privilege::Hypervisor => CSR_HTVAL,
                Privilege::Machine => CSR_MTVAL,
            }, trap.value);
        }
    }

    fn check_interrupt(&mut self) -> Option<Interrupt> {
        let mie = self.csr.read_direct(CSR_MIE);
        let mip = self.csr.read_direct(CSR_MIP);
        let cause = mie & mip & 0xfff;
        match cause {
            0x800 => Some(Interrupt::MachineExternal),
            0x400 => panic!("Unexpected event happend!"),
            0x200 => Some(Interrupt::SupervisorExternal),
            0x100 => Some(Interrupt::UserExternal),
            0x080 => Some(Interrupt::MachineTimer),
            0x040 => panic!("Unexpected event happend!"),
            0x020 => Some(Interrupt::SupervisorTimer),
            0x010 => Some(Interrupt::UserTimer),
            0x008 => Some(Interrupt::MachineSoftware),
            0x004 => panic!("Unexpected event happend!"),
            0x002 => Some(Interrupt::SupervisorSoftware),
            0x001 => Some(Interrupt::UserSoftware),
            _ => None
        }
    }

    fn interrupt_handler(&mut self, interrupt: Interrupt) {
        println!("  >> Interrupt: {:?} ({:?})", interrupt, self.privilege);






        // clear interrupt.
        {
            let mip = self.csr.read_direct(CSR_MIP);
            self.csr.write_direct(CSR_MIP, mip & !match interrupt {
                Interrupt::MachineExternal => 0x800,
                Interrupt::SupervisorExternal => 0x200,
                Interrupt::UserExternal => 0x100,
                Interrupt::MachineTimer => 0x080,
                Interrupt::SupervisorTimer => 0x020,
                Interrupt::UserTimer => 0x010,
                Interrupt::MachineSoftware => 0x008,
                Interrupt::SupervisorSoftware => 0x002,
                Interrupt::UserSoftware => 0x001,
            });
        }
        self.wfi = false;

        panic!("TODO!!");
    }
}
