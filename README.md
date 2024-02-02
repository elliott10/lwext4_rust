# lwext4 in Rust
To provide the rust interface, [lwext4](https://github.com/gkostka/lwext4.git) is abstracted in Rust language.

_lwext4 is an ext2/ext3/ext4 filesystem library in C for microcontrollers_

## Supported features

* `lwext4_rust` for x86_64, riscv64 and aarch64 on Rust OS is supported
* filetypes: regular, directories, softlinks
* journal recovery & transactions
* memory as block cache

## Quick start
please define env: `LIBC_BUILD_TARGET_DIR`
```
cargo build -vv --target <x86_64-unknown-none | riscv64gc-unknown-none-elf>
```
OR If you need to compile the lwext4 C library separately, 

please run `make musl-generic -C c/lwext4 ARCH=<x86_64|riscv64|aarch64>`

## Dependencies
* only C library on Rust OS
* Rust development environment
* C musl-based cross compile toolchains
  
eg: `x86_64-linux-musl-gcc`, or `riscv64-linux-musl-gcc`, or `aarch64-linux-musl-gcc`

## Reference

<!-- ![lwext4](https://cloud.githubusercontent.com/assets/8606098/11697327/68306d88-9eb9-11e5-8807-81a2887f077e.png) -->
* [lwext4](https://github.com/gkostka/lwext4.git)
* [arceos-lwip](https://github.com/Centaurus99/arceos-lwip.git)
