use crate::*;

use std::alloc;
use std::str;
use std::fmt;
use std::slice;

pub struct SchemaString {
    block_ptr: block::BlockPointer<u8>, // Byte array of string memory
    len: usize,
}

impl<'a> SchemaString {
    fn as_str(&self) -> Option<&'a str> {
        if self.block_ptr.is_null() {
            None
        } else {
            unsafe {
                let slice= slice::from_raw_parts(self.block_ptr.get_pointer(), self.len);
                Some(str::from_utf8_unchecked(slice))
            }
        }
    }

    //fn len(&self) -> usize {
    //    self.len
    //}
}

impl Schematize for SchemaString {
    fn schema_default() -> SchemaString {
        SchemaString {
            block_ptr: block::BlockPointer::null(),
            len: 0,
        }
    }

    fn serialize(&self, context: &mut SerializeContext) {
        match self.as_str() {
            Some(str) => {
                context.print("\"");
                context.print(str);
                context.print("\"");
            },
            None => {
                context.print("\"\"");
            }
        }
    }

    fn build_layout(schema_value: &SchemaValue, layout: alloc::Layout, offsets: &mut Vec<usize>)
        -> Result<alloc::Layout, alloc::LayoutError> {
        match schema_value {
            SchemaValue::String(schema_string) => {
                if schema_string.len() > 0 {
                    let string_layout= alloc::Layout::for_value(schema_string.as_bytes());
                    let (new_layout, offset)= layout.extend(string_layout)?;
                    offsets.push(offset);

                    Ok(new_layout.pad_to_align())
                } else {
                    // string of zero length. no-op.
                    Ok(layout)
                }
            }
            _ => {
                // hit wrong schema value. no-op.
                Ok(layout)
            }
        }
    }

    fn deserialize(schema_value: &SchemaValue, context: &mut DeserializeContext) -> SchemaResult<SchemaString> {
        match schema_value {
            SchemaValue::String(schema_string) => {
                let bytes= schema_string.as_bytes();
                if bytes.len() > 0 {
                    // Get the block pointer offset for this string
                    let offset_index= context.offset_index;
                    assert!(offset_index < context.offsets.len());
                    let byte_offset= context.offsets[offset_index];

                    context.offset_index+= 1;

                    unsafe {
                        // Copy the string bytes over
                        let ptr= context.block_ptr.add(byte_offset) as *mut u8;
                        for index in 0..bytes.len() {
                            *ptr.add(index)= bytes[index];
                        }

                        Ok(SchemaString {
                            block_ptr: block::BlockPointer::from_raw_parts(context.block_ptr, byte_offset),
                            len: bytes.len(),
                        })
                    }
                } else {
                    // string of zero length. return an empty string, using no dynamic memory.
                    Ok(SchemaString::schema_default())
                }
            },
            _ => {
                println!("Deserialize hit a wrong value for field '{}'. Expected: String, found: {:?}",
                    context.get_path(),
                    schema_value);
                Err(SchemaError::WrongSchemaValue)
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

impl fmt::Display for SchemaString {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self.as_str() {
            Some(str) => write!(f, "\"{}\"", str),
            None => write!(f, "\"\"")
        }
    }
}
