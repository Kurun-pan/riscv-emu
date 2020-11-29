use crate::bus::bus::Device;
use crate::console::Console;
use crate::cpu::cpu_csr::*;
use crate::cpu::cpu_instruction::{Opecode, OPECODES};
use crate::cpu::cpu_instruction_comp::*;
use crate::cpu::mmu::Mmu;
use crate::cpu::trap::*;
use crate::machine::Machine;

#[derive(Clone)]
pub enum Xlen {
    X32 = 0,
    X64 = 1,
}

#[derive(Clone, Debug)]
pub enum Privilege {
    User = 0,
    Supervisor = 1,
    Hypervisor = 2,
    Machine = 3,
}

pub struct Cpu {
    cycle: u64,
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
    pub fn new(machine_: Machine, console: Box<dyn Console>, testmode_: bool) -> Self {
        let mut cpu = Cpu {
            cycle: 0,
            pc: 0,
            wfi: false,
            xlen: Xlen::X64,
            privilege: Privilege::Machine,
            x: [0; 32],
            f: [0.0; 32],
            csr: Csr::new(),
            mmu: Mmu::new(Xlen::X64, machine_, console),
            testmode: testmode_,
        };
        cpu.x[0xb] = cpu.mmu.get_bus().get_base_address(Device::DTB) as i64; // initial value for Linux booting (DTB start address).
        cpu
    }

    pub fn reset(&mut self) {
        self.pc = 0;
        self.cycle = 0;
        self.privilege = Privilege::Machine;
        self.wfi = false;
        self.xlen = Xlen::X64;
        self.x = [0; 32];
        self.f = [0.0; 32];
    }

    pub fn set_pc(&mut self, pc: u64) {
        self.pc = pc;
    }

    pub fn set_xlen(&mut self, xlen: Xlen) {
        self.xlen = xlen;
        self.mmu.set_xlen(&self.xlen);
    }

    pub fn tick(&mut self) {
        match self.check_interrupts() {
            Some(interrupt) => self.interrupt_handler(interrupt),
            None => {}
        }

        if !self.wfi {
            let instruction_addr = self.pc;
            match self.tick_execute() {
                Ok(()) => {}
                Err(e) => self.catch_exception(e, instruction_addr),
            }
        }

        // run peripherals.
        let bus = self.mmu.get_bus();
        let irqs = bus.tick();

        // handle interrupt.
        self.tick_interrupt(&irqs);

        self.cycle = self.cycle.wrapping_add(1);
        self.csr.write_direct(CSR_CYCLE, self.cycle);
        self.csr.tick();
    }

    fn tick_execute(&mut self) -> Result<(), Trap> {
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
                _ => debug_message += "   ",
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
            println!("{}", debug_message);
        }
        match (instruction.operation)(self, instruction_addr, word) {
            Err(e) => return Err(e),
            _ => {}
        }
        self.x[0] = 0; // x0 is hardwired to the constant 0.

        return Ok(());
    }

    fn tick_interrupt(&mut self, irqs: &Vec<bool>) {
        let bus = self.mmu.get_bus();

        // set external interrupts to CSR register.
        if irqs[Privilege::Machine as usize] {
            self.csr.read_modify_write_direct(CSR_MIP, CSR_IP_MEIP, 0);
        } else {
            self.csr.read_modify_write_direct(CSR_MIP, 0, CSR_IP_MEIP);
        }

        if irqs[Privilege::Hypervisor as usize] {
            self.csr.read_modify_write_direct(CSR_MIP, CSR_IP_HEIP, 0);
        } else {
            self.csr.read_modify_write_direct(CSR_MIP, 0, CSR_IP_HEIP);
        }

        if irqs[Privilege::Supervisor as usize] {
            self.csr.read_modify_write_direct(CSR_SIP, CSR_IP_SEIP, 0);
        } else {
            self.csr.read_modify_write_direct(CSR_SIP, 0, CSR_IP_SEIP);
        }

        if irqs[Privilege::User as usize] {
            self.csr.read_modify_write_direct(CSR_UIP, CSR_IP_UEIP, 0);
        } else {
            self.csr.read_modify_write_direct(CSR_UIP, 0, CSR_IP_UEIP);
        }

        // set timer interrupt.
        if bus.is_pending_timer_interrupt(0) {
            self.csr.read_modify_write_direct(CSR_MIP, CSR_IP_MTIP, 0);
        } else {
            self.csr.read_modify_write_direct(CSR_MIP, 0, CSR_IP_MTIP);
        }

        // set software interrupt.
        if bus.is_pending_software_interrupt(0) {
            self.csr.read_modify_write_direct(CSR_MIP, CSR_IP_MSIP, 0);
        } else {
            self.csr.read_modify_write_direct(CSR_MIP, 0, CSR_IP_MSIP);
        }
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
                        value: self.pc.wrapping_sub(2),
                    }),
                };
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
        if self.testmode {
            println!(
                "  >> Exception: {:?} ({:016x}) {:?}, {:x}",
                trap.exception, trap.value, self.privilege, self.pc
            );
        }

        let trap_code = trap.exception as u8;
        let previous_privilege = self.privilege.clone();
        let next_privilege = self.get_next_privilege(trap_code, false);
        self.change_privilege(next_privilege);
        self.update_csr_trap_registers(addr, trap_code, trap.value, previous_privilege, false);
        self.pc = self.get_trap_next_pc();
    }

    fn check_interrupts(&mut self) -> Option<Interrupt> {
        let mie = self.csr.read_direct(CSR_MIE);
        let mip = self.csr.read_direct(CSR_MIP);
        let cause = mie & mip & 0xfff;
        //println!("mie: {:x}, mip: {:x}", mie, mip);

        // Check in order of priority.
        if cause & CSR_IP_MEIP > 0 && self.select_handling_interrupt(Interrupt::MachineExternal) {
            return Some(Interrupt::MachineExternal);
        }
        if cause & CSR_IP_MSIP > 0 && self.select_handling_interrupt(Interrupt::MachineSoftware) {
            return Some(Interrupt::MachineSoftware);
        }
        if cause & CSR_IP_MTIP > 0 && self.select_handling_interrupt(Interrupt::MachineTimer) {
            return Some(Interrupt::MachineTimer);
        }
        if cause & CSR_IP_HEIP > 0 {
            panic!("Unexpected event happend!");
        }
        if cause & CSR_IP_HTIP > 0 {
            panic!("Unexpected event happend!");
        }
        if cause & CSR_IP_HSIP > 0 {
            panic!("Unexpected event happend!");
        }
        if cause & CSR_IP_SEIP > 0 && self.select_handling_interrupt(Interrupt::SupervisorExternal)
        {
            return Some(Interrupt::SupervisorExternal);
        }
        if cause & CSR_IP_SSIP > 0 && self.select_handling_interrupt(Interrupt::SupervisorSoftware)
        {
            return Some(Interrupt::SupervisorSoftware);
        }
        if cause & CSR_IP_STIP > 0 && self.select_handling_interrupt(Interrupt::SupervisorTimer) {
            return Some(Interrupt::SupervisorTimer);
        }
        if cause & CSR_IP_UEIP > 0 && self.select_handling_interrupt(Interrupt::UserExternal) {
            return Some(Interrupt::UserExternal);
        }
        if cause & CSR_IP_UTIP > 0 && self.select_handling_interrupt(Interrupt::UserTimer) {
            return Some(Interrupt::UserTimer);
        }
        if cause & CSR_IP_USIP > 0 && self.select_handling_interrupt(Interrupt::UserSoftware) {
            return Some(Interrupt::UserSoftware);
        }
        None
    }

    fn interrupt_handler(&mut self, interrupt: Interrupt) {
        if self.testmode {
            let mie = self.csr.read_direct(CSR_MIE);
            let mip = self.csr.read_direct(CSR_MIP);
            println!("mie = {:x}, mip = {:x}", mie, mip);
            println!(
                "  >> Interrupt: {:?} ({:x}, {:?})",
                interrupt, self.pc, self.privilege
            );
        }

        let trap_code = interrupt as u8;
        let previous_privilege = self.privilege.clone();
        let next_privilege = self.get_next_privilege(trap_code, true);

        self.change_privilege(next_privilege);
        self.update_csr_trap_registers(self.pc, trap_code, self.pc, previous_privilege, true);
        self.pc = self.get_trap_next_pc();

        self.wfi = false;
    }

    fn _clear_interrupt(&mut self, interrupt: Interrupt) {
        let mip = self.csr.read_direct(CSR_MIP);
        self.csr.write_direct(
            CSR_MIP,
            mip & !match interrupt {
                Interrupt::MachineExternal => 0x800,
                Interrupt::SupervisorExternal => 0x200,
                Interrupt::UserExternal => 0x100,
                Interrupt::MachineTimer => 0x080,
                Interrupt::SupervisorTimer => 0x020,
                Interrupt::UserTimer => 0x010,
                Interrupt::MachineSoftware => 0x008,
                Interrupt::SupervisorSoftware => 0x002,
                Interrupt::UserSoftware => 0x001,
            },
        );
    }

    fn select_handling_interrupt(&mut self, interrupt: Interrupt) -> bool {
        let trap_code = interrupt as u8;
        let next_privilege = self.get_next_privilege(trap_code, true);
        let ie = match next_privilege {
            Privilege::User => self.csr.read_direct(CSR_UIE),
            Privilege::Supervisor => self.csr.read_direct(CSR_SIE),
            Privilege::Hypervisor => self.csr.read_direct(CSR_HIE),
            Privilege::Machine => self.csr.read_direct(CSR_MIE),
        };
        let status = self.csr.read_direct(match self.privilege {
            Privilege::User => CSR_USTATUS,
            Privilege::Supervisor => CSR_SSTATUS,
            Privilege::Hypervisor => CSR_HSTATUS,
            Privilege::Machine => CSR_MSTATUS,
        });

        let next_privilege_level = next_privilege.clone() as u8;
        let privilege_level = self.privilege.clone() as u8;
        if next_privilege_level < privilege_level {
            return false;
        }

        let uie = status & 1;
        let sie = (status >> 1) & 1;
        let hie = (status >> 2) & 1;
        let mie = (status >> 3) & 1;
        if privilege_level == next_privilege_level {
            match self.privilege {
                Privilege::User => {
                    if uie == 0 {
                        return false;
                    }
                }
                Privilege::Supervisor => {
                    if sie == 0 {
                        return false;
                    }
                }
                Privilege::Hypervisor => {
                    if hie == 0 {
                        return false;
                    }
                }
                Privilege::Machine => {
                    if mie == 0 {
                        return false;
                    }
                }
            };
        }

        match interrupt {
            Interrupt::MachineExternal => {
                let meie = (ie >> 11) & 1;
                if meie == 0 {
                    return false;
                }
            }
            Interrupt::MachineSoftware => {
                let msie = (ie >> 3) & 1;
                if msie == 0 {
                    return false;
                }
            }
            Interrupt::MachineTimer => {
                let mtie = (ie >> 7) & 1;
                if mtie == 0 {
                    return false;
                }
            }
            // TODO: support Hypervisor interrupts.
            Interrupt::SupervisorExternal => {
                let seie = (ie >> 9) & 1;
                if seie == 0 {
                    return false;
                }
            }
            Interrupt::SupervisorSoftware => {
                let ssie = (ie >> 1) & 1;
                if ssie == 0 {
                    return false;
                }
            }
            Interrupt::SupervisorTimer => {
                let stie = (ie >> 5) & 1;
                if stie == 0 {
                    return false;
                }
            }
            Interrupt::UserExternal => {
                let ueie = (ie >> 8) & 1;
                if ueie == 0 {
                    return false;
                }
            }
            Interrupt::UserSoftware => {
                let usie = ie & 1;
                if usie == 0 {
                    return false;
                }
            }
            Interrupt::UserTimer => {
                let utie = (ie >> 4) & 1;
                if utie == 0 {
                    return false;
                }
            }
        };
        true
    }

    /// update CSR/xEPC, xCAUSE, xTVAL, xSTATUS registers by interrupts.
    fn update_csr_trap_registers(
        &mut self,
        exception_pc: u64,
        trap_code: u8,
        trap_value: u64,
        previous_privilege: Privilege,
        is_interrupt: bool,
    ) {
        self.csr.write_direct(
            match self.privilege {
                Privilege::User => CSR_UEPC,
                Privilege::Supervisor => CSR_SEPC,
                Privilege::Hypervisor => CSR_HEPC,
                Privilege::Machine => CSR_MEPC,
            },
            exception_pc,
        );

        let cause = self.get_cause(trap_code, is_interrupt);
        self.csr.write_direct(
            match self.privilege {
                Privilege::User => CSR_UCAUSE,
                Privilege::Supervisor => CSR_SCAUSE,
                Privilege::Hypervisor => CSR_HCAUSE,
                Privilege::Machine => CSR_MCAUSE,
            },
            cause,
        );

        self.csr.write_direct(
            match self.privilege {
                Privilege::User => CSR_UTVAL,
                Privilege::Supervisor => CSR_STVAL,
                Privilege::Hypervisor => CSR_HTVAL,
                Privilege::Machine => CSR_MTVAL,
            },
            trap_value,
        );

        let status_reg = match self.privilege {
            Privilege::User => CSR_USTATUS,
            Privilege::Supervisor => CSR_SSTATUS,
            Privilege::Hypervisor => CSR_HSTATUS,
            Privilege::Machine => CSR_MSTATUS,
        };
        let p = self.privilege.clone() as u8;
        let ie = ((self.csr.read_direct(status_reg) >> p) & 0x1) as u64;
        self.csr.read_modify_write_direct(
            status_reg,
            match self.privilege {
                Privilege::User => panic!("TODO"),
                Privilege::Supervisor => (ie << 5) | ((previous_privilege as u64) << 8),
                Privilege::Hypervisor => panic!("TODO"),
                Privilege::Machine => (ie << 7) | ((previous_privilege as u64) << 11),
            },
            match self.privilege {
                Privilege::User => panic!("TODO"),
                Privilege::Supervisor => 0x122,
                Privilege::Hypervisor => panic!("TODO"),
                Privilege::Machine => 0x1888,
            },
        );
    }

    fn get_trap_next_pc(&mut self) -> u64 {
        self.pc = self.csr.read_direct(match self.privilege {
            Privilege::User => CSR_UTVEC,
            Privilege::Supervisor => CSR_STVEC,
            Privilege::Hypervisor => CSR_HTVEC,
            Privilege::Machine => CSR_MTVEC,
        });
        self.pc
    }

    fn get_cause(&mut self, trap_code: u8, is_interrupt: bool) -> u64 {
        let mut cause = trap_code as u64;
        if is_interrupt {
            cause |= match self.xlen {
                Xlen::X64 => 0x80000000_00000000 as u64,
                Xlen::X32 => 0x00000000_80000000 as u64,
            };
        }
        cause
    }

    fn get_next_privilege(&mut self, trap_code: u8, is_interrupt: bool) -> Privilege {
        let cause = self.get_cause(trap_code, is_interrupt) & 0xf;
        let mdeleg = self.csr.read_direct(match is_interrupt {
            true => CSR_MIDELEG,
            _ => CSR_MEDELEG,
        }) & 0xffffffff_fffff777;
        //let hdeleg = self.csr.read_direct(match is_interrupt {
        //        true => CSR_HIDELEG,
        //        _ => CSR_HEDELEG
        //}) & 0xffffffff_fffff333;
        let sdeleg = self.csr.read_direct(match is_interrupt {
            true => CSR_SIDELEG,
            _ => CSR_SEDELEG,
        }) & 0xffffffff_fffff111;

        match ((mdeleg >> cause) & 1) > 0 {
            //true => match ((hdeleg >> cause) & 1) > 0 {
            true => match ((sdeleg >> cause) & 1) > 0 {
                true => Privilege::User,
                false => Privilege::Supervisor,
            },
            //    false => Privilege::Hypervisor,
            //},
            false => Privilege::Machine,
        }
    }

    fn change_privilege(&mut self, next_privilege: Privilege) {
        self.privilege = next_privilege;
        self.mmu.set_privilege(&self.privilege);
    }
}
