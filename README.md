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
- [ ] RV32C

### 64-bit

- [x] RV64I
- [x] RV64M
- [ ] RV64F
- [ ] RV64D
- [ ] RV64V
- [x] RV64A
- [ ] RV64C

## Surpport Virtual Memory

- [x] SV32
- [x] SV39
- [ ] SV48
- [ ] SV57
- [ ] SV64

# Tests

## Regression Tests for Instructions

```
$ cargo test
```

# References

## Tests

- [riscv/riscv-tests](https://github.com/riscv/riscv-tests)

