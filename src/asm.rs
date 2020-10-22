#![cfg(target_arch="aarch64")]
#![allow(unused)]

pub use cortex_a::asm::*;

#[inline(always)]
/// No-op for a repeat of `cycles`
pub fn block(cycles: u32) {
    for _ in 0..cycles { nop() }
}

#[inline(always)]
/// Data Memory Barrier
///
/// The DMB instruction is a memory barrier instruction that ensures
/// the relative order of memory accesses before the barrier with
/// memory accesses after the barrier. The DMB instruction does not
/// ensure the completion of any of the memory accesses for which it
/// ensures relative order.
pub fn dmb() {
    unsafe {
        asm!("dmb sy", options(nomem, nostack));
    }
}

/// Data Synchronization Barrier
///
/// A DSB instruction is a memory barrier that ensures that memory
/// accesses that occur before the DSB instruction have completed
/// before the completion of the DSB instruction. In doing this, it
/// acts as a stronger barrier than a DMB and all ordering that is
/// created by a DMB with specific options is also generated by a DSB
/// with the same options.
pub mod dsb {
    macro_rules! define_dsb {
        ($(#[$m:meta])* $ty:ident;) => {
            $(#[$m])*
            #[inline(always)]
            pub fn $ty() { unsafe {
                asm!(concat!("dsb ", stringify!($ty)), options(nomem, nostack));
            } }
        };
        ($($(#[$m:meta])* $ty:ident;)*) => {
            $(define_dsb!($(#[$m])* $ty;);)*
        };
    }
    define_dsb!(
        /// Full system is the required shareability domain, reads and
        /// writes are the required access types, both before and after
        /// the barrier instruction. This option is referred to as the
        /// full system barrier.
        sy;
        /// Full system is the required shareability domain, writes
        /// are the required access type, both before and after the
        /// barrier instruction.
        st;
        /// Full system is the required shareability domain, reads are
        /// the required access type before the barrier instruction,
        /// and reads and writes are the required access types after
        /// the barrier instruction.
        ld;
        /// Inner Shareable is the required shareability domain, reads
        /// and writes are the required access types, both before and
        /// after the barrier instruction.
        ish;
        /// Inner Shareable is the required shareability domain,
        /// writes are the required access type, both before and after
        /// the barrier instruction.
        ishst;
        /// Inner Shareable is the required shareability domain, reads
        /// are the required access type before the barrier
        /// instruction, and reads and writes are the required access
        /// types after the barrier instruction.
        ishld;
        /// Non-shareable is the required shareability domain, reads
        /// and writes are the required access, both before and after
        /// the barrier instruction.
        nsh;
        /// Non-shareable is the required shareability domain, writes
        /// are the required access type, both before and after the
        /// barrier instruction.
        nshst;
        /// Non-shareable is the required shareability domain, reads
        /// are the required access type before the barrier
        /// instruction, and reads and writes are the required access
        /// types after the barrier instruction.
        nshld;
        /// Outer Shareable is the required shareability domain, reads
        /// and writes are the required access types, both before and
        /// after the barrier instruction.
        osh;
        /// Outer Shareable is the required shareability domain,
        /// writes are the required access type, both before and after
        /// the barrier instruction.
        oshst;
        /// Outer Shareable is the required shareability domain, reads
        /// are the required access type before the barrier
        /// instruction, and reads and writes are the required access
        /// types after the barrier instruction.
        oshld;
    );
}

#[inline(always)]
/// Call `func` until it returns `true`, blocking for `wait` cycles
/// between each try.
pub fn block_until<F>(mut func: F, wait: u32)
where
    F: FnMut() -> bool,
{
    loop {
        if func() {
            return;
        }
        block(wait);
    }
}
