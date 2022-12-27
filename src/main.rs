use serializer::SchemaSerializer;
use std::collections::HashMap;

#[derive(Debug)]
pub enum SchemaValue
{
    Null,
    Object(HashMap<&'static str, Box<SchemaValue>>),
    Integer32(i32),
    Float32(f32),
    Bool(bool),
}

pub trait SchemaSerializer
{
    fn serialize(&self) -> Box<SchemaValue>;
}

impl SchemaSerializer for i32
{
    fn serialize(&self) -> Box<SchemaValue>
    {
        Box::new(SchemaValue::Integer32(*self))
    }
}

impl SchemaSerializer for f32
{
    fn serialize(&self) -> Box<SchemaValue>
    {
        Box::new(SchemaValue::Float32(*self))
    }
}

impl SchemaSerializer for bool
{
    fn serialize(&self) -> Box<SchemaValue>
    {
        Box::new(SchemaValue::Bool(*self))
    }
}

#[derive(SchemaSerializer)]
struct Data
{
    x: i32,
    y: i32,
    z: i32,
    w: f32,
}

fn main() {
    let datum= Data { x: -1, y: 20, z: 3, w: -1.2 };

    let data= datum.serialize();
    println!("{:?}", data);
}
