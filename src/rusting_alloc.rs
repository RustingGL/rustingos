use core::alloc::{GlobalAlloc, Layout};
use core::ptr::null_mut;

pub const HEAP_SIZE: usize = 100 * 1024 * 1024;

pub struct Allocator {
    heap: [u8; HEAP_SIZE],
    alloc: [u64; HEAP_SIZE / 64],
}

impl Allocator {
    pub const fn new() -> Self {
        Self {
            heap: [0; HEAP_SIZE],
            alloc: [0; HEAP_SIZE / 64],
        }
    }

    fn is_free(&self, index: usize) -> bool {
        let word = index / 64;
        let bit = index % 64;
        (self.alloc[word] >> bit) & 1 == 0
    }

    fn mark(&mut self, index: usize, used: bool) {
        let word = index / 64;
        let bit = index % 64;
        if used {
            self.alloc[word] |= 1 << bit;
        } else {
            self.alloc[word] &= !(1 << bit);
        }
    }
}

unsafe impl GlobalAlloc for Allocator {
    unsafe fn alloc(&self, layout: Layout) -> *mut u8 {
        let size = layout.size();
        let align = layout.align();

        let mut start = 0;
        let mut count = 0;

        for i in 0..HEAP_SIZE {
            if i % align != 0 {
                count = 0;
                continue;
            }

            if self.is_free(i) {
                if count == 0 {
                    start = i;
                }
                count += 1;

                if count >= size {
                    // Mark bits as used
                    #[allow(invalid_reference_casting)]
                    let alloc_mut = &mut *(self as *const _ as *mut Allocator);
                    for j in start..start + size {
                        alloc_mut.mark(j, true);
                    }

                    return self.heap.as_ptr().add(start) as *mut u8;
                }
            } else {
                count = 0;
            }
        }

        null_mut()
    }

    unsafe fn dealloc(&self, ptr: *mut u8, layout: Layout) {
        let heap_start = self.heap.as_ptr() as usize;
        let offset = (ptr as usize).wrapping_sub(heap_start);

        if offset >= HEAP_SIZE {
            return;
        }

        #[allow(invalid_reference_casting)]
        let alloc_mut = &mut *(self as *const _ as *mut Allocator);

        for i in offset..offset + layout.size() {
            alloc_mut.mark(i, false);
        }
    }
}
