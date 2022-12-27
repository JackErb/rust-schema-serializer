extern crate proc_macro;
extern crate proc_macro2;

use quote::quote;

type Fields= syn::punctuated::Punctuated<syn::Field, syn::token::Comma>;

fn derive_schema_default_fn(item_ident: &syn::Ident, fields: &Fields) -> proc_macro2::TokenStream
{
    // Set the schema default values
    let fields_init_default= fields.iter().map(|field| -> proc_macro2::TokenStream
        {
            let field_ident= &field.ident;
            let field_type= &field.ty;

            quote! ( #field_ident : #field_type::schema_default() )
        });

    // Set the schema default value overrides defined by helper attributes
    let fields_schema_default=
        fields.iter().map(|field|
                // Filter for all schema_default markup on this field
                field.attrs.iter()
                    .filter(|attr| attr.path.is_ident("schema_default"))
                    .map(|attr| -> proc_macro2::TokenStream
                        {
                            // Parse the schema default expression
                            let attr_tokens= attr.parse_args::<proc_macro2::TokenStream>()
                                .expect("Unable to parse schema_default attribute");
                            quote! ( #attr_tokens )
                        })
            ).flatten();

    quote! (
        fn schema_default() -> #item_ident
        {
            let mut schema_default= #item_ident { #(#fields_init_default),* };
            #(schema_default.#fields_schema_default;)*
            schema_default
        }
    )
}

fn derive_serialize_fn(fields: &Fields) -> proc_macro2::TokenStream
{
    let fields_serialize= fields.iter().map(|field| -> proc_macro2::TokenStream
        {
            let field_ident= &field.ident;
            quote! (
                fields_map.insert(stringify!(#field_ident), self.#field_ident.serialize());
            )
        });

    quote!(
        fn serialize(&self) -> SchemaValue
        {
            let mut fields_map= std::collections::HashMap::<&'static str, SchemaValue>::new();
            #(#fields_serialize)*

            SchemaValue::Object(fields_map)
        }
    )
}


fn derive_deserialize_fn(fields: &Fields) -> proc_macro2::TokenStream
{
    let fields_deserialize= fields.iter().map(|field| -> proc_macro2::TokenStream
        {
            let field_ident= &field.ident;
            quote! (
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
                    #(#fields_deserialize)*
                }
                _ => unimplemented!("Deserialize object hit a wrong value {:?}", schema_value),
            }
        }
    )
}


#[proc_macro_derive(Schematize, attributes(schema_default))]
pub fn derive_schematize_impl(item: proc_macro::TokenStream) -> proc_macro::TokenStream
{
    let item_ast: syn::DeriveInput= syn::parse_macro_input!(item);
    let item_ident= &item_ast.ident;

    let schema_impl= match item_ast.data
    {
        syn::Data::Struct(data_struct) =>
            match data_struct.fields
            {
                syn::Fields::Named(syn::FieldsNamed { named, .. }) =>
                {
                    let fields_schema_default_fn= derive_schema_default_fn(item_ident, &named);
                    let fields_serialize_fn= derive_serialize_fn(&named);
                    let fields_deserialize_fn= derive_deserialize_fn(&named);

                    quote! (
                        impl Schematize for #item_ident
                        {
                            #fields_schema_default_fn
                            #fields_serialize_fn
                            #fields_deserialize_fn
                        }
                    )
                },
                syn::Fields::Unnamed(_) => unimplemented!("Serialize is not implemented for unnamed fields struct, name: {}", item_ident),
                syn::Fields::Unit => unimplemented!("Serialize is not implemented for unit type, name: {}", item_ident),
            }
        syn::Data::Enum(_) => todo!("Serialize is not implemented for enum, name: {}", item_ident),
        syn::Data::Union(_) => unimplemented!("Serialize is not implemented for union, name: {}", item_ident),
    };

    //println!("{}", schema_impl);

    schema_impl.into()
}
