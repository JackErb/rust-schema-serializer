use schema::Schematize;
use std::collections::HashMap;
use std::vec::Vec;

mod string;
use string::SchemaString;


#[derive(Debug)]
pub enum SchemaValue
{
    Object(HashMap<&'static str, SchemaValue>), // a hash map of the schematized fields in this struct
    Integer32(i32),
    Float32(f32),
    Bool(bool),
    Array(Vec<SchemaValue>), // dynamically sized array, this is also used to represent static arrays
    EnumVariant(&'static str), // todo: support fields with an optional object attached
    //String(Box<&str>),
    // TODO:
    //   String
    //   Impl (schema owner pointer/mix in pattern)
    //     - the mix in pattern could be accomplished via Enums with fields instead, maybe easier
    //   Array
    //     - we have the schema array type, but need to support serializing into a dynamic sized array
}

pub trait Schematize
{
    fn schema_default() -> Self;
    fn serialize(&self) -> SchemaValue;
    fn deserialize(&mut self, schema_value: &SchemaValue);
}

impl Schematize for i32
{
    fn schema_default() -> i32 { 0 }
    fn serialize(&self) -> SchemaValue
    {
        SchemaValue::Integer32(*self)
    }
    fn deserialize(&mut self, schema_value: &SchemaValue)
    {
        match schema_value
        {
            SchemaValue::Integer32(schema_num) => *self= *schema_num,
            _ => unimplemented!("Deserialize i32 hit a wrong value {:?}", schema_value)
        }
    }
}

impl Schematize for f32
{
    fn schema_default() -> f32 { 0.0 }
    fn serialize(&self) -> SchemaValue
    {
        SchemaValue::Float32(*self)
    }
    fn deserialize(&mut self, schema_value: &SchemaValue)
    {
        match schema_value
        {
            SchemaValue::Float32(schema_num) => *self= *schema_num,
            _ => unimplemented!("Deserialize f32 hit a wrong value {:?}", schema_value)
        }
    }
}

impl Schematize for bool
{
    fn schema_default() -> bool { false }
    fn serialize(&self) -> SchemaValue
    {
        SchemaValue::Bool(*self)
    }
    fn deserialize(&mut self, schema_value: &SchemaValue)
    {
        match schema_value
        {
            SchemaValue::Bool(schema_bool) => *self= *schema_bool,
            _ => unimplemented!("Deserialize bool hit a wrong value {:?}", schema_value)
        }
    }
}

impl<T: Schematize, const N: usize> Schematize for [T; N]
{
    fn schema_default() -> [T; N] { unimplemented!("schema_default() is not supported on arrays."); }
    fn serialize(&self) -> SchemaValue
    {
        let vector= self.iter().map(|item| item.serialize()).collect();
        SchemaValue::Array(vector)
    }
    fn deserialize(&mut self, schema_value: &SchemaValue)
    {
        match schema_value
        {
            SchemaValue::Array(schema_vector) =>
            {
                assert!(schema_vector.len() == N, "Deserialize hit an array of the wrong size.");
                for (index, item) in schema_vector.iter().enumerate()
                {
                    self[index].deserialize(item);
                }
            }
            _ => unimplemented!("Deserialize array hit a wrong value")
        }
    }
}

#[derive(Schematize, Debug)]
enum DataType
{
    Primary,
    Secondary,
    Tertiary,
}

#[derive(Schematize, Debug)]
struct InnerData
{
    flag: bool,
    #[schema_default(type_enum=DataType::Tertiary)]
    type_enum: DataType,
}

#[derive(Schematize, Debug)]
struct Data
{
    #[schema_default(point[0]=-1)]
    point: [i32; 3],

    #[schema_default(inner.flag=false)]
    inner: InnerData,

    string: SchemaString;
}

fn main() {
    let datum= Data { point: [1, 2, 3], inner: InnerData { flag: true, type_enum: DataType::Secondary } };
    println!("{:?}", datum);

    let value= datum.serialize();
    //println!("{:?}", value);

    let mut datum2= Data::schema_default();
    //println!("{:?}", datum2);

    datum2.deserialize(&value);
    println!("{:?}", datum2);
}
