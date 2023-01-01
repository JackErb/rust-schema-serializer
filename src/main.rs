pub mod block;
mod schema_types;
mod schema_string;
mod schema_array;
mod parser;

use schema_macros::Schematize;
pub use schema_array::SchemaArray;
pub use schema_string::SchemaString;

use std::collections;
use std::vec::Vec;
use std::env;
use std::alloc;

#[derive(Debug)]
pub enum SchemaValue<'a> {
    Object(collections::HashMap<&'a str, SchemaValue<'a>>), // represents a schematized struct
    Integer(i64),
    Decimal(f64),
    Bool(bool),
    Array(Vec<SchemaValue<'a>>), // array of arbitrary size
    String(&'a str),             // string of arbitrary size
    EnumVariant(&'a str), // todo: support fields with an optional object attached
    Null,
    // TODO:
    //   String
    //   Impl (schema owner pointer/mix in pattern)
    //     - the mix in pattern could be accomplished via Enums with fields instead, maybe easier
    //   Array
    //     - we have the schema array type, but need to support serializing into a dynamic sized array
}

#[derive(Debug)]
pub enum SchemaError {
    WrongSchemaValue,
    MissingField,
    WrongSizedArray,
    NumberOutOfBounds,
    UnknownField,
    UnknownIdentifier,
}

type SchemaResult<T>= Result<T, SchemaError>;

pub struct DeserializeContext<'a> {
    block_ptr: *mut u8,    // the allocated block of memory to deserialize into
    offsets: Vec<usize>,   // built recursively in Schematize::build_layout()
    offset_index: usize,   // incremented in recursive Schematize::deserialize() calls

    // TODO: this should be debug only
    path: Vec<&'a str>, // The field path when deserializing nested objects, e.g. inner.point.x
}

impl DeserializeContext<'_> {
    pub fn get_path(&self) -> String {
        self.path.join(".")
    }
}

type BuildLayoutResult = Result<alloc::Layout, alloc::LayoutError>;

pub trait Schematize {
    fn schema_default() -> Self;
    fn serialize(&self) -> SchemaValue;

    // In order to deserialize, you must first build the layout to allocate the memory.
    // These two functions must traverse their fields in the same way.
    fn build_layout(_schema_value: &SchemaValue, layout: alloc::Layout, _offsets: &mut Vec<usize>)
        -> BuildLayoutResult {
        /* NO-OP. Most types don't use any dynamic memory */
        Ok(layout)
    }
    fn deserialize(schema_value: &SchemaValue, context: &mut DeserializeContext) -> SchemaResult<Self> where Self: Sized;
}

#[derive(Schematize, Debug)]
enum DataType {
    Primary,
    Secondary,
    Tertiary,
}

#[derive(Schematize, Debug)]
struct InnerData {
    flag: bool,
    #[schema_default(type_enum=DataType::Tertiary)]
    type_enum: DataType,
}

#[derive(Schematize, Debug)]
struct Data {
    //#[schema_default(point[0]=-1)]
    //point: [i32; 3],

    #[schema_default(inner.flag=false)]
    inner: InnerData,
}

#[derive(Schematize, Debug)]
struct Wrapper {
    s: SchemaString,
}

#[derive(Schematize, Debug)]
struct StringWrapper {
    string: SchemaArray::<Wrapper>,
}

#[derive(Schematize, Debug)]
struct ParserData {
    point: SchemaArray::<SchemaArray::<SchemaString>>,
}

fn parse_test() {
    let args: Vec<String>= env::args().collect();
    if args.len() > 1 {
        let file_path= &args[1];
        let block_definition= parser::load_definition::<ParserData>(&file_path);

        match block_definition {
            Some(definition) => {
                println!("Successfully loaded block definition '{}'", file_path);
                println!("{:?}", definition.get_definition())
            },
            None => {
                println!("Failed to load block definition '{}'", file_path);
            }
        }
    }
}

fn main() {
    parse_test();
}
