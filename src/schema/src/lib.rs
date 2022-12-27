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

mod schema_derive;

use schema_derive::*;
use quote::quote;

#[proc_macro_derive(Schematize, attributes(schema_default))]
pub fn derive_schematize_impl(
    item: proc_macro::TokenStream) -> proc_macro::TokenStream
{
    // The abstract syntax tree representing the parsed item
    let item_ast: syn::DeriveInput= syn::parse_macro_input!(item);
    let item_ident= &item_ast.ident;

    // Generate the token stream for the schema implementation of this item.
    let schema_impl: proc_macro2::TokenStream= match item_ast.data
    {
        syn::Data::Struct(data_struct) =>
            match data_struct.fields
            {
                syn::Fields::Named(fields_named) =>
                {
                    let fields= fields_named.named;

                    // Generated the Schematize implementation for this struct
                    let fields_schema_default_fn= derive_schema_default_fn(item_ident, &fields);
                    let fields_serialize_fn= derive_serialize_fn(&fields);
                    let fields_deserialize_fn= derive_deserialize_fn(&fields);

                    quote! (
                        impl Schematize for #item_ident
                        {
                            #fields_schema_default_fn
                            #fields_serialize_fn
                            #fields_deserialize_fn
                        }
                    )
                },
                _ => unimplemented!("Schematize only supports named structs"),
            }
        syn::Data::Enum(_) => todo!("Serialize is not yet implemented for enum."),
        _ => unimplemented!("Schematize only supports structs & enums")
    };

    println!("{}", schema_impl);

    schema_impl.into()
}
