#![allow(unused)]
use proc_macro::TokenStream;
use syn::parse::{Parse, ParseStream, Parser};
use syn::punctuated::Punctuated;
use syn::{Expr, Ident, Result, Token};

#[derive(Debug)]
struct Item {
    ident: Ident,
    sep: Token![->],
    msg: Expr,
}

impl Parse for Item {
    fn parse(input: ParseStream) -> Result<Self> {
        Ok(Self {
            ident: input.parse()?,
            sep: input.parse()?,
            msg: input.parse()?,
        })
    }
}

#[proc_macro]
pub fn colorize(input: TokenStream) -> TokenStream {
    let parser = Punctuated::<Item, Token![,]>::parse_terminated;
    let args = parser.parse(input).unwrap();
    println!("{:?}", args);
    TokenStream::new()
}
