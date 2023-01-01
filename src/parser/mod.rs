mod tokens;
mod schema;
mod debug;

use crate::*;
use tokens::Token;
use tokens::Symbol;

use std::marker;
use std::fs;
use std::str;
use std::alloc;

pub type ParseResult<T>= Result<T, &'static str>;

pub struct BlockDefinition<T> {
    block_handle: block::BlockHandle<T>, // this CANNOT be null
    phantom: marker::PhantomData<T>
}

impl<'a, T> BlockDefinition<T> {
    pub fn get_block_handle(&self) -> &block::BlockHandle<T> {
        &self.block_handle
    }

    pub fn get_definition(&self) -> &'a T {
        assert!(!self.block_handle.is_null());
        unsafe { &*self.block_handle.get_pointer() }
    }

    fn get_definition_mut(&self) -> &'a mut T {
        assert!(!self.block_handle.is_null());
        unsafe { &mut *self.block_handle.get_pointer_mut() }
    }
}

fn build_definition<T: Schematize>(contents: &str) -> ParseResult<BlockDefinition<T>> {
    // Parse the file contents into a schem value representation
    let tokens= tokens::string_to_tokens(contents)?;
    let schema_value= schema::tokens_to_schema_value(&tokens)?;

    // TODO: Validity check of the structure, optionally tuning it up w/ default values, etc.

    // Build the memory layout for the schema definition
    let layout= alloc::Layout::new::<T>();
    let mut layout_offsets= Vec::<usize>::new();

    let layout_result= T::build_layout(&schema_value, layout, &mut layout_offsets);
    let layout;
    match layout_result {
        Ok(built_layout) => {
            layout= built_layout.pad_to_align();
        },
        Err(_) => return Err("Failed to build layout for definition."),
    };

    // Allocate the block using the layout
    let block_definition= BlockDefinition {
        block_handle: block::allocate_block(layout),
        phantom: marker::PhantomData,
    };

    // Deserialize the definition into the block memory
    let mut context= DeserializeContext {
        block_ptr: block_definition.get_block_handle().get_pointer_mut_as::<u8>(),
        offsets: layout_offsets,
        offset_index: 0,
        path: Vec::new(),
    };

    let result= T::deserialize(&schema_value, &mut context);
    match result {
        Ok(deserialized_definition) => {
            *block_definition.get_definition_mut()= deserialized_definition;
            Ok(block_definition)
        },
        Err(e) => {
            println!("SchemaError::{:?}", e);
            Err("Failed to deserialize schema definition.")
        }
    }
}

pub fn load_definition<T: Schematize>(file_path: &str) -> Option<BlockDefinition<T>> {
    // TODO: We should have a caching system so if a definition is requested multiple times,
    // it can reuse the existing memory.

    let file_contents= fs::read_to_string(file_path);
    match file_contents {
        Ok(file_contents) =>
            match build_definition(&file_contents) {
                Ok(definition) => Some(definition),
                Err(err) => {
                    println!("Failed to load definition '{}'.\n Error: {}", file_path, err);
                    None
                }
            }
        Err(err) => {
            println!("Failed to read file contents '{}'.\n Error: {}", file_path, err);
            None
        }
    }
}
