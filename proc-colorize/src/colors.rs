use quote::{format_ident, quote};
use syn::{punctuated::Punctuated, token::Comma, Error, Ident, LitStr, Result};

use crate::fstring::{placeholders, ArgRef};
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
    let value = fstring.value();
    let placeholders = placeholders(&value).map_err(|msg| Error::new(fstring.span(), msg))?;
    let args: Vec<Args> = args.into_iter().collect();

    let mut out = String::with_capacity(value.len());
    let mut last = 0;

    for placeholder in placeholders {
        out.push_str(&value[last..placeholder.range.start]);

        // Style follows the argument the placeholder refers to, plus any `tag =>` prefix
        let own = match placeholder.arg {
            ArgRef::Index(n) => match args.get(n) {
                Some(Args::Item(item)) => Some(&item.ident),
                _ => None,
            },
            ArgRef::Name(_) => None,
        };
        let tag = match (&id, own) {
            (Some(all), Some(own)) => Some(format_ident!("{}{}", all, own, span = own.span())),
            (Some(all), None) => Some(all.clone()),
            (None, own) => own.cloned(),
        };

        let text = &value[placeholder.range.clone()];
        match tag {
            Some(tag) => {
                out.push_str(&color_str(&tag)?);
                out.push_str(text);
                out.push_str("\x1b[0m");
            }
            None => out.push_str(text),
        }
        last = placeholder.range.end;
    }
    out.push_str(&value[last..]);

    let fstring_args = args.into_iter().map(|arg| match arg {
        Args::Item(item) => item.msg,
        Args::Expr(ex) => ex,
    });

    Ok(quote! {
        ::std::format!(
            #out,
            #(
                #fstring_args
            ),*
        )
    })
}
