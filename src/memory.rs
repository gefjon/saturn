use crate::asm::{dsb, dmb};
use core::sync::atomic::{AtomicUsize, Ordering};

pub const MEM_START: *mut u8 = 0x0 as _;

/// one gibibyte
pub const MEM_SIZE: usize = 0x4000_0000;

pub const MEM_END: *mut u8 = MEM_SIZE as _;

pub const PAGE_SIZE: usize = 0x1000;

pub const N_PAGES: usize = MEM_SIZE / PAGE_SIZE;

pub const MMIO_BASE: usize = 0x3F00_0000;

// These are all defined in `/link.ld`
extern "C" {
    pub static __text_start: u64;

    pub static mut __bss_start: u64;
    pub static mut __bss_end: u64;

    pub static mut __data_start: u64;
    pub static mut __data_end: u64;
    pub static __data_loadaddr: u64;

    pub static __kernel_end: u64;
}

pub unsafe fn init_data() {
    r0::zero_bss(&mut __bss_start, &mut __bss_end);
    r0::init_data(&mut __data_start, &mut __data_end, &__data_loadaddr);

    // ensure the above writes have completed before refencing kernel pages
    dmb::nsh();

    reference_kernel_pages();
    reference_mmio_pages();

    // ensure all writes are seen by the whole system
    dsb::sy();
}

#[derive(Default)]
pub struct PageInfo {
    refcount: AtomicUsize,
}

pub static PAGES: [PageInfo; N_PAGES] = [PageInfo {
    refcount: AtomicUsize::new(0),
}; N_PAGES];

#[derive(Copy, Clone)]
/// Note: outside of the module boundary, it should be impossible for
/// safe code to create an out-of-bounds `PageNo`.
pub struct PageNo(usize);

impl PageNo {
    pub fn in_range(self) -> bool {
        let PageNo(idx) = self;
        idx < N_PAGES
    }
    fn validate(self) -> Option<Self> {
        if self.in_range() {
            Some(self)
        } else {
            None
        }
    }
    pub fn of_ptr<T>(ptr: *const T) -> Option<Self> {
        PageNo::from_physaddr(ptr as usize)
    }
    pub fn from_physaddr(addr: usize) -> Option<Self> {
        let masked = addr & !(PAGE_SIZE - 1);
        PageNo(masked / PAGE_SIZE).validate()
    }
    pub fn from_idx(idx: usize) -> Option<Self> {
        PageNo(idx).validate()
    }
    pub fn info(self) -> &'static PageInfo {
        let PageNo(idx) = self;
        &PAGES[idx]
    }
    pub fn physaddr(self) -> usize {
        let PageNo(idx) = self;
        idx * PAGE_SIZE
    }
}

impl core::ops::Deref for PageNo {
    type Target = PageInfo;
    fn deref(&self) -> &Self::Target {
        self.info()
    }
}

impl PageInfo {
    pub fn references(&self) -> usize {
        self.refcount.load(Ordering::Relaxed)
    }
    pub fn add_ref(&self) {
        self.refcount.fetch_add(1, Ordering::Relaxed);
    }
    /// invariants: self.refcount must be at least 1, and a previous
    /// referant must no longer exist.
    pub unsafe fn remove_ref(&self) {
        self.refcount.fetch_sub(1, Ordering::Relaxed);
    }
}

/// returns the first address not occupied by the kernel. the linker
/// script ensures this will be 4096-byte (aka page) aligned
fn kernel_end_addr() -> usize { unsafe {
    (&__kernel_end) as *const u64 as usize
} }

/// returns the index of the first page not occupied by the kernel
fn kernel_end_page_idx() -> usize {
    kernel_end_addr() / PAGE_SIZE
}

fn reference_kernel_pages() {
    for i in (0..kernel_end_addr()).step_by(PAGE_SIZE) {
        PageNo::from_physaddr(i).unwrap().add_ref();
    }
}

fn reference_mmio_pages() {
    for i in (MMIO_BASE..MEM_SIZE).step_by(PAGE_SIZE) {
        PageNo::from_physaddr(i).unwrap().add_ref();
    }
}

/// returns a `PageNo` corresponding to a previously-free page which
/// will have its refcount set to 1
pub fn alloc_a_page() -> Option<PageNo> {
    for i in 0..N_PAGES {
        let pn = PageNo::from_idx(i).unwrap();
        if let Ok(_) = pn.refcount.compare_exchange(0, 1, Ordering::Relaxed, Ordering::Relaxed) {
            return Some(pn);
        }
    }
    None
}
