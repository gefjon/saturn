use core::ptr::{self, NonNull, slice_from_raw_parts_mut};
use core::option::Option;
use core::convert::{From, TryFrom};
use core::cmp::Ord;
use core::mem::{MaybeUninit, size_of, align_of};
use core::slice;
use num_enum::{IntoPrimitive, TryFromPrimitive};
use spin::Mutex;

#[derive(Copy, Clone, PartialEq)]
#[repr(transparent)]
struct NextBlock {
    ptr: Option<NonNull<FreeBlock>>,
}

unsafe impl Send for NextBlock {}

impl From<*mut FreeBlock> for NextBlock {
    fn from(ptr: *mut FreeBlock) -> Self { Self {
        ptr: NonNull::new(ptr),
    } }
}

impl NextBlock {
    const fn none() -> Self { Self { ptr: None } }
    const fn some(ptr: NonNull<FreeBlock>) -> Self { Self { ptr: Some(ptr) } }
}

struct FreeBlock {
    next: NextBlock,
    size: BlockSize,
}

impl FreeBlock {
    fn buddy(&self) -> NonNull<FreeBlock> {
        let addr = self as *const FreeBlock as usize;
        let size = self.size.to_size();
        let buddy_addr = addr ^ size;
        NonNull::new(buddy_addr as *mut FreeBlock).unwrap()
    }
    /// returns the new buddy block created by splitting
    fn split(&mut self) -> NonNull<FreeBlock> {
        self.size = BlockSize::from_idx(self.size.to_idx() - 1);
        self.buddy()
    }
}

#[derive(Copy, Clone, IntoPrimitive, TryFromPrimitive)]
#[repr(usize)]
enum BlockSize {
    K4,
    K8,
    K16,
    K32,
    K64,
    K128,
    K256,
    K512,
    M1,
    M2,
    M4,
    M8,
    M16,
    M32,
    M64,
    M128,
    M256,
    M512,
    G1,
}

impl BlockSize {
    fn to_idx(self) -> usize { usize::from(self) }
    fn to_size(self) -> usize { 1 << (12 + self.to_idx()) }
    fn from_idx(idx: usize) -> Self {
        Self::try_from(idx).unwrap()
    }
    fn from_size(size: usize) -> Self {
        assert!(size.count_ones() == 1);
        Self::from_idx(size.trailing_zeros() as usize)
    }
    fn largest_align_within(align: usize, remaining_space: usize) -> Self {
        BlockSize::from_size(Ord::min(
            Ord::min(align, remaining_space),
            BlockSize::G1.to_size()
        ))
    }
    fn largest_in_range(start: usize, end: usize) -> Self {
        let aligned_to = 1 << (start.trailing_zeros() as usize);
        let remaining_size = end - start;
        Self::largest_align_within(aligned_to, remaining_size)
    }
    fn from_size_align(size: usize, align: usize) -> Self {
        // can't make a blocksize bigger or more aligned than the largest block
        assert!(size <= BlockSize::G1.to_size());
        assert!(align <= BlockSize::G1.to_size());
        // the only way to ensure a large alignment is to allocate a large size
        let min_size = Ord::max(size, align);
        // sizes are always powers of two
        let round_size = next_power_of_two(min_size);
        // can't make a blocksize smaller than the smallest block
        let size = Ord::max(round_size, BlockSize::K4.to_size());
        BlockSize::from_size(size)
    }
}

const N_SIZES: usize = (BlockSize::G1 as usize) + 1;

struct FrameAlloc {
    free_blocks: [NextBlock; N_SIZES],
    start: usize,
    end: usize,
    allocations: Option<NonNull<[Option<BlockSize>]>>,
}

unsafe impl Send for FrameAlloc {}

impl FrameAlloc {
    fn add_free_block(&mut self, start: *mut FreeBlock, size: BlockSize) {
        unsafe {
            ptr::write(start, FreeBlock {
                next: self.free_blocks[size.to_idx()],
                size,
            });
        }
        self.free_blocks[size.to_idx()] = NextBlock::from(start);
    }
    fn init_free_blocks(&mut self, free_mem_start: usize, free_mem_end: usize) {
        self.start = free_mem_start;
        self.end = free_mem_end;
        let mut next_block = free_mem_start;
        while next_block < free_mem_end {
            let block_size = BlockSize::largest_in_range(next_block, free_mem_end);
            let block_ptr = next_block as *mut FreeBlock;
            self.add_free_block(block_ptr, block_size);
            next_block += block_size.to_size();
        }
    }
    /// must be called after init_free_blocks, since it performs an allocation
    fn init_allocations(&mut self) {
        let size = self.end - self.start;
        let pages = size / BlockSize::K4.to_size();
        let allocations_size = pages * size_of::<Option<BlockSize>>();
        let allocations_align = align_of::<Option<BlockSize>>();
        let allocations: NonNull<Option<BlockSize>> = self.alloc(allocations_size, allocations_align)
            .unwrap().cast();

        for i in 0..pages { unsafe  {
            *allocations.as_ptr().offset(i as isize) = None;
        } }
        
        let allocations = slice_from_raw_parts_mut(
            allocations.as_ptr(),
            pages,
        )
            ;
        self.allocations = Some(NonNull::new(allocations).unwrap());
    }
    const fn uninit() -> Self { Self {
        free_blocks: [NextBlock::none(); N_SIZES],
        start: 0,
        end: 0,
        allocations: None,
    } }

    /// if `block` is `Some` and its buddy is free, remove the buddy
    /// from the free set and return it. Otherwise, return `None`.
    fn find_buddy(&mut self, block: NextBlock) -> NextBlock {
        if let Some(block_ptr) = block.ptr {
            let block = unsafe { block_ptr.as_ref() };
            let buddy_addr = block.buddy().as_ptr() as usize;
            
            let mut potential_buddy = &mut self.free_blocks[block.size.to_idx()];
            while let Some(potential_buddy_ptr) = potential_buddy.ptr {
                if (potential_buddy_ptr.as_ptr() as usize) == buddy_addr {
                    let buddy = unsafe { &mut *potential_buddy_ptr.as_ptr() };
                    *potential_buddy = buddy.next;
                    return NextBlock::some(potential_buddy_ptr)
                } else {
                    potential_buddy = &mut unsafe { &mut *potential_buddy_ptr.as_ptr() }.next;
                }
            }
        }
        NextBlock::none()
    }

    fn pop_free_block(&mut self, size: BlockSize) -> NextBlock {
        let place = &mut self.free_blocks[size.to_idx()];
        let next = *place;
        if let Some(block) = next.ptr {
            *place = unsafe { block.as_ref() }.next;
        }
        next
    }

    fn split_blocks_until(&mut self, size: BlockSize) -> NextBlock {
        let mut idx = size.to_idx() + 1;
        while idx <= BlockSize::G1.to_idx() {
            let next_block = self.pop_free_block(BlockSize::from_idx(idx));
            if let Some(block_ptr) = next_block.ptr {
                let split_me = unsafe { &mut *block_ptr.as_ptr() };
                let mut new_size = idx - 1;
                while new_size > size.to_idx() {
                    let buddy = split_me.split();
                    self.add_free_block(buddy.as_ptr(), BlockSize::from_idx(new_size));
                    new_size -= 1;
                }
                return next_block
            } else {
                idx += 1;
            }
        }
        NextBlock::none()
    }

    fn allocations_mut(&mut self) -> Option<&mut [Option<BlockSize>]> {
        self.allocations.map(|p| unsafe { &mut *p.as_ptr() })
    }

    fn mark_allocated(&mut self, block: NonNull<u8>, size: BlockSize) {
        if let Some(allocations) = self.allocations_mut() {
            let addr = block.as_ptr() as usize;
            let page = addr / BlockSize::K4.to_size();
            allocations[page] = Some(size);
        }
    }

    fn alloc(&mut self, size: usize, align: usize) -> Option<NonNull<u8>> {
        let size = BlockSize::from_size_align(size, align);
        if let Some(block_ptr) = self.pop_free_block(size).ptr {
            Some(block_ptr.cast())
        } else if let Some(block_ptr) = self.split_blocks_until(size).ptr {
            Some(block_ptr.cast())
        } else { None }
    }
}

fn next_power_of_two(n: usize) -> usize {
    if n.count_ones() == 1 { n } else {
        1 << (usize::BITS - n.leading_zeros())
    }
}

static ALLOCATOR: Mutex<FrameAlloc> = Mutex::new(FrameAlloc::uninit());

/// invariant: must be called exactly once on startup.
/// `free_mem_start` and `free_mem_end` must be at least page aligned,
/// and refer to a region of memory to which no other referents exist.
pub unsafe fn init(free_mem_start: usize, free_mem_end: usize) {
    let mut allocator = ALLOCATOR.lock();
    allocator.init_free_blocks(free_mem_start, free_mem_end);
    allocator.init_allocations();
}

pub fn alloc(size: usize, align: usize) -> Option<NonNull<u8>> {
    ALLOCATOR.lock().alloc(size, align)
}

#[cfg(test)]
mod test {
    use super::next_power_of_two;
    #[test]
    fn round_up_to_power_of_two() {
        assert_eq!(next_power_of_two(7), 8);
        assert_eq!(next_power_of_two(8), 8);
        assert_eq!(next_power_of_two(9), 16);
    }
}
