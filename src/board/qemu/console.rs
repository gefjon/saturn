use spin::{Mutex, MutexGuard};
use core::fmt::{self, Write};

struct QemuOut {
    ptr: *mut u8,
}

unsafe impl Send for QemuOut {}

impl QemuOut {
    /// Invariants: `ptr` must point to the Qemu fake-serial-out; only one of these can ever exist.
    const unsafe fn new(ptr: *mut u8) -> Self {
        Self { ptr }
    }
}

static CONSOLE: Mutex<QemuOut> = Mutex::new(
    unsafe { QemuOut::new(0x3f20_1000 as *mut u8) }
);

impl Write for QemuOut {
    fn write_str(&mut self, s: &str) -> fmt::Result {
        for c in s.bytes() {
            unsafe {
                core::ptr::write_volatile(self.ptr, c);
            }
        }
        Ok(())
    }
}

pub fn lock_console() -> MutexGuard<'static, impl Write> {
    CONSOLE.lock()
}

pub unsafe fn force_unlock_console() {
    if CONSOLE.is_locked() {
        CONSOLE.force_unlock();
    }
}

pub unsafe fn init_console() {}
