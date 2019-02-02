#![cfg(target_arch="aarch64")]

/// Wait For Event
#[inline(always)]
pub unsafe fn _wfe() {
    asm!("wfe"
         :
         :
         :
         : "volatile"
    )
}

#[inline(always)]
/// No-op for a repeat of `cycles`
pub fn block(cycles: u32) {
    for _ in 0..cycles {
        unsafe { _nop() }
    }
}

#[inline(always)]
/// Call `func` until it returns `true`, blocking for `wait` cycles
/// between each try.
pub fn block_until<F>(mut func: F, wait: u32)
where
    F: FnMut() -> bool,
{
    loop {
        if func() { return }
        block(wait);
    }
}

/// No Operation
#[inline(always)]
pub unsafe fn _nop() {
    asm!("nop"
         :
         :
         :
         : "volatile"
    )
}
