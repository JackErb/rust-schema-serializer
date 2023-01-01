use std::marker;
use std::ptr;
use std::alloc;

// Refers to an allocated block of memory
// This is allocated on the heap. But in the future it may refer to an index in datum arrays.
// The entire block contains a schematized definition, as well as any dynamic memory it's using.
pub struct BlockHandle<T> {
    ptr: *mut T,
    phantom: marker::PhantomData<T>,
}

impl<T> BlockHandle<T> {
    pub fn get_pointer(&self) -> *const T {
        self.ptr as *const T
    }

    pub fn get_pointer_mut(&self) -> *mut T {
        self.ptr
    }

    pub fn get_pointer_mut_as<P>(&self) -> *mut P {
        self.ptr as *mut P
    }

    pub fn is_null(&self) -> bool {
        self.ptr.is_null()
    }
}

pub fn allocate_block<T>(layout: alloc::Layout) -> BlockHandle<T> {
    // TODO: this is a memory leak
    let ptr= unsafe {
        std::alloc::alloc(layout) as *mut T
    };

    BlockHandle {
        ptr: ptr,
        phantom: marker::PhantomData
    }
}

// A pointer to an item within the block handle.
// `offset` MUST be properly aligned, std::alloc::Layout should be used when allocating the
// block to make this guarantee.
pub struct BlockPointer<T> {
    handle: BlockHandle<T>,
    offset: usize,
}

impl<T> BlockPointer<T> {
    pub fn get_pointer(&self) -> *const T {
        unsafe {
            self.handle.get_pointer().add(self.offset)
        }
    }

    pub fn get_pointer_mut(&self) -> *mut T {
        unsafe {
            self.handle.get_pointer_mut().add(self.offset)
        }
    }

    pub fn is_null(&self) -> bool {
        self.handle.is_null()
    }

    pub fn null() -> BlockPointer<T> {
        BlockPointer {
            handle: BlockHandle {
                ptr: ptr::null_mut(),
                phantom: marker::PhantomData,
            },
            offset: 0,
        }
    }

    // TODO: shouldn't need this once I stop allocating on the heap.
    pub fn from_raw_parts(ptr: *mut T, offset: usize) -> BlockPointer<T> {
        BlockPointer {
            handle: BlockHandle {
                ptr: ptr,
                phantom: marker::PhantomData,
            },
            offset: offset,
        }
    }
}
