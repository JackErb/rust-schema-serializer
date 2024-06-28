mod tokens;
mod schema;
mod debug;

use crate::*;

use std::marker;
use std::fs;
use std::str;
use std::alloc;

pub type ParseResult<T>= Result<T, &'static str>;

pub struct BlockDefinition<T> {
    // TODO: include tag name handle in here
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

    // Recursively build the layout on the fields in this schematize type.
    // This is a no-op unless there are fields using dynamic memory (e.g. strings, vectors)
    let layout_result= T::build_layout(&schema_value, layout, &mut layout_offsets);

    let layout;
    match layout_result {
        Ok(built_layout) => {
            layout= built_layout.pad_to_align();
        },
        Err(_) => return Err("Failed to build layout for definition."),
    };

    // Allocate the block memory
    let block_definition= BlockDefinition {
        block_handle: block::allocate_block(layout),
        phantom: marker::PhantomData,
    };

    // Memory will be written directly to `block_ptr`, using the offset as defined by `offsets[offset_index]`.
    // offset_index is increment as we recursively deserialize fields. `build_layout()` and `deserialize()`
    // MUST use the same logic in determining if the schema value is using dynamic memory, and MUST traverse
    // the fields in the same order so that the offsets line up correctly.
    //
    // This means that the *.def file MUST be a valid representation of the object with the
    // fields defined in the same order as the Rust structure.
    // TODO: Enforce this variant.
    let mut deserialize_context= DeserializeContext {
        block_ptr: block_definition.get_block_handle().get_pointer_mut_as::<u8>(),
        offsets: layout_offsets,
        offset_index: 0,
        path: Vec::new(),  // used for debug inspection
    };

    // Deserialize the definition into the block memory
    let result= T::deserialize(&schema_value, &mut deserialize_context);
    match result {
        Ok(deserialized_definition) => {
            *block_definition.get_definition_mut()= deserialized_definition;
            Ok(block_definition)
        },
        Err(e) => {
            println!("  SchemaError::{:?}", e);
            Err("Failed to deserialize schema definition.")
        }
    }
}

// Reads, parses, and schematizes the the given definition file from disk
pub fn load_definition<T: Schematize>(file_path: &str) -> Result<BlockDefinition<T>, &str> {
    // TODO:
    // - We should have a caching system so if a definition is requested multiple times,
    //   it can reuse the existing memory.
    // - Input options about loading definitions, e.g. versioning markup on structs, or markup/commands
    //   to force deserialize an object into its default schema values if it hits an error.


    let file_contents= fs::read_to_string(file_path);
    match file_contents {
        Ok(file_contents) =>
            build_definition(&file_contents),
        Err(err) => {
            println!("Failed to read file contents '{}'.\n Error: {}", file_path, err);
            Err("Failed to read file contents.")
        }
    }
}

// Given a schematized object, write its definition file to disk.
pub fn serialize_definition<T: Schematize>(object: BlockDefinition<T>) -> String {
    // Serialize the block into a formatted string
    let mut serialize_context= SerializeContext {
        string: String::new(),
        tabs: 0,
    };
    object.get_definition().serialize(&mut serialize_context);

    serialize_context.string

    //fs::write(file_path, serialize_context.string.as_bytes()).expect("Unable to write file.");
}
