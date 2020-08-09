use crate::cpu::cpu::{Cpu, Xlen};
use crate::elf_loader::{EMachine, EiClass, ElfLoader, ShType};
use crate::system_bus::DRAM_BASE_ADDRESS;

use std::path::Path;

pub struct Emulator {
    pub cpu: Cpu,
    tohost: u64,
}

impl Emulator {
    pub fn new() -> Emulator {
        Self {
            cpu: Cpu::new(),
            tohost: 0,
        }
    }

    pub fn reset(&mut self) {
        self.cpu.reset()
    }

    pub fn set_pc(&mut self, addr: u64) {
        self.cpu.set_pc(addr)
    }

    pub fn load_dram_data(&mut self, data: Vec<u8>) {
        self.cpu.mmu.bus.dram.initialize(data);
    }

    pub fn load_program(&mut self, filename: &Path) {
        let loader = match ElfLoader::new(filename) {
            Ok(elf_loader) => elf_loader,
            Err(()) => panic!(),
        };

        match loader.is_elf() {
            false => panic!("{} is not ELF file.", filename.display()),
            _ => {}
        }

        let elf_header = loader.get_elf_header();
        match elf_header.e_machine {
            EMachine::EM_RISCV => {}
            _ => panic!("{} is not program for RISC-V machine!", filename.display()),
        }
        self.cpu.set_pc(elf_header.e_entry);
        self.cpu.set_xlen(match elf_header.e_indent.ei_classs {
            EiClass::ELFCLASS32 => Xlen::X32,
            EiClass::ELFCLASS64 => Xlen::X64,
            _ => panic!("Unexpected class size: {:?}", elf_header.e_indent.ei_classs),
        });

        let sec_headers = loader.get_section_header(&elf_header);
        let mut progbits_sec_headers = vec![];
        let mut symtab_sec_headers = vec![];
        let mut strtab_sec_headers = vec![];
        for i in 0..sec_headers.len() {
            match sec_headers[i].sh_type {
                ShType::SHT_PROGBITS => progbits_sec_headers.push(&sec_headers[i]),
                ShType::SHT_SYSMTAB => symtab_sec_headers.push(&sec_headers[i]),
                ShType::SHT_STRTAB => strtab_sec_headers.push(&sec_headers[i]),
                _ => {}
            }
        }

        for i in 0..progbits_sec_headers.len() {
            if progbits_sec_headers[i].sh_addr >= DRAM_BASE_ADDRESS
                && progbits_sec_headers[i].sh_offset > 0
            {
                for j in 0..progbits_sec_headers[i].sh_size {
                    let data = loader.read8((progbits_sec_headers[i].sh_offset + j) as usize);
                    match self
                        .cpu
                        .mmu
                        .write8(progbits_sec_headers[i].sh_addr + j as u64, data)
                    {
                        Err(e) => panic!("{:?}", e.exception),
                        _ => {}
                    }
                }
            }
        }

        self.tohost = match loader.search_tohost(&progbits_sec_headers, &strtab_sec_headers) {
            Some(addr) => addr,
            None => panic!("Not found .tohost section!!")
        };
        println!(".tohost = {:x}", self.tohost);
    }

    pub fn run(&mut self) -> Result<u64, ()> {
        println!("Start RISC-V Emulator!");
        loop {
            self.cpu.tick();
            match self.cpu.mmu.read32(self.tohost) {
                Ok(data) => match data {
                    0 => {},
                    1 => return Ok(1),
                    _ => return Err(())
                },
                Err(e) => panic!("Faild to read .tohost")
            }
        }
    }
}
