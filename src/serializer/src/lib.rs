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

#[proc_macro_derive(SchemaSerializer)]
pub fn derive_serializer_fn(input: proc_macro::TokenStream) -> proc_macro::TokenStream
{
    let ast: syn::DeriveInput= syn::parse_macro_input!(input);
    let ident= &ast.ident;

    let debug_print_impl= match ast.data
    {
        syn::Data::Struct(data_struct) =>
            match data_struct.fields
            {
                syn::Fields::Named(syn::FieldsNamed { named, .. }) =>
                {
                    let print_statements= named.iter().map(|field| -> proc_macro2::TokenStream
                        {
                            let field_ident= &field.ident;
                            let field_type= &field.ty;
                            quote! (
                                println!("{} : {} = {}",
                                    stringify!(#field_ident),
                                    stringify!(#field_type),
                                    self.#field_ident);
                            )
                        });

                    let quote! (
                        impl Serializer for #ident
                        {
                            fn debug_print(&self)
                            {
                                #(#print_statements)*
                            }
                        }
                    )
                },
                syn::Fields::Unnamed(_) => unimplemented!("Serialize is not implemented for Unnamed fields."),
                syn::Fields::Unit => unimplemented!("Serialize is not implemented for unit type."),
            }
        syn::Data::Enum(_) => todo!("Serialize is not implemented for enum, name: {}", ident),
        syn::Data::Union(_) => unimplemented!("Serialize is not implemented for union, name: {}", ident),
    };

    serializer_impl.into()
}

/*
#[proc_macro_derive(SchemaDeserializer)]
pub fn derive_deserializer_fn(input: proc_macro::TokenStream) -> proc_macro::TokenStream
{
    let ast= syn::parse_macro_input!(input);
    let ident= &ast.ident;

    let deserializer_impl= match ast.data
    {
        syn::Data::Struct(data_struct) =>
            match data_struct.fields
            {
                syn::Fields::Named(syn::FieldsNamed { named, .. }) =>
                {
                    let print_statements= named.iter().map(|field| -> proc_macro2::TokenStream
                        {
                            let field_ident= &field.ident;
                            let field_type= &field.ty;
                            quote! (
                                println!("{} : {} = {}",
                                    stringify!(#field_ident),
                                    stringify!(#field_type),
                                    self.#field_ident);
                            )
                        });

                    quote! (
                        impl Serializer for #ident
                        {
                            fn debug_print(&self)
                            {
                                #(#print_statements)*
                            }
                        }
                    )
                },
                syn::Fields::Unnamed(_) => unimplemented!("Deserialize is not implemented for Unnamed fields."),
                syn::Fields::Unit => unimplemented!("Deserialize is not implemented for unit type."),
            }
        syn::Data::Enum(_) => todo!("Deserialize is not implemented for enum, name: {}", ident),
        syn::Data::Union(_) => unimplemented!("Deserialize is not implemented for union, name: {}", ident),
    }
}
*/
