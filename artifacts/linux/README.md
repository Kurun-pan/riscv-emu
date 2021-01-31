# How to build Linux

### Build Linux kernel

```
$ git clone https://github.com/torvalds/linux -b v5.4
$ cd linux
$ make ARCH=riscv CROSS_COMPILE=riscv64-linux- defconfig
$ make ARCH=riscv CROSS_COMPILE=riscv64-linux- menuconfig
# Select "Platform type" -> "Maximum Physical Memory" -> "2GiB"
$ make ARCH=riscv CROSS_COMPILE=riscv64-linux- -j 4
$ cd ..
```

Note: Does the latest version fail to boot?

### Build OpenSBI boot loader

```
$ git clone https://github.com/riscv/opensbi.git -b v0.9
$ cd opensbi
$ make CROSS_COMPILE=riscv64-linux- PLATFORM=generic FW_PAYLOAD_PATH=../linux/arch/riscv/boot/Image
```

## Build Device Tree

```
$ dtc -O dtb -I dts -o ./artifacts/linux/dtb/qemu_virtio.dtb ./artifacts/linux/dtb/qemu_virtio.dts
```

## Make Rootfs iamge

### Build Buxybox

```
$ git clone https://github.com/mirror/busybox.git -b 1_33_stable
$ cd busybox/
$ make ARCH=riscv CROSS_COMPILE=riscv64-linux- defconfig
$ make ARCH=riscv CROSS_COMPILE=riscv64-linux- menuconfig
# Check "Settings" -> "Build static binary (no shared libs)"
$ make ARCH=riscv CROSS_COMPILE=riscv64-linux- install
$ cd ..
```

### Make rootfs image

```
$ mkdir rootfs
$ cd rootfs
$ dd if=/dev/zero of=rootfs.img bs=1M count=50
$ mkfs.ext2 -L riscv-rootfs rootfs.img
$ sudo mkdir /mnt/rootfs
$ sudo mount rootfs.img /mnt/rootfs
$ sudo cp -ar ../busybox/_install/* /mnt/rootfs
$ sudo mkdir /mnt/rootfs/{dev,home,mnt,proc,sys,tmp,var}
$ sudo chown -R -h root:root /mnt/rootfs
$ df /mnt/rootfs
Filesystem     1K-blocks  Used Available Use% Mounted on
/dev/loop10        49584  2164     44860   5% /mnt/rootfs
$ sudo umount /mnt/rootfs
$ sudo rmdir /mnt/rootfs
```

# References

 - [OpenSBI/Generic Platform/QEMU Virt Machine](https://github.com/riscv/opensbi/blob/master/docs/platform/qemu_virt.md)
 - [How to build Linux + OpenSBI for riscv-rust](https://github.com/takahirox/riscv-rust/tree/master/resources/linux/opensbi)
