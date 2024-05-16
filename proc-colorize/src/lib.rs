use proc_macro::TokenStream;
use syn::{
    parse::{Parse, ParseStream},
    parse_macro_input,
    punctuated::Punctuated,
    Error, Expr, Ident, LitStr, Result, Token,
};

mod colors;

#[allow(dead_code)]
#[derive(Debug)]
struct WithFormatString {
    fstring: LitStr,
    sep: Token![,],
    rest: TokenStream,
}

#[allow(dead_code)]
#[derive(Debug)]
struct ColorizeAll {
    ident: Ident,
    tok: Token![=>],
    rest: TokenStream,
}

#[allow(dead_code)]
#[derive(Debug)]
pub(crate) struct ColorizeItem {
    pub ident: Ident,
    pub sep: Token![->],
    pub msg: Expr,
}

#[derive(Debug)]
pub(crate) enum Args {
    Item(ColorizeItem),
    Expr(Expr),
}

impl Parse for WithFormatString {
    fn parse(input: ParseStream) -> Result<Self> {
        Ok(Self {
            fstring: input.parse()?,
            sep: input.parse()?,
            rest: input.parse::<proc_macro2::TokenStream>()?.into(),
        })
    }
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

#[proc_macro]
/// Adds ANSI color escape sequences to inputs
///
/// ## Usage
///
/// `colorize!` takes a series of inputs, with or without tokens, and converts the inputs into a `String` with ANSI escape sequences added in.
///
/// The returned `String` is primarily useful for printing out to a terminal which is capable of showing color.
/// However, if all you want to do is print, and want to cut out the extra code, use `print_color` instead.
///
/// ## Valid inputs
/// `colorize!` uses the same formatting style as [`format!`](std::format!) so it follows the same
/// argument rules.
///
/// ```
/// # use colorize_proc_macro as colorize;
/// use std::path::PathBuf;
/// use colorize::colorize;
///
/// let user_path = PathBuf::from("/home/color/my_new_file.txt");
/// let pretty_path = colorize!("{:?}", Fgu->user_path.clone());
///
/// assert_eq!(
///     String::from("\x1b[32;4m\"/home/color/my_new_file.txt\"\x1b[0m"),
///     pretty_path
/// );
/// ```
///
/// ## Tokens
/// Tokens can change color or font styling depending on their order and usage.
///
/// #### Styling
/// 1. b -> bold
/// 2. u -> underline
/// 3. i -> italic
///
/// #### Color
/// Color tokens start with an `F` (for foreground) or `B` (for background)
///
/// 1. Fb/Bb -> blue
/// 2. Fr/Br -> red
/// 3. Fg/Bg -> green
/// 4. Fy/By -> yellow
/// 5. Fm/By -> magenta
/// 6. Fc/Bc -> cyan
/// 7. Fw/Bw -> white
/// 8. Fk/Bk -> black
///
/// #### Special Newline Token
/// If you want to add a newline  within the string, include a `N` token at the start
/// of the word(s) you wish to be on the newline. This is the same as just adding '\n' to the
/// string, so it's up to you to use it or not.
///
///
/// Example -
/// ```
/// # use colorize_proc_macro as colorize;
/// use colorize::colorize;
///
/// let color_string = colorize!(
///     "{} {}",
///     b->"Hello", // First line
///     Nb->"world, it's me!" // "world..." will be on the new line
/// );
///
/// let same_color_string = colorize!(
///    "{} \n{}",
///    b->"Hello",
///    b->"world, it's me!"
/// );
///
/// assert_eq!(color_string, same_color_string);
/// ```
///
/// #### Format Multiple Inputs
/// You also have the ability to apply a token to multiple inputs by using `=>` at the beginning of the call.
///
/// ```
/// # use colorize_proc_macro as colorize;
/// use colorize::colorize;
///
/// let color_string = colorize!("{} {}", b => Fg->"Hello", By->"world");
/// ```
/// In the above example, "Hello" will have a green foreground, and "world" will have a yellow background. The preceeding `b =>` applies bold formatting to both.
///
/// ### Examples
/// ```
/// # use colorize_proc_macro as colorize;
/// use colorize::colorize;
///
/// // Returns "Hello" in bold green
/// let color_string = colorize!("{}", Fgb->"Hello");
/// assert_eq!(String::from("\x1b[32;1mHello\x1b[0m"), color_string);
///
/// // Returns "Hello" in italic blue and "World" underlined in magenta
/// // ", it's me" will be unformatted
/// let color_string = colorize!("{} {} {}", iFb->"Hello", Fmu->"world", ", it's me!");
/// assert_eq!(String::from("\x1b[3;34mHello\x1b[0m \x1b[35;4mworld\x1b[0m , it's me!"), color_string);
/// ```
pub fn colorize(input: TokenStream) -> TokenStream {
    let inp = input.clone();

    let (fstring, inp) = match syn::parse::<WithFormatString>(inp) {
        Ok(r) => (r.fstring, r.rest),
        Err(e) => return e.into_compile_error().into(),
    };

    let (args, id) = match syn::parse::<ColorizeAll>(inp.clone()) {
        Ok(r) => {
            if let Err(e) = valid_color_all(&r.ident) {
                e.into_compile_error();
            }
            let rem = r.rest;
            let a = parse_macro_input!(rem with Punctuated::<Args, Token![,]>::parse_terminated);

            (a, Some(r.ident))
        }
        Err(_) => {
            let a = parse_macro_input!(inp with Punctuated::<Args, Token![,]>::parse_terminated);
            (a, None)
        }
    };

    let res = match crate::colors::parse_fstring(&fstring, args, id) {
        Ok(r) => r,
        Err(e) => return e.into_compile_error().into(),
    };

    res.into()
}
