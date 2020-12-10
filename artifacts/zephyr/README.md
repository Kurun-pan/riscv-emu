# How to build for Zephyr OS for Linux (Ubuntu 20.04)

## Install packges

```
$ sudo apt install --no-install-recommends git cmake ninja-build gperf \
  ccache dfu-util device-tree-compiler wget python3-pip python3-setuptools \
  python3-wheel xz-utils file make gcc gcc-multilib \
  python3-dev libglib2.0-dev libpixman-1-dev
```

### Getting Zephyr souce code

```
$ git clone https://github.com/zephyrproject-rtos/zephyr
```

## Setup

```
$ cd zephyr
$ pip3 install --user -r scripts/requirements.txt
$ export ZEPHYR_TOOLCHAIN_VARIANT=zephyr
$ export ZEPHYR_SDK_INSTALL_DIR="/opt/zephyr-sdk/"
$ . ./zephyr-env.sh
```

## Install Zephyr SDK

```
$ wget https://github.com/zephyrproject-rtos/sdk-ng/releases/download/v0.11.3/zephyr-sdk-0.11.3-setup.run
$ sudo sh zephyr-sdk-0.11.3-setup.run -- -d $ZEPHYR_SDK_INSTALL_DIR
```

## Building sample project

```
$ mkdir build-example
$ cd build-example
$ cmake -DBOARD=qemu_riscv32 $ZEPHYR_BASE/samples/hello_world
$ make -j 4
```

`zephyr/zephyr.elf` will be generated.
