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

/*
struct ObjectData {
    collections: collections::HashMap<&'static str, SchemaValue>,
    offsets: collections::HashMap<&'static str, usize>, // contains offsets of dynamic fields
}
*/

#[derive(Debug)]
pub enum SchemaValue<'a> {
    Object(collections::HashMap<&'a str, SchemaValue<'a>>), // represents a schematized struct
    Integer(i64),
    Decimal(f64),
    Bool(bool),
    Array(Vec<SchemaValue<'a>>), // dynamically sized array, this is also used to represent static arrays
    String(&'a str),
    EnumVariant(&'a str), // todo: support fields with an optional object attached
    Null,
    // TODO:
    //   String
    //   Impl (schema owner pointer/mix in pattern)
    //     - the mix in pattern could be accomplished via Enums with fields instead, maybe easier
    //   Array
    //     - we have the schema array type, but need to support serializing into a dynamic sized array
}

pub trait Schematize {

    fn schema_default() -> Self;
    fn serialize(&self) -> SchemaValue;
    fn deserialize(schema_value: &SchemaValue) -> Self;

    //fn build_layout(schema_value: &SchemaValue) -> (std::Layout, )
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
    #[schema_default(point[0]=-1)]
    point: [i32; 3],

    #[schema_default(inner.flag=false)]
    inner: InnerData,
}

#[derive(Schematize, Debug)]
struct ParserData {
    x: i32,
    point: SchemaArray::<SchemaArray::<i32>>,
    str: SchemaString,
    data: InnerData,
}

fn serde_test() {
    let datum= Data { point: [1, 2, 3], inner: InnerData { flag: true, type_enum: DataType::Secondary } };
    println!("{:?}", datum);

    let serialized_value= datum.serialize();

    let datum2= Data::deserialize(&serialized_value);
    println!("{:?}", datum2);
}

fn parse_test() {
    let args: Vec<String>= env::args().collect();
    println!("{:?}", args);
    if args.len() > 1 {
        let file_path= &args[1];
        println!("Reading block definition '{}'", file_path);
        let block_definition= parser::load_definition::<ParserData>(&file_path);

        println!("{:?}", block_definition.unwrap().get_definition());
    } else {

    }
}

fn main() {
    //serde_test();
    parse_test();
}
