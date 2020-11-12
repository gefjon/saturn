use crate::driver::uart::Pc16550d;
use crate::sync::Mutex;

pub static CONSOLE: Mutex<Pc16550d> = Mutex::new(unsafe {
    Pc16550d::new(0xff1a_0000usize as _)
});
