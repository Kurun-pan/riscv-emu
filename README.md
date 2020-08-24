# riscv-emu

`riscv-emu` is a RISC-V emulator that is written in Rust.

## Demo

### xv6
![animation](./demo/xv6-riscv.gif)

## Support Status

### Instructions

- [x] RV32/64I
- [x] RV32/64M
- [ ] RV32/64F
- [ ] RV32/64D
- [ ] RV32/64V
- [x] RV32/64A
- [x] RV32/64C (Partially implemented)

### Virtual Memory

- [x] SV32
- [x] SV39
- [ ] SV48
- [ ] SV57
- [ ] SV64

### Peripherals

- [x] Timer ([CLINT](https://static.dev.sifive.com/FU540-C000-v1.0.pdf))
- [x] Uart (UART 16550)
- [x] Interrupt Controller (PLIC)
- [x] Virtio Disk

### Support OS

 - [xv6-riscv](https://github.com/mit-pdos/xv6-riscv)

## How to run

### xv6

```
$ cargo run --release
```

## Tests

### Instruction Regression Tests

```
$ cargo test
```

## References

- [riscv/riscv-tests](https://github.com/riscv/riscv-tests)
