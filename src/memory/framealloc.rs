use crate::memory::{Paddr, PAGE_SIZE, GIGABYTE, Pointer};
use core::convert::From;
use spin::Mutex;
use core::ptr;
use core::ops::{Deref, DerefMut};

#[repr(transparent)]
#[derive(PartialEq, Eq)]
/// an owning pointer to a `FreeBlock`.
///
/// holding one of these constitutes unique ownership over the `FreeBlock` at `Paddr`, so:
/// - the pointers can't be copied or cloned
/// - there must be a valid, initialized `FreeBlock` at `Paddr`
/// - constructing these is unsafe and is handled by `make_freeblock`
struct FreeBlockPtr(Paddr);

const fn log2(size: u64) -> usize {
    size.trailing_zeros() as _
}

const fn expt2(log_size: usize) -> u64 {
    1 << log_size
}

const MIN_BLOCK: usize = log2(PAGE_SIZE);
const MAX_BLOCK: usize = log2(GIGABYTE);
const N_BLOCK_SIZES: usize = (MAX_BLOCK - MIN_BLOCK + 1) as _;

const fn log_size_index(log_size: usize) -> usize {
    log_size - MIN_BLOCK
}

const fn valid_block_size(size: u64) -> bool {
    let log_size = log2(size);
    (log_size <= MAX_BLOCK) && (log_size >= MIN_BLOCK)
}

struct FreeBlock {
    log_size: usize,
    next: Option<FreeBlockPtr>,
}

/// the address of `blk`'s buddy, which may or may not be an actual block.
///
/// every freeblock is aligned to its size. each pair of buddies must be aligned to the
/// next larger size, so one buddy will have an `(addr / size)` which ends in 0 and its
/// buddy will be immediately after, and the other will have that quantity end in 1 and
/// its buddy will be immediately before. so to compute the addr of a buddy, flip the low
/// bit of `(addr / size)`.
fn freeblock_buddy(blk: Paddr, log_size: usize) -> Paddr {
    let ptr = u64::from(blk);
    let aligned = ptr >> log_size;
    Paddr::from((aligned ^ 1) << log_size)
}

impl Deref for FreeBlockPtr {
    type Target = FreeBlock;
    fn deref(&self) -> &Self::Target {
        let ptr = self.0.as_const();
        unsafe { &*ptr }
    }
}

impl DerefMut for FreeBlockPtr {
    fn deref_mut(&mut self) -> &mut Self::Target {
        let ptr = self.0.as_mut();
        unsafe { &mut *ptr }
    }
}

impl FreeBlockPtr {
    fn split(self) -> (Self, Self) {
        let newsize = self.log_size - 1;
        let my_start = self.0;
        assert!(valid_block_size(expt2(newsize)));
        let buddy = freeblock_buddy(my_start, newsize);
        unsafe {
            // ok because we got took ownership of this memory with `self`
            (
                make_freeblock(my_start, newsize, None),
                make_freeblock(buddy, newsize, None),
            )
        }
    }
    unsafe fn merge(self, buddy: Paddr) -> Self {
        assert!(buddy == freeblock_buddy(self.0, self.log_size));
        let start = core::cmp::min(self.0, buddy);
        let size = self.log_size + 1;
        make_freeblock(start, size, None)
    }
}

/// takes unique ownership of the `expt2(log_size)` bytes of memory starting at `start`.
unsafe fn make_freeblock(
    start: Paddr,
    log_size: usize,
    next: Option<FreeBlockPtr>,
) -> FreeBlockPtr {
    let ptr = u64::from(start.0) as *mut FreeBlock;
    ptr::write(ptr, FreeBlock { log_size, next });
    FreeBlockPtr(start)
}

struct FrameAllocator {
    freelist: [Option<FreeBlockPtr>; N_BLOCK_SIZES],
}

unsafe fn duplicate_freeblock(&FreeBlockPtr(ptr): &FreeBlockPtr) -> FreeBlockPtr {
    FreeBlockPtr(ptr)
}

fn find_buddy(
    start: Paddr,
    log_size: usize,
    freelist: Option<FreeBlockPtr>,
) -> (Option<FreeBlockPtr>, Option<FreeBlockPtr>) {
    if let Some(mut first) = freelist {
        let target = freeblock_buddy(start, log_size);
        if first.0 == target {
            let tail = first.next.take();
            (Some(first), tail)
        } else {
            // we do some shenanigans here to get a shared local mutable ref to various
            // parts of the linked list. this is nasty, but i can't think of a better way,
            // and it's safe since none of the references escape, and no two `&mut`s ever
            // coexist to the same place.
            let mut already_traversed = unsafe { duplicate_freeblock(&first) };
            while let Some(mut next) = already_traversed.next.take() {
                if next.0 == target {
                    already_traversed.next = next.next.take();
                    return (Some(next), Some(first))
                } else {
                    already_traversed.next = Some(unsafe { duplicate_freeblock(&next) });
                    already_traversed = next;
                }
            }
            (None, Some(first))
        }
    } else {
        (None, None)
    }
}

impl core::ops::Index<usize> for FrameAllocator {
    type Output = Option<FreeBlockPtr>;
    fn index(&self, i: usize) -> &Option<FreeBlockPtr> {
        &self.freelist[log_size_index(i)]
    }
}

impl core::ops::IndexMut<usize> for FrameAllocator {
    fn index_mut(&mut self, i: usize) -> &mut Option<FreeBlockPtr> {
        &mut self.freelist[log_size_index(i)]
    }
}

impl FrameAllocator {
    fn alloc(&mut self, size: u64) -> Option<FreeBlockPtr> {
        assert!(valid_block_size(size));
        let log_size = log2(size) as usize;
        if let Some(mut blk) = self[log_size].take() {
            let tail = blk.next.take();
            self[log_size] = tail;
            Some(blk)
        } else if log_size == MAX_BLOCK {
            None
        } else if let Some(larger) = self.alloc(size << 1) {
            let (ret, buddy) = larger.split();
            self[log_size] = Some(buddy);
            Some(ret)
        } else {
            None
        }
    }
    fn free(&mut self, start: Paddr, size: u64) {
        assert!(valid_block_size(size));
        let log_size = log2(size);
        let tail = self[log_size].take();
        let (buddy, tail) = find_buddy(start, log_size, tail);
        if let Some(buddy) = buddy {
            self[log_size] = tail;
            self.free(start.min(buddy.0), expt2(log_size + 1));
        } else {
            self[log_size] = Some(unsafe {
                make_freeblock(start, log_size, tail)
            });
        }
    }
    unsafe fn add_block(&mut self, start: Paddr, log_size: usize) {
        let tail = self[log_size].take();
        self[log_size] = Some(make_freeblock(start, log_size, tail));
    }
}

const NONE_FREEBLOCK: Option<FreeBlockPtr> = None;

static FRAME_ALLOCATOR: Mutex<FrameAllocator> = Mutex::new(FrameAllocator {
    freelist: [NONE_FREEBLOCK; N_BLOCK_SIZES],
});

pub fn alloc_frame(size: u64) -> Option<Paddr> {
    if let Some(block) = FRAME_ALLOCATOR.lock().alloc(size) {
        Some(block.0)
    } else {
        None
    }
}

pub unsafe fn free_frame(frame: Paddr, size: u64) {
    FRAME_ALLOCATOR.lock().free(frame, size);
}

struct FramesIterator {
    start: Paddr,
    end: Paddr,
}

impl Iterator for FramesIterator {
    type Item = (Paddr, usize);
    fn next(&mut self) -> Option<Self::Item> {
        if self.start >= self.end {
            None
        } else {
            let space_avail = log2(self.end.0 - self.start.0);
            if space_avail < MIN_BLOCK {
                None
            } else {
                let best_align = log2(self.start.0);
                let chosen_size = best_align.min(space_avail).min(MAX_BLOCK);
                let start = self.start;
                self.start.0 += expt2(chosen_size);
                Some((start, chosen_size))
            }
        }
    }
}

/// takes unique ownership of all the memory in the range `start..end`. usual invariants
/// apply; no other references to that memory may exist.
pub unsafe fn init_frame_allocator(start: Paddr, end: Paddr) {
    let mut alloc = FRAME_ALLOCATOR.try_lock()
        .expect("FRAME_ALLOCATOR already locked when initializing.");
    for (start, size) in (FramesIterator { start, end: Paddr(end.0 + 1) }) {
        alloc.add_block(start, size);
    }
}
