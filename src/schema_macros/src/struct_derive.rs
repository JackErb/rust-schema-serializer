extern crate proc_macro2;

use quote::quote;

type StructFields= syn::punctuated::Punctuated<syn::Field, syn::token::Comma>;

// Cast to an explicit enum type, panicking if it fails.
macro_rules! cast {
    ($target: expr, $pat: path) => {
        {
            if let $pat(a) = $target {
                a
            } else {
                panic!(
                    "mismatch variant when cast to {}",
                    stringify!($pat));
            }
        }
    };
}

fn generate_default_value(field_type: &syn::Type) -> proc_macro2::TokenStream {
    match field_type
    {
        syn::Type::Array(array) => {
            let default_value= generate_default_value(&array.elem);
            let size= cast!(&cast!(&array.len, syn::Expr::Lit).lit, syn::Lit::Int);
            quote! {
                [#default_value; #size]
            }
        }
        _ =>
            quote! {
                #field_type::schema_default()
            }
    }
}


pub fn derive_default_fn(
    item_ident: &syn::Ident,
    fields: &StructFields
) -> proc_macro2::TokenStream {

    // Generate the token stream for initializing the default struct
    //  e.g. x: 0, y: 0.0, points: [0,0,0]
    let fields_init_default= fields.iter().map(
        |field| -> proc_macro2::TokenStream {
            let field_ident= &field.ident;
            let schema_default_value= generate_default_value(&field.ty);
            quote! {
                #field_ident : #schema_default_value
            }
        });

    // Generate the token stream for setting schema default values. These are defined by macro helper attributes.
    //  e.g.     #[schema_default(x=32)]
    //           i32 x;
    // the statement `x=32` is inlined into the schema_default function.
    // this allows more complex statements:
    //  e.g.    #[schema_default(inner.points[0]=32)]
    //          InnerStruct inner;
    let fields_schema_default=
        fields.iter().map(|field|
                // Look for any schema_default markup on this field
                field.attrs.iter()
                    .filter(|attr| attr.path.is_ident("schema_default"))
                    .map(|attr| -> proc_macro2::TokenStream {
                            // Parse the expression, then return the token stream
                            let attr_tokens= attr.parse_args::<proc_macro2::TokenStream>()
                                .expect("Unable to parse schema_default attribute");
                            quote! ( #attr_tokens )
                        })
            ).flatten();

    quote! {
        fn schema_default() -> #item_ident {
            // Create a default, zero-ed out #item_ident.
            let mut schema_default= #item_ident { #(#fields_init_default),* };

            // Set any overrides specified by schema_default markup
            #(schema_default.#fields_schema_default;)*

            schema_default
        }
    }
}

pub fn derive_serialize_fn(fields: &StructFields) -> proc_macro2::TokenStream {
    // Generate the token stream for building the field map
    let fields_serialize= fields.iter().map(
        |field| -> proc_macro2::TokenStream {
            let field_ident= &field.ident;
            quote! {
                // Insert to the map, recurisvely calling serialize on the field.
                //    e.g. ("x", SchemaValue::Integer(32))
                fields_map.insert(stringify!(#field_ident), self.#field_ident.serialize());
            }
        });

    quote! {
        fn serialize(&self) -> SchemaValue {
            // Build the hash map representing this object
            let mut fields_map= std::collections::HashMap::<&str, SchemaValue>::new();
            #(#fields_serialize)*

            SchemaValue::Object(fields_map)
        }
    }
}

pub fn derive_build_layout_fn(
    fields: &StructFields,
) -> proc_macro2::TokenStream {
    let fields_build_layout= fields.iter().map (
        |field| -> proc_macro2::TokenStream {
            let field_ident= &field.ident;
            let field_type= &field.ty;
            quote! {
                let schema_value= fields_map.get(stringify!(#field_ident)).unwrap_or(&SchemaValue::Null);
                let layout= <#field_type>::build_layout(schema_value, layout, offsets)?;
            }
        }
    );

    quote! {
        fn build_layout(schema_value: &SchemaValue, layout: alloc::Layout, offsets: &mut Vec<usize>)
            -> Result<alloc::Layout, alloc::LayoutError> {
            match schema_value {
                SchemaValue::Object(fields_map) => {
                    #(#fields_build_layout)*

                    Ok(layout)
                },
                _ => {
                    Ok(layout)
                }
            }

        }
    }
}

pub fn derive_deserialize_fn(
    item_ident: &syn::Ident,
    fields: &StructFields
) -> proc_macro2::TokenStream {

    let fields_count= fields.len();

    let fields_validity_check= fields.iter().map(
        |field| -> proc_macro2::TokenStream {
            let field_ident= &field.ident;
            quote! {
                if !fields_map.contains_key(stringify!(#field_ident)) {
                    let field_path= format!("{}.{}", context.get_path(), stringify!(#field_ident));
                    println!("Deserialize object '{}' is missing field '{:?}'", stringify!(#item_ident), stringify!(#field_ident));
                    println!("Field path '{}'", field_path);
                    return Err(SchemaError::MissingField);
                }
            }
        }
    );

    let fields_deserialize= fields.iter().map(
        |field| -> proc_macro2::TokenStream {
            let field_ident= &field.ident;
            let field_type= &field.ty;
            quote! {
                // Deserialize the field given the schema value
                #field_ident: {
                    context.path.push(stringify!(#field_ident));
                    let value= <#field_type>::deserialize(&fields_map[stringify!(#field_ident)], context)?;
                    context.path.pop();
                    value
                }
            }
        });

    quote! {
        fn deserialize(schema_value: &SchemaValue, context: &mut DeserializeContext) -> SchemaResult<#item_ident> {
            match schema_value {
                SchemaValue::Object(fields_map) => {
                    // Perform validity checks on the map
                    #(#fields_validity_check)*

                    if fields_map.len() != #fields_count {
                        println!("Deserialize object {} contains extraneous unknown field(s).", stringify!(#item_ident));
                        return Err(SchemaError::UnknownIdentifier);
                    }

                    // Create the deserialized object with all of its deserialized fields
                    Ok(#item_ident { #(#fields_deserialize),* })
                },
                _ => {
                    println!("Deserialize hit a wrong value for field '{}'. Expected: Object({}), found: {:?}",
                        context.get_path(),
                        stringify!(#item_ident),
                        schema_value);
                    return Err(SchemaError::WrongSchemaValue);
                }
            }
        }
    }
}
