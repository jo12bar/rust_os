use super::Locked;
use alloc::alloc::{GlobalAlloc, Layout};
use core::{
    mem,
    ptr::{self, NonNull},
};

/// The block sizes to use.
///
/// The sizes must be power of 2 becuase they are also used as the block
/// alignment (alignments must always be powers of 2).
///
/// Don't define any block sizes smaller than 8, as each block must be capable
/// of storing a 64-bit pointer to the next block when freed. Allocations
/// greater than 2048 bytes will fall back to a linked-list allocator.
const BLOCK_SIZES: &[usize] = &[0, 16, 32, 64, 128, 256, 512, 1024, 2048];

/// Choose an appropriate block size for the given layout.
///
/// Returns an index into the `BLOCK_SIZES` array.
fn list_index(layout: &Layout) -> Option<usize> {
    let required_block_size = layout.size().max(layout.align());
    BLOCK_SIZES.iter().position(|&s| s >= required_block_size)
}

/// A node in a linked list.
struct ListNode {
    next: Option<&'static mut ListNode>,
}

/// An allocator taht hands out fixed sizes of blocks of memory at a time. While
/// this results in some internal fragmentation, overall fragmentation is
/// basically on par with linked-list allocators. By using fixed memory sizes
/// allocation and deallocation can be made much faster than linked-list
/// allocators.
///
/// Will fall back to a linked-list allocator for allocations greater than 2048
/// bytes.
pub struct FixedSizeBlockAllocator {
    list_heads: [Option<&'static mut ListNode>; BLOCK_SIZES.len()],
    fallback_allocator: linked_list_allocator::Heap,
}

impl FixedSizeBlockAllocator {
    /// Creates an empty `FixedSizeBlockAllocator`.
    pub const fn new() -> Self {
        FixedSizeBlockAllocator {
            list_heads: [None; BLOCK_SIZES.len()],
            fallback_allocator: linked_list_allocator::Heap::empty(),
        }
    }

    /// Initialize the allocator with the given heap bounds.
    ///
    /// # Safety
    /// This function is unsafe because the caller must guarantee that the given
    /// heap bounds are valid and that the heap is unused. This method must be
    /// called only once.
    pub unsafe fn init(&mut self, heap_start: usize, heap_size: usize) {
        self.fallback_allocator.init(heap_start, heap_size);
    }

    /// Allocates using the fallback allocator.
    fn fallback_alloc(&mut self, layout: Layout) -> *mut u8 {
        match self.fallback_allocator.allocate_first_fit(layout) {
            Ok(ptr) => ptr.as_ptr(),
            Err(_) => ptr::null_mut(),
        }
    }
}

unsafe impl GlobalAlloc for Locked<FixedSizeBlockAllocator> {
    unsafe fn alloc(&self, layout: Layout) -> *mut u8 {
        let mut allocator = self.lock();

        match list_index(&layout) {
            Some(index) => {
                match allocator.list_heads[index].take() {
                    Some(node) => {
                        allocator.list_heads[index] = node.next.take();
                        node as *mut ListNode as *mut u8
                    }

                    None => {
                        // no block exists in list -> allocate new block
                        let block_size = BLOCK_SIZES[index];

                        // only works if all block sizes are a power of 2!
                        let block_align = block_size;
                        let layout = Layout::from_size_align(block_size, block_align).unwrap();

                        allocator.fallback_alloc(layout)
                    }
                }
            }

            None => allocator.fallback_alloc(layout),
        }
    }

    unsafe fn dealloc(&self, ptr: *mut u8, layout: Layout) {
        let mut allocator = self.lock();

        match list_index(&layout) {
            Some(index) => {
                let new_node = ListNode {
                    next: allocator.list_heads[index].take(),
                };

                // Verify that block has size and alignment required for storing node
                assert!(mem::size_of::<ListNode>() <= BLOCK_SIZES[index]);
                assert!(mem::align_of::<ListNode>() <= BLOCK_SIZES[index]);

                let new_node_ptr = ptr as *mut ListNode;
                new_node_ptr.write(new_node);

                allocator.list_heads[index] = Some(&mut *new_node_ptr);
            }

            None => {
                let ptr = NonNull::new(ptr).unwrap();
                allocator.fallback_allocator.deallocate(ptr, layout);
            }
        }
    }
}
