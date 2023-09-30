use crate::{print};
use crate::println;


pub const EI_NIDENT: usize = 16;

pub type Elf64_Addr = u64;
pub type Elf64_Off = u64;
pub type Elf64_Half = u16;
pub type Elf64_Word = u32;
pub type Elf64_Sword = i32;
pub type Elf64_Xword = u64;
pub type Elf64_Sxword = i64;

#[repr(C)]
pub struct Elf64_Ehdr {
    pub e_ident: [u8; EI_NIDENT],
    pub e_type: Elf64_Half,
    pub e_machine: Elf64_Half,
    pub e_version: Elf64_Word,
    pub e_entry: Elf64_Addr,
    pub e_phoff: Elf64_Off,
    pub e_shoff: Elf64_Off,
    pub e_flags: Elf64_Word,
    pub e_ehsize: Elf64_Half,
    pub e_phentsize: Elf64_Half,
    pub e_phnum: Elf64_Half,
    pub e_shentsize: Elf64_Half,
    pub e_shnum: Elf64_Half,
    pub e_shstrndx: Elf64_Half,
}

#[repr(C)]
pub struct Elf64_Phdr {
    pub p_type: Elf64_Word,
    pub p_flags: Elf64_Word,
    pub p_offset: Elf64_Off,
    pub p_vaddr: Elf64_Addr,
    pub p_paddr: Elf64_Addr,
    pub p_filesz: Elf64_Xword,
    pub p_memsz: Elf64_Xword,
    pub p_align: Elf64_Xword,
}

pub const PT_NULL: u32 = 0;
pub const PT_LOAD: u32 = 1;
pub const PT_DYNAMIC: u32 = 2;
pub const PT_INTERP: u32 = 3;
pub const PT_NOTE: u32 = 4;
pub const PT_SHLIB: u32 = 5;
pub const PT_PHDR: u32 = 6;
pub const PT_TLS: u32 = 7;

#[repr(C)]
pub struct Elf64_Dyn {
    pub d_tag: Elf64_Sxword,
    pub d_un: Elf64_DynUnion,
}

#[repr(C)]
pub union Elf64_DynUnion {
    pub d_val: Elf64_Xword,
    pub d_ptr: Elf64_Addr,
}

#[repr(C)]
pub struct Elf64_Rela {
    pub r_offset: Elf64_Addr,
    pub r_info: Elf64_Xword,
    pub r_addend: Elf64_Sxword,
}

pub const R_X86_64_RELATIVE: u32 = 8;

pub fn ELF64_R_SYM(i: u64) -> u32 {
    (i >> 32) as u32
}

pub fn ELF64_R_TYPE(i: u64) -> u32 {
    (i & 0xffffffff) as u32
}

pub fn ELF64_R_INFO(s: u32, t: u32) -> u64 {
    ((s as u64) << 32) + (t as u64)
}

