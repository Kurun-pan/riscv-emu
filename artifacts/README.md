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

## Building Zephyr OS for Linux (Ubuntu 20.04)

### Install packges

```
$ sudo apt install --no-install-recommends git cmake ninja-build gperf \
  ccache dfu-util device-tree-compiler wget python3-pip python3-setuptools \
  python3-wheel xz-utils file make gcc gcc-multilib \
  python3-dev libglib2.0-dev libpixman-1-dev
```

#### Getting Zephyr souce code

```
$ git clone https://github.com/zephyrproject-rtos/zephyr
```

### Setup

```
$ cd zephyr
$ pip3 install --user -r scripts/requirements.txt
$ export ZEPHYR_TOOLCHAIN_VARIANT=zephyr
$ export ZEPHYR_SDK_INSTALL_DIR="/opt/zephyr-sdk/"
$ . ./zephyr-env.sh
```

### Install Zephyr SDK

```
$ wget https://github.com/zephyrproject-rtos/sdk-ng/releases/download/v0.11.3/zephyr-sdk-0.11.3-setup.run
$ sudo sh zephyr-sdk-0.11.3-setup.run -- -d $ZEPHYR_SDK_INSTALL_DIR
```

### Building sample project

```
$ mkdir build-example
$ cd build-example
$ cmake -DBOARD=qemu_riscv32 $ZEPHYR_BASE/samples/hello_world
$ make -j 4
```

`zephyr/zephyr.elf` will be generated.

## Links

 - [Nuttx](https://bitbucket.org/nuttx/nuttx/src/master/)
 - [xv6-riscv](https://github.com/mit-pdos/xv6-riscv)
