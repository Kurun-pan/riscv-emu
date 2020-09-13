// PLIC (Platform-Level Interrupt Controller)
// https://static.dev.sifive.com/FU540-C000-v1.0.pdf

// qemu puts programmable interrupt controller here.
/*
#define PLIC 0x0c000000L
#define PLIC_PRIORITY (PLIC + 0x0)
#define PLIC_PENDING (PLIC + 0x1000)
#define PLIC_MENABLE(hart) (PLIC + 0x2000 + (hart)*0x100)
#define PLIC_SENABLE(hart) (PLIC + 0x2080 + (hart)*0x100)
#define PLIC_MPRIORITY(hart) (PLIC + 0x200000 + (hart)*0x2000)
#define PLIC_SPRIORITY(hart) (PLIC + 0x201000 + (hart)*0x2000)
#define PLIC_MCLAIM(hart) (PLIC + 0x200004 + (hart)*0x2000)
#define PLIC_SCLAIM(hart) (PLIC + 0x201004 + (hart)*0x2000)
*/

use crate::peripherals::intc::Intc;

const _PLIC_PRIORITY_BASE: u64 = 0;
const PLIC_PENDING_BASE: u64 = 0x1000;
const PLIC_MENABLE_BASE: u64 = 0x2000;
const PLIC_SENABLE_BASE: u64 = 0x2080;
const PLIC_MTHRESHOLD_BASE: u64 = 0x200000;
const PLIC_STHRESHOLD_BASE: u64 = 0x201000;
const PLIC_MCLAIM_BASE: u64 = 0x200004;
const PLIC_SCLAIM_BASE: u64 = 0x201004;

const PLIC_CORE_MAX: usize = 5;
const PLIC_INT_MAX: usize = 0x1000 / 4;

pub struct Plic {
    priority: [u32; PLIC_INT_MAX],
    pending: u32, // RO
    menable: [u32; PLIC_CORE_MAX],
    senable: [u32; PLIC_CORE_MAX],
    mthreshold: [u32; PLIC_CORE_MAX],
    sthreshold: [u32; PLIC_CORE_MAX],
    mclaim: [u32; PLIC_CORE_MAX],
    sclaim: [u32; PLIC_CORE_MAX],
}

impl Plic {
    pub fn new() -> Self {
        Plic {
            priority: [0; PLIC_INT_MAX],
            pending: 0,
            menable: [0; PLIC_CORE_MAX],
            senable: [0; PLIC_CORE_MAX],
            mthreshold: [0; PLIC_CORE_MAX],
            sthreshold: [0; PLIC_CORE_MAX],
            mclaim: [0; PLIC_CORE_MAX],
            sclaim: [0; PLIC_CORE_MAX],
        }
    }
}

impl Intc for Plic {
    fn tick(&mut self, core: usize, interrupts: Vec<usize>) -> Vec<bool> {
        let mut irq_m = 0;
        let mut max_priority_m = 0;
        let mut irq_s = 0;
        let mut max_priority_s = 0;
        for id in interrupts {
            if ((self.menable[core] >> id) & 0x1) > 0 {
                if self.priority[id] > self.mthreshold[core] && self.priority[id] > max_priority_m {
                    irq_m = id as u32;
                    max_priority_m = self.priority[id];
                }
            }
            if ((self.senable[core] >> id) & 0x1) > 0 {
                if self.priority[id] > self.sthreshold[core] && self.priority[id] > max_priority_s {
                    irq_s = id as u32;
                    max_priority_s = self.priority[id];
                }
            }
        }

        let mut irqs = vec![false, false, false, false];
        if irq_m != 0 {
            irqs[3] = true;
            self.mclaim[core] = irq_m;
        }
        irqs[2] = false; // Hypervisor
        if irq_s != 0 {
            irqs[1] = true;
            self.sclaim[core] = irq_s;
        }
        irqs[0] = false; // User
        irqs
    }

    /// The PLIC memory map has been designed to only require naturally
    /// aligned 32-bit memory accesses.
    fn read(&mut self, addr: u64) -> u32 {
        let e_addr = addr & 0x3f_fffc;
        if e_addr < PLIC_PENDING_BASE {
            let idx = e_addr >> 2;
            if idx < PLIC_INT_MAX as u64 {
                return self.priority[idx as usize];
            } else {
                panic!("Read to reserved area: {:x}", addr);
            }
        }
        if e_addr < PLIC_MENABLE_BASE {
            match e_addr {
                0x1000 => return self.pending,
                _ => panic!("Read to reserved area: {:x}", addr),
            }
        } else if e_addr < PLIC_MTHRESHOLD_BASE {
            if e_addr & 0x80 == 0 {
                if e_addr <= PLIC_MENABLE_BASE + 0x100 * PLIC_CORE_MAX as u64 {
                    let idx = ((e_addr - PLIC_MENABLE_BASE) / 0x100) as usize;
                    return self.menable[idx];
                } else {
                    panic!("Write to reserved area: {:x}", addr);
                }
            } else {
                if e_addr <= PLIC_SENABLE_BASE + 0x100 * PLIC_CORE_MAX as u64 {
                    let idx = ((e_addr - PLIC_SENABLE_BASE) / 0x100) as usize;
                    return self.senable[idx];
                } else {
                    panic!("Write to reserved area: {:x}", addr);
                }
            }
        } else {
            if e_addr & 0x1000 == 0 {
                if e_addr & 0x4 == 0 {
                    if e_addr <= PLIC_MTHRESHOLD_BASE + 0x2000 * PLIC_CORE_MAX as u64 {
                        let idx = ((e_addr - PLIC_MTHRESHOLD_BASE) / 0x2000) as usize;
                        return self.mthreshold[idx];
                    } else {
                        panic!("Write to reserved area: {:x}", addr);
                    }
                } else {
                    if e_addr <= PLIC_MCLAIM_BASE + 0x2000 * PLIC_CORE_MAX as u64 {
                        let idx = ((e_addr - PLIC_MCLAIM_BASE) / 0x2000) as usize;
                        return self.mclaim[idx];
                    } else {
                        panic!("Write to reserved area: {:x}", addr);
                    }
                }
            } else {
                if e_addr & 0x4 == 0 {
                    if e_addr <= PLIC_STHRESHOLD_BASE + 0x2000 * PLIC_CORE_MAX as u64 {
                        let idx = ((e_addr - PLIC_STHRESHOLD_BASE) / 0x2000) as usize;
                        return self.sthreshold[idx];
                    } else {
                        panic!("Write to reserved area: {:x}", addr);
                    }
                } else {
                    if e_addr <= PLIC_SCLAIM_BASE + 0x2000 * PLIC_CORE_MAX as u64 {
                        let idx = ((e_addr - PLIC_SCLAIM_BASE) / 0x2000) as usize;
                        return self.sclaim[idx];
                    } else {
                        panic!("Read to reserved area: {:x}", addr);
                    }
                }
            }
        }
    }

    fn write(&mut self, addr: u64, data: u32) {
        let e_addr = addr & 0x3f_fffc;
        if e_addr < PLIC_PENDING_BASE {
            let idx = e_addr >> 2;
            if idx < PLIC_INT_MAX as u64 {
                self.priority[idx as usize] = data;
            } else {
                panic!("Write to reserved area: {:x}", addr);
            }
        } else if e_addr < PLIC_MENABLE_BASE {
            match e_addr {
                0x1000 => self.pending = data,
                _ => panic!("Write to reserved area: {:x}", addr),
            }
        } else if e_addr < PLIC_MTHRESHOLD_BASE {
            if e_addr & 0x80 == 0 {
                if e_addr <= PLIC_MENABLE_BASE + 0x100 * PLIC_CORE_MAX as u64 {
                    let idx = ((e_addr - PLIC_MENABLE_BASE) / 0x100) as usize;
                    self.menable[idx] = data;
                } else {
                    panic!("Write to reserved area: {:x}", addr);
                }
            } else {
                if e_addr <= PLIC_SENABLE_BASE + 0x100 * PLIC_CORE_MAX as u64 {
                    let idx = ((e_addr - PLIC_SENABLE_BASE) / 0x100) as usize;
                    self.senable[idx] = data;
                } else {
                    panic!("Write to reserved area: {:x}", addr);
                }
            }
        } else {
            if e_addr & 0x1000 == 0 {
                if e_addr & 0x4 == 0 {
                    if e_addr <= PLIC_MTHRESHOLD_BASE + 0x2000 * PLIC_CORE_MAX as u64 {
                        let idx = ((e_addr - PLIC_MTHRESHOLD_BASE) / 0x2000) as usize;
                        self.mthreshold[idx] = data;
                    } else {
                        panic!("Write to reserved area: {:x}", addr);
                    }
                } else {
                    if e_addr <= PLIC_MCLAIM_BASE + 0x2000 * PLIC_CORE_MAX as u64 {
                        let idx = ((e_addr - PLIC_MCLAIM_BASE) / 0x2000) as usize;
                        // clear the interrupt when it writes the same interrupt id to the register.
                        if self.mclaim[idx] == data {
                            self.mclaim[idx] = 0;
                        }
                    } else {
                        panic!("Write to reserved area: {:x}", addr);
                    }
                }
            } else {
                if e_addr & 0x4 == 0 {
                    if e_addr <= PLIC_STHRESHOLD_BASE + 0x2000 * PLIC_CORE_MAX as u64 {
                        let idx = ((e_addr - PLIC_STHRESHOLD_BASE) / 0x2000) as usize;
                        self.sthreshold[idx] = data;
                    } else {
                        panic!("Write to reserved area: {:x}", addr);
                    }
                } else {
                    if e_addr <= PLIC_SCLAIM_BASE + 0x2000 * PLIC_CORE_MAX as u64 {
                        let idx = ((e_addr - PLIC_SCLAIM_BASE) / 0x2000) as usize;
                        // clear the interrupt when it writes the same interrupt id to the register.
                        if self.sclaim[idx] == data {
                            self.sclaim[idx] = 0;
                        }
                    } else {
                        panic!("Write to reserved area: {:x}", addr);
                    }
                }
            }
        }
    }
}
