//! A set of Rust macros to assist in turning text into colors for printing on the terminal.
//! ## Usage
//!
//! `colorize!` takes a series of inputs, with or without tokens, and converts the inputs into a `String` with ANSI escape sequences added in.
//!
//! The returned `String` is primarily useful for printing out to a terminal which is capable of showing color.
//! However, if all you want to do is print, and want to cut out the extra code, use [`print_color`] instead.
//!
//! ## Valid inputs
//! `colorize!` uses the same formatting style as [`format!`](std::format!) so it follows the same
//! argument rules.
//!
//! ```
//! # use colorize_proc_macro as colorize;
//! use std::path::PathBuf;
//! use colorize::colorize;
//!
//! let user_path = PathBuf::from("/home/color/my_new_file.txt");
//! let pretty_path = colorize!("{:?}", Fgu->user_path.clone());
//!
//! assert_eq!(
//!     String::from("\x1b[32;4m\"/home/color/my_new_file.txt\"\x1b[0m"),
//!     pretty_path
//! );
//! ```
//!
//! ## Tokens
//! Tokens can change color or font styling depending on their order and usage.
//!
//! #### Styling
//! 1. b -> bold
//! 2. u -> underline
//! 3. i -> italic
//!
//! #### Color
//! Color tokens start with an `F` (for foreground) or `B` (for background)
//!
//! 1. Fb/Bb -> blue
//! 2. Fr/Br -> red
//! 3. Fg/Bg -> green
//! 4. Fy/By -> yellow
//! 5. Fm/By -> magenta
//! 6. Fc/Bc -> cyan
//! 7. Fw/Bw -> white
//! 8. Fk/Bk -> black
//!
//! #### Special Newline Token
//! If you want to add a newline  within the string, include a `N` token at the start
//! of the word(s) you wish to be on the newline.
//!
//! **Adding the actual `\n` character will cause issues, use the token!!**
//!
//! Example -
//! ```
//! # use colorize_proc_macro as colorize;
//! use colorize::colorize;
//!
//! let color_string = colorize!(
//!     "{} {}",
//!     b->"Hello", // First line
//!     Nb->"world, it's me!" // "world..." will be on the new line
//! );
//! ```
//!
//! #### Format Multiple Inputs
//! You also have the ability to apply a token to multiple inputs by using `=>` at the beginning of the call.
//!
//! ```
//! # use colorize_proc_macro as colorize;
//! use colorize::colorize;
//!
//! let color_string = colorize!("{} {}", b => Fg->"Hello", By->"world");
//! ```
//! In the above example, "Hello" will have a green foreground, and "world" will have a yellow background. The preceeding `b =>` applies bold formatting to both.
//!
//! ### Examples
//! ```
//! # use colorize_proc_macro as colorize;
//! use colorize::colorize;
//!
//! // Returns "Hello" in bold green
//! let color_string = colorize!("{}", Fgb->"Hello");
//! assert_eq!(String::from("\x1b[32;1mHello\x1b[0m"), color_string);
//!
//! // Returns "Hello" in italic blue and "World" underlined in magenta
//! // ", it's me" will be unformatted
//! let color_string = colorize!("{} {} {}", iFb->"Hello", Fmu->"world", ", it's me!");
//! assert_eq!(String::from("\x1b[3;34mHello\x1b[0m \x1b[35;4mworld\x1b[0m , it's me!"), color_string);
//! ```

#[doc(hidden)]
#[allow(unused)]
pub use paste::paste;

pub use colorize_proc_macro::colorize;

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
/// print_color!("{} {}", Fgb->"Hello", "world")
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
    fn test_colorize() {
        let col_str = colorize!("{} {} {} {} {}", Fgb->"hello again", N->"hello", "and", BrFb->"goodbye", b->"again" );
        assert_eq!(
            String::from("\x1b[32;1mhello again\x1b[0m \n\x1b[mhello\x1b[0m and \x1b[41;34mgoodbye\x1b[0m \x1b[1magain\x1b[0m"),
            col_str
        )
    }

    #[test]
    fn test_colorize_all() {
        let col_str = colorize!("{} {} {}", b => Fg->"One", "two", Fb->"three");

        assert_eq!(
            String::from("\x1b[1;32mOne\x1b[0m \x1b[1mtwo\x1b[0m \x1b[1;34mthree\x1b[0m"),
            col_str
        )
    }

    #[test]
    fn test_debug() {
        use std::path::PathBuf;
        let path = PathBuf::from_str("some").unwrap();
        let col = colorize!("{} {:?} {}", b->"Moving", Fgb->path.clone(), b->"to");
        assert_eq!(
            String::from("\x1b[1mMoving\x1b[0m \x1b[32;1m\"some\"\x1b[0m \x1b[1mto\x1b[0m"),
            col
        );

        print_color!("{} {:?} {} {:?}", b => "Moving", Fg->path, "to", Fg->PathBuf::from("other"))
    }
}
