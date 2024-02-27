use std::fs::File;
use std::io::{BufReader, BufRead, Error};

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
