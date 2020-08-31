# How to build

## Install Toolchain for RISC-V

```
$ git clone --recursive https://github.com/riscv/riscv-gnu-toolchain
$ cd riscv-gnu-toolchain
$ ./configure --prefix=/opt/riscv
$ sudo make
$ export PATH=$PATH:/opt/riscv/bin
```

## Building NuttX

### Install kconfig-frontends

```
$ git clone https://bitbucket.org/nuttx/tools.git
$ cd tools/kconfig-frontends/
$ ./configure
$ make
$ sudo make install
$ sudo /sbin/ldconfig
```

### Getting NuttX

```
$ mkdir nuttx
$ cd nuttx/
$ git clone https://bitbucket.org/nuttx/nuttx.git
$ git clone https://bitbucket.org/nuttx/apps.git
```

### Configuration

```
$ cd nuttx/
$ ./tools/configure.sh hifive1-revb:nsh
```

### Editing `./defconfig` file and Make

```
Delete CONFIG_ARCH_CHIP_FE310_G002=y
Add CONFIG_ARCH_CHIP_FE310_QEMU=y
```

```
$ make
```

## Building xv6

```
$ git clone https://github.com/mit-pdos/xv6-riscv
$ cd xv6-riscv
$ make
$ make fs.img
```

## Links

 - [Nuttx](https://bitbucket.org/nuttx/nuttx/src/master/)
 - [xv6-riscv](https://github.com/mit-pdos/xv6-riscv)
