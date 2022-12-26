use serializer::SchemaSerializer;
mod object_storage;

use object_storage::ObjectStorageValue;
use object_storage::ObjectStorage;

pub trait Serializer
{
    fn debug_print(&self);
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

    datum.debug_print();
}
