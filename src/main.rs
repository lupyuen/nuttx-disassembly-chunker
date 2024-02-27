//! Split a NuttX Disassembly into Chunks for display by NuttX Log Parser in PureScript
//! Need to chunk nuttx.S (or qjs.S) by address:
//! nuttx-8000ad90.S, nuttx-8000ae00.S, nuttx-8000b000.S, nuttx-80010000.S
//! Code Addresses are at 0x8000_0000 to 0x8006_4a28 (403 KB)
//! Spanning 277K lines of code!

// We dump the qjs disassembly to qjs.S
// riscv64-unknown-elf-objdump \
//   --syms --source --reloc --demangle --line-numbers --wide \
//   --debugging \
//   ../apps/bin/qjs \
//   >qjs.S \
//   2>&1

// qjs.S looks like this:
// https://github.com/lupyuen/nuttx-tinyemu/blob/main/docs/purescript/qjs.S
// 0000000080007028 <_start>:
// _start():
// /Users/Luppy/ox64/nuttx/arch/risc-v/src/common/crt0.c:166
// void _start(int argc, char *argv[])
// {
//     80007028:	1141                	addi	sp,sp,-16

use std::fs::File;
use std::io::{BufReader, BufRead, Error};

/// Byte Size of a Single Chunk
const CHUNK_SIZE: usize = 10 * 1024;  // 41 Files of 10 KB Chunks

/// Split a NuttX Disassembly into Chunks for display by NuttX Log Parser in PureScript
fn main() -> Result<(), Error> {

    // Open the NuttX Disassembly File
    let input = File::open("/Users/Luppy/riscv/nuttx-tinyemu/docs/purescript/qjs.S")?;
    let buffered = BufReader::new(input);

    // Find lines that begin with `    80007028:`
    let re = regex::Regex::new("    ([0-9a-f]+):").unwrap();
    let mut linenum = 0;
    for line in buffered.lines() {
        linenum += 1;
        if linenum > 40_000 { break; }
        let line = line?;
        println!("{}", line);

        let chunk: usize = linenum / CHUNK_SIZE;

        // `addr` becomes `80007028`
        if let Some(cap) = re.captures_iter(&line).next() {
            if let Some(addr) = cap.get(1) {
                let addr = addr.as_str();
                println!("chunk={}, addr={}", chunk, addr);    
            }
        }    
    }

    Ok(())
}
