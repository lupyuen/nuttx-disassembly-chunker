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
    let mut count = 0;
    for line in buffered.lines() {
        println!("{}", line?);

        count += 1;
        if count > 14_000 { break; }
    }

    Ok(())
}

/*
use error_chain::error_chain;

use regex::Regex;
use std::fmt;

error_chain!{
    foreign_links {
        Regex(regex::Error);
        Io(std::io::Error);
    }
}

struct PhoneNumber<'a> {
    area: &'a str,
    exchange: &'a str,
    subscriber: &'a str,
}

impl<'a> fmt::Display for PhoneNumber<'a> {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "1 ({}) {}-{}", self.area, self.exchange, self.subscriber)
    }
}

fn main() -> Result<()> {
    let phone_text = "
    +1 505 881 9292 (v) +1 505 778 2212 (c) +1 505 881 9297 (f)
    (202) 991 9534
    Alex 5553920011
    1 (800) 233-2010
    1.299.339.1020";

    let re = Regex::new(
        r#"(?x)
          (?:\+?1)?                       # Country Code Optional
          [\s\.]?
          (([2-9]\d{2})|\(([2-9]\d{2})\)) # Area Code
          [\s\.\-]?
          ([2-9]\d{2})                    # Exchange Code
          [\s\.\-]?
          (\d{4})                         # Subscriber Number"#,
    )?;

    let phone_numbers = re.captures_iter(phone_text).filter_map(|cap| {
        let groups = (cap.get(2).or(cap.get(3)), cap.get(4), cap.get(5));
        match groups {
            (Some(area), Some(ext), Some(sub)) => Some(PhoneNumber {
                area: area.as_str(),
                exchange: ext.as_str(),
                subscriber: sub.as_str(),
            }),
            _ => None,
        }
    });

    assert_eq!(
        phone_numbers.map(|m| m.to_string()).collect::<Vec<_>>(),
        vec![
            "1 (505) 881-9292",
            "1 (505) 778-2212",
            "1 (505) 881-9297",
            "1 (202) 991-9534",
            "1 (555) 392-0011",
            "1 (800) 233-2010",
            "1 (299) 339-1020",
        ]
    );

    Ok(())
}
*/
