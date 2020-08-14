# riscv-emu

`riscv-emu` is a RISC-V emulator that is written in Rust.

# Status

## Support Instructions

### 32-bit

- [x] RV32I
- [x] RV32M
- [ ] RV32F
- [ ] RV32D
- [ ] RV32V
- [x] RV32A
- [x] RV32C (Partially implemented)

### 64-bit

- [x] RV64I
- [x] RV64M
- [ ] RV64F
- [ ] RV64D
- [ ] RV64V
- [x] RV64A
- [x] RV64C (Partially implemented)

## Surpport Virtual Memory

- [x] SV32
- [x] SV39
- [ ] SV48
- [ ] SV57
- [ ] SV64

## Peripherals

- [x] Timer ([CLINT](https://static.dev.sifive.com/FU540-C000-v1.0.pdf))
- [x] Uart (UART 16550)
- [x] Interrupt Controller (PLIC)
- [ ] Virtio

## Support OS

I'm working now..
 - [xv6-riscv](https://github.com/mit-pdos/xv6-riscv)

# Tests

## Instruction Regression Tests

```
$ cargo test
```

# References

## Tests

- [riscv/riscv-tests](https://github.com/riscv/riscv-tests)

