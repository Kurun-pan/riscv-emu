# Setup build environment

## Install Toolchain for RISC-V

```
$ git clone --recursive https://github.com/riscv/riscv-gnu-toolchain
$ cd riscv-gnu-toolchain
$ ./configure --prefix=/opt/riscv
$ sudo make
$ export PATH=$PATH:/opt/riscv/bin
```

# How to build

 - [Linux]()
 - [Nuttx](./nuttx/README.md)
 - [xv6](./xv6/README.md)
 - [Zephyr](./zephyr/README.md)
 - [FreeRTOS]()
