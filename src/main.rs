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
    let input = File::open("/Users/Luppy/riscv/nuttx-tinyemu/docs/purescript/qjs.S")?;
    let buffered = BufReader::new(input);

    let mut count = 0;
    for line in buffered.lines() {
        println!("{}", line?);
        count += 1;
        if count > 10 { break; }
    }

    Ok(())
}
