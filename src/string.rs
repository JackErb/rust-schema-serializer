use std::str;

// Refers to an allocated block of memory
// This is allocated on the heap. But in the future it may refer to an index in datum arrays.
#[allow(non_camel_case_types)]
pub struct BlockHandle
{
    ptr: *mut usize,

}

#[allow(non_camel_case_types)]
pub struct BlockPointer<T>
{
    handle: BlockHandle,
    offset: u16,
    phantom: std::marker::PhantomData<T>

}

impl<T> BlockPointer<T>
{
    pub unsafe fn get_pointer(&self) -> *const T
    {
        self.handle.ptr.add(self.offset as usize) as *const T
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
