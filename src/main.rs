use std::env;
use std::fs::File;
use std::io::{self, Read, Seek, SeekFrom};

const EI_NIDENT: usize = 16;

#[repr(C)]
#[derive(Debug)]
struct Elf64_Ehdr {
    e_ident: [u8; EI_NIDENT],
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

fn describe_type(e_type: u16) -> &'static str {
    match e_type {
        0 => "NONE (No file type)",
        1 => "REL (Relocatable file)",
        2 => "EXEC (Executable file)",
        3 => "DYN (Shared object file)",
        4 => "CORE (Core file)",
        _ => "UNKNOWN",
    }
}

fn describe_machine(e_machine: u16) -> &'static str {
    match e_machine {
        0 => "NONE (No machine)",
        3 => "i386 (Intel 80386)",
        62 => "X86_64 (AMD x86-64 architecture)",
        _ => "UNKNOWN",
    }
}

fn main() -> io::Result<()> {
    let default_path = "/bin/ls";
    let path = env::args().nth(1).unwrap_or(default_path.to_string());
    let mut file = File::open(&path)?;

    let mut ehdr = Elf64_Ehdr {
        e_ident: [0; EI_NIDENT],
        e_type: 0,
        e_machine: 0,
        e_version: 0,
        e_entry: 0,
        e_phoff: 0,
        e_shoff: 0,
        e_flags: 0,
        e_ehsize: 0,
        e_phentsize: 0,
        e_phnum: 0,
        e_shentsize: 0,
        e_shnum: 0,
        e_shstrndx: 0,
    };

    file.seek(SeekFrom::Start(0))?;
    file.read_exact(unsafe { std::slice::from_raw_parts_mut(&mut ehdr as *mut _ as *mut u8, std::mem::size_of::<Elf64_Ehdr>()) })?;
    print_elf_header(ehdr);

    Ok(())
}

fn print_elf_header(ehdr: Elf64_Ehdr) {
    println!("ELF Header:");
    println!("  Magic:   {:02x} {:02x} {:02x} {:02x} {:02x} {:02x} {:02x} {:02x} {:02x} {:02x} {:02x} {:02x} {:02x} {:02x} {:02x} {:02x}",
        ehdr.e_ident[0], ehdr.e_ident[1], ehdr.e_ident[2], ehdr.e_ident[3],
        ehdr.e_ident[4], ehdr.e_ident[5], ehdr.e_ident[6], ehdr.e_ident[7],
        ehdr.e_ident[8], ehdr.e_ident[9], ehdr.e_ident[10], ehdr.e_ident[11],
        ehdr.e_ident[12], ehdr.e_ident[13], ehdr.e_ident[14], ehdr.e_ident[15]);
    println!("  Type:                              {}", describe_type(ehdr.e_type));
    println!("  Machine:                           {}", describe_machine(ehdr.e_machine));
    println!("  Version:                           0x{:x}", ehdr.e_version);
    println!("  Entry point address:               0x{:x}", ehdr.e_entry);
    println!("  Start of program headers:          {}", ehdr.e_phoff);
    println!("  Start of section headers:          {}", ehdr.e_shoff);
    println!("  Flags:                             0x{:x}", ehdr.e_flags);
    println!("  Size of this header:               {}(bytes)", ehdr.e_ehsize);
    println!("  Size of program headers:           {}(bytes)", ehdr.e_phentsize);
    println!("  Number of program headers:         {}", ehdr.e_phnum);
    println!("  Size of section headers:           {} (bytes)", ehdr.e_shentsize);
    println!("  Number of section headers:         {}", ehdr.e_shnum);
    println!("  Section header string table index: {}", ehdr.e_shstrndx);

}
