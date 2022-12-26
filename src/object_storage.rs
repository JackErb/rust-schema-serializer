use std::iter::Map;

#[derive(Debug)]
pub enum ObjectStorageValue
{
    Null,
    Object(Map<String, Box<ObjectStorageValue>>),
    Integer32(i32),
    Float32(f32),
    //Bool(bool),
}

// TODO:
// recursive descent parser to read the file, building the values chain as it goes
    // - how are these stored? on the heap? how do lifetimes work here?
//

pub trait ObjectStorage
{
    fn to_object_storage_value(&self) -> ObjectStorageValue;
}

impl ObjectStorage for i32
{
    fn to_object_storage_value(&self) -> ObjectStorageValue
    {
        ObjectStorageValue::Integer32(*self)
    }
}

impl ObjectStorage for f32
{
    fn to_object_storage_value(&self) -> ObjectStorageValue
    {
        ObjectStorageValue::Float32(*self)
    }
}
