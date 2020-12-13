# How to build Linux

### Build Linux kernel

```
$ git clone https://github.com/torvalds/linux -b v5.9
$ cd linux
$ make ARCH=riscv CROSS_COMPILE=riscv64-linux- defconfig
$ make ARCH=riscv CROSS_COMPILE=riscv64-linux- menuconfig
$ make ARCH=riscv CROSS_COMPILE=riscv64-linux- all
```

### Build OpenSBI boot loader

```
$ git clone https://github.com/riscv/opensbi.git -b v0.8
$ cd opensbi
$ make CROSS_COMPILE=riscv64-linux- PLATFORM=generic \
       FW_PAYLOAD_PATH=../linux/arch/riscv/boot/Image
```

## Build Device Tree

```
$ dtc -O dtb -I dts -o ./artifacts/linux/dtb/qemu_virtio.dtb ./artifacts/linux/dtb/qemu_virtio.dts
```

## Create Rootfs iamge


# Links

 - [OpenSBI/Generic Platform/QEMU Virt Machine](https://github.com/riscv/opensbi/blob/master/docs/platform/qemu_virt.md)
