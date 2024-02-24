#![allow(unused)]
use proc_macro::TokenStream;
use proc_macro2::TokenTree;
use quote::{quote, ToTokens};
use syn::parse::{Parse, ParseStream, Parser};
use syn::punctuated::Punctuated;
use syn::token::Token;
use syn::Error;
use syn::{parse_macro_input, Expr, Ident, Result, Token, TypePath};

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
        } else {
            input.parse().map(Items::Expr)
        }
    }
}

fn color_str(input: &Expr, tag: &Ident) -> Result<proc_macro2::TokenStream> {
    let str_tag = tag.to_string();
    let mut it = str_tag.chars().peekable();
    let mut attr: Vec<&str> = vec![];
    let mut newline = "";

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
                        e => {
                            return Err(Error::new(
                                tag.span(),
                                format!("'F{e}' Invalid foreground option - '{e}'"),
                            ))
                        }
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
                        e => {
                            return Err(Error::new(
                                tag.span(),
                                format!("'B{e}' Invalid background option - '{e}'"),
                            ))
                        }
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
            _ => {
                return Err(Error::new(
                    tag.span(),
                    format!("Invalid format identifier '{m}'"),
                ))
            }
        }
    }

    let attrs = attr.join(";");

    Ok(quote! {
        format!(
            "{}\x1b[{}m{}\x1b[0m",
            #newline,
            #attrs,
            format!("{:?}", #input).replace('\"', "")
        )
    })
}

#[allow(clippy::let_and_return)]
#[proc_macro]
pub fn colorize(input: TokenStream) -> TokenStream {
    let args = parse_macro_input!(input with Punctuated::<Items, Token![,]>::parse_terminated);

    let mut res: Vec<proc_macro2::TokenStream> = vec![];

    for a in args.iter() {
        match a {
            Items::Item(item) => match color_str(&item.msg, &item.ident) {
                Ok(r) => res.push(r),
                Err(e) => return e.into_compile_error().into(),
            },
            Items::Expr(expr) => res.push(quote! { format!("{:?}", #expr).replace("\"", "") }),
        }
    }

    let res = quote! {
        [#(
            #res
        ),*].join(" ")
    }
    .into();

    res
}
