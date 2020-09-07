# riscv-emu

`riscv-emu` is a RISC-V emulator that is written in Rust. xv6, NuttX and FreeRTOS are supported.

## Demo

### NuttX

```
$ cargo run --release
$ ./target/release/riscv-emu -k ./artifacts/nuttx/nuttx -m SiFive_e
```

![animation](./demo/nuttx-riscv.gif)

### xv6

```
$ cargo run --release
$ ./target/release/riscv-emu -k ./artifacts/xv6/kernel -f ./artifacts/xv6/fs.img -m SiFive_u
```

![animation](./demo/xv6-riscv.gif)

### FreeRTOS

```
$ cargo run --release
$ ./target/release/riscv-emu -k ./artifacts/freertos/RTOSDemo.elf -m SiFive_e
```


## Tests

### Instruction Regression Tests

```
$ cargo test
```


## Support Status

### Instructions

- [x] RV32/64I
- [x] RV32/64M
- [ ] RV32/64F
- [ ] RV32/64D
- [ ] RV32/64V
- [x] RV32/64A
- [x] RV32/64C (Almost implemented)

### Virtual Memory

- [x] SV32
- [x] SV39
- [ ] SV48
- [ ] SV57
- [ ] SV64

### SoC/Peripherals

#### General
- [x] Uart (UART 16550)
- [x] Virtio Disk

#### [FU540-C000](https://static.dev.sifive.com/FU540-C000-v1.0.pdf)
- [x] CLINT (Timer)
- [x] PLIC (Interrupt Controller)

#### [FE310](https://static.dev.sifive.com/FE310-G000.pdf)
- [x] UART
- [x] PRCI
- [x] GPIO
- [x] SPI Flash
- [x] DTIM (SRAM)

### Support OS

 - [Nuttx](https://bitbucket.org/nuttx/nuttx/src/master/)
 - [xv6-riscv](https://github.com/mit-pdos/xv6-riscv)
 - [FreeRTOS](https://www.freertos.org/)

## Links

- [riscv/riscv-tests](https://github.com/riscv/riscv-tests)
