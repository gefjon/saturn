#![cfg(target_arch="aarch64")]
#![allow(unused)]

pub use cortex_a::asm::*;

#[inline(always)]
/// No-op for a repeat of `cycles`
pub fn block(cycles: u32) {
    for _ in 0..cycles { nop() }
}

macro_rules! define_barrier_insn {
    ($(#[$m:meta])* $fname:ident $insn:expr) => {
        $(#[$m])*
        #[inline(always)]
        pub fn $fname() { unsafe {
            asm!($insn, options(nomem, nostack));
        } }
    };
}

macro_rules! define_barrier {
    ($(#[$m:meta])* $insn:ident) => {
        $(#[$m])* pub mod $insn {
            define_barrier_insn!(
                /// Full system is the required shareability domain, reads and
                /// writes are the required access types, both before and after
                /// the barrier instruction. This option is referred to as the
                /// full system barrier.
                sy concat!(stringify!($insn), " sy")
            );
            define_barrier_insn!(
                /// Full system is the required shareability domain, writes
                /// are the required access type, both before and after the
                /// barrier instruction.
                st concat!(stringify!($insn), " st")
            );
            define_barrier_insn!(
                /// Full system is the required shareability domain, reads are
                /// the required access type before the barrier instruction,
                /// and reads and writes are the required access types after
                /// the barrier instruction.
                ld concat!(stringify!($insn), " ld")
            );
            define_barrier_insn!(
                /// Inner Shareable is the required shareability domain, reads
                /// and writes are the required access types, both before and
                /// after the barrier instruction.
                ish concat!(stringify!($insn), " ish")
            );
            define_barrier_insn!(
                /// Inner Shareable is the required shareability domain,
                /// writes are the required access type, both before and after
                /// the barrier instruction.
                ishst concat!(stringify!($insn), " ishst")
            );
            define_barrier_insn!(
                /// Inner Shareable is the required shareability domain, reads
                /// are the required access type before the barrier
                /// instruction, and reads and writes are the required access
                /// types after the barrier instruction.
                ishld concat!(stringify!($insn), " ishld")
            );
            define_barrier_insn!(
                /// Non-shareable is the required shareability domain, reads
                /// and writes are the required access, both before and after
                /// the barrier instruction.
                nsh concat!(stringify!($insn), " nsh")
            );
            define_barrier_insn!(
                /// Non-shareable is the required shareability domain, writes
                /// are the required access type, both before and after the
                /// barrier instruction.
                nshst concat!(stringify!($insn), " nshst")
            );
            define_barrier_insn!(
                /// Non-shareable is the required shareability domain, reads
                /// are the required access type before the barrier
                /// instruction, and reads and writes are the required access
                /// types after the barrier instruction.
                nshld concat!(stringify!($insn), " nshld")
            );
            define_barrier_insn!(
                /// Outer Shareable is the required shareability domain, reads
                /// and writes are the required access types, both before and
                /// after the barrier instruction.
                osh concat!(stringify!($insn), " osh")
            );
            define_barrier_insn!(
                /// Outer Shareable is the required shareability domain,
                /// writes are the required access type, both before and after
                /// the barrier instruction.
                oshst concat!(stringify!($insn), " oshst")
            );
            define_barrier_insn!(
                /// Outer Shareable is the required shareability domain, reads
                /// are the required access type before the barrier
                /// instruction, and reads and writes are the required access
                /// types after the barrier instruction.
                oshld concat!(stringify!($insn), " oshld")
            );
        }
    }
}

define_barrier!(
    /// Data Synchronization Barrier
    ///
    /// A DSB instruction is a memory barrier that ensures that memory
    /// accesses that occur before the DSB instruction have completed
    /// before the completion of the DSB instruction. In doing this, it
    /// acts as a stronger barrier than a DMB and all ordering that is
    /// created by a DMB with specific options is also generated by a DSB
    /// with the same options.
    dsb
);

define_barrier!(
    /// Data Memory Barrier
    ///
    /// The DMB instruction is a memory barrier instruction that ensures
    /// the relative order of memory accesses before the barrier with
    /// memory accesses after the barrier. The DMB instruction does not
    /// ensure the completion of any of the memory accesses for which it
    /// ensures relative order.
    dmb
);

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
