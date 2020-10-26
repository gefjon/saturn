use spin::{Mutex, MutexGuard};
use core::fmt::{self, Write, Arguments};

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

fn lock_console() -> MutexGuard<'static, impl Write> {
    CONSOLE.lock()
}

pub unsafe fn force_unlock_console() {
    if CONSOLE.is_locked() {
        CONSOLE.force_unlock();
    }
}

pub fn with_console<F, R>(f: F) -> Result<R, fmt::Error>
where
    F: FnOnce(&mut dyn Write) -> Result<R, fmt::Error>,
{
    let c = &mut *lock_console();
    f(c)
}

pub fn print_str(s: &str) -> fmt::Result {
    lock_console().write_str(s)
}

pub fn print(arg: Arguments<'_>) -> fmt::Result {
    lock_console().write_fmt(arg)
}

#[macro_export]
macro_rules! print {
    ($($arg:tt)*) => {{
        $crate::console::print(format_args!($($arg)*)).expect("print failed");
    };}
}

#[macro_export]
macro_rules! println {
    () => {{
        $crate::console::print_str("\n").expect("println failed");
    };};
    ($($arg:tt)*) => {{
        $crate::console::print(format_args_nl!($($arg)*)).expect("println failed");
    };};
}
