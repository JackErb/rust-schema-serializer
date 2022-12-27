use schema::Schematize;
use std::collections::HashMap;

#[derive(Debug)]
pub enum SchemaValue
{
    Null,
    Object(HashMap<&'static str, SchemaValue>),
    Integer32(i32),
    Float32(f32),
    Bool(bool),
    // TODO:
    //  String
    //  Enum
    //  Impl (schema owner pointer)
    //  Array (static, dynamic)
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

#[derive(Schematize, Debug)]
struct InnerData
{
    w: f32,
    flag: bool,
}

#[derive(Schematize, Debug)]
struct Data
{
    x: i32,
    y: i32,
    z: i32,
    inner: InnerData,
}

fn main() {
    let datum= Data { x: -1, y: 20, z: 3, inner: InnerData { w: -1.2, flag: true } };
    println!("{:?}", datum);

    let value= datum.serialize();
    println!("{:?}", value);

    let mut datum2= Data::schema_default();
    datum2.deserialize(&value);
    println!("{:?}", datum2);
}
