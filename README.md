# Split a NuttX Disassembly into Chunks for display by NuttX Log Parser in PureScript

See https://github.com/lupyuen/nuttx-purescript-parser

To chunk a NuttX Disassembly...

```bash
## Chunk NuttX Disassembly $HOME/qjs.S into
## $HOME/qjs-chunk/qjs-80001000.S
## $HOME/qjs-chunk/qjs-80002000.S
## ...

chunkpath=$HOME
chunkbase=qjs
mkdir -p $chunkpath/$chunkbase-chunk
rm -f $chunkpath/$chunkbase-chunk/*
cargo run -- $chunkpath $chunkbase
```

To create the NuttX Disassembly...

```bash
## Dump Disassembly of `qjs` to `qjs.S`
## For NuttX Kernel, change `qjs` to `nuttx`
riscv64-unknown-elf-objdump \
  --syms --source --reloc --demangle --line-numbers --wide \
  --debugging \
  ../apps/bin/qjs \
  >qjs.S \
  2>&1
```

NuttX Disassembly looks like this: [qjs.S](https://github.com/lupyuen/nuttx-tinyemu/blob/main/docs/purescript/qjs.S)

```text
0000000080007028 <_start>:
_start():
nuttx/arch/risc-v/src/common/crt0.c:166
void _start(int argc, char *argv[])
{
    80007028:	1141                	addi	sp,sp,-16
```
