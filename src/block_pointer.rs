use std::marker;
use std::ptr;

// Refers to an allocated block of memory
// This is allocated on the heap. But in the future it may refer to an index in datum arrays.
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

    pub fn is_null(&self) -> bool {
        self.ptr.is_null()
    }
}

/*
fn allocate_block_handle<T>(size: usize) -> BlockHandle<T> {
    // TODO: this is a memory leak
    let ptr= unsafe {
        std::alloc::alloc(layout) as *mut T
    };
}
*/

pub struct BlockPointer<T> {
    handle: BlockHandle<T>,
    offset: u16,
}

impl<T> BlockPointer<T> {
    pub fn get_pointer(&self) -> *const T {
        unsafe {
            self.handle.get_pointer().add(self.offset as usize)
        }
    }

    pub fn get_pointer_mut(&self) -> *mut T {
        unsafe {
            self.handle.get_pointer_mut().add(self.offset as usize)
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

    // THIS IS DANGEROUS, it ideally shouldn't be public
    pub fn from_raw_parts(ptr: *mut T, offset: u16) -> BlockPointer<T> {
        BlockPointer {
            handle: BlockHandle {
                ptr: ptr,
                phantom: marker::PhantomData,
            },
            offset: offset,
        }
    }
}
