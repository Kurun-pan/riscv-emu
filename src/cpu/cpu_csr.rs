use crate::cpu::cpu::Privilege;
use crate::cpu::trap::{Trap, Traps};

pub const CSR_MSTATUS: u16 = 0x300;
pub const CSR_MEPC:    u16 = 0x341;

pub struct Csr {
    csr: [u64; 4096],
}

impl Csr {
    pub fn new() -> Self {
        let csr = Csr { csr: [0; 4096] };
        csr
    }

    pub fn read(
        &mut self,
        addr: u16,
        instruction_addr: u64,
        cur_privilege: &Privilege,
    ) -> Result<u64, Trap> {
        let privilege = ((addr >> 8) & 0x3) as u8;
        let cur_level = self.to_u8(&cur_privilege);
        match privilege <= cur_level {
            true => Ok(self.read_direct(addr)),
            _ => Err(Trap {
                factor: Traps::IllegalInstruction,
                value: instruction_addr,
            }),
        }
    }

    pub fn read_direct(&mut self, addr: u16) -> u64 {
        self.csr[addr as usize]
    }

    pub fn write(
        &mut self,
        addr: u16,
        data: u64,
        instruction_addr: u64,
        cur_privilege: &Privilege,
    ) -> Result<(), Trap> {
        let privilege = ((addr >> 8) & 0x3) as u8;
        let cur_level = self.to_u8(&cur_privilege);
        match privilege <= cur_level {
            true => {
                self.write_direct(addr, data);
                Ok(())
            }
            _ => Err(Trap {
                factor: Traps::IllegalInstruction,
                value: instruction_addr,
            }),
        }
    }

    pub fn write_direct(&mut self, addr: u16, data: u64) {
        self.csr[addr as usize] = data;
    }

    fn to_u8(&self, privilege: &Privilege) -> u8 {
        match privilege {
            Privilege::User => 0,
            Privilege::Supervisor => 1,
            Privilege::Hypervisor => 2,
            Privilege::Machine => 3,
        }
    }
}
