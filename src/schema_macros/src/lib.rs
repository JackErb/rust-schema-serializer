/*
    Defines the procedural macro #[derive(Schematize)] which can be added to items (structs/enums)
    to generate schema functionality.

    Functionality includes:
      schema_default
       - generates a default version of the struct, respecting any #[schema_default(...)] markup
      serialize
       - generates a schematized object representation of the object
      deserialize
       - deserializes the schematized object into an instance of the item
*/

extern crate proc_macro;
extern crate proc_macro2;

mod struct_derive;
mod enum_derive;

use quote::quote;

#[proc_macro_derive(Schematize, attributes(schema_default))]
pub fn derive_schematize_impl(item: proc_macro::TokenStream) -> proc_macro::TokenStream {
    // The abstract syntax tree representing the parsed item
    let item_ast: syn::DeriveInput= syn::parse_macro_input!(item);
    let item_ident= &item_ast.ident;

    // Generate the token stream for the schema implementation of this item.
    let schema_impl: proc_macro2::TokenStream= match item_ast.data {
        syn::Data::Struct(data_struct) =>
            match data_struct.fields {
                syn::Fields::Named(fields_named) => {
                    let fields= fields_named.named;

                    // Generate the Schematize implementation for this struct
                    let struct_schema_default_fn= struct_derive::derive_default_fn(item_ident, &fields);
                    let struct_serialize_fn= struct_derive::derive_serialize_fn(&fields);
                    let struct_build_layout_fn= struct_derive::derive_build_layout_fn(&fields);
                    let struct_deserialize_fn= struct_derive::derive_deserialize_fn(item_ident, &fields);

                    quote! {
                        impl Schematize for #item_ident {
                            #struct_schema_default_fn
                            #struct_serialize_fn
                            #struct_build_layout_fn
                            #struct_deserialize_fn
                        }
                    }
                },
                _ => unimplemented!("Schematize only supports named struct fields"),
            }
        syn::Data::Enum(data_enum) => {
            let variants= &data_enum.variants;

            // Generate the Schematize implementation for this struct
            let enum_schema_default_fn= enum_derive::derive_default_fn(item_ident, &variants);
            let enum_serialize_fn= enum_derive::derive_serialize_fn(item_ident, &variants);
            let enum_deserialize_fn= enum_derive::derive_deserialize_fn(item_ident, &variants);

            quote! {
                impl Schematize for #item_ident {
                    #enum_schema_default_fn
                    #enum_serialize_fn
                    #enum_deserialize_fn
                }
            }
        }
        _ => unimplemented!("Schematize only supports structs & enums")
    };

    //println!("{}", schema_impl);

    schema_impl.into()
}
