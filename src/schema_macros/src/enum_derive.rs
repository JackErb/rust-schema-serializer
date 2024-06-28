extern crate proc_macro2;

use quote::quote;

type EnumVariants= syn::punctuated::Punctuated<syn::Variant, syn::token::Comma>;

pub fn derive_default_fn(
    enum_ident: &syn::Ident,
    enum_variants: &EnumVariants) -> proc_macro2::TokenStream {

    assert!(enum_variants.len() > 0, "Cannot schematize uninhabitable enum.");

    // TODO: Look for schema_default markup
    let default_variant= &enum_variants[0];

    let variant_construct= if default_variant.fields.len() > 0 {
        let fields_default= default_variant.fields.iter().map(
            |field| -> proc_macro2::TokenStream {
                let field_type= &field.ty;

                return quote! {
                    #field_type::schema_default()
                }
            }
        );

       quote! { #default_variant(#(#fields_default),*) }
    } else {
        quote! { #default_variant }
    };

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

            let variant_ident= &variant.ident;

            return match &variant.fields {
                syn::Fields::Unit => quote! {
                    #enum_ident::#variant_ident => context.print(stringify!(#variant_ident)),
                },
                syn::Fields::Unnamed(fields) => {
                    assert!(fields.unnamed.len() == 1, "Can have max of one field in an enum variant.");

                    return quote! {
                        #enum_ident::#variant_ident(field) => {
                            context.print(stringify!(#variant_ident));
                            context.print(" {");

                            context.tabs+= 1;
                            context.println();
                            context.print_tabs();
                            field.serialize(context);
                            context.println();
                            context.tabs-= 1;

                            context.print_tabs();
                            context.print("}");
                        },
                    }
                },
                syn::Fields::Named(_) => panic!("Named fields in enums are not supported.")
            };
        });

    quote! {
        fn serialize(&self, context: &mut SerializeContext) {
            match self {
                #(#variants_serialize)*
            }
        }
    }
}

pub fn derive_build_layout_fn(enum_variants: &EnumVariants) -> proc_macro2::TokenStream {

    // match Self
    //    case Primary => Ok(layout)
    //    case Tertiary(field) => Ok(i32::build_layout(field, layout, offsets)

    let variants_build_layout= enum_variants.iter().map(
        |variant| -> proc_macro2::TokenStream {
            let variant_ident= &variant.ident;

            let build_layout_variant= match &variant.fields {
                syn::Fields::Unit => quote! { Ok(layout) },
                syn::Fields::Unnamed(fields) => {
                    assert!(fields.unnamed.len() == 1, "Can have max of one field in an enum variant.");

                    let field_type= &fields.unnamed[0].ty;

                    quote! {
                        Ok(#field_type::build_layout(enum_field, layout, offsets)?)
                    }
                },
                syn::Fields::Named(_) => panic!("Named fields in enums are not supported."),
            };

            return quote! {
                stringify!(#variant_ident) => #build_layout_variant,
            }
        }
    );

    quote! {
        fn build_layout(schema_value: &SchemaValue, layout: alloc::Layout, offsets: &mut Vec<usize>)
            -> Result<alloc::Layout, alloc::LayoutError> {
            match schema_value {
                SchemaValue::EnumVariant(enum_name, enum_field) => {
                    match *enum_name {
                        #(#variants_build_layout)*
                        _ => {
                            // wrong value, no-op
                            Ok(layout)
                        }
                    }
                },
                _ => {
                    // wrong value, no-op
                    Ok(layout)
                }
            }
        }
    }
}

pub fn derive_deserialize_fn(
    enum_ident: &syn::Ident,
    enum_variants: &EnumVariants) -> proc_macro2::TokenStream {

    let variants_deserialize= enum_variants.iter().map(
        |variant| -> proc_macro2::TokenStream {
            let variant_ident= &variant.ident;

            let deserialize_variant= match &variant.fields {
                syn::Fields::Unit => quote! {
                    #enum_ident::#variant_ident
                },
                syn::Fields::Unnamed(fields) => {
                    assert!(fields.unnamed.len() == 1, "Can have max of one field in an enum variant.");

                    let field_type= &fields.unnamed[0].ty;

                    quote! {
                        #enum_ident::#variant_ident(#field_type::deserialize(enum_field, context)?)
                    }
                },
                syn::Fields::Named(_) => panic!("Named fields in enums are not supported.")
            };

            return quote! {
                stringify!(#variant_ident) => #deserialize_variant,
            }
        });



    quote! {
        fn deserialize(schema_value: &SchemaValue, context: &mut DeserializeContext) -> SchemaResult<#enum_ident> {
            Ok(match schema_value {
                SchemaValue::EnumVariant(enum_name, enum_field) =>
                    match *enum_name {
                        #(#variants_deserialize)*
                        _ => {
                            println!("Deserialize hit an unexpected identifier for field '{}'. Expected: EnumVariant, found: {}.",
                                context.get_path(),
                                enum_name);
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
