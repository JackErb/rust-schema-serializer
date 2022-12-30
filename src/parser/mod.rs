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

fn parse_definition<T: Schematize>(contents: &str) -> ParseResult<BlockDefinition<T>> {
    let tokens= tokens::string_to_tokens(contents)?;
    let schema_value= schema::tokens_to_schema_value(&tokens)?;

    let mut definition= BlockDefinition {
        block_handle: block::allocate_block_handle(),
        phantom: marker::PhantomData,
    };

    // TODO: Deserialize should return a result rather than panicking
    *definition.get_definition_mut()= T::deserialize(&schema_value);
    Ok(definition)

    // Calculate the block size
    //let block_size= T::calculate_size();
    // Allocate a new block
    // Deserialize the schema_value into the block's memory
}

pub fn load_definition<T: Schematize>(file_path: &str) -> Option<BlockDefinition<T>> {
    let file_contents= fs::read_to_string(file_path);
    match file_contents {
        Ok(file_contents) =>
            match parse_definition(&file_contents) {
                Ok(definition) => Some(definition),
                Err(err) => {
                    println!("Failed to load definition (path: {}).\n Error: {}", file_path, err);
                    None
                }
            }
        Err(err) => {
            println!("Failed to read file contents (path: {}).\n Error: {}", file_path, err);
            None
        }
    }
}
