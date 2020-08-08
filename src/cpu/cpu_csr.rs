use crate::cpu::cpu::PrivilegeMode;
use crate::trap::{Trap, Traps};

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
        cur_privilege: &PrivilegeMode,
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
        cur_privilege: &PrivilegeMode,
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

    fn to_u8(&self, privilege: &PrivilegeMode) -> u8 {
        match privilege {
            PrivilegeMode::User => 0,
            PrivilegeMode::Supervisor => 1,
            PrivilegeMode::Hypervisor => 2,
            PrivilegeMode::Machine => 3,
        }
    }
}
