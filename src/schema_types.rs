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

            fn deserialize(schema_value: &SchemaValue, context: &mut DeserializeContext) -> SchemaResult<$type> {
                match schema_value {
                    $($schema_value)*(num) => {
                        println!("Deserialize {}", *num);
                        if *num < <$type>::MIN as $cast_type || *num > <$type>::MAX as $cast_type {
                            println!("Deserialize {} hit a value that is out of bounds {:?}", stringify!($type), schema_value);
                            return Err(SchemaError::NumberOutOfBounds);
                        }
                        Ok(*num as $type)
                    }
                    _ => {
                        println!("Deserialize hit a wrong value for field '{}'. Expected: {}, found: {:?}",
                            context.get_path(),
                            stringify!($($schema_value)*),
                            schema_value);
                        return Err(SchemaError::WrongSchemaValue);
                    }
                }
            }
        }
    }
}

schematize_num!(u8,  i64, 0, SchemaValue::Integer);
schematize_num!(i32, i64, 0, SchemaValue::Integer);
schematize_num!(f32, f64, 0.0, SchemaValue::Decimal);

impl Schematize for bool {
    fn schema_default() -> bool { false }

    fn serialize(&self) -> SchemaValue {
        SchemaValue::Bool(*self)
    }

    fn deserialize(schema_value: &SchemaValue, context: &mut DeserializeContext) -> SchemaResult<bool> {
        match schema_value {
            SchemaValue::Bool(schema_bool) => Ok(*schema_bool),
            _ => {
                println!("Deserialize hit a wrong value for field '{}'. Expected: Bool, found: {:?}",
                    context.get_path(),
                    schema_value);
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
    
    fn deserialize(schema_value: &SchemaValue, context: &mut DeserializeContext) -> SchemaResult<[T; N]> {
        match schema_value {
            SchemaValue::Array(schema_vector) => {
                if schema_vector.len() != N {
                    println!("Deserialize hit a static array of the wrong size for field '{}'. \
                        Expected: {}, found: {}", context.get_path(), N, schema_vector.len());
                    return Err(SchemaError::WrongSizedArray);
                }

                let mut array: [T; N]= [T::schema_default(); N];
                for (index, item) in schema_vector.iter().enumerate() {
                    array[index]= T::deserialize(item, context)?;
                }
                Ok(array)
            },
            _ => {
                println!("Deserialize hit a wrong value for field '{}'. Expected: {}, found: {:?}",
                    context.get_path(),
                    stringify!([T; N]),
                    schema_value);
                return Err(SchemaError::WrongSchemaValue);
            }
        }
    }
}
