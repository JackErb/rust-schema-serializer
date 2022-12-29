extern crate proc_macro2;

use quote::quote;

type EnumVariants= syn::punctuated::Punctuated<syn::Variant, syn::token::Comma>;

pub fn derive_default_fn(
    enum_ident: &syn::Ident,
    enum_variants: &EnumVariants) -> proc_macro2::TokenStream
{
    assert!(enum_variants.len() > 0, "Cannot schematize uninhabitable enum.");

    // TODO: Look for schema_default markup
    let first_variant= &enum_variants[0].ident;
    quote! {
        fn schema_default() -> #enum_ident {
            #enum_ident::#first_variant
        }
    }
}

pub fn derive_serialize_fn(
    enum_ident: &syn::Ident,
    enum_variants: &EnumVariants) -> proc_macro2::TokenStream
{
    // Generate map from enum value to string representing enum
    let variants_serialize= enum_variants.iter().map(
        |variant| -> proc_macro2::TokenStream
        {
            assert!(matches!(variant.fields, syn::Fields::Unit), "Only unit enum variants are supported");
            let variant_ident= &variant.ident;
            quote! (
                #enum_ident::#variant_ident => stringify!(#variant_ident),
            )
        });

    quote! {
        fn serialize(&self) -> SchemaValue {
            SchemaValue::EnumVariant(match self {
                #(#variants_serialize)*
            })
        }
    }
}

pub fn derive_deserialize_fn(
    enum_ident: &syn::Ident,
    enum_variants: &EnumVariants) -> proc_macro2::TokenStream
{
    let variants_deserialize= enum_variants.iter().map(
        |variant| -> proc_macro2::TokenStream
        {
            assert!(matches!(variant.fields, syn::Fields::Unit), "Only unit enum variants are supported");
            let variant_ident= &variant.ident;
            quote! (
                stringify!(#variant_ident) => #enum_ident::#variant_ident,
            )
        });

    quote! {
        fn deserialize(&mut self, schema_value: &SchemaValue) {
            *self= match schema_value {
                SchemaValue::EnumVariant(field_name) =>
                    match *field_name{
                        #(#variants_deserialize)*
                        _ => unimplemented!("Deserialize enum hit an unrecognized variant {}", field_name)
                    },
                _ => unimplemented!("Deserialize enum hit a wrong value {:?}", schema_value)
            };
        }
    }
}
