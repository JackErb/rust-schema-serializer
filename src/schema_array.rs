use crate::*;
use super::block;

use std::alloc;
use std::fmt;

pub struct SchemaArray<T> {
    block_ptr: block::BlockPointer<T>,
    len: usize,
}

impl<'a, T> SchemaArray<T> {
    pub fn as_slice(&self) -> Option<&'a [T]> {
        if self.block_ptr.is_null() {
            None
        } else {
            Some(unsafe {
                std::slice::from_raw_parts(self.block_ptr.get_pointer(), self.len)
            })
        }
    }

    pub fn len(&self) -> usize {
        self.len
    }

    // TODO: This is very dangerous
    pub fn from_raw_parts(block_ptr: block::BlockPointer<T>, len: usize) -> SchemaArray<T> {
        SchemaArray {
            block_ptr: block_ptr,
            len: len,
        }
    }
}

impl<T: Schematize> Schematize for SchemaArray<T> {
    fn schema_default() -> SchemaArray<T> {
        SchemaArray {
            block_ptr: block::BlockPointer::null(),
            len: 0,
        }
    }

    fn serialize(&self) -> SchemaValue<'_> {
        let vec: Vec<SchemaValue>= match self.as_slice() {
            Some(slice) => slice.iter().map(|element| element.serialize()).collect(),
            None => Vec::new()
        };
        SchemaValue::Array(vec)
    }

    fn deserialize(schema_value: &SchemaValue) -> SchemaResult<SchemaArray<T>> {
        // TODO: this shouldn't be allocated on the heap, instead a block allocator should be
        // passed into deserialize
        match schema_value {
            SchemaValue::Array(vec) => {
                let layout= alloc::Layout::array::<T>(vec.len()).expect("Attempted to deserialize an array that is too large.");

                // TODO: This is a memory leak
                unsafe {
                    let ptr= std::alloc::alloc(layout) as *mut T;
                    for index in 0..vec.len() {
                        *ptr.add(index)= T::deserialize(&vec[index])?;
                    }

                    Ok(SchemaArray {
                        block_ptr: block::BlockPointer::from_raw_parts(ptr, 0),
                        len: vec.len(),
                    })
                }
            },
            _ => {
                println!("Deserialize schema array hit a wrong value {:?}", schema_value);
                return Err(SchemaError::WrongSchemaValue);
            }
        }
    }
}

impl<T: fmt::Debug> fmt::Debug for SchemaArray<T> {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self.as_slice() {
            Some(slice) => {
                write!(f, "[")?;
                for index in 0..slice.len() {
                    write!(f, "{:?}", slice[index])?;
                    if index != slice.len()-1 {
                        write!(f, ", ")?;
                    }
                }
                write!(f, "]")
            },
            None => write!(f, "NULL")
        }
    }
}
