use std::fs::File;
use std::io::{BufReader, BufRead, Error};

// We dump the qjs disassembly to qjs.S
// riscv64-unknown-elf-objdump \
//   --syms --source --reloc --demangle --line-numbers --wide \
//   --debugging \
//   ../apps/bin/qjs \
//   >qjs.S \
//   2>&1

// qjs.S looks like this:
// 0000000080007028 <_start>:
// _start():
// /Users/Luppy/ox64/nuttx/arch/risc-v/src/common/crt0.c:166
// void _start(int argc, char *argv[])
// {
//     80007028:	1141                	addi	sp,sp,-16

fn main() -> Result<(), Error> {

    // Open the NuttX Disassembly File
    let input = File::open("/Users/Luppy/riscv/nuttx-tinyemu/docs/purescript/qjs.S")?;
    let buffered = BufReader::new(input);

    // Find lines that begin with `    80007028:`
    let re = regex::Regex::new("    ([0-9a-f]+):").unwrap();
    let mut linenum = 0;
    for line in buffered.lines() {
        linenum += 1;
        if linenum > 14_000 { break; }
        let line = line?;
        println!("{}", line);

        // `addr` becomes `80007028`
        if let Some(cap) = re.captures_iter(&line).next() {
            if let Some(addr) = cap.get(1) {
                let addr = addr.as_str();
                println!("addr={}", addr);    
            }
        }    
    }

    Ok(())
}
