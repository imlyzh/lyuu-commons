


pub const UART0: u64 = 0x10000000;
pub const UART0_IRQ: u64 = 10;


pub const VIRTIO0: u64 = 0x10001000;
pub const VIRTIO0_IRQ: u64 = 1;


pub const CLINT: u64 = 0x2000000;

pub const fn clint_mtimecmp(hartid: u64) -> u64 {
    CLINT + 0x4000 + 8*hartid
}

pub const CLINT_MTINE: u64 = CLINT + 0xBFF8;


pub const PLIC: u64 = 0x0c000000;
pub const PLIC_PRIORITY: u64 = PLIC;
pub const PLIC_PENDING: u64 = PLIC + 0x1000;

pub const fn plic_menable(hart: u64) -> u64 {
    PLIC + 0x2000 + hart*0x100
}

pub const fn plic_senable(hart: u64) -> u64 {
    PLIC + 0x2080 + hart*0x100
}

pub const fn plic_mpriority(hart: u64) -> u64 {
    PLIC + 0x200000 + hart*0x2000
}

pub const fn plic_spriority(hart: u64) -> u64 {
    PLIC + 0x201000 + hart*0x2000
}

pub const fn plic_mclaim(hart: u64) -> u64 {
    PLIC + 0x200004 + hart*0x2000
}

pub const fn plic_sclaim(hart: u64) -> u64 {
    PLIC + 0x201004 + hart*0x2000
}


pub const KERNBASE: u64 = 0x80000000;
pub const PHYSTOP: u64 = KERNBASE + 128*1024*1024;

