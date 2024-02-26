use proc_macro::TokenStream;
use quote::{format_ident, quote};
use syn::{
    parse::{Parse, ParseStream},
    parse_macro_input,
    punctuated::Punctuated,
    Error, Expr, Ident, Result, Token,
};

#[allow(dead_code)]
#[derive(Debug)]
struct ColorizeAll {
    ident: Ident,
    tok: Token![=>],
    rest: TokenStream,
}

#[allow(dead_code)]
#[derive(Debug)]
struct ColorizeItem {
    ident: Ident,
    sep: Token![->],
    msg: Expr,
}

#[derive(Debug)]
enum Args {
    Item(ColorizeItem),
    Expr(Expr),
}

impl Parse for ColorizeAll {
    fn parse(input: ParseStream) -> Result<Self> {
        Ok(Self {
            ident: input.parse()?,
            tok: input.parse()?,
            rest: input.parse::<proc_macro2::TokenStream>()?.into(),
        })
    }
}

impl Parse for ColorizeItem {
    fn parse(input: ParseStream) -> Result<Self> {
        Ok(Self {
            ident: input.parse()?,
            sep: input.parse()?,
            msg: input.parse()?,
        })
    }
}

impl Parse for Args {
    fn parse(input: ParseStream) -> Result<Self> {
        if input.peek(Ident) && input.peek2(Token![->]) {
            input.parse().map(Args::Item)
        } else {
            input.parse().map(Args::Expr)
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
        ::std::format!(
            "{}\x1b[{}m{}\x1b[0m",
            #newline,
            #attrs,
            ::std::format!("{:?}", #input).replace('\"', "")
        )
    })
}

fn valid_color_all(tag: &Ident) -> Result<()> {
    let str_tag = tag.to_string();
    let mut it = str_tag.chars().peekable();

    while let Some(t) = it.next() {
        match t {
            'b' | 'i' | 'u' | 'N' => continue,
            'F' => {
                if let Some(n) = it.peek() {
                    match n {
                        'k' | 'r' | 'g' | 'y' | 'b' | 'm' | 'c' | 'w' => continue,
                        e => {
                            return Err(Error::new(
                                tag.span(),
                                format!("'F{e}' Invalid foreground option - '{e}'"),
                            ))
                        }
                    }
                } else {
                    return Err(Error::new(
                        tag.span(),
                        "Forground option must be followed by a valid identifier",
                    ));
                }
            }
            'B' => {
                if let Some(n) = it.peek() {
                    match n {
                        'k' | 'r' | 'g' | 'y' | 'b' | 'm' | 'c' | 'w' => continue,
                        e => {
                            return Err(Error::new(
                                tag.span(),
                                format!("'B{e}' Invalid background option - '{e}'"),
                            ))
                        }
                    }
                } else {
                    return Err(Error::new(
                        tag.span(),
                        "Background option must be followed by a valid identifier",
                    ));
                }
            }
            _ => {
                return Err(Error::new(
                    tag.span(),
                    format!("Invalid format identifier '{t}'"),
                ))
            }
        }
    }
    Ok(())
}

#[allow(clippy::let_and_return)]
#[proc_macro]
pub fn colorize(input: TokenStream) -> TokenStream {
    let inp = input.clone();

    let (args, id) = match syn::parse::<ColorizeAll>(inp) {
        Ok(r) => {
            if let Err(e) = valid_color_all(&r.ident) {
                e.into_compile_error();
            }
            let rem = r.rest;
            let a = parse_macro_input!(rem with Punctuated::<Args, Token![,]>::parse_terminated);

            (a, Some(r.ident))
        }
        Err(_) => {
            let a = parse_macro_input!(input with Punctuated::<Args, Token![,]>::parse_terminated);
            (a, None)
        }
    };

    let mut res: Vec<proc_macro2::TokenStream> = vec![];

    for a in args.iter() {
        match a {
            Args::Item(item) => {
                let ident = if let Some(ref i) = id {
                    let new_id = format_ident!("{}{}", i, &item.ident, span = item.ident.span());
                    new_id
                } else {
                    item.ident.clone()
                };

                match color_str(&item.msg, &ident) {
                    Ok(r) => res.push(r),
                    Err(e) => return e.into_compile_error().into(),
                }
            }
            Args::Expr(expr) => {
                if let Some(ref i) = id {
                    match color_str(expr, i) {
                        Ok(r) => res.push(r),
                        Err(e) => return e.into_compile_error().into(),
                    }
                } else {
                    res.push(quote! { ::std::format!("{:?}", #expr).replace("\"", "") })
                }
            }
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
