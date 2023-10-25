# ELF Header Reader in Rust
This program is a simple utility written in Rust that reads and displays the ELF (Executable and Linkable Format) header of a given binary. The tool mimics the behavior of the readelf -h command, showing basic details about the ELF file.

## Features
- Displays the magic bytes of the ELF header.
- Describes the type of the ELF (e.g., REL, EXEC, DYN, CORE).
- Describes the machine type (e.g., i386, X86_64).
- Shows other pertinent details such as version, entry point address, program headers' start and size, section headers' start and size, and more.

## Usage
To use the program, simply compile it and then run:

```
> cargo run
// or
> cargo run /bin/wc
```
If you don't specify the path to the target file, the program defaults to reading the header of /bin/ls.

### Sample Output
The output might look something like:

```
ELF Header:
  Magic:   7f 45 4c 46 02 01 01 00 00 00 00 00 00 00 00 00
  Type:                              EXEC (Executable file)
  Machine:                           X86_64 (AMD x86-64 architecture)
  Version:                           0x1
  Entry point address:               0x400080
  Start of program headers:          64
  Start of section headers:          123456
  Flags:                             0x0
  Size of this header:               64(bytes)
  Size of program headers:           56(bytes)
  Number of program headers:         9
  Size of section headers:           64 (bytes)
  Number of section headers:         30
  Section header string table index: 29
```

## Implementation Details
The program is written in Rust and uses the standard libraries for file I/O and command-line argument parsing but no crates. The ELF header structure and constants are based on the ELF64 specification.
