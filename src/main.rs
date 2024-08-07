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
    // represents a schematized struct
    Object(collections::HashMap<&'a str, SchemaValue<'a>>),
    Integer(i64),
    Decimal(f64),
    Bool(bool),
    // array of arbitrary size
    Array(Vec<SchemaValue<'a>>),
    // string of arbitrary size
    String(&'a str),
    EnumVariant(&'a str, Box<SchemaValue<'a>>),
    Null,

    // TODO:
    //   Impl (schema owner pointer/mix in pattern)
    //     - we do support mix-ins in a way with enum variants w/ fields, but that breaks down
    //       for recursive cases (e.g. an enum which can itself reference the same enum)
    //   Schema pointers, e.g. pointing to some memory within same .def file.
    //     - point to an entry of an array
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

pub struct DeserializeContext {
    block_ptr: *mut u8,    // the allocated block of memory to deserialize into
    offsets: Vec<usize>,   // built recursively in Schematize::build_layout()
    offset_index: usize,   // incremented in recursive Schematize::deserialize() calls

    // TODO: this should be debug only
    path: Vec<String>, // The field path when deserializing nested objects, e.g. inner.point.x
}

impl DeserializeContext {
    pub fn get_path(&self) -> String {
        let full_path=self.path.join("");
        if !full_path.is_empty() {
            // trim off the first character which is a duplicate period
            let mut chars= full_path.chars();
            chars.next();
            String::from(chars.as_str())
        } else {
            full_path
        }
    }
}

pub struct SerializeContext {
    string: String,
    tabs: i16,
}

impl SerializeContext {
    fn print(&mut self, content: &str) {
        self.string.push_str(content);
    }

    fn print_tabs(&mut self) {
        for _ in 0..self.tabs {
            self.string.push_str("  ");
        }
    }

    fn println(&mut self) {
        self.string.push_str("\n");
    }
}

type BuildLayoutResult = Result<alloc::Layout, alloc::LayoutError>;

pub trait Schematize {
    fn schema_default() -> Self;

    // Write the data of this object to a string. This is the inverse of parser::load_definition<>
    fn serialize(&self, context: &mut SerializeContext);

    // In order to deserialize, you must first build the layout to allocate the memory.
    fn build_layout(_schema_value: &SchemaValue, layout: alloc::Layout, _offsets: &mut Vec<usize>)
        -> BuildLayoutResult {
        // NO-OP. Most types don't use any dynamic memory
        Ok(layout)
    }

    fn deserialize(schema_value: &SchemaValue, context: &mut DeserializeContext) -> SchemaResult<Self> where Self: Sized;
}

#[derive(Schematize, Debug)]
enum DataType {
    Primary,
    Secondary,
    Tertiary(StringWrapper)
}

#[derive(Schematize, Debug)]
struct InnerData {
    flag: bool,
    #[schema_default(type_enum=DataType::Secondary)]
    type_enum: DataType,
}

#[derive(Schematize, Debug)]
struct Data {
    point: [i32; 3],
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
    s: SchemaString,
    inner: InnerData,
    point: SchemaArray::<SchemaArray::<SchemaString>>,
    inners: SchemaArray::<SchemaArray::<InnerData>>,
}

fn parse_test() {
    let args: Vec<String>= env::args().collect();
    if args.len() > 1 {
        let file_path= &args[1];
        let block_definition= parser::load_definition::<ParserData>(&file_path);

        match block_definition {
            Ok(definition) => {
                println!("Successfully loaded block definition '{}'", file_path);
                println!("{:?}", definition.get_definition());

                println!("Serializing definition");
                println!("\n{}", parser::serialize_definition(definition));
            },
            Err(_) => {
                println!("Failed to load block definition '{}'", file_path);
            }
        }
    } else {
        println!("No arguments supplied. Terminating program.");
    }
}

fn main() {
    parse_test();
}
