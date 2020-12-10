const HEADER_MAGIC: u32 = 0x464c457f; // 0x7f 'E' 'L' 'F'
const TOHOST: u64 = 0x0074736f686f742e; // .tohost

pub struct ElfHeader {
    pub e_indent: Ei,
    pub e_type: EType,
    pub e_machine: EMachine,
    pub e_version: EVersion,

    pub e_entry: u64,
    pub e_phoff: u64,
    pub e_shoff: u64,
    pub e_flags: u32,
    pub e_ehsize: u16,
    pub e_phentsize: u16,
    pub e_phnum: u16,
    pub e_shentsize: u16,
    pub e_shnum: u16,
    pub e_shstrndx: u16,
}

pub struct ProgramHeader {
    pub p_type: u32,
    pub p_flags: u32,
    pub p_offset: u64,
    pub p_vaddr: u64,
    pub p_paddr: u64,
    pub p_filesz: u64,
    pub p_memsz: u64,
    pub p_alignz: u64,
}

pub struct SectionHeader {
    pub sh_name: u32,
    pub sh_type: ShType,
    pub sh_flags: u64,
    pub sh_addr: u64,
    pub sh_offset: u64,
    pub sh_size: u64,
    pub sh_link: u32,
    pub sh_info: u32,
    pub sh_addralign: u64,
    pub sh_entsize: u64,
}

#[derive(Debug)]
pub enum ShType {
    Null = 0x0,          // Section header table entry unused
    Progbits = 0x1,      // Program data (.text, data, etc)
    Sysmtab = 0x2,       // Symbol table
    Strtab = 0x3,        // String table
    Rela = 0x4,          // Relocation entries with addends
    Hash = 0x5,          // Symbol hash table
    Dynamic = 0x6,       // Dynamic linking information
    Note = 0x7,          // Notes
    Nobits = 0x8,        // Program space with no data (bss)
    Rel = 0x9,           // Relocation entries, no addends
    Shlib = 0xA,         // Reserved
    Dynsym = 0xB,        // Dynamic linker symbol table
    InitArray = 0xE,     // Array of constructors
    FiniArray = 0xF,     // Array of destructors
    PreinitArray = 0x10, // Array of pre-constructors
    Group = 0x11,        // Section group
    SymtabShndx = 0x12,  // Extended section indices
    Num = 0x13,          // Number of defined types
    Loproc = 0x70000000, //
    Hiproc = 0x7F000000, //
    //Louser = 0x80000000, //
    //Hiuser = 0xFFFFFFFF, //
}

#[derive(Debug)]
pub enum ShFlag {
    Write = 0x1,
    Alloc = 0x2,
    Execinstr = 0x4,
    Merge = 0x10,
    Strings = 0x20,
    InfoLink = 0x40,
    LinkOrder = 0x80,
    OsNonconforming = 0x100,
    Group = 0x200,
    Tls = 0x400,
    Maskos = 0x0ff00000,
    //MaskProc = 0xf0000000,
    Ordered = 0x4000000,
    Exclude = 0x8000000,
}

#[derive(Debug)]
pub struct Ei {
    pub ei_classs: EiClass,
    pub ei_data: EiData,
    pub ei_version: EiVersion,
    pub ei_osabi: EiOsAbi,
    pub ei_abiversion: u8,
}

#[derive(Debug)]
pub enum EiClass {
    None,
    Class32,
    Class64,
}

#[derive(Debug)]
pub enum EiData {
    None,
    D2Lsb,
    D2Msb,
}

#[derive(Debug)]
pub enum EiVersion {
    None,
    Current,
}

#[derive(Debug)]
pub enum EiOsAbi {
    SystemV,
    HpUx,
    NetBsd,
    Linux,
    GnuHurd,
    Solaris,
    Aix,
    Irix,
    FreeBsd,
    Tru64,
    NovellModestro,
    OpenBsd,
    OpenVms,
    NonStopKernel,
    Aros,
    FenixOs,
    CloudAbi,
    StartusTechnologiesOpenVos,
}

#[derive(Debug)]
pub enum EType {
    None,
    Rel,
    Exec,
    Dyn,
    Core,
    Loos,
    Hios,
    Loproc,
    Hiproc,
}

#[derive(Debug)]
pub enum EMachine {
    None,      // Unknown machine
    M32,       // AT&T WE 32100
    SPARC,     // SPARC
    X86,       // x86
    M68K,      // Motorola 68000
    M88K,      // Motorola 88000
    I486,      // Intel 80486
    I860,      // Intel 80860
    MIPS,      // MIPS R3000 Big-Endian only
    MipsRs4Be, // MIPS R4000 Big-Endian
    PARISC,    // HPPA
    I960,      // Intel 80960
    PPC,       // PowerPC
    PPC64,     // PowerPC 64-bit
    S390,      // S390 including S390x
    ARM,       // ARM (up to ARMv7)
    SUPERH,    // SuperH
    IA64,      // IA-64,
    AMD64,     // amd64,
    TMS320,    // TMS320C5000 Family
    ARM64,     // ARM 64-bit
    RISCV,     // RISC-V
}

#[derive(Debug)]
pub enum EVersion {
    None,
    Current,
}

pub struct ElfLoader {
    data: Vec<u8>,
}

impl ElfLoader {
    pub fn new(data: Vec<u8>) -> Result<Self, ()> {
        Ok(Self { data: data })
    }

    pub fn is_elf(&self) -> bool {
        self.read32(0) == HEADER_MAGIC
    }

    pub fn get_elf_header(&self) -> ElfHeader {
        let ei = Ei {
            ei_classs: match self.read8(4) {
                0 => EiClass::None,
                1 => EiClass::Class32,
                2 => EiClass::Class64,
                n => panic!("Unknown e_ident class {}", n),
            },
            ei_data: match self.read8(5) {
                0 => EiData::None,
                1 => EiData::D2Lsb,
                2 => EiData::D2Msb,
                n => panic!("Unknown e_ident endian {}", n),
            },
            ei_version: match self.read8(6) {
                0 => EiVersion::None,
                1 => EiVersion::Current,
                n => panic!("Unknown e_ident version {}", n),
            },
            ei_osabi: match self.read8(7) {
                0x00 => EiOsAbi::SystemV,
                0x01 => EiOsAbi::HpUx,
                0x02 => EiOsAbi::NetBsd,
                0x03 => EiOsAbi::Linux,
                0x04 => EiOsAbi::GnuHurd,
                0x06 => EiOsAbi::Solaris,
                0x07 => EiOsAbi::Aix,
                0x08 => EiOsAbi::Irix,
                0x09 => EiOsAbi::FreeBsd,
                0x0A => EiOsAbi::Tru64,
                0x0B => EiOsAbi::NovellModestro,
                0x0C => EiOsAbi::OpenBsd,
                0x0D => EiOsAbi::OpenVms,
                0x0E => EiOsAbi::NonStopKernel,
                0x0F => EiOsAbi::Aros,
                0x10 => EiOsAbi::FenixOs,
                0x11 => EiOsAbi::CloudAbi,
                0x12 => EiOsAbi::StartusTechnologiesOpenVos,
                n => panic!("Unknown e_ident version {}", n),
            },
            ei_abiversion: self.read8(8),
        };

        let e_entry = match ei.ei_classs {
            EiClass::Class32 => self.read32(0x18) as u64,
            _ => self.read64(0x18),
        };
        let e_phoff = match ei.ei_classs {
            EiClass::Class32 => self.read32(0x1C) as u64,
            _ => self.read64(0x20),
        };
        let e_shoff = match ei.ei_classs {
            EiClass::Class32 => self.read32(0x20) as u64,
            _ => self.read64(0x28),
        };
        let e_flags = match ei.ei_classs {
            EiClass::Class32 => self.read32(0x24),
            _ => self.read32(0x30),
        };
        let e_ehsize = match ei.ei_classs {
            EiClass::Class32 => self.read16(0x28),
            _ => self.read16(0x34),
        };
        let e_phentsize = match ei.ei_classs {
            EiClass::Class32 => self.read16(0x2A),
            _ => self.read16(0x36),
        };
        let e_phnum = match ei.ei_classs {
            EiClass::Class32 => self.read16(0x2C),
            _ => self.read16(0x38),
        };
        let e_shentsize = match ei.ei_classs {
            EiClass::Class32 => self.read16(0x2E),
            _ => self.read16(0x3A),
        };
        let e_shnum = match ei.ei_classs {
            EiClass::Class32 => self.read16(0x30),
            _ => self.read16(0x3C),
        };
        let e_shstrndx = match ei.ei_classs {
            EiClass::Class32 => self.read16(0x32),
            _ => self.read16(0x3E),
        };

        ElfHeader {
            e_indent: ei,
            e_type: match self.read16(0x10) {
                0x0000 => EType::None,
                0x0001 => EType::Rel,
                0x0002 => EType::Exec,
                0x0003 => EType::Dyn,
                0x0004 => EType::Core,
                0xFE00 => EType::Loos,
                0xFEFF => EType::Hios,
                0xFF00 => EType::Loproc,
                0xFFFF => EType::Hiproc,
                n => panic!("Unknown type {:04X}", n),
            },
            e_machine: match self.read8(0x12) {
                0x00 => EMachine::None,
                0x01 => EMachine::M32,
                0x02 => EMachine::SPARC,
                0x03 => EMachine::X86,
                0x04 => EMachine::M68K,
                0x05 => EMachine::M88K,
                0x06 => EMachine::I486,
                0x07 => EMachine::I860,
                0x08 => EMachine::MIPS,
                0x09 => EMachine::MipsRs4Be,
                0x0A => EMachine::PARISC,
                0x13 => EMachine::I960,
                0x14 => EMachine::PPC,
                0x15 => EMachine::PPC64,
                0x16 => EMachine::S390,
                0x28 => EMachine::ARM,
                0x2A => EMachine::SUPERH,
                0x32 => EMachine::IA64,
                0x3E => EMachine::AMD64,
                0x8C => EMachine::TMS320,
                0xB7 => EMachine::ARM64,
                0xF3 => EMachine::RISCV,
                n => panic!("Unknown machine {:02X}", n),
            },
            e_version: match self.read32(0x14) {
                0 => EVersion::None,
                1 => EVersion::Current,
                n => panic!("Unknown elf version {:02X}", n),
            },
            e_entry: e_entry,
            e_phoff: e_phoff,
            e_shoff: e_shoff,
            e_flags: e_flags,
            e_ehsize: e_ehsize,
            e_phentsize: e_phentsize,
            e_phnum: e_phnum,
            e_shentsize: e_shentsize,
            e_shnum: e_shnum,
            e_shstrndx: e_shstrndx,
        }
    }

    pub fn get_program_header(&self, elf_header: &ElfHeader) -> Vec<ProgramHeader> {
        let mut phs = Vec::new();
        for i in 0..elf_header.e_phnum {
            let offset = elf_header.e_phoff as usize + ((elf_header.e_phentsize * i) as usize);

            phs.push(match elf_header.e_indent.ei_classs {
                EiClass::Class32 => ProgramHeader {
                    p_type: self.read32(offset),
                    p_offset: self.read32(offset + 4) as u64,
                    p_vaddr: self.read32(offset + 8) as u64,
                    p_paddr: self.read32(offset + 12) as u64,
                    p_filesz: self.read32(offset + 16) as u64,
                    p_memsz: self.read32(offset + 20) as u64,
                    p_flags: self.read32(offset + 24),
                    p_alignz: self.read32(offset + 28) as u64,
                },
                _ => ProgramHeader {
                    p_type: self.read32(offset),
                    p_flags: self.read32(offset + 4),
                    p_offset: self.read64(offset + 8),
                    p_vaddr: self.read64(offset + 16),
                    p_paddr: self.read64(offset + 24),
                    p_filesz: self.read64(offset + 32),
                    p_memsz: self.read64(offset + 40),
                    p_alignz: self.read64(offset + 48),
                },
            });
        }
        phs
    }

    pub fn get_section_header(&self, elf_header: &ElfHeader) -> Vec<SectionHeader> {
        let mut shs = Vec::new();
        for i in 0..elf_header.e_shnum {
            let offset = elf_header.e_shoff as usize + ((elf_header.e_shentsize * i) as usize);
            let sh_name = self.read32(offset);
            let sh_type = match self.read32(offset + 4) {
                0x00 => ShType::Null,
                0x01 => ShType::Progbits,
                0x02 => ShType::Sysmtab,
                0x03 => ShType::Strtab,
                0x04 => ShType::Rela,
                0x05 => ShType::Hash,
                0x06 => ShType::Dynamic,
                0x07 => ShType::Note,
                0x08 => ShType::Nobits,
                0x09 => ShType::Rel,
                0x0A => ShType::Shlib,
                0x0B => ShType::Dynsym,
                0x0E => ShType::InitArray,
                0x0F => ShType::FiniArray,
                0x10 => ShType::PreinitArray,
                0x11 => ShType::Group,
                0x12 => ShType::SymtabShndx,
                0x13 => ShType::Num,
                n => match n {
                    0x70000000..=0x7FFFFFFF => ShType::Loproc,
                    //0x80000000..=0x8FFFFFFF => ShType::Louser,
                    n => panic!("Unknown type version {:08X}", n),
                },
            };
            let sh_flags = match elf_header.e_indent.ei_classs {
                EiClass::Class32 => self.read32(offset + 8) as u64,
                _ => self.read64(offset + 8),
            };
            let sh_addr = match elf_header.e_indent.ei_classs {
                EiClass::Class32 => self.read32(offset + 0x0C) as u64,
                _ => self.read64(offset + 0x10),
            };
            let sh_offset = match elf_header.e_indent.ei_classs {
                EiClass::Class32 => self.read32(offset + 0x10) as u64,
                _ => self.read64(offset + 0x18),
            };
            let sh_size = match elf_header.e_indent.ei_classs {
                EiClass::Class32 => self.read32(offset + 0x14) as u64,
                _ => self.read64(offset + 0x20),
            };
            let sh_link = match elf_header.e_indent.ei_classs {
                EiClass::Class32 => self.read32(offset + 0x18),
                _ => self.read32(offset + 0x28),
            };
            let sh_info = match elf_header.e_indent.ei_classs {
                EiClass::Class32 => self.read32(offset + 0x1C),
                _ => self.read32(offset + 0x2C),
            };
            let sh_addralign = match elf_header.e_indent.ei_classs {
                EiClass::Class32 => self.read32(offset + 0x20) as u64,
                _ => self.read64(offset + 0x30),
            };
            let sh_entsize = match elf_header.e_indent.ei_classs {
                EiClass::Class32 => self.read32(offset + 0x24) as u64,
                _ => self.read64(offset + 0x38),
            };

            shs.push(SectionHeader {
                sh_name: sh_name,
                sh_type: sh_type,
                sh_flags: sh_flags,
                sh_addr: sh_addr,
                sh_offset: sh_offset,
                sh_size: sh_size,
                sh_link: sh_link,
                sh_info: sh_info,
                sh_addralign: sh_addralign,
                sh_entsize: sh_entsize,
            });
        }
        shs
    }

    /// find .tohost section and get address of that.
    pub fn search_tohost(
        &self,
        progbits_sec_headers: &Vec<&SectionHeader>,
        strtab_sec_headers: &Vec<&SectionHeader>,
    ) -> Option<u64> {
        for i in 0..progbits_sec_headers.len() {
            for j in 0..strtab_sec_headers.len() {
                let offset = (progbits_sec_headers[i].sh_name as u64
                    + strtab_sec_headers[j].sh_offset) as usize;
                match self.read64(offset) {
                    TOHOST => return Some(progbits_sec_headers[i].sh_addr),
                    _ => {}
                }
            }
        }
        None
    }

    pub fn read8(&self, offset: usize) -> u8 {
        self.data[offset]
    }

    fn read16(&self, offset: usize) -> u16 {
        let mut data = 0;
        for i in 0..2 {
            data |= (self.data[offset + i] as u16) << (8 * i);
        }
        data
    }

    fn read32(&self, offset: usize) -> u32 {
        let mut data = 0;
        for i in 0..4 {
            data |= (self.data[offset + i] as u32) << (8 * i);
        }
        data
    }

    fn read64(&self, offset: usize) -> u64 {
        let mut data = 0;
        for i in 0..8 {
            data |= (self.data[offset + i] as u64) << (8 * i);
        }
        data
    }
}
