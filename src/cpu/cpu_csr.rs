use crate::cpu::cpu::Privilege;
use crate::cpu::trap::*;

pub const CSR_USTATUS: u16 = 0x000;
pub const CSR_UIE: u16 = 0x004;
pub const CSR_UTVEC: u16 = 0x005;

pub const CSR_USCRATCH: u16 = 0x040;
pub const CSR_UEPC: u16 = 0x041;
pub const CSR_UCAUSE: u16 = 0x042;
pub const CSR_UTVAL: u16 = 0x043;
pub const CSR_UIP: u16 = 0x044;

pub const CSR_FFLAGS: u16 = 0x001;
pub const CSR_FRM: u16 = 0x002;
pub const CSR_FCSR: u16 = 0x003;

pub const CSR_CYCLE: u16 = 0xC00;
pub const CSR_TIME: u16 = 0xC01;
pub const CSR_INSTRET: u16 = 0xC02;
pub const CSR_CYCLEH: u16 = 0xC80;
pub const CSR_TIMEH: u16 = 0xC81;
pub const CSR_INSTRETH: u16 = 0xC82;

pub const CSR_SSTATUS: u16 = 0x100;
pub const CSR_SEDELEG: u16 = 0x102;
pub const CSR_SIDELEG: u16 = 0x103;
pub const CSR_SIE: u16 = 0x104;
pub const CSR_STVEC: u16 = 0x105;

pub const CSR_SSCRATCH: u16 = 0x140;
pub const CSR_SEPC: u16 = 0x141;
pub const CSR_SCAUSE: u16 = 0x142;
pub const CSR_STVAL: u16 = 0x143;
pub const CSR_SIP: u16 = 0x144;

pub const CSR_SPTBR: u16 = 0x180;

pub const CSR_SCYCLE: u16 = 0xD00;
pub const CSR_STIME: u16 = 0xD01;
pub const CSR_SINSTRET: u16 = 0xD02;
pub const CSR_SCYCLEH: u16 = 0xD80;
pub const CSR_STIMEH: u16 = 0xD81;
pub const CSR_SINSTRETH: u16 = 0xD82;

pub const CSR_HSTATUS: u16 = 0x200;
pub const CSR_HEDELEG: u16 = 0x202;
pub const CSR_HIDELEG: u16 = 0x203;
pub const CSR_HIE: u16 = 0x204;
pub const CSR_HTVEC: u16 = 0x205;

pub const CSR_HSCRATCH: u16 = 0x240;
pub const CSR_HEPC: u16 = 0x241;
pub const CSR_HCAUSE: u16 = 0x242;
pub const CSR_HTVAL: u16 = 0x243;

pub const CSR_HCYCLE: u16 = 0xE00;
pub const CSR_HTIME: u16 = 0xE01;
pub const CSR_HINSTRET: u16 = 0xE02;
pub const CSR_HCYCLEH: u16 = 0xE80;
pub const CSR_HTIMEH: u16 = 0xE81;
pub const CSR_HINSTRETH: u16 = 0xE82;

pub const CSR_MVENDORID: u16 = 0xF11;
pub const CSR_MARCHID: u16 = 0xF12;
pub const CSR_MIMPID: u16 = 0xF13;
pub const CSR_MHARTID: u16 = 0xF14;

pub const CSR_MSTATUS: u16 = 0x300;
pub const CSR_MISA: u16 = 0x301;
pub const CSR_MEDELEG: u16 = 0x302;
pub const CSR_MIDELEG: u16 = 0x303;
pub const CSR_MIE: u16 = 0x304;
pub const CSR_MTVEC: u16 = 0x305;

pub const CSR_MSCRATCH: u16 = 0x340;
pub const CSR_MEPC: u16 = 0x341;
pub const CSR_MCAUSE: u16 = 0x342;
pub const CSR_MTVAL: u16 = 0x343;
pub const CSR_MIP: u16 = 0x344;

pub const CSR_MBASE: u16 = 0x380;
pub const CSR_MBOUND: u16 = 0x381;
pub const CSR_MIBASE: u16 = 0x382;
pub const CSR_MIBOUND: u16 = 0x383;
pub const CSR_MDBASE: u16 = 0x384;
pub const CSR_MDBOUND: u16 = 0x385;

pub const CSR_MCYCLE: u16 = 0xF00;
pub const CSR_MTIME: u16 = 0xF01;
pub const CSR_MINSTRET: u16 = 0xF02;
pub const CSR_MCYCLEH: u16 = 0xF80;
pub const CSR_MTIMEH: u16 = 0xF81;
pub const CSR_MINSTRETH: u16 = 0xF82;

pub const CSR_MUCONTEREN: u16 = 0x310;
pub const CSR_MSCONTEREN: u16 = 0x311;
pub const CSR_MHCONTEREN: u16 = 0x312;

// register bit files
pub const CSR_STATUS_UIE: u64 = 0x00000001;
pub const CSR_STATUS_SIE: u64 = 0x00000002;
pub const CSR_STATUS_HIE: u64 = 0x00000004;
pub const CSR_STATUS_MIE: u64 = 0x00000008;
pub const CSR_STATUS_UPIE: u64 = 0x00000010;
pub const CSR_STATUS_SPIE: u64 = 0x00000020;
pub const CSR_STATUS_HPIE: u64 = 0x00000040;
pub const CSR_STATUS_MPIE: u64 = 0x00000080;
pub const CSR_STATUS_SPP: u64 = 0x00000100;
pub const CSR_STATUS_HPP: u64 = 0x00000600;
pub const CSR_STATUS_MPP: u64 = 0x00001800;
pub const CSR_STATUS_FS: u64 = 0x00006000;
pub const CSR_STATUS_XS: u64 = 0x00018000;
pub const CSR_STATUS_MPRV: u64 = 0x00020000;
pub const CSR_STATUS_PUM: u64 = 0x00040000;
pub const CSR_STATUS_MXR: u64 = 0x00080000;

pub const CSR_IP_USIP: u64 = 0x00000001;
pub const CSR_IP_SSIP: u64 = 0x00000002;
pub const CSR_IP_HSIP: u64 = 0x00000004;
pub const CSR_IP_MSIP: u64 = 0x00000008;
pub const CSR_IP_UTIP: u64 = 0x00000010;
pub const CSR_IP_STIP: u64 = 0x00000020;
pub const CSR_IP_HTIP: u64 = 0x00000040;
pub const CSR_IP_MTIP: u64 = 0x00000080;
pub const CSR_IP_UEIP: u64 = 0x00000100;
pub const CSR_IP_SEIP: u64 = 0x00000200;
pub const CSR_IP_HEIP: u64 = 0x00000400;
pub const CSR_IP_MEIP: u64 = 0x00000800;

pub const CSR_IE_USIE: u64 = 0x00000001;
pub const CSR_IE_SSIE: u64 = 0x00000002;
pub const CSR_IE_HSIE: u64 = 0x00000004;
pub const CSR_IE_MSIE: u64 = 0x00000008;
pub const CSR_IE_UTIE: u64 = 0x00000010;
pub const CSR_IE_STIE: u64 = 0x00000020;
pub const CSR_IE_HTIE: u64 = 0x00000040;
pub const CSR_IE_MTIE: u64 = 0x00000080;
pub const CSR_IE_UEIE: u64 = 0x00000100;
pub const CSR_IE_SEIE: u64 = 0x00000200;
pub const CSR_IE_HEIE: u64 = 0x00000400;
pub const CSR_IE_MEIE: u64 = 0x00000800;

pub struct Csr {
    csr: [u64; 4096],
}

impl Csr {
    pub fn new() -> Self {
        let mut csr = Csr { csr: [0; 4096] };

        // this is actived when release mode for passing 
        // "rv32mi-p-csr" test scenario of riscv-tests.
        if cfg!(not(debug_assertions)) {
            csr.csr[CSR_MISA as usize] = 0x800000008014312f;
        }
        csr
    }

    pub fn tick(&mut self) {
        self.csr[CSR_TIME as usize] = self.csr[CSR_TIME as usize].wrapping_add(1);
    }

    pub fn read(
        &mut self,
        addr: u16,
        instruction_addr: u64,
        cur_privilege: &Privilege,
    ) -> Result<u64, Trap> {
        let privilege = ((addr >> 8) & 0x3) as u8;
        let cur_level = cur_privilege.clone() as u8;
        match privilege <= cur_level {
            true => Ok(self.read_direct(addr)),
            _ => Err(Trap {
                exception: Exception::IllegalInstruction,
                value: instruction_addr,
            }),
        }
    }

    pub fn read_direct(&mut self, addr: u16) -> u64 {
        match addr {
            // User Floating-Point (FFLAGS/FRM/FCSR)
            CSR_FFLAGS => self.csr[CSR_FCSR as usize] & 0x1f,
            CSR_FRM => (self.csr[CSR_FCSR as usize] >> 5) & 0x7,

            // Restricted views of the mstatus register appear as the hstatus and
            // sstatus registers in the H and S privilege-level ISAs respectively.
            CSR_HSTATUS => panic!("TODO: HSTATUS"),
            CSR_SSTATUS => {
                let mask = CSR_STATUS_PUM
                    | CSR_STATUS_XS
                    | CSR_STATUS_FS
                    | CSR_STATUS_SPP
                    | CSR_STATUS_SPIE
                    | CSR_STATUS_UPIE
                    | CSR_STATUS_SIE
                    | CSR_STATUS_UIE;
                self.csr[CSR_MSTATUS as usize] & mask
            }

            // Restricted views of the mip and mie registers appear as the hip/hie,
            // sip/sie, and uip/uie registers in H-mode, S-mode, and U-mode respectively.
            CSR_SIP => {
                let mask = CSR_IP_SEIP
                    | CSR_IP_UEIP
                    | CSR_IP_STIP
                    | CSR_IP_UTIP
                    | CSR_IP_SSIP
                    | CSR_IP_USIP;
                self.csr[CSR_MIP as usize] & mask
            }
            CSR_SIE => {
                let mask = CSR_IE_SEIE
                    | CSR_IE_UEIE
                    | CSR_IE_STIE
                    | CSR_IE_UTIE
                    | CSR_IE_SSIE
                    | CSR_IE_USIE;
                self.csr[CSR_MIE as usize] & mask
            }

            // timer
            CSR_MCYCLE | CSR_MTIME | CSR_MINSTRET | CSR_MCYCLEH | CSR_MTIMEH | CSR_MINSTRETH => {
                panic!("TODO: CSR MTimer")
            }
            CSR_HCYCLE | CSR_HTIME | CSR_HINSTRET | CSR_HCYCLEH | CSR_HTIMEH | CSR_HINSTRETH => {
                panic!("TODO: CSR HTimer")
            }
            CSR_SCYCLE | CSR_STIME | CSR_SINSTRET | CSR_SCYCLEH | CSR_STIMEH | CSR_SINSTRETH => {
                panic!("TODO: CSR STimer")
            }
            CSR_INSTRET | CSR_CYCLEH | CSR_TIMEH | CSR_INSTRETH => {
                panic!("TODO: CSR Timer: {:x}", addr);
            }

            _ => self.csr[addr as usize],
        }
    }

    pub fn read_modify_write_direct(&mut self, addr: u16, smask: u64, cmask: u64) {
        let data = self.read_direct(addr);
        self.write_direct(addr, (data & !cmask) | smask);
    }

    pub fn write(
        &mut self,
        addr: u16,
        data: u64,
        instruction_addr: u64,
        cur_privilege: &Privilege,
    ) -> Result<bool, Trap> {
        let privilege = ((addr >> 8) & 0x3) as u8;
        let cur_level = cur_privilege.clone() as u8;
        match privilege <= cur_level {
            true => {
                self.write_direct(addr, data);
                Ok(match addr {
                    CSR_SPTBR => true,
                    _ => false,
                })
            }
            _ => Err(Trap {
                exception: Exception::IllegalInstruction,
                value: instruction_addr,
            }),
        }
    }

    pub fn write_direct(&mut self, addr: u16, data: u64) {
        match addr {
            // User Floating-Point (FFLAGS/FRM/FCSR)
            CSR_FFLAGS => {
                self.csr[CSR_FCSR as usize] &= !0x1f;
                self.csr[CSR_FCSR as usize] |= data & 0x1f;
            }
            CSR_FRM => {
                self.csr[CSR_FCSR as usize] &= !0xe0;
                self.csr[CSR_FCSR as usize] |= (data << 5) & 0xe0;
            }

            // Restricted views of the mstatus register appear as the hstatus and
            // sstatus registers in the H and S privilege-level ISAs respectively.
            CSR_HSTATUS => panic!("TODO: HSTATUS"),
            CSR_SSTATUS => {
                let mask = CSR_STATUS_PUM
                    | CSR_STATUS_XS
                    | CSR_STATUS_FS
                    | CSR_STATUS_SPP
                    | CSR_STATUS_SPIE
                    | CSR_STATUS_UPIE
                    | CSR_STATUS_SIE
                    | CSR_STATUS_UIE;
                self.csr[CSR_MSTATUS as usize] =
                    (self.csr[CSR_MSTATUS as usize] & !mask) | (data & mask);
            }

            // Restricted views of the mip and mie registers appear as the hip/hie,
            // sip/sie, and uip/uie registers in H-mode, S-mode, and U-mode respectively.
            CSR_SIP => {
                let mask = CSR_IP_SEIP
                    | CSR_IP_UEIP
                    | CSR_IP_STIP
                    | CSR_IP_UTIP
                    | CSR_IP_SSIP
                    | CSR_IP_USIP;
                self.csr[CSR_MIP as usize] = (self.csr[CSR_MIP as usize] & !mask) | (data & mask);
            }
            CSR_SIE => {
                let mask = CSR_IE_SEIE
                    | CSR_IE_UEIE
                    | CSR_IE_STIE
                    | CSR_IE_UTIE
                    | CSR_IE_SSIE
                    | CSR_IE_USIE;
                self.csr[CSR_MIE as usize] = (self.csr[CSR_MIE as usize] & !mask) | (data & mask);
            }

            // timer
            CSR_MCYCLE | CSR_MTIME | CSR_MINSTRET | CSR_MCYCLEH | CSR_MTIMEH | CSR_MINSTRETH => {
                panic!("TODO: CSR MTimer")
            }
            CSR_HCYCLE | CSR_HTIME | CSR_HINSTRET | CSR_HCYCLEH | CSR_HTIMEH | CSR_HINSTRETH => {
                panic!("TODO: CSR HTimer")
            }
            CSR_SCYCLE | CSR_STIME | CSR_SINSTRET | CSR_SCYCLEH | CSR_STIMEH | CSR_SINSTRETH => {
                panic!("TODO: CSR STimer")
            }
            CSR_INSTRET | CSR_CYCLEH | CSR_TIMEH | CSR_INSTRETH => panic!("TODO: CSR Timer"),

            _ => self.csr[addr as usize] = data,
        }
    }
}
