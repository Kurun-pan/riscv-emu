// Virtio (Virtual I/O Device)
// https://docs.oasis-open.org/virtio/virtio/v1.1/csprd01/virtio-v1.1-csprd01.html
// https://github.com/mit-pdos/xv6-riscv/blob/riscv/kernel/virtio_disk.c
// https://syuu1228.github.io/howto_implement_hypervisor/part12.html
// https://syuu1228.github.io/howto_implement_hypervisor/part20.html

use crate::memory::Memory;
use crate::system_bus::DRAM_ADDRESS_START;

const CONFIG_QUEUE_NUM_MAX: u32 = 0x16;
const CONFIG_DISK_SECTOR_SIZE: u64 = 512;
const CONFIG_DMA_DELAY: u64 = 128;

const VIRTIO_MAGIC_VALUE: u64 = 0x000;
const VIRTIO_VERSION: u64 = 0x004;
const VIRTIO_DEVICE_ID: u64 = 0x008;
const VIRTIO_VENDOR_ID: u64 = 0x00c;
const VIRTIO_DEVICE_FEATURES: u64 = 0x010;
const VIRTIO_DEVICE_FEATURES_SEL: u64 = 0x014;
const VIRTIO_DRIVER_FEATURES: u64 = 0x020;
const VIRTIO_GUEST_PAGE_SIZE: u64 = 0x028;
const VIRTIO_QUEUE_SEL: u64 = 0x030;
const VIRTIO_QUEUE_NUM_MAX: u64 = 0x034;
const VIRTIO_QUEUE_NUM: u64 = 0x038;
const VIRTIO_QUEUE_ALIGIN: u64 = 0x03c;
const VIRTIO_QUEUE_PFN: u64 = 0x040;
const VIRTIO_QUEUE_NOTIFY: u64 = 0x050;
const VIRTIO_INTERRUPT_STATUS: u64 = 0x060;
const VIRTIO_INTERRUPT_ACK: u64 = 0x64;
const VIRTIO_DEVICE_STATUS: u64 = 0x070;

const VIRTIO_INTERRUPT_QUEUE: u32 = 0x1;
const VIRTIO_INTERRUPT_CONFIGURATION: u32 = 0x2;

// Descriptor flags
const DESCRIPTOR_SIZE: u64 = 16;
const VRING_DESC_F_NEXT: u16 = 0x1;
const VRING_DESC_F_WRITE: u16 = 0x2;
const _VRING_DESC_F_INDIRECT: u16 = 0x4;

const OK: u8 = 0;
const _IOERR: u8 = 1;
const _UNSUPP: u8 = 2;

struct Virtqueue {
    descriptor_table_head: u64,
    available_ring_head: u64,
    used_ring_head: u64,
}

struct Descriptor {
    addr: u64,
    len: u32,
    flags: u16,
    next: u64,
}

pub struct Virtio {
    /// current clock cycle.
    cycle: u64,
    /// real user disk data.
    disk_image: Vec<u64>,
    /// last available ring index
    last_available_idx: u64,

    ///	Flags representing device features understood and activated by the driver (WO)
    driver_features: u32,
    /// Guest page size (WO)
    guest_page_size: u32,
    /// Virtual queue index (WO)
    queue_sel: u32,
    /// Virtual queue size (WO)
    queue_num: u32,
    /// Used Ring alignment in the virtual queue (WO)
    queue_align: u32,
    /// Guest physical page number of the virtual queue (R/W)
    queue_pfn: u32,
    /// Queue notifier (WO)
    queue_notify: Vec<u64>,
    /// Interrupt status (RO)
    interrupt_status: u32,
    ///	Device status (R/W)
    device_status: u32,
}

impl Virtio {
    pub fn new() -> Self {
        Virtio {
            cycle: 0,
            disk_image: vec![],
            last_available_idx: 0,
            driver_features: 0,
            guest_page_size: 0,
            queue_sel: 0,
            queue_num: 0,
            queue_align: 0x1000, // TODO: check the reson.
            queue_pfn: 0,
            queue_notify: Vec::new(),
            interrupt_status: 0,
            device_status: 0,
        }
    }

    pub fn init(&mut self, data: Vec<u8>) {
        self.disk_image.resize((data.len() + 7) / 8, 0);
        for i in 0..data.len() {
            let idx = (i >> 3) as usize;
            let pos = (i % 8) * 8;
            self.disk_image[idx] |= (data[i] as u64) << pos;
        }
    }

    pub fn tick(&mut self, dram: &mut Memory) {
        self.cycle = self.cycle.wrapping_add(1);
        if self.queue_notify.len() > 0 && (self.cycle == self.queue_notify[0] + CONFIG_DMA_DELAY) {
            self.transfer(dram);
            self.interrupt_status |= VIRTIO_INTERRUPT_QUEUE;
            self.queue_notify.remove(0);
        }
    }

    pub fn is_irq(&mut self) -> bool {
        self.interrupt_status & 0x3 > 0
    }

    pub fn read(&mut self, addr: u64) -> u32 {
        match addr {
            VIRTIO_MAGIC_VALUE => 0x74726976, // "virt" string
            VIRTIO_VERSION => 0x1,            // Legacy device returns value 0x1.
            VIRTIO_DEVICE_ID => 0x2,          // device type; 1 is net, 2 is disk
            VIRTIO_VENDOR_ID => 0x554d4551,   // from xv6-riscv source code.
            VIRTIO_DEVICE_FEATURES => 0,      // TODO: supoort features.
            VIRTIO_QUEUE_NUM_MAX => CONFIG_QUEUE_NUM_MAX,
            VIRTIO_QUEUE_PFN => self.queue_pfn,
            VIRTIO_INTERRUPT_STATUS => self.interrupt_status,
            VIRTIO_DEVICE_STATUS => self.device_status,
            _ => panic!("Read to reserved area: {:x}", addr),
        }
    }

    pub fn write(&mut self, addr: u64, data: u32) {
        match addr {
            VIRTIO_DEVICE_FEATURES_SEL => panic!("TODO!"),
            VIRTIO_DRIVER_FEATURES => self.driver_features = data,
            VIRTIO_GUEST_PAGE_SIZE => self.guest_page_size = data,
            VIRTIO_QUEUE_SEL => self.queue_sel = data,
            VIRTIO_QUEUE_NUM => self.queue_num = data,
            VIRTIO_QUEUE_ALIGIN => self.queue_align = data,
            VIRTIO_QUEUE_PFN => self.queue_pfn = data,
            VIRTIO_QUEUE_NOTIFY => self.queue_notify.push(self.cycle),
            VIRTIO_INTERRUPT_ACK => {
                if data & VIRTIO_INTERRUPT_QUEUE > 0 {
                    self.interrupt_status &= !VIRTIO_INTERRUPT_QUEUE;
                }
                if data & VIRTIO_INTERRUPT_CONFIGURATION > 0 {
                    self.interrupt_status &= !VIRTIO_INTERRUPT_CONFIGURATION;
                }
            }
            VIRTIO_DEVICE_STATUS => self.device_status = data,
            _ => panic!("Write to reserved area: {:x}", addr),
        }
    }

    fn transfer(&mut self, dram: &mut Memory) {
        let queue_size = self.queue_num as u64;
        let vq = self.get_virtqueue();

        // get latest entry idx from available ring.
        /* Available Ring
         * ----------------
         * u16 flags
         * u16 idx
         * u16[QUEUE_NUM] ring
         * u16 used_event
         */
        let descriptor_idx = (dram.read16(
            vq.available_ring_head
                .wrapping_add(4 + self.last_available_idx * 2),
        ) as u64)
            % queue_size;

        // first descriptor (virtio_blk_outhdr)
        let descriptor0 = self.get_descriptor(dram, vq.descriptor_table_head, descriptor_idx);
        let sector_idx = dram.read64(descriptor0.addr.wrapping_add(8));

        // Read/Write disk
        let descriptor1 = self.get_descriptor(dram, vq.descriptor_table_head, descriptor0.next);
        let disk_addr = sector_idx * CONFIG_DISK_SECTOR_SIZE;
        if (descriptor1.flags & VRING_DESC_F_WRITE) == 0 {
            // write only from Host side.
            if (descriptor1.addr % 8) == 0 && (descriptor1.len % 8) == 0 && (disk_addr % 8) == 0 {
                self.dma_memory_to_disk(dram, descriptor1.addr, disk_addr, descriptor1.len as u64);
            } else {
                for i in 0..descriptor1.len as u64 {
                    let data = dram.read8(descriptor1.addr + i);
                    self.write_disk8(disk_addr + i, data);
                }
            }
        } else {
            // read only from Host side.
            if (descriptor1.addr % 8) == 0 && (descriptor1.len % 8) == 0 && (disk_addr % 8) == 0 {
                self.dma_disk_to_memory(dram, descriptor1.addr, disk_addr, descriptor1.len as u64);
            } else {
                for i in 0..descriptor1.len as u64 {
                    let data = self.read_disk8(disk_addr + i);
                    dram.write8(descriptor1.addr + i, data);
                }
            }
        }

        // put result.
        {
            let descriptor2 = self.get_descriptor(dram, vq.descriptor_table_head, descriptor1.next);
            dram.write8(descriptor2.addr, OK);
            debug_assert!(
                (descriptor2.flags & VRING_DESC_F_NEXT) != 0,
                "Thrid descriptor is not last entry: {:x}",
                descriptor2.flags
            );
            debug_assert!(
                (descriptor2.flags & VRING_DESC_F_WRITE) == 0,
                "Thrid descriptor is not write type: {:x}",
                descriptor2.flags
            );
            debug_assert!(
                descriptor2.len != 1,
                "Thrid descriptor len is more than one: {:x}",
                descriptor2.flags
            );
        }

        // update used ring.
        {
            // put latest entry idx to used ring.
            /* Used Ring
             * ----------------
             * u16 flags
             * u16 idx
             * UsedRingEntry[QUEUE_NUM] ring
             * u16 avail_event
             */
            dram.write32(
                vq.used_ring_head
                    .wrapping_add(4 + self.last_available_idx * 8),
                descriptor_idx as u32,
            );

            // update latest entry of used ring.
            self.last_available_idx = self.last_available_idx.wrapping_add(1) % queue_size;
            dram.write16(
                vq.used_ring_head.wrapping_add(2),
                self.last_available_idx as u16,
            );
        }
    }

    fn get_virtqueue(&mut self) -> Virtqueue {
        let queue_size = self.queue_num as u64;

        // descriptor table head address
        let v_desc_table_addr;
        {
            let page_addr = self.queue_pfn as u64 * self.guest_page_size as u64;
            v_desc_table_addr = page_addr.wrapping_sub(DRAM_ADDRESS_START);
        }

        // available ring head address
        let v_available_addr = v_desc_table_addr.wrapping_add(queue_size * 16);

        // used ring head address
        let v_used_addr;
        {
            let align = self.queue_align as u64;
            v_used_addr =
                ((v_available_addr.wrapping_add(4 + queue_size * 2 + align - 1)) / align) * align;
        }

        Virtqueue {
            descriptor_table_head: v_desc_table_addr,
            available_ring_head: v_available_addr,
            used_ring_head: v_used_addr,
        }
    }

    fn get_descriptor(&mut self, dram: &mut Memory, table_head: u64, prev: u64) -> Descriptor {
        /* Descriptor entiry
         * -----------------
         * u64 addr
         * u32 len
         * u16 flags
         * u16 next
         */
        let queue_size = self.queue_num as u64;
        let entity = table_head + DESCRIPTOR_SIZE * prev;
        Descriptor {
            addr: dram.read64(entity) - DRAM_ADDRESS_START,
            len: dram.read32(entity.wrapping_add(8)),
            flags: dram.read16(entity.wrapping_add(12)),
            next: (dram.read16(entity.wrapping_add(14)) as u64) % queue_size,
        }
    }

    fn dma_disk_to_memory(&mut self, dram: &mut Memory, mem_addr: u64, disk_addr: u64, len: u64) {
        for i in 0..(len / 8) {
            let idx = ((disk_addr + i * 8) >> 3) as usize;
            dram.write64(mem_addr + i * 8, self.disk_image[idx]);
        }
    }

    fn dma_memory_to_disk(&mut self, dram: &mut Memory, mem_addr: u64, disk_addr: u64, len: u64) {
        for i in 0..(len / 8) {
            let idx = ((disk_addr + i * 8) >> 3) as usize;
            self.disk_image[idx] = dram.read64(mem_addr + i * 8);
        }
    }

    fn read_disk8(&mut self, addr: u64) -> u8 {
        let idx = (addr >> 3) as usize;
        let pos = (addr % 8) * 8;
        (self.disk_image[idx] >> pos) as u8
    }

    fn write_disk8(&mut self, addr: u64, data: u8) {
        let idx = (addr >> 3) as usize;
        let pos = (addr % 8) * 8;
        self.disk_image[idx] = (self.disk_image[idx] & !(0xff << pos)) | ((data as u64) << pos);
    }
}
