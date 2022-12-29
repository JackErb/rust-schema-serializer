use crate::Schematize;
use crate::SchemaValue;

use std::marker::PhantomData;

// Refers to an allocated block of memory
// This is allocated on the heap. But in the future it may refer to an index in datum arrays.
#[allow(non_camel_case_types)]
pub struct BlockHandle {
    ptr: *mut usize,
}

#[allow(non_camel_case_types)]
pub struct BlockPointer<T> {
    handle: BlockHandle,
    offset: u16,
    phantom: PhantomData<T>

}

impl<T> BlockPointer<T> {
    pub fn get_pointer(&self) -> *const T {
        unsafe {
            self.handle.ptr.add(self.offset as usize) as *const T
        }
    }

    pub fn is_null(&self) -> bool {
        self.handle.ptr.is_null()
    }
}

struct DynamicArray<T> {
    block_ptr: BlockPointer<T>,
    len: usize,
}

impl<'a, T> DynamicArray<T> {
    fn as_slice(&self) -> Option<&'a [T]> {
        if self.block_ptr.is_null() {
            None
        } else {
            Some(unsafe {
                std::slice::from_raw_parts(self.block_ptr.get_pointer(), self.len)
            })
        }
    }
}

impl<T: Schematize> Schematize for DynamicArray<T> {
    fn schema_default() -> DynamicArray<T> {
        DynamicArray {
            block_ptr: BlockPointer {
                handle: BlockHandle {
                    ptr: std::ptr::null_mut()
                },
                offset: 0,
                phantom: PhantomData
            },
            len: 0,
        }
    }

    fn serialize(&self) -> SchemaValue {
        let vec= match self.as_slice() {
            Some(slice) => slice.iter().map(|element| element.serialize()).collect(),
            None => Vec::new()
        };
        SchemaValue::Array(vec)
    }

    fn deserialize(&mut self, schema_value: &SchemaValue) {
        // TODO: this shouldn't be allocated on the heap, instead a block allocator should be
        // passed into deserialize
        match schema_value {
            SchemaValue::Array(vec) => {
                let layout= std::alloc::Layout::array::<T>(vec.len()).expect("Attempted to deserialize an array that is too large.");

                // TODO: This is a memory leak
                let ptr= unsafe {
                    std::alloc::alloc(layout)
                };

                self.len= vec.len();
                self.block_ptr= BlockPointer {
                    handle: BlockHandle {
                        ptr: ptr as *mut usize
                    },
                    offset: 0,
                    phantom: PhantomData
                }
            },
            _ => unimplemented!("Deserialize array hit a wrong value"),
        }
    }
}

/*
This should probably be redone to make use of a global string hash table.
The SchemaString would be a wrapper around a hash value that is used to index into the table.
  Pros:
    - Doesn't duplicate strings in memory
    - Easier to create runtime debug strings while editing tags in game
  Cons:
    - How do we deal with hash collisions?
      - In release there must be NO collisions
      - In debug, we can store the string inline in SchemaString instead.

#[allow(non_camel_case_types)]
pub struct SchemaString
{
    ptr: BlockPointer<u8>,
    len: i8,
}

impl SchemaString
{
    fn to_string<'a>(&self) -> &'a str
    {
        if !pself.ptr.
        unsafe
        {
            let char_ptr: *const u8= self.ptr.get_pointer();
            let slice: &[u8]= std::slice::from_raw_parts(char_ptr, self.len as usize);

            str::from_utf8(slice).unwrap()
        }
    }
}

impl crate::Schematizer for SchemaString
{
    fn schema_default() -> SchemaString
    {
        SchemaString { ptr: std::ptr::null(), len: 0 }
    }
}
*/
