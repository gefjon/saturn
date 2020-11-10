use crate::asm::{dsb};

mod framealloc;

use crate::board::memory::MMIO_START;

#[derive(Copy, Clone)]
#[repr(transparent)]
/// An address in kernelspace, i.e. with the top 16 bits set to one.
pub struct KernelAddr<T> { ptr: *mut T }

#[derive(Copy, Clone)]
#[repr(transparent)]
pub struct PhysAddr { addr: usize }

impl PhysAddr {
    pub const fn from_usize(addr: usize) -> Self { Self { addr } }
}

impl From<usize> for PhysAddr { fn from(addr: usize) -> Self { Self::from_usize(addr) } }

impl<T> From<PhysAddr> for KernelAddr<T> {
    fn from(PhysAddr { addr }: PhysAddr) -> Self { Self::new(addr as *mut T) }
}

impl<T> KernelAddr<T> {
    /// invariant: `ptr` must have its high 16 bits set to one.
    const unsafe fn new_unchecked(ptr: *mut T) -> Self { Self { ptr } }

    const fn new(ptr: *mut T) -> Self {
        let addr = unsafe { ptr as usize };
        let addr = addr | 0xffff_0000_0000_0000usize;
        let ptr = addr as *mut T;
        unsafe { Self::new_unchecked(ptr) }
    }

    /// invariant: `self.ptr` must point to a valid `T` with a lifetime `'a`.
    pub unsafe fn deref<'a>(self) -> &'a T { &*self.ptr }

    /// invariant: `self.ptr` must point to a valid `T` with a
    /// lifetime `'a`; no other references to that `T` may exist.
    pub unsafe fn deref_mut<'a>(self) -> &'a mut T { &mut *self.ptr }
}

// These are all defined in `link.ld`
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

    framealloc::init(&__kernel_end as *const u64 as usize, MMIO_START);

    // ensure all writes are seen by the whole system
    dsb::sy();
}
