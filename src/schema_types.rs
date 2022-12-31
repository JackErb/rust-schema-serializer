/*
    Schematize implementations for primitive types
*/

use crate::SchemaValue;
use crate::Schematize;

macro_rules! schematize_int {
    ($type: ty) => {
        impl Schematize for $type {
            fn schema_default() -> $type { 0 }

            fn serialize(&self) -> SchemaValue {
                SchemaValue::Integer(*self as i64)
            }

            fn deserialize(schema_value: &SchemaValue) -> $type {
                match schema_value {
                    SchemaValue::Integer(num) => {
                        if *num < <$type>::MIN as i64 || *num > <$type>::MAX as i64 {
                            unimplemented!("Deserialize i32 hit a value that is out of bounds {:?}", schema_value);
                        }
                        *num as $type
                    }
                    _ => unimplemented!("Deserialize {} hit a wrong value {:?}", stringify!($type), schema_value)
                }
            }
        }
    }
}

schematize_int!(u8);
schematize_int!(i32);

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
            _ => unimplemented!("Deserialize array hit a wrong value {:?}", schema_value)
        }
    }
}
