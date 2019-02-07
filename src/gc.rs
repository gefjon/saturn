use core::marker::PhantomData;

#[repr(transparent)]
pub struct GcPtr<T: ?Sized> {
    ptr: *const T,
}
