/*
    Schematize implementations for primitive types
*/

use crate::*;

macro_rules! schematize_num {
    ($type: ty, $cast_type: ty, $default_value: expr, $($schema_value:tt)*) => {
        impl Schematize for $type {
            fn schema_default() -> $type { $default_value }

            fn serialize(&self) -> SchemaValue {
                $($schema_value)*(*self as $cast_type)
            }

            fn deserialize(schema_value: &SchemaValue) -> SchemaResult<$type> {
                match schema_value {
                    $($schema_value)*(num) => {
                        if *num < <$type>::MIN as $cast_type || *num > <$type>::MAX as $cast_type {
                            println!("Deserialize {} hit a value that is out of bounds {:?}", stringify!($type), schema_value);
                            return Err(SchemaError::NumberOutOfBounds);
                        }
                        Ok(*num as $type)
                    }
                    _ => {
                        println!("Deserialize {} hit a wrong value {:?}", stringify!($type), schema_value);
                        return Err(SchemaError::WrongSchemaValue);
                    }
                }
            }
        }
    }
}

schematize_num!(u8,  i64, 0,   SchemaValue::Integer);
schematize_num!(i32, i64, 0,   SchemaValue::Integer);
schematize_num!(f32, f64, 0.0, SchemaValue::Decimal);

impl Schematize for bool {
    fn schema_default() -> bool { false }

    fn serialize(&self) -> SchemaValue {
        SchemaValue::Bool(*self)
    }

    fn deserialize(schema_value: &SchemaValue) -> SchemaResult<bool> {
        match schema_value {
            SchemaValue::Bool(schema_bool) => Ok(*schema_bool),
            _ => {
                println!("Deserialize bool hit a wrong value {:?}", schema_value);
                return Err(SchemaError::WrongSchemaValue);
            }
        }
    }
}

impl<T: Schematize + Copy, const N: usize> Schematize for [T; N] {
    fn schema_default() -> [T; N] {
        [T::schema_default(); N]
    }

    fn serialize(&self) -> SchemaValue {
        let vector= self.iter().map(|item| item.serialize()).collect();
        SchemaValue::Array(vector)
    }

    fn deserialize(schema_value: &SchemaValue) -> SchemaResult<[T; N]> {
        match schema_value {
            SchemaValue::Array(schema_vector) => {
                if schema_vector.len() != N {
                    println!("Deserialize static array hit an array of the wrong size.\
                        Found {}, expected: {}", schema_vector.len(), N);
                    return Err(SchemaError::WrongSizedArray);
                }

                let mut array: [T; N]= [T::schema_default(); N];
                for (index, item) in schema_vector.iter().enumerate() {
                    array[index]= T::deserialize(item)?;
                }
                Ok(array)
            },
            _ => {
                println!("Deserialize static array hit a wrong value {:?}", schema_value);
                return Err(SchemaError::WrongSchemaValue);
            }
        }
    }
}
