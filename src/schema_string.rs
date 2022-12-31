use crate::*;

use std::alloc;
use std::str;
use std::fmt;

pub struct SchemaString {
    array: SchemaArray<u8>, // Byte array of string memory
}

impl<'a> SchemaString {
    fn as_str(&self) -> Option<&'a str> {
        let slice= self.array.as_slice()?;
        unsafe {
            Some(str::from_utf8_unchecked(slice))
        }
    }

    fn len(&self) -> usize {
        match self.as_str() {
            Some(str) => str.len(),
            None => 0
        }
    }
}

impl Schematize for SchemaString {
    fn schema_default() -> SchemaString {
        SchemaString {
            array: SchemaArray::schema_default()
        }
    }

    fn serialize(&self) -> SchemaValue<'_> {
        match self.as_str() {
            Some(str) => SchemaValue::String(str),
            None => SchemaValue::String("")
        }
    }

    fn deserialize(schema_value: &SchemaValue) -> SchemaResult<SchemaString> {
        match schema_value {
            SchemaValue::String(schema_str) => {
                // TODO: This is a memory leak
                unsafe {
                    let bytes= schema_str.as_bytes();
                    let layout= alloc::Layout::for_value(bytes);

                    // Allocate the new pointer on the heap
                    let raw_ptr= alloc::alloc(layout) as *mut u8;

                    // Write the string bytes to the block pointer
                    for index in 0..bytes.len() {
                        *raw_ptr.add(index)= bytes[index];
                    }

                    let block_ptr= block::BlockPointer::from_raw_parts(raw_ptr, 0);

                    Ok(SchemaString {
                        array: SchemaArray::from_raw_parts(block_ptr, bytes.len()),
                    })
                }
            },
            _ => {
                println!("Deserialize schema string hit a wrong value {:?}", schema_value);
                return Err(SchemaError::WrongSchemaValue);
            }
        }
    }
}

impl fmt::Debug for SchemaString {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self.as_str() {
            Some(str) => write!(f, "\"{}\"", str),
            None => write!(f, "\"\"")
        }
    }
}
