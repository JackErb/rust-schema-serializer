mod tokens;

use crate::block_pointer::BlockHandle;
use crate::SchemaValue;
use crate::Schematize;
use tokens::Symbol;
use tokens::Token;

use std::marker::PhantomData;
use std::fs;
use std::str;
use std::iter;

pub struct BlockDefinition<T> {
    block_handle: BlockHandle<T>, // this CANNOT be null
    phantom: PhantomData<T>
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

fn tokens_to_schema_value(tokens: Vec<Token>) -> Result<SchemaValue, &'static str> {
    Err("Unimplemented")
}

fn parse_definition<T: Schematize>(contents: &str) -> Result<BlockDefinition<T>, &'static str> {
    let tokens= tokens::string_to_tokens(contents)?;
    let schema_value= tokens_to_schema_value(tokens)?;

    // Calculate the block size
    // Allocate a new block
    // Deserialize the schema_value into the block's memory
    Err("Unimplemented")
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
