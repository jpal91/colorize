#![allow(unused)]
use proc_macro::TokenStream;
use proc_macro2::TokenTree;
use quote::{quote, ToTokens};
use syn::parse::{Parse, ParseStream, Parser};
use syn::punctuated::Punctuated;
use syn::token::Token;
use syn::Error;
use syn::{parse_macro_input, Expr, Ident, Result, Token, TypePath};

type TT<'a> = &'a (dyn quote::ToTokens);

#[derive(Debug)]
struct Item {
    ident: Ident,
    sep: Token![->],
    msg: Expr,
}

#[derive(Debug)]
enum Items {
    Item(Item),
    Expr(Expr),
    Ident(Ident),
    Path(TypePath),
    Any(proc_macro2::TokenStream),
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

impl Parse for Items {
    fn parse(input: ParseStream) -> Result<Self> {
        if input.peek(Ident) && input.peek2(Token![->]) {
            input.parse().map(Items::Item)
        // } else if input.peek(Ident) {
        //     input.parse().map(Items::Ident)
        // } else if input.peek(syn::Stmt::Expr) {
        } else {
            // input.parse().map(Items::Expr)
            input.parse().map(Items::Any)
        }
    }
}

fn color_str(input: Expr, tag: Ident) -> Result<proc_macro2::TokenStream> {
    let str_tag = tag.to_string();
    let mut it = str_tag.chars().peekable();
    let mut attr: Vec<&str> = vec![];
    let mut newline = "";
    // let input = format!("{:?}", input).replace('\"', "");

    while let Some(m) = it.next() {
        match m {
            'F' => {
                if let Some(n) = it.peek() {
                    let col = match n {
                        'k' => "30",
                        'r' => "31",
                        'g' => "32",
                        'y' => "33",
                        'b' => "34",
                        'm' => "35",
                        'c' => "36",
                        'w' => "37",
                        _ => return Err(Error::new(tag.span(), "Invalid format")),
                    };
                    if !col.is_empty() {
                        it.next();
                        attr.push(col)
                    }
                }
            }
            'B' => {
                if let Some(n) = it.peek() {
                    let col = match n {
                        'k' => "40",
                        'r' => "41",
                        'g' => "42",
                        'y' => "43",
                        'b' => "44",
                        'm' => "45",
                        'c' => "46",
                        'w' => "47",
                        _ => return Err(Error::new(tag.span(), "Invalid format")),
                    };
                    if !col.is_empty() {
                        it.next();
                        attr.push(col)
                    }
                }
            }
            'b' => attr.push("1"),
            'i' => attr.push("3"),
            'u' => attr.push("4"),
            'N' => newline = "\n",
            _ => {}
        }
    }

    let attrs = attr.join(";");

    Ok(
        quote! {format!("{}\x1b[{}m{}\x1b[0m", #newline, #attrs, format!("{:?}", #input).replace('\"', ""))},
    )
}

#[proc_macro]
pub fn colorize(input: TokenStream) -> TokenStream {
    // println!("{:?}", input);
    // let inp = input.clone();
    // // let args = Punctuated::<Expr, Token![,]>::parse_terminated.parse(inp).unwrap();
    // let args = parse_macro_input!(inp as syn::Macro);
    // println!("{:?}", args);

    let parser = Punctuated::<Expr, Token![,]>::parse_terminated;
    let args = match parser.parse(input) {
        Ok(e) => e,
        Err(e) => return quote! {""}.into(), //return e.into_compile_error().into()
    };

    let mut res: Vec<proc_macro2::TokenStream> = vec![];
    println!("here {:?}", args);
    // for a in args.iter() {
    //     match a {
    //         Items::Item(item) => {
    //             match color_str(item.msg.clone(), item.ident.clone()) {
    //                 Ok(r) => res.push(r),
    //                 Err(e) => return e.into_compile_error().into()
    //             }
    //         },
    //         Items::Any(item) => {
    //             res.push(quote!{ format!("{:?}", #item)})
    //         },
    //         Items::Ident(item) => {
    //             res.push(item.to_token_stream())
    //         }
    //         _ => {}
    //     }
    // }

    println!("{:?}", res);
    // let some = format!("thing", "{}")
    // let res = quote!{
    //     [#(
    //         #res
    //     ),*].join(" ")
    // }.into();
    // println!("{:?}", res);
    // res
    // args.to_token_stream().into()
    // TokenStream::new()
    quote! {
        "ehllo"
    }
    .into()
}
