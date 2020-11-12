use core::cell::UnsafeCell;

pub struct Mutex<T: ?Sized> {
    inner: UnsafeCell<T>,
}

unsafe impl<T: Send> Send for Mutex<T> {}
unsafe impl<T: Send> Sync for Mutex<T> {}

pub struct MutexGuard<'a, T: ?Sized + 'a> {
    data: &'a mut T,
}

impl<T> Mutex<T> {
    pub const fn new(data: T) -> Self {
        Mutex {
            inner: UnsafeCell::new(data),
        }
    }
}

impl<T: ?Sized> Mutex<T> {
    pub fn lock(&self) -> MutexGuard<T> {
        MutexGuard {
            data: unsafe { &mut *self.inner.get() }
        }
    }
    pub unsafe fn force_unlock(&self) { }
    pub fn is_locked(&self) -> bool { false }
}

impl<T: ?Sized> core::ops::Deref for MutexGuard<'_, T> {
    type Target = T;
    fn deref(&self) -> &T {
        self.data
    }
}

impl<T: ?Sized> core::ops::DerefMut for MutexGuard<'_, T> {
    fn deref_mut(&mut self) -> &mut T {
        self.data
    }
}
