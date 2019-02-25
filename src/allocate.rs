use core::alloc::{GlobalAlloc, Layout};
use core::sync::atomic::{AtomicPtr, Ordering};

#[alloc_error_handler]
fn alloc_error(l: Layout) -> ! {
    panic!("ALLOC ERROR: {:?}", l)
}

pub struct Allocator {
    next_free_block: AtomicPtr<u8>,
}

impl Allocator {
    pub const fn uninit() -> Self {
        Allocator {
            next_free_block: AtomicPtr::new(core::ptr::null_mut()),
        }
    }
    pub unsafe fn init(&self, mem: *mut u8) {
        extern "C" {
            static mut __bss_end: u8;
        }
        self.next_free_block.store(mem, Ordering::Release);
    }

    pub unsafe fn try_an_alloc(&self) {
        let layout = Layout::from_size_align(16, 16).unwrap();
        let p: *mut (u64, u64) = self.alloc(layout) as _;
        let pair = &mut *p;
        pair.0 = 0xdeadbeef;
        pair.1 = 0xdeadbabe;

        assert_eq!(*p, (0xdeadbeef, 0xdeadbabe));
        self.dealloc(p as *mut u8, layout);
    }
}

unsafe impl GlobalAlloc for Allocator {
    unsafe fn alloc(&self, layout: Layout) -> *mut u8 {
        let (size, align) = (layout.size(), layout.align());
        let base_ptr = self.next_free_block.load(Ordering::Acquire);

        let allocation_ptr = base_ptr.offset(base_ptr.align_offset(align) as _);

        self.next_free_block
            .store(allocation_ptr.offset(size as _), Ordering::Release);

        allocation_ptr
    }
    /// We don't actually re-use memory right now
    unsafe fn dealloc(&self, _ptr: *mut u8, _layout: Layout) {}
}
