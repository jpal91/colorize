//! A set of Rust macros to assist in turning text into colors for printing on the terminal.
//!
//! ```
//! use colorize::{colorize, print_color};
//!
//! // Convert text into a String with colors
//! // Returns "Hello" with a red background and green foreground color, combined with "world" in bold, separated by a space
//! let color_string = colorize!(BrFg->"Hello", b->"world");
//! assert_eq!(String::from("\x1b[41;32mHello\x1b[0m \x1b[1mworld\x1b[0m"), color_string);
//!
//! // Add a format token to multiple inputs using `=>`
//! // The below example will produce "Hello" with a green foreground, "world" with a blue background, both in bold.
//! let color_string = colorize!(b => Fg->"Hello", Bb->"world");
//! assert_eq!(
//!     String::from("\x1b[1;32mHello\x1b[0m \x1b[1;44mworld\x1b[0m"),
//!     color_string
//! );
//!
//! // println a color string
//! // Prints "Hello" with yellow letters, "world" with blue letters and underlined
//! print_color!(Fy->"Hello", Bbu->"world");
//! ```
//!
//! #### Special Thanks
//! This crate was originally inspired by the [row](https://github.com/phsym/prettytable-rs/blob/master/src/row.rs) macro in [prettytable](https://github.com/phsym/prettytable-rs).

#[macro_use]
mod macros;

#[allow(unused)]
pub use paste::paste;

/// Helper function called by [`colorize!`] to convert tokens/string to color text
///
/// This function is not strictly needed for usage of this crate, but is made available in case.
///
/// `color_str` takes any input that implements `Debug` and strips the quotation marks produced, resulting in a normal looking string.
pub fn color_str<T: std::fmt::Debug>(input: T, tag: &str) -> String {
    let mut it = tag.chars().peekable();
    let mut attr: Vec<&str> = vec![];
    let mut newline = "";
    let input = format!("{:?}", input).replace('\"', "");

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
                        _ => "",
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
                        _ => "",
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

    format!("{}\x1b[{}m{}\x1b[0m", newline, attr.join(";"), input)
}

/// Adds ANSI color escape sequences to Strings
///
/// ## Usage
///
/// `colorize!` takes a series of inputs, with or without tokens, and converts the inputs into a `String` with ANSI escape sequences added in.
///
/// The returned `String` is primarily useful for printing out to a terminal which is capable of showing color.
/// However, if all you want to do is print, and want to cut out the extra code, use [`print_color`] instead.
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
/// of the word(s) you wish to be on the newline.
///
/// **Adding the actual `\n` character will cause issues, use the token!!**
///
/// Example -
/// ```
///
/// use colorize::colorize;
///
/// let color_string = colorize!(
///     b->"Hello", // First line
///     Nb->"world, it's me!" // "world..." will be on the new line
/// );
/// ```
///
/// #### Fomrat Multiple Inputs
/// You also have the ability to apply a token to multiple inputs by using `=>` at the beginning of the call.
///
/// ```
///
/// use colorize::colorize;
///
/// let color_string = colorize!(b => Fg->"Hello", By->"world");
/// ```
/// In the above example, "Hello" will have a green foreground, and "world" will have a yellow background. The preceeding `b =>` applies bold formatting to both.
///
/// ### Examples
/// ```
/// use colorize::colorize;
///
/// // Returns "Hello" in bold green
/// let color_string = colorize!(Fgb->"Hello");
/// assert_eq!(String::from("\x1b[32;1mHello\x1b[0m"), color_string);
///
/// // Returns "Hello" in italic blue and "World" underlined in magenta
/// // ", it's me" will be unformatted
/// let color_string = colorize!(iFb->"Hello", Fmu->"world", ", it's me!");
/// assert_eq!(String::from("\x1b[3;34mHello\x1b[0m \x1b[35;4mworld\x1b[0m , it's me!"), color_string);
/// ```
#[macro_export]
macro_rules! colorize {
    ( $($any:tt)* ) => { $crate::macro_colorize!([]; $($any)*, ) };
}

/// `println!` using the [`colorize!`] macro
///
///
/// See [`colorize!`] for more details
///
/// ## Usage
/// ```
/// use colorize::print_color;
///
/// // Will println to the console with "Hello" bold and green, world will be unformatted
/// print_color!(Fgb->"Hello", "world")
/// ```
#[macro_export]
macro_rules! print_color {
    () => (println!(""));
    ( $($any:tt)* ) => ( println!("{}", $crate::colorize!($($any)*)) );
}

#[cfg(test)]
mod tests {
    use std::str::FromStr;

    use super::{colorize, print_color};

    #[test]
    fn test_print_color() {
        print_color!(Fr->"testing", Fbbi->"testing1", b->"testing2", x->"testing3", "testing4", Fgbu->"testing5");
        print_color!("hello");
    }

    #[test]
    fn test_colorize() {
        let col_str =
            colorize!(Fgb->"hello again", N->"hello", "and", BrFb->"goodbye", b->"again" );
        assert_eq!(
            String::from("\x1b[32;1mhello again\x1b[0m \n\x1b[mhello\x1b[0m and \x1b[41;34mgoodbye\x1b[0m \x1b[1magain\x1b[0m"),
            col_str
        )
    }

    #[test]
    fn test_colorize_all() {
        let col_str = colorize!(b => Fg->"One", "two", Fb->"three");

        assert_eq!(
            String::from("\x1b[1;32mOne\x1b[0m \x1b[1mtwo\x1b[0m \x1b[1;34mthree\x1b[0m"),
            col_str
        )
    }

    #[test]
    fn test_debug() {
        use std::path::PathBuf;
        let path = PathBuf::from_str("some").unwrap();
        let col = colorize!(b->"Moving", Fgb->path, b->"to");
        println!("{}", col);
    }
}
