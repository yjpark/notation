use fehler::throws;
use proc_macro2::TokenStream;
use quote::{quote, ToTokens};
use syn::parse::{Error, Parse, ParseStream, Result};
use syn::{braced, bracketed, parenthesized, token};

use crate::bar::BarDsl;
use crate::entry::{EntryDsl, MultibleDsl};
use crate::fretted::pick::PickDsl;
use crate::fretted::shape::ShapeDsl;
use crate::layer::LayerDsl;
use crate::line::LineDsl;
use crate::section::SectionDsl;
use crate::slice::SliceDsl;
use crate::track::TrackDsl;

macro_rules! impl_dsl {
    ($dsl_type:ident) => {
        impl Parse for $dsl_type {
            fn parse(input: ParseStream) -> Result<Self> {
                if input.peek(token::Brace) {
                    let content;
                    braced!(content in input);
                    Self::parse_without_brace(&content)
                } else {
                    Self::parse_without_brace(input)
                }
            }
        }
        impl $dsl_type {
            #[throws(Error)]
            pub fn parse_vec(input: ParseStream) -> Vec<$dsl_type> {
                let mut result = vec![];
                if input.peek(token::Bracket) {
                    let content;
                    bracketed!(content in *input);
                    while !content.is_empty() {
                        result.push(content.parse()?);
                    }
                }
                result
            }
            pub fn quote_vec(v: &Vec<$dsl_type>) -> TokenStream {
                let items: Vec<TokenStream> = v.iter().map(
                        |x| quote! { #x }
                ).collect();
                quote! {
                    vec![
                        #(#items),*
                    ]
                }
            }
        }
    }
}

impl_dsl!(EntryDsl);
impl_dsl!(LineDsl);
impl_dsl!(SliceDsl);
impl_dsl!(TrackDsl);
impl_dsl!(LayerDsl);
impl_dsl!(BarDsl);
impl_dsl!(SectionDsl);

macro_rules! impl_multible_dsl {
    ($dsl_type:ident) => {
        impl Parse for $dsl_type {
            fn parse(input: ParseStream) -> Result<Self> {
                Self::parse_multible(input, false)
            }
        }
        impl $dsl_type {
            fn parse_multible(input: ParseStream, multied: bool) -> Result<Self> {
                if input.peek(token::Paren) {
                    let content;
                    parenthesized!(content in input);
                    Self::parse_without_paren(&content, multied, true)
                } else {
                    Self::parse_without_paren(input, multied, false)
                }
            }
            #[throws(Error)]
            pub fn parse_vec(input: ParseStream) -> Vec<$dsl_type> {
                let mut result = vec![];
                if input.peek(token::Bracket) {
                    let content;
                    bracketed!(content in *input);
                    while !content.is_empty() {
                        result.push(Self::parse_multible(&content, true)?);
                    }
                }
                result
            }
        }
        impl Parse for MultibleDsl<$dsl_type> {
            fn parse(input: ParseStream) -> Result<Self> {
                let items =
                    if input.peek(token::Bracket) {
                        $dsl_type::parse_vec(input)?
                    } else {
                        vec![
                            $dsl_type::parse(input)?
                        ]
                    };
                Ok(Self { items } )
            }
        }
        impl ToTokens for MultibleDsl<$dsl_type> {
            fn to_tokens(&self, tokens: &mut TokenStream) {
                let items = &self.items;
                tokens.extend(quote! {
                    #(#items),*
                });
            }
        }
    }
}

impl_multible_dsl!(PickDsl);
impl_multible_dsl!(ShapeDsl);
