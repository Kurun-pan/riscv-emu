use std::fs::File;
use std::io::Read;
use std::path::Path;

use crate::bus::bus::Device;
use crate::cpu::cpu::{Cpu, Xlen};
use crate::elf_loader::{EMachine, EiClass, ElfLoader, ShType};
use crate::console::Console;

pub struct Emulator {
    cpu: Cpu,
    testmode: bool,
    tohost: u64,
}

impl Emulator {
    pub fn new(tty: Box<dyn Console>, testmode_: bool) -> Emulator {
        Self {
            cpu: Cpu::new(tty, testmode_),
            testmode: testmode_,
            tohost: 0,
        }
    }

    pub fn reset(&mut self) {
        self.cpu.reset()
    }

    pub fn set_pc(&mut self, addr: u64) {
        self.cpu.set_pc(addr)
    }

    pub fn set_disk_data(&mut self, filename: &Path) {
        match File::open(&filename) {
            Ok(mut file) => {
                let mut data = vec![];
                match file.read_to_end(&mut data) {
                    Err(why) => panic!("Failed to read {}: {}", filename.display(), why),
                    _ => {}
                };
                let bus = self.cpu.mmu.get_bus();
                bus.set_device_data(Device::Disk, data);
            }
            Err(why) => panic!("Falied to open {}: {}", filename.display(), why),
        };
    }

    pub fn set_dram_data(&mut self, data: Vec<u8>) {
        let bus = self.cpu.mmu.get_bus();
        bus.set_device_data(Device::Dram, data);
    }

    pub fn load_program(&mut self, filename: &Path) {
        let loader = match ElfLoader::new(filename) {
            Ok(elf_loader) => elf_loader,
            Err(()) => panic!(),
        };

        if !loader.is_elf() {
            panic!("{} is invalid ELF file.", filename.display());
        }

        let elf_header = loader.get_elf_header();
        match elf_header.e_machine {
            EMachine::RISCV => {}
            _ => panic!("{} is not program for RISC-V machine!", filename.display()),
        }
        self.cpu.set_pc(elf_header.e_entry);
        self.cpu.set_xlen(match elf_header.e_indent.ei_classs {
            EiClass::Class32 => Xlen::X32,
            EiClass::Class64 => Xlen::X64,
            _ => panic!("Unexpected class size: {:?}", elf_header.e_indent.ei_classs),
        });

        let sec_headers = loader.get_section_header(&elf_header);
        let mut progbits_sec_headers = vec![];
        let mut strtab_sec_headers = vec![];
        for i in 0..sec_headers.len() {
            match sec_headers[i].sh_type {
                ShType::Progbits => progbits_sec_headers.push(&sec_headers[i]),
                ShType::Strtab => strtab_sec_headers.push(&sec_headers[i]),
                _ => {}
            }
        }

        let target_device_addr;
        if cfg!(feature = "nuttx") {
            target_device_addr = self.cpu.mmu.get_bus().get_base_address(Device::SpiFlash);
        } else {
            target_device_addr = self.cpu.mmu.get_bus().get_base_address(Device::Dram);
        }

        let program_headers = loader.get_program_header(&elf_header);
        for i in 0..progbits_sec_headers.len() {
            if !((progbits_sec_headers[i].sh_addr >= target_device_addr)
                && progbits_sec_headers[i].sh_offset > 0)
            {
                continue;
            }

            let mut p_addr = progbits_sec_headers[i].sh_addr;
            for k in 0..program_headers.len() {
                if progbits_sec_headers[i].sh_addr == program_headers[k].p_vaddr {
                    p_addr = program_headers[k].p_paddr;
                    break;
                }
            }

            for j in 0..progbits_sec_headers[i].sh_size {
                let data = loader.read8((progbits_sec_headers[i].sh_offset + j) as usize);
                match self.cpu.mmu.write8(p_addr + j as u64, data) {
                    Err(e) => panic!("{:?}", e.exception),
                    _ => {}
                }
            }
        }

        if self.testmode {
            self.tohost = match loader.search_tohost(&progbits_sec_headers, &strtab_sec_headers) {
                Some(addr) => addr,
                None => 0,
            };
        }
    }

    pub fn run(&mut self) -> Result<u32, u32> {
        loop {
            self.cpu.tick();
            if self.testmode && self.tohost != 0 {
                match self.cpu.mmu.read32_direct(self.tohost) {
                    Ok(data) => match data {
                        0 => {}
                        1 => return Ok(1),
                        n => return Err(n),
                    },
                    Err(e) => panic!("Faild to read .tohost: {:?}", e.exception),
                }
            }
        }
    }
}
