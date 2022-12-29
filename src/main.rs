use schema::Schematize;
use std::collections::HashMap;
use std::vec::Vec;
use std::marker::Copy;

mod schema_types;

#[derive(Debug)]
pub enum SchemaValue {
    Object(HashMap<&'static str, SchemaValue>), // a hash map of the schematized fields in this struct
    Integer32(i32),
    Float32(f32),
    Bool(bool),
    Array(Vec<SchemaValue>), // dynamically sized array, this is also used to represent static arrays
    EnumVariant(&'static str), // todo: support fields with an optional object attached
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
    // should this be instead ???
    // fn deserialize(&SchemaValue) -> Self;
    fn deserialize(schema_value: &SchemaValue) -> Self;
}

impl Schematize for i32 {
    fn schema_default() -> i32 { 0 }
    fn serialize(&self) -> SchemaValue {
        SchemaValue::Integer32(*self)
    }
    fn deserialize(schema_value: &SchemaValue) -> i32 {
        match schema_value {
            SchemaValue::Integer32(schema_num) => *schema_num,
            _ => unimplemented!("Deserialize i32 hit a wrong value {:?}", schema_value)
        }
    }
}

impl Schematize for f32 {
    fn schema_default() -> f32 { 0.0 }
    fn serialize(&self) -> SchemaValue {
        SchemaValue::Float32(*self)
    }
    fn deserialize(schema_value: &SchemaValue) -> f32 {
        match schema_value {
            SchemaValue::Float32(schema_num) => *schema_num,
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

fn main() {
    let datum= Data { point: [1, 2, 3], inner: InnerData { flag: true, type_enum: DataType::Secondary } };
    println!("{:?}", datum);

    let value= datum.serialize();
    //println!("{:?}", value);

    let datum2= Data::deserialize(&value);
    println!("{:?}", datum2);
}
