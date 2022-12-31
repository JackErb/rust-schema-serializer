mod tokens;
mod schema;
mod debug;

use crate::block;
use crate::Schematize;
use tokens::Token;
use tokens::Symbol;

use std::marker;
use std::fs;
use std::str;

pub type ParseResult<T>= Result<T, &'static str>;

pub struct BlockDefinition<T> {
    block_handle: block::BlockHandle<T>, // this CANNOT be null
    phantom: marker::PhantomData<T>
}

impl<'a, T> BlockDefinition<T> {
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
    let tokens= tokens::string_to_tokens(contents)?;
    let schema_value= schema::tokens_to_schema_value(&tokens)?;

    let definition= BlockDefinition {
        block_handle: block::allocate_block_handle(),
        phantom: marker::PhantomData,
    };

    // Deserialize the definition
    *definition.get_definition_mut()= match T::deserialize(&schema_value) {
        Ok(definition) => definition,
        Err(e) => {
            println!("SchemaError::{:?}", e);
            return Err("Failed to deserialize schema definition.");
        }
    };

    Ok(definition)

    // Calculate the block size
    //let block_size= T::calculate_size();
    // Allocate a new block
    // Deserialize the schema_value into the block's memory
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
