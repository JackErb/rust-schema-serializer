extern crate proc_macro;
extern crate proc_macro2;

use quote::quote;

/* Serializer does the following things:
    - Serialize function to write the state of the data structure to JSON
        - Includes a hash of the serialized structure type for versioning reasons

    STRETCH:
    - Creates a default constructor respecting helper attributes
        - configurable default values for types
*/

fn derive_serialize_fn(fields: syn::punctuated::Iter<syn::Field>) -> proc_macro2::TokenStream
{
    let fields_serialize= fields.map(|field| -> proc_macro2::TokenStream
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


fn derive_deserialize_fn(fields: syn::punctuated::Iter<syn::Field>) -> proc_macro2::TokenStream
{
    let fields_deserialize= fields.map(|field| -> proc_macro2::TokenStream
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

fn derive_schema_default_fn(input_ident: &syn::Ident, fields: syn::punctuated::Iter<syn::Field>) -> proc_macro2::TokenStream
{
    let fields_default= fields.map(|field| -> proc_macro2::TokenStream
        {
            let field_ident= &field.ident;
            let field_type= &field.ty;
            quote! (
                #field_ident : #field_type::schema_default()
            )
        });

    quote! (
        fn schema_default() -> #input_ident
        {
            #input_ident { #(#fields_default),* }
        }
    )
}


#[proc_macro_derive(Schematize)]
pub fn derive_schematize_impl(input: proc_macro::TokenStream) -> proc_macro::TokenStream
{
    let ast: syn::DeriveInput= syn::parse_macro_input!(input);
    let input_ident= &ast.ident;

    let serializer_impl= match ast.data
    {
        syn::Data::Struct(data_struct) =>
            match data_struct.fields
            {
                syn::Fields::Named(syn::FieldsNamed { named, .. }) =>
                {
                    let fields_schema_default_fn= derive_schema_default_fn(input_ident, named.iter());
                    let fields_serialize_fn= derive_serialize_fn(named.iter());
                    let fields_deserialize_fn= derive_deserialize_fn(named.iter());

                    quote! (
                        impl Schematize for #input_ident
                        {
                            #fields_schema_default_fn
                            #fields_serialize_fn
                            #fields_deserialize_fn
                        }
                    )
                },
                syn::Fields::Unnamed(_) => unimplemented!("Serialize is not implemented for unnamed fields struct."),
                syn::Fields::Unit => unimplemented!("Serialize is not implemented for unit type."),
            }
        syn::Data::Enum(_) => todo!("Serialize is not implemented for enum, name: {}", input_ident),
        syn::Data::Union(_) => unimplemented!("Serialize is not implemented for union, name: {}", input_ident),
    };

    println!("{}", serializer_impl);

    serializer_impl.into()
}
