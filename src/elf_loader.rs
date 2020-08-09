use memmap::Mmap;
use std::fs::File;

use std::path::Path;

const HEADER_MAGIC: [u8; 4] = [0x7f, 0x45, 0x4c, 0x46]; // 0x7f 'E' 'L' 'F'
                                                        //const TOHOST: [u8; 8] = [0x2e, 0x74, 0x6f, 0x68, 0x6f, 0x73, 0x74, 0x00]; // .tohost
const TOHOST: u64 = 0x0074736f686f742e;

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
    SHT_NULL = 0x0,           // Section header table entry unused
    SHT_PROGBITS = 0x1,       // Program data (.text, data, etc)
    SHT_SYSMTAB = 0x2,        // Symbol table
    SHT_STRTAB = 0x3,         // String table
    SHT_RELA = 0x4,           // Relocation entries with addends
    SHT_HASH = 0x5,           // Symbol hash table
    SHT_DYNAMIC = 0x6,        // Dynamic linking information
    SHT_NOTE = 0x7,           // Notes
    SHT_NOBITS = 0x8,         // Program space with no data (bss)
    SHT_REL = 0x9,            // Relocation entries, no addends
    SHT_SHLIB = 0xA,          // Reserved
    SHT_DYNSYM = 0xB,         // Dynamic linker symbol table
    SHT_INIT_ARRAY = 0xE,     // Array of constructors
    SHT_FINI_ARRAY = 0xF,     // Array of destructors
    SHT_PREINIT_ARRAY = 0x10, // Array of pre-constructors
    SHT_GROUP = 0x11,         // Section group
    SHT_SYMTAB_SHNDX = 0x12,  // Extended section indices
    SHT_NUM = 0x13,           // Number of defined types
    SHT_LOPROC = 0x70000000,  //
    SHT_HIPROC = 0x7F000000,  //
    SHT_LOUSER = 0x80000000,  //
    SHT_HIUSER = 0xFFFFFFFF,  //
}

/*
#[derive(Debug)]
pub enum ShFlag {
    SHF_WRITE = 0x1,
    SHF_ALLOC = 0x2,
    SHF_EXECINSTR = 0x4,
    SHF_MERGE = 0x10,
    SHF_STRINGS = 0x20,
    SHF_INFO_LINK = 0x40,
    SHF_LINK_ORDER = 0x80,
    SHF_OS_NONCONFORMING = 0x100,
    SHF_GROUP = 0x200,
    SHF_TLS = 0x400,
    SHF_MASKOS = 0x0ff00000,
    SHF_MASKPROC = 0xf0000000,
    SHF_ORDERED = 0x4000000,
    SHF_EXCLUDE = 0x8000000,
}
*/

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
    ELFCLASSNONE,
    ELFCLASS32,
    ELFCLASS64,
}

#[derive(Debug)]
pub enum EiData {
    ELFDATANONE,
    ELFDATA2LSB,
    ELFDATA2MSB,
}

#[derive(Debug)]
pub enum EiVersion {
    EV_NONE,
    EV_CURRENT,
}

#[derive(Debug)]
pub enum EiOsAbi {
    ELFOSABI_SYSTEM_V,
    ELFOSABI_HP_UX,
    ELFOSABI_NETBSD,
    ELFOSABI_LINUX,
    ELFOSABI_GNU_HURD,
    ELFOSABI_SOLARIS,
    ELFOSABI_AIX,
    ELFOSABI_IRIX,
    ELFOSABI_FREEBSD,
    ELFOSABI_TRU64,
    ELFOSABI_NOVELL_MODESTO,
    ELFOSABI_OPEN_BSD,
    ELFOSABI_OPEN_VMS,
    ELFOSABI_NON_STOP_KERNEL,
    ELFOSABI_AROS,
    ELFOSABI_FENIX_OS,
    ELFOSABI_CLOUD_ABI,
    ELFOSABI_STARTUS_TECHNOLOGIES_OPEN_VOS,
}

#[derive(Debug)]
pub enum EType {
    ET_NONE,
    ET_REL,
    ET_EXEC,
    ET_DYN,
    ET_CORE,
    ET_LOOS,
    ET_HIOS,
    ET_LOPROC,
    ET_HIPROC,
}

#[derive(Debug)]
pub enum EMachine {
    EM_NONE,        // Unknown machine
    EM_M32,         // AT&T WE 32100
    EM_SPARC,       // SPARC
    EM_x86,         // x86
    EM_68K,         // Motorola 68000
    EM_88K,         // Motorola 88000
    EM_486,         // Intel 80486
    EM_860,         // Intel 80860
    EM_MIPS,        // MIPS R3000 Big-Endian only
    EM_MIPS_RS4_BE, // MIPS R4000 Big-Endian
    EM_PARISC,      // HPPA
    EM_960,         // Intel 80960
    EM_PPC,         // PowerPC
    EM_PPC64,       // PowerPC 64-bit
    EM_S390,        // S390 including S390x
    EM_ARM,         // ARM (up to ARMv7)
    EM_SUPERH,      // SuperH
    EM_IA64,        // IA-64,
    EM_AMD64,       // amd64,
    EM_TMS320,      // TMS320C5000 Family
    EM_ARM64,       // ARM 64-bit
    EM_RISCV,       // RISC-V
}

#[derive(Debug)]
pub enum EVersion {
    EV_NONE,
    EV_CURRENT,
}

pub struct ElfLoader {
    mapped_file: Mmap,
}

impl ElfLoader {
    pub fn new(filename: &Path) -> Result<Self, ()> {
        let file = match File::open(&filename) {
            Ok(file) => file,
            Err(why) => panic!("Couldn't open {}: {}", filename.display(), why),
        };
        Ok(ElfLoader {
            mapped_file: unsafe {
                match Mmap::map(&file) {
                    Ok(mmap) => mmap,
                    Err(why) => panic!("Couldn't read {}: {}", filename.display(), why),
                }
            },
        })
    }

    pub fn is_elf(&self) -> bool {
        self.mapped_file[0..4] == HEADER_MAGIC
    }

    pub fn get_elf_header(&self) -> ElfHeader {
        let ei = Ei {
            ei_classs: match self.mapped_file[4] {
                0 => EiClass::ELFCLASSNONE,
                1 => EiClass::ELFCLASS32,
                2 => EiClass::ELFCLASS64,
                n => panic!("Unknown e_ident class {}", n),
            },
            ei_data: match self.mapped_file[5] {
                0 => EiData::ELFDATANONE,
                1 => EiData::ELFDATA2LSB,
                2 => EiData::ELFDATA2MSB,
                n => panic!("Unknown e_ident endian {}", n),
            },
            ei_version: match self.mapped_file[6] {
                0 => EiVersion::EV_NONE,
                1 => EiVersion::EV_CURRENT,
                n => panic!("Unknown e_ident version {}", n),
            },
            ei_osabi: match self.mapped_file[7] {
                0x00 => EiOsAbi::ELFOSABI_SYSTEM_V,
                0x01 => EiOsAbi::ELFOSABI_HP_UX,
                0x02 => EiOsAbi::ELFOSABI_NETBSD,
                0x03 => EiOsAbi::ELFOSABI_LINUX,
                0x04 => EiOsAbi::ELFOSABI_GNU_HURD,
                0x06 => EiOsAbi::ELFOSABI_SOLARIS,
                0x07 => EiOsAbi::ELFOSABI_AIX,
                0x08 => EiOsAbi::ELFOSABI_IRIX,
                0x09 => EiOsAbi::ELFOSABI_FREEBSD,
                0x0A => EiOsAbi::ELFOSABI_TRU64,
                0x0B => EiOsAbi::ELFOSABI_NOVELL_MODESTO,
                0x0C => EiOsAbi::ELFOSABI_OPEN_BSD,
                0x0D => EiOsAbi::ELFOSABI_OPEN_VMS,
                0x0E => EiOsAbi::ELFOSABI_NON_STOP_KERNEL,
                0x0F => EiOsAbi::ELFOSABI_AROS,
                0x10 => EiOsAbi::ELFOSABI_FENIX_OS,
                0x11 => EiOsAbi::ELFOSABI_CLOUD_ABI,
                0x12 => EiOsAbi::ELFOSABI_STARTUS_TECHNOLOGIES_OPEN_VOS,
                n => panic!("Unknown e_ident version {}", n),
            },
            ei_abiversion: self.mapped_file[8],
        };

        let e_entry = match ei.ei_classs {
            EiClass::ELFCLASS32 => self.read32(0x18) as u64,
            _ => self.read64(0x18),
        };
        let e_phoff = match ei.ei_classs {
            EiClass::ELFCLASS32 => self.read32(0x1C) as u64,
            _ => self.read64(0x20),
        };
        let e_shoff = match ei.ei_classs {
            EiClass::ELFCLASS32 => self.read32(0x20) as u64,
            _ => self.read64(0x28),
        };
        let e_flags = match ei.ei_classs {
            EiClass::ELFCLASS32 => self.read32(0x24),
            _ => self.read32(0x30),
        };
        let e_ehsize = match ei.ei_classs {
            EiClass::ELFCLASS32 => self.read16(0x28),
            _ => self.read16(0x34),
        };
        let e_phentsize = match ei.ei_classs {
            EiClass::ELFCLASS32 => self.read16(0x2A),
            _ => self.read16(0x36),
        };
        let e_phnum = match ei.ei_classs {
            EiClass::ELFCLASS32 => self.read16(0x2C),
            _ => self.read16(0x38),
        };
        let e_shentsize = match ei.ei_classs {
            EiClass::ELFCLASS32 => self.read16(0x2E),
            _ => self.read16(0x3A),
        };
        let e_shnum = match ei.ei_classs {
            EiClass::ELFCLASS32 => self.read16(0x30),
            _ => self.read16(0x3C),
        };
        let e_shstrndx = match ei.ei_classs {
            EiClass::ELFCLASS32 => self.read16(0x32),
            _ => self.read16(0x3E),
        };

        ElfHeader {
            e_indent: ei,
            e_type: match self.read16(0x10) {
                0x0000 => EType::ET_NONE,
                0x0001 => EType::ET_REL,
                0x0002 => EType::ET_EXEC,
                0x0003 => EType::ET_DYN,
                0x0004 => EType::ET_CORE,
                0xFE00 => EType::ET_LOOS,
                0xFEFF => EType::ET_HIOS,
                0xFF00 => EType::ET_LOPROC,
                0xFFFF => EType::ET_HIPROC,
                n => panic!("Unknown type {:04X}", n),
            },
            e_machine: match self.mapped_file[0x12] {
                0x00 => EMachine::EM_NONE,
                0x01 => EMachine::EM_M32,
                0x02 => EMachine::EM_SPARC,
                0x03 => EMachine::EM_x86,
                0x04 => EMachine::EM_68K,
                0x05 => EMachine::EM_88K,
                0x06 => EMachine::EM_486,
                0x07 => EMachine::EM_860,
                0x08 => EMachine::EM_MIPS,
                0x09 => EMachine::EM_MIPS_RS4_BE,
                0x0A => EMachine::EM_PARISC,
                0x13 => EMachine::EM_960,
                0x14 => EMachine::EM_PPC,
                0x15 => EMachine::EM_PPC64,
                0x16 => EMachine::EM_S390,
                0x28 => EMachine::EM_ARM,
                0x2A => EMachine::EM_SUPERH,
                0x32 => EMachine::EM_IA64,
                0x3E => EMachine::EM_AMD64,
                0x8C => EMachine::EM_TMS320,
                0xB7 => EMachine::EM_ARM64,
                0xF3 => EMachine::EM_RISCV,
                n => panic!("Unknown machine {:02X}", n),
            },
            e_version: match self.read32(0x14) {
                0 => EVersion::EV_NONE,
                1 => EVersion::EV_CURRENT,
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

    pub fn get_section_header(&self, elf_header: &ElfHeader) -> Vec<SectionHeader> {
        let mut shs = Vec::new();
        for i in 0..elf_header.e_shnum {
            let offset = elf_header.e_shoff as usize + ((elf_header.e_shentsize * i) as usize);
            let sh_name = self.read32(offset);
            let sh_type = match self.read32(offset + 4) {
                0x00 => ShType::SHT_NULL,
                0x01 => ShType::SHT_PROGBITS,
                0x02 => ShType::SHT_SYSMTAB,
                0x03 => ShType::SHT_STRTAB,
                0x04 => ShType::SHT_RELA,
                0x05 => ShType::SHT_HASH,
                0x06 => ShType::SHT_DYNAMIC,
                0x07 => ShType::SHT_NOTE,
                0x08 => ShType::SHT_NOBITS,
                0x09 => ShType::SHT_REL,
                0x0A => ShType::SHT_SHLIB,
                0x0B => ShType::SHT_DYNSYM,
                0x0E => ShType::SHT_INIT_ARRAY,
                0x0F => ShType::SHT_FINI_ARRAY,
                0x10 => ShType::SHT_PREINIT_ARRAY,
                0x11 => ShType::SHT_GROUP,
                0x12 => ShType::SHT_SYMTAB_SHNDX,
                0x13 => ShType::SHT_NUM,
                n => match n {
                    0x70000000...0x7FFFFFFF => ShType::SHT_LOPROC,
                    0x80000000...0x8FFFFFFF => ShType::SHT_LOUSER,
                    n => panic!("Unknown type version {:08X}", n),
                },
            };
            let sh_flags = match elf_header.e_indent.ei_classs {
                EiClass::ELFCLASS32 => self.read32(offset + 8) as u64,
                _ => self.read64(offset + 8),
            };
            let sh_addr = match elf_header.e_indent.ei_classs {
                EiClass::ELFCLASS32 => self.read32(offset + 0x0C) as u64,
                _ => self.read64(offset + 0x10),
            };
            let sh_offset = match elf_header.e_indent.ei_classs {
                EiClass::ELFCLASS32 => self.read32(offset + 0x10) as u64,
                _ => self.read64(offset + 0x18),
            };
            let sh_size = match elf_header.e_indent.ei_classs {
                EiClass::ELFCLASS32 => self.read32(offset + 0x14) as u64,
                _ => self.read64(offset + 0x20),
            };
            let sh_link = match elf_header.e_indent.ei_classs {
                EiClass::ELFCLASS32 => self.read32(offset + 0x18),
                _ => self.read32(offset + 0x28),
            };
            let sh_info = match elf_header.e_indent.ei_classs {
                EiClass::ELFCLASS32 => self.read32(offset + 0x1C),
                _ => self.read32(offset + 0x2C),
            };
            let sh_addralign = match elf_header.e_indent.ei_classs {
                EiClass::ELFCLASS32 => self.read32(offset + 0x20) as u64,
                _ => self.read64(offset + 0x30),
            };
            let sh_entsize = match elf_header.e_indent.ei_classs {
                EiClass::ELFCLASS32 => self.read32(offset + 0x24) as u64,
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
        self.mapped_file[offset]
    }

    fn read16(&self, offset: usize) -> u16 {
        let mut data = 0;
        for i in 0..2 {
            data |= (self.mapped_file[offset + i] as u16) << (8 * i);
        }
        data
    }

    fn read32(&self, offset: usize) -> u32 {
        let mut data = 0;
        for i in 0..4 {
            data |= (self.mapped_file[offset + i] as u32) << (8 * i);
        }
        data
    }

    fn read64(&self, offset: usize) -> u64 {
        let mut data = 0;
        for i in 0..8 {
            data |= (self.mapped_file[offset + i] as u64) << (8 * i);
        }
        data
    }
}
