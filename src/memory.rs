use crate::asm::{dsb};

#[derive(Copy, Clone)]
/// An address in kernelspace, i.e. with the top 16 bits set to one.
pub struct KernelAddr<T>(*mut T);

#[derive(Copy, Clone)]
pub struct PhysAddr(addr: u64);

impl From<u64> for PhysAddr { fn from(a: u64) -> Self { Self(a) } }

impl<T> From<PhysAddr> for KernelAddr<T> {
    fn from(PhysAddr(addr): PhysAddr) -> Self {
        let addr = addr | 0xff00_0000;
        unsafe { Self::new_unchecked(addr as *mut T) }
    }
}

#[derive(Copy, Clone)]
/// An address in userspace, i.e. with the top 16 bits set to zero.
pub struct UserAddr(*mut T);

impl<T> KernelAddr<T> {
    /// invariant: `ptr` must have its high 16 bits set to one.
    const unsafe fn new_unchecked(ptr: *mut T) -> Self { Self { ptr } }

    /// invariant: `self.ptr` must point to a valid `T` with a lifetime `'a`.
    pub unsafe fn deref<'a>(self) -> &'a T {
        &*self.ptr
    }

    /// invariant: `self.ptr` must point to a valid `T` with a
    /// lifetime `'a`; no other references to that `T` may exist..
    pub unsafe fn deref_mut<'a>(self) -> &'a mut T {
        &mut *self.ptr
    }
}

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

    // ensure all writes are seen by the whole system
    dsb::sy();
}
