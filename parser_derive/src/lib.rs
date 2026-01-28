extern crate proc_macro;

use proc_macro::TokenStream;
use quote::quote;
use syn::{
    parse_macro_input, AngleBracketedGenericArguments, Data, DeriveInput, Fields, GenericArgument,
    PathArguments, Type, TypePath,
};

/// Proc macro to derive From<T> implementations for each variant of an enum.
#[proc_macro_derive(FromVariants)]
pub fn derive_from_variants(input: TokenStream) -> TokenStream {
    let input = parse_macro_input!(input as DeriveInput);
    let enum_name = input.ident;

    let Data::Enum(data_enum) = input.data else {
        return syn::Error::new_spanned(
            enum_name,
            "#[derive(FromVariants)] can only be used on enums",
        )
        .to_compile_error()
        .into();
    };

    let impls = data_enum.variants.iter().filter_map(|variant| {
        let variant_name = &variant.ident;

        // Only handle tuple variants like Foo(Bar)
        match &variant.fields {
            Fields::Unnamed(fields) if fields.unnamed.len() == 1 => {
                let ty = &fields.unnamed.first().unwrap().ty;

                // Check if the type is a Box<T>
                if let Type::Path(TypePath { path, .. }) = ty {
                    if let Some(segment) = path.segments.last() {
                        let ident = &segment.ident;
                        if ident == "Box"
                            && matches!(&segment.arguments, PathArguments::AngleBracketed(_))
                        {
                            if let PathArguments::AngleBracketed(AngleBracketedGenericArguments {
                                args,
                                ..
                            }) = &segment.arguments
                            {
                                if let Some(GenericArgument::Type(inner_ty)) = args.first() {
                                    return Some(quote! {
                                        impl std::convert::From<#inner_ty> for #enum_name {
                                            fn from(value: #inner_ty) -> Self {
                                                #enum_name::#variant_name(#ident::new(value))
                                            }
                                        }
                                    });
                                }
                            }
                        }
                    }
                }

                Some(quote! {
                    impl std::convert::From<#ty> for #enum_name {
                        fn from(value: #ty) -> Self {
                            #enum_name::#variant_name(value)
                        }
                    }
                })
            }
            _ => None,
        }
    });

    TokenStream::from(quote! {
        #(#impls)*
    })
}

// #[proc_macro_derive(Expr)]
// pub fn derive_expression(input: TokenStream) -> TokenStream {
//     let input = parse_macro_input!(input as DeriveInput);
//     impl_expression(&input)
// }
//
// #[proc_macro_derive(Stmt)]
// pub fn derive_statement(input: TokenStream) -> TokenStream {
//     let input = parse_macro_input!(input as DeriveInput);
//     impl_statement(&input)
// }
//
// fn impl_expression(input: &DeriveInput) -> TokenStream {
//     let name = &input.ident; // The name of the struct
//
//     let expanded = quote! {
//         impl From<#name> for Expression {
//             fn from(value: #name) -> Expression {
//                 Expression::#name(Box::new(value))
//             }
//         }
//     };
//
//     TokenStream::from(expanded)
// }
//
// fn impl_statement(input: &DeriveInput) -> TokenStream {
//     let name = &input.ident; // The name of the struct
//
//     let expanded = quote! {
//         impl From<#name> for Statement {
//             fn from(value: #name) -> Statement {
//                 Statement::#name(Box::new(value))
//             }
//         }
//     };
//
//     TokenStream::from(expanded)
// }
