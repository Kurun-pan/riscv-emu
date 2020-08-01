use crate::cpu::cpu::Cpu;

pub struct Emulator {
    pub cpu: Cpu,
}

impl Emulator {
    pub fn new() -> Emulator {
        Self {
            cpu: Cpu::new(),
        }
    }

    pub fn reset(&mut self) {
        self.cpu.reset()
    }

    pub fn set_pc(&mut self, pc: u64) {
        self.cpu.set_pc(pc);
    }

    pub fn load_dram_data(&mut self, data: Vec<u8>) {
        self.cpu.mmu.bus.dram.initialize(data);
    }

    pub fn run(&mut self) {
        println!("Start RISC-V Emulator!");
        loop {
            self.cpu.tick();
        }
    }    
}
