/// ELF parsing for symbol resolution
use anyhow::{Context, Result};
use std::fs::File;
use std::io::{Read, Seek, SeekFrom};

#[repr(C)]
struct Elf64Ehdr {
    e_ident: [u8; 16],
    e_type: u16,
    e_machine: u16,
    e_version: u32,
    e_entry: u64,
    e_phoff: u64,
    e_shoff: u64,
    e_flags: u32,
    e_ehsize: u16,
    e_phentsize: u16,
    e_phnum: u16,
    e_shentsize: u16,
    e_shnum: u16,
    e_shstrndx: u16,
}

#[repr(C)]
struct Elf64Shdr {
    sh_name: u32,
    sh_type: u32,
    sh_flags: u64,
    sh_addr: u64,
    sh_offset: u64,
    sh_size: u64,
    sh_link: u32,
    sh_info: u32,
    sh_addralign: u64,
    sh_entsize: u64,
}

#[repr(C)]
struct Elf64Sym {
    st_name: u32,
    st_info: u8,
    st_other: u8,
    st_shndx: u16,
    st_value: u64,
    st_size: u64,
}

const SHT_SYMTAB: u32 = 2;
const SHT_DYNSYM: u32 = 11;
const SHT_STRTAB: u32 = 3;

pub struct Symbol {
    pub name: String,
    pub addr: u64,
    pub size: u64,
}

pub fn parse_symbols(path: &str) -> Result<Vec<Symbol>> {
    let mut file = File::open(path)?;
    let mut ehdr = [0u8; 64];
    file.read_exact(&mut ehdr)?;
    
    let e_shoff = u64::from_le_bytes(ehdr[40..48].try_into().unwrap());
    let e_shnum = u16::from_le_bytes(ehdr[60..62].try_into().unwrap());
    let e_shstrndx = u16::from_le_bytes(ehdr[62..64].try_into().unwrap());
    
    let mut sections = Vec::new();
    file.seek(SeekFrom::Start(e_shoff))?;
    
    for _ in 0..e_shnum {
        let mut shdr = [0u8; 64];
        file.read_exact(&mut shdr)?;
        sections.push(shdr);
    }
    
    let mut symbols = Vec::new();
    
    for (i, shdr) in sections.iter().enumerate() {
        let sh_type = u32::from_le_bytes(shdr[4..8].try_into().unwrap());
        
        if sh_type == SHT_SYMTAB || sh_type == SHT_DYNSYM {
            let sh_offset = u64::from_le_bytes(shdr[24..32].try_into().unwrap());
            let sh_size = u64::from_le_bytes(shdr[32..40].try_into().unwrap());
            let sh_link = u32::from_le_bytes(shdr[40..44].try_into().unwrap()) as usize;
            
            if sh_link >= sections.len() {
                continue;
            }
            
            let strtab_shdr = &sections[sh_link];
            let strtab_offset = u64::from_le_bytes(strtab_shdr[24..32].try_into().unwrap());
            let strtab_size = u64::from_le_bytes(strtab_shdr[32..40].try_into().unwrap());
            
            file.seek(SeekFrom::Start(strtab_offset))?;
            let mut strtab = vec![0u8; strtab_size as usize];
            file.read_exact(&mut strtab)?;
            
            file.seek(SeekFrom::Start(sh_offset))?;
            let sym_count = sh_size / 24;
            
            for _ in 0..sym_count {
                let mut sym = [0u8; 24];
                file.read_exact(&mut sym)?;
                
                let st_name = u32::from_le_bytes(sym[0..4].try_into().unwrap()) as usize;
                let st_value = u64::from_le_bytes(sym[8..16].try_into().unwrap());
                let st_size = u64::from_le_bytes(sym[16..24].try_into().unwrap());
                
                if st_name < strtab.len() {
                    let name_end = strtab[st_name..].iter().position(|&b| b == 0).unwrap_or(0);
                    if name_end > 0 {
                        let name = String::from_utf8_lossy(&strtab[st_name..st_name + name_end]).to_string();
                        if !name.is_empty() && st_value != 0 {
                            symbols.push(Symbol { name, addr: st_value, size: st_size });
                        }
                    }
                }
            }
        }
    }
    
    Ok(symbols)
}

pub fn find_symbol<'a>(symbols: &'a [Symbol], name: &str) -> Option<&'a Symbol> {
    symbols.iter().find(|s| s.name == name || s.name.contains(name))
}
