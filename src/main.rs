use schema_macros::Schematize;
use schema_types::DynamicArray;

use std::collections;
use std::vec::Vec;
use std::marker::Copy;
use std::env;

pub mod block;
mod schema_types;
mod parser;

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

impl Schematize for i32 {
    fn schema_default() -> i32 { 0 }

    fn serialize(&self) -> SchemaValue {
        SchemaValue::Integer(*self as i64)
    }

    fn deserialize(schema_value: &SchemaValue) -> i32 {
        match schema_value {
            SchemaValue::Integer(num) => {
                if *num < i32::MIN as i64 || *num > i32::MAX as i64 {
                    unimplemented!("Deserialize i32 hit a value that is out of bounds {:?}", schema_value);
                }
                *num as i32
            }
            _ => unimplemented!("Deserialize i32 hit a wrong value {:?}", schema_value)
        }
    }
}

impl Schematize for f32 {
    fn schema_default() -> f32 { 0.0 }

    fn serialize(&self) -> SchemaValue {
        SchemaValue::Decimal(*self as f64)
    }

    fn deserialize(schema_value: &SchemaValue) -> f32 {
        match schema_value {
            SchemaValue::Decimal(schema_num) =>  {
                // Note that this is downcasting... should we do any bounds checks?
                *schema_num as f32
            }
            _ => unimplemented!("Deserialize f32 hit a wrong value {:?}", schema_value)
        }
    }
}

impl Schematize for bool {
    fn schema_default() -> bool { false }

    fn serialize(&self) -> SchemaValue {
        SchemaValue::Bool(*self)
    }

    fn deserialize(schema_value: &SchemaValue) -> bool {
        match schema_value {
            SchemaValue::Bool(schema_bool) => *schema_bool,
            _ => unimplemented!("Deserialize bool hit a wrong value {:?}", schema_value)
        }
    }
}

impl<T: Schematize + Copy, const N: usize> Schematize for [T; N] {
    fn schema_default() -> [T; N] { unimplemented!("schema_default() is not supported on arrays."); }

    fn serialize(&self) -> SchemaValue {
        let vector= self.iter().map(|item| item.serialize()).collect();
        SchemaValue::Array(vector)
    }

    fn deserialize(schema_value: &SchemaValue) -> [T; N] {
        match schema_value {
            SchemaValue::Array(schema_vector) => {
                assert!(schema_vector.len() == N, "Deserialize hit an array of the wrong size.");
                let mut array: [T; N]= [T::schema_default(); N];
                for (index, item) in schema_vector.iter().enumerate() {
                    array[index]= T::deserialize(item);
                }
                array
            },
            _ => unimplemented!("Deserialize array hit a wrong value")
        }
    }
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
    point: DynamicArray::<DynamicArray::<i32>>,
    variant: DataType,
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
