use std::path::Path;
use crate::tty::Tty;
use crate::cpu::cpu::{Cpu, Xlen};
use crate::elf_loader::{EMachine, EiClass, ElfLoader, ShType};
use crate::system_bus::DRAM_ADDRESS_START;

pub struct Emulator {
    cpu: Cpu,
    testmode: bool,
    tohost: u64,
}

impl Emulator {
    pub fn new(tty: Box<dyn Tty>, testmode: bool) -> Emulator {
        Self {
            cpu: Cpu::new(tty, testmode),
            testmode: testmode,
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
        self.cpu.mmu.get_bus().dram.initialize(data);
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
        let mut symtab_sec_headers = vec![];
        let mut strtab_sec_headers = vec![];
        for i in 0..sec_headers.len() {
            match sec_headers[i].sh_type {
                ShType::Progbits => progbits_sec_headers.push(&sec_headers[i]),
                ShType::Sysmtab => symtab_sec_headers.push(&sec_headers[i]),
                ShType::Strtab => strtab_sec_headers.push(&sec_headers[i]),
                _ => {}
            }
        }

        for i in 0..progbits_sec_headers.len() {
            if progbits_sec_headers[i].sh_addr >= DRAM_ADDRESS_START
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

        if self.testmode {
            self.tohost = match loader.search_tohost(&progbits_sec_headers, &strtab_sec_headers) {
                Some(addr) => addr,
                None => 0
            };
            println!(".tohost = {:x}", self.tohost);
        }
    }

    pub fn run(&mut self) -> Result<u32, u32> {
        println!("Start RISC-V Emulator!");
        loop {
            self.cpu.tick();
            if self.testmode && self.tohost != 0 {
                match self.cpu.mmu.read32_direct(self.tohost) {
                    Ok(data) => match data {
                        0 => {},
                        1 => return Ok(1),
                        n => return Err(n)
                    },
                    Err(e) => panic!("Faild to read .tohost: {:?}", e.exception)
                }
            }
        }
    }
}
