extern crate proc_macro2;

use quote::quote;

type StructFields= syn::punctuated::Punctuated<syn::Field, syn::token::Comma>;

pub(crate)
fn derive_schema_default_fn(
    item_ident: &syn::Ident,
    fields: &StructFields) -> proc_macro2::TokenStream
{
    // Generate the token stream for initializing the default struct
    let fields_init_default= fields.iter().map(|field| -> proc_macro2::TokenStream
        {
            let field_ident= &field.ident;
            let field_type= &field.ty;

            quote! ( #field_ident : #field_type::schema_default() )
        });

    // Generate the token stream for schema default values. These are defined by macro helper attributes
    //  e.g.     #[schema_default(x=32)]
    //           i32 x;
    // the statement `x=32` is inlined into the schema_default function.
    // this allows more complex statements:
    //  e.g.    #[schema_default(inner.x=32)]
    //          InnerStruct inner;
    let fields_schema_default=
        fields.iter().map(|field|
                // Look for any schema_default markup on this field
                field.attrs.iter()
                    .filter(|attr| attr.path.is_ident("schema_default"))
                    .map(|attr| -> proc_macro2::TokenStream
                        {
                            // Parse the expression, then return the token stream
                            let attr_tokens= attr.parse_args::<proc_macro2::TokenStream>()
                                .expect("Unable to parse schema_default attribute");
                            quote! ( #attr_tokens )
                        })
            ).flatten();

    quote! (
        fn schema_default() -> #item_ident
        {
            // Create a basic schema default, zero-ed out #item_ident.
            let mut schema_default= #item_ident { #(#fields_init_default),* };

            // Set any overrides specified by schema_default markup
            #(schema_default.#fields_schema_default;)*

            schema_default
        }
    )
}

pub(crate)
fn derive_serialize_fn(
    fields: &StructFields) -> proc_macro2::TokenStream
{
    // Generate the token stream for building the field map
    let fields_serialize= fields.iter().map(|field| -> proc_macro2::TokenStream
        {
            let field_ident= &field.ident;
            quote! (
                // Insert to the map, recurisvely calling serialize on the field.
                //    e.g. ("x", SchemaValue::Integer(32))
                fields_map.insert(stringify!(#field_ident), self.#field_ident.serialize());
            )
        });

    quote!(
        fn serialize(&self) -> SchemaValue
        {
            // Build the hash map representing this object
            let mut fields_map= std::collections::HashMap::<&'static str, SchemaValue>::new();
            #(#fields_serialize)*

            SchemaValue::Object(fields_map)
        }
    )
}

pub(crate)
fn derive_deserialize_fn(
    fields: &StructFields) -> proc_macro2::TokenStream
{
    let fields_deserialize= fields.iter().map(|field| -> proc_macro2::TokenStream
        {
            let field_ident= &field.ident;
            quote! (
                // Deserialize the field given the schema value from the object's schema
                self.#field_ident.deserialize(&fields_map[stringify!(#field_ident)]);
            )
        });

    quote!(
        fn deserialize(&mut self, schema_value: &SchemaValue)
        {
            match schema_value
            {
                SchemaValue::Object(fields_map) =>
                {
                    // Recursively call deserialize on all fields
                    #(#fields_deserialize)*
                }
                _ => unimplemented!("Deserialize object hit a wrong value {:?}", schema_value),
            }
        }
    )
}