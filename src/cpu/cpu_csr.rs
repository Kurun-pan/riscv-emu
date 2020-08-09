use crate::cpu::cpu::Privilege;
use crate::cpu::trap::*;

pub const CSR_USTATUS:   u16 = 0x000;
pub const CSR_UIE:       u16 = 0x004;
pub const CSR_UTVEC:     u16 = 0x005;

pub const CSR_USCRATCH:  u16 = 0x040;
pub const CSR_UEPC:      u16 = 0x041;
pub const CSR_UCAUSE:    u16 = 0x042;
pub const CSR_UTVAL:     u16 = 0x043;
pub const CSR_UIP:       u16 = 0x044;

pub const CSR_FFLAGS:    u16 = 0x001;
pub const CSR_FRM:       u16 = 0x002;
pub const CSR_FCSR:      u16 = 0x003;

pub const CSR_CYCLE:     u16 = 0xC00;
pub const CSR_TIME:      u16 = 0xC01;
pub const CSR_INSTRET:   u16 = 0xC02;
pub const CSR_CYCLEH:    u16 = 0xC80;
pub const CSR_TIMEH:     u16 = 0xC81;
pub const CSR_INSTRETH:  u16 = 0xC82;

pub const CSR_SSTATUS:   u16 = 0x100;
pub const CSR_SEDELEG:   u16 = 0x102;
pub const CSR_SIDELEG:   u16 = 0x103;
pub const CSR_SIE:       u16 = 0x104;
pub const CSR_STVEC:     u16 = 0x105;

pub const CSR_SSCRATCH:  u16 = 0x140;
pub const CSR_SEPC:      u16 = 0x141;
pub const CSR_SCAUSE:    u16 = 0x142;
pub const CSR_STVAL:     u16 = 0x143;
pub const CSR_SIP:       u16 = 0x144;

pub const CSR_SPTBR:     u16 = 0x180;

pub const CSR_SCYCLE:    u16 = 0xD00;
pub const CSR_STIME:     u16 = 0xD01;
pub const CSR_SINSTRET:  u16 = 0xD02;
pub const CSR_SCYCLEH:   u16 = 0xD80;
pub const CSR_STIMEH:    u16 = 0xD81;
pub const CSR_SINSTRETH: u16 = 0xD82;

pub const CSR_HSTATUS:   u16 = 0x200;
pub const CSR_HEDELEG:   u16 = 0x202;
pub const CSR_HIDELEG:   u16 = 0x203;
pub const CSR_HIE:       u16 = 0x204;
pub const CSR_HTVEC:     u16 = 0x205;

pub const CSR_HSCRATCH:  u16 = 0x240;
pub const CSR_HEPC:      u16 = 0x241;
pub const CSR_HCAUSE:    u16 = 0x242;
pub const CSR_HTVAL:     u16 = 0x243;

pub const CSR_HCYCLE:    u16 = 0xE00;
pub const CSR_HTIME:     u16 = 0xE01;
pub const CSR_HINSTRET:  u16 = 0xE02;
pub const CSR_HCYCLEH:   u16 = 0xE80;
pub const CSR_HTIMEH:    u16 = 0xE81;
pub const CSR_HINSTRETH: u16 = 0xE82;

pub const CSR_MISA:      u16 = 0xF10;
pub const CSR_MVENDORID: u16 = 0xF11;
pub const CSR_MARCHID:   u16 = 0xF12;
pub const CSR_MIMPID:    u16 = 0xF13;
pub const CSR_MHARTID:   u16 = 0xF14;

pub const CSR_MSTATUS:   u16 = 0x300;
pub const CSR_MEDELEG:   u16 = 0x302;
pub const CSR_MIDELEG:   u16 = 0x303;
pub const CSR_MIE:       u16 = 0x304;
pub const CSR_MTVEC:     u16 = 0x305;

pub const CSR_MSCRATCH:  u16 = 0x340;
pub const CSR_MEPC:      u16 = 0x341;
pub const CSR_MCAUSE:    u16 = 0x342;
pub const CSR_MTVAL:     u16 = 0x343;
pub const CSR_MIP:       u16 = 0x344;

pub const CSR_MBASE:     u16 = 0x380;
pub const CSR_MBOUND:    u16 = 0x381;
pub const CSR_MIBASE:    u16 = 0x382;
pub const CSR_MIBOUND:   u16 = 0x383;
pub const CSR_MDBASE:    u16 = 0x384;
pub const CSR_MDBOUND:   u16 = 0x385;

pub const CSR_MCYCLE:    u16 = 0xF00;
pub const CSR_MTIME:     u16 = 0xF01;
pub const CSR_MINSTRET:  u16 = 0xF02;
pub const CSR_MCYCLEH:   u16 = 0xF80;
pub const CSR_MTIMEH:    u16 = 0xF81;
pub const CSR_MINSTRETH: u16 = 0xF82;

pub const CSR_MUCONTEREN: u16 = 0x310;
pub const CSR_MSCONTEREN: u16 = 0x311;
pub const CSR_MHCONTEREN: u16 = 0x312;

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
                exception: Exception::IllegalInstruction,
                value: instruction_addr
            })
        }
    }

    pub fn read_direct(&mut self, addr: u16) -> u64 {
        self.csr[addr as usize]
    }

    pub fn read_modify_write_direct(&mut self, addr: u16, bitmask: u64) {
        let data = self.csr[addr as usize];
        self.csr[addr as usize] = data | bitmask;
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
                exception: Exception::IllegalInstruction,
                value: instruction_addr
            })
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
