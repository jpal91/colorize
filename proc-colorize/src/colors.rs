use quote::{format_ident, quote};
use syn::{punctuated::Punctuated, token::Comma, Error, Expr, Ident, LitStr, Result};

use crate::Args;

fn color_str(tag: &Ident) -> Result<String> {
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

    Ok(format!("{}\x1b[{}m", newline, attrs))
}

pub fn parse_fstring(
    fstring: &LitStr,
    args: Punctuated<Args, Comma>,
    id: Option<Ident>,
) -> Result<proc_macro2::TokenStream> {
    let mut chars: Vec<u8> = vec![];
    let mut fstring_args: Vec<Expr> = vec![];
    let mut open = false;
    let mut args = args.into_iter();
    let mut add_closer = false;

    for ch in fstring.value().as_bytes().iter() {
        match ch {
            b'{' => {
                if open {
                    return Err(Error::new(
                        fstring.span(),
                        "One or more of your '{}' args are not closed",
                    ));
                }

                open = true;

                match &args.next() {
                    Some(Args::Item(item)) => {
                        let ident = if let Some(ref i) = id {
                            format_ident!("{}{}", i, &item.ident, span = item.ident.span())
                        } else {
                            item.ident.clone()
                        };
                        let prefix = color_str(&ident);
                        chars.extend(prefix?.as_bytes());
                        fstring_args.push(item.msg.to_owned());
                        add_closer = true;
                    }
                    Some(Args::Expr(ex)) => {
                        if let Some(ref i) = id {
                            let prefix = color_str(i);
                            chars.extend(prefix?.as_bytes());
                            add_closer = true;
                        }
                        fstring_args.push(ex.to_owned());
                    }
                    None => {}
                };
                chars.push(ch.to_owned());
            }
            b'}' => {
                if !open {
                    return Err(Error::new(
                        fstring.span(),
                        "One or more of your '{}' args are not closed",
                    ));
                }
                open = false;
                chars.push(ch.to_owned());

                if add_closer {
                    chars.extend(b"\x1b[0m");
                    add_closer = false;
                }
            }
            any => chars.push(any.to_owned()),
        }
    }

    let fstring = String::from_utf8(chars).unwrap();

    Ok(quote! {
        ::std::format!(
            #fstring,
            #(
                #fstring_args
            ),*
        )
    })
}
