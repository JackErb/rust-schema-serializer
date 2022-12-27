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
    let input_ident= &ast.ident;

    let serializer_impl= match ast.data
    {
        syn::Data::Struct(data_struct) =>
            match data_struct.fields
            {
                syn::Fields::Named(syn::FieldsNamed { named, .. }) =>
                {
                    let fields_serialize_impl= named.iter().map(|field| -> proc_macro2::TokenStream
                        {
                            let field_ident= &field.ident;
                            quote! (
                                fields_map.insert(stringify!(#field_ident), self.#field_ident.serialize());
                            )
                        });

                    quote! (
                        impl SchemaSerializer for #input_ident
                        {
                            fn serialize(&self) -> Box<SchemaValue>
                            {
                                let mut fields_map= std::collections::HashMap::<&'static str, Box::<SchemaValue>>::new();
                                #(#fields_serialize_impl)*

                                Box::new(SchemaValue::Object(fields_map))
                            }
                        }
                    )
                },
                syn::Fields::Unnamed(_) => unimplemented!("Serialize is not implemented for Unnamed fields."),
                syn::Fields::Unit => unimplemented!("Serialize is not implemented for unit type."),
            }
        syn::Data::Enum(_) => todo!("Serialize is not implemented for enum, name: {}", input_ident),
        syn::Data::Union(_) => unimplemented!("Serialize is not implemented for union, name: {}", input_ident),
    };

    //println!("{}", serializer_impl);

    serializer_impl.into()
}
