use spin::{Mutex, MutexGuard};
use core::fmt::{Write, Arguments, Result};

pub struct QemuOut {
    ptr: *mut u8,
}

unsafe impl Send for QemuOut {}

impl QemuOut {
    /// Invariants: `ptr` must point to the Qemu fake-serial-out; only one of these can ever exist.
    const unsafe fn new(ptr: *mut u8) -> Self {
        Self { ptr }
    }
}

pub static CONSOLE: Mutex<QemuOut> = Mutex::new(unsafe { QemuOut::new(0x3f20_1000 as *mut u8) });

impl Write for QemuOut {
    fn write_str(&mut self, s: &str) -> Result {
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

pub fn print_str(s: &str) -> Result {
    lock_console().write_str(s)
}

pub fn print(arg: Arguments<'_>) -> Result {
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
