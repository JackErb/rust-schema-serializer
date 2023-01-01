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

// helper function to write a formatted array to serialize string
pub fn serialize_array<T: Schematize>(slice: &[T], context: &mut SerializeContext) {
    context.print("[");

    context.tabs+= 1;
    context.println();
    context.print_tabs();

    for (index, element) in slice.iter().enumerate() {
        element.serialize(context);
        if index != slice.len()-1 {
            context.print(",\n");
            context.print_tabs();
        }
    }
    context.tabs-= 1;
    context.println();
    context.print_tabs();
    context.print("]");
}

impl<T: Schematize> Schematize for SchemaArray<T> {
    fn schema_default() -> SchemaArray<T> {
        SchemaArray {
            block_ptr: block::BlockPointer::null(),
            len: 0,
        }
    }

    fn serialize(&self, context: &mut SerializeContext) {
        match self.as_slice() {
            Some(slice) => {
                serialize_array(slice, context);
            },
            None => {
                context.print("[]");
            }
        }
    }

    fn build_layout(schema_value: &SchemaValue, layout: alloc::Layout, offsets: &mut Vec<usize>)
        -> BuildLayoutResult {
        match schema_value {
            SchemaValue::Array(vector) => {
                // Need to build layout for elements...
                // First we allocate the static sized block.
                // If the elements are dynamically sized, well, more allocations...

                if vector.len() > 0 {
                    // allocate entire static array block for the elements
                    let array_layout= alloc::Layout::array::<T>(vector.len())?;
                    let (mut new_layout, offset)= layout.extend(array_layout)?;
                    offsets.push(offset);

                    // Build the layout for everything else. This is a no-op unless T is using dynamic memory.
                    new_layout= new_layout.pad_to_align();
                    for item in vector {
                        new_layout= T::build_layout(item, new_layout, offsets)?;
                    }

                    Ok(new_layout)
                } else {
                    // array of size zero. no-op
                    Ok(layout)
                }
            },
            _ => {
                // wrong value. no-op
                Ok(layout)
            }
        }
    }

    fn deserialize(schema_value: &SchemaValue, context: &mut DeserializeContext) -> SchemaResult<SchemaArray<T>> {
        match schema_value {
            SchemaValue::Array(vector) => {
                if vector.len() > 0 {
                    // Get the block pointer offset for this array
                    assert!(context.offset_index < context.offsets.len());
                    let byte_offset= context.offsets[context.offset_index];
                    let block_pointer= block::BlockPointer::from_raw_parts(context.block_ptr as *mut T, byte_offset);

                    context.offset_index+= 1;

                    unsafe {
                        // Deserialize all the elements
                        for index in 0..vector.len() {
                            context.path.push(format!("[{}]", index));
                            *block_pointer.get_pointer_mut().add(index)= T::deserialize(&vector[index], context)?;
                            context.path.pop();
                        }
                    }

                    Ok(SchemaArray {
                        block_ptr: block::BlockPointer::from_raw_parts(context.block_ptr as *mut T, byte_offset),
                        len: vector.len(),
                    })
                } else {
                    // array of size zero. return an empty array.
                    Ok(SchemaArray::schema_default())
                }
            },
            _ => {
                println!("Deserialize hit a wrong value for field '{}'. Expected: Array, found: {:?}",
                    context.get_path(),
                    schema_value);
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
            None => write!(f, "[]")
        }
    }
}
