extern crate proc_macro2;

use quote::quote;

type EnumVariants= syn::punctuated::Punctuated<syn::Variant, syn::token::Comma>;

pub fn derive_default_fn(
    enum_ident: &syn::Ident,
    enum_variants: &EnumVariants) -> proc_macro2::TokenStream {

    assert!(enum_variants.len() > 0, "Cannot schematize uninhabitable enum.");

    // TODO: Look for schema_default markup
    let default_variant= &enum_variants[0];

    let variant_construct: proc_macro2::TokenStream;
    if default_variant.fields.len() > 0 {
        let fields_default= default_variant.fields.iter().map(
            |field| -> proc_macro2::TokenStream {
                let field_type= &field.ty;

                return quote! {
                    #field_type::schema_default()
                }
            }
        );

        variant_construct= quote! { #default_variant(#(#fields_default),*) };

        todo!("Serialize, Deserialize, and Build Layout all have to be implemented to fully support enums w/ fields.");
    } else {
        variant_construct= quote! { #default_variant };
    }

    quote! {
        fn schema_default() -> #enum_ident {
            #enum_ident::#variant_construct
        }
    }
}

pub fn derive_serialize_fn(
    enum_ident: &syn::Ident,
    enum_variants: &EnumVariants) -> proc_macro2::TokenStream {

    // Generate map from enum value to string representing enum
    let variants_serialize= enum_variants.iter().map(
        |variant| -> proc_macro2::TokenStream {
            assert!(matches!(variant.fields, syn::Fields::Unit), "Only unit enum variants are supported");
            let variant_ident= &variant.ident;
            return quote! {
                #enum_ident::#variant_ident => context.print(stringify!(#variant_ident)),
            }
        });

    quote! {
        fn serialize(&self, context: &mut SerializeContext) {
            match self {
                #(#variants_serialize)*
            }
        }
    }
}

pub fn derive_deserialize_fn(
    enum_ident: &syn::Ident,
    enum_variants: &EnumVariants) -> proc_macro2::TokenStream {

    let variants_deserialize= enum_variants.iter().map(
        |variant| -> proc_macro2::TokenStream {
            assert!(matches!(variant.fields, syn::Fields::Unit), "Only unit enum variants are supported");
            let variant_ident= &variant.ident;
            return quote! {
                stringify!(#variant_ident) => #enum_ident::#variant_ident,
            }
        });



    quote! {
        fn deserialize(schema_value: &SchemaValue, context: &mut DeserializeContext) -> SchemaResult<#enum_ident> {
            Ok(match schema_value {
                SchemaValue::EnumVariant(field_name) =>
                    match *field_name{
                        #(#variants_deserialize)*
                        _ => {
                            println!("Deserialize hit an unexpected identifier for field '{}'. Expected: EnumVariant, found: {}.",
                                context.get_path(),
                                field_name);
                            println!("Could this be incorrectly spelled enum variant or removed from the new schema?");
                            return Err(SchemaError::UnknownIdentifier);
                        }
                    },
                _ => {
                    println!("Deserialize hit a wrong value for field '{}'. Expected: EnumVariant, found: {:?}",
                        context.get_path(),
                        schema_value);
                    return Err(SchemaError::WrongSchemaValue);
                }
            })
        }
    }
}
