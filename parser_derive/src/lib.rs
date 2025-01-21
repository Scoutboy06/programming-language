extern crate proc_macro;

use proc_macro::TokenStream;
use quote::quote;
use syn::{parse_macro_input, DeriveInput};

#[proc_macro_derive(Expr)]
pub fn derive_expression(input: TokenStream) -> TokenStream {
    let input = parse_macro_input!(input as DeriveInput);
    impl_expression(&input)
}

#[proc_macro_derive(Stmt)]
pub fn derive_statement(input: TokenStream) -> TokenStream {
    let input = parse_macro_input!(input as DeriveInput);
    impl_statement(&input)
}

fn impl_expression(input: &DeriveInput) -> TokenStream {
    let name = &input.ident; // The name of the struct

    let expanded = quote! {
        impl From<#name> for Expression {
            fn from(value: #name) -> Expression {
                Expression::#name(Box::new(value))
            }
        }
    };

    TokenStream::from(expanded)
}

fn impl_statement(input: &DeriveInput) -> TokenStream {
    let name = &input.ident; // The name of the struct

    let expanded = quote! {
        impl From<#name> for Statement {
            fn from(value: #name) -> Statement {
                Statement::#name(Box::new(value))
            }
        }
    };

    TokenStream::from(expanded)
}
