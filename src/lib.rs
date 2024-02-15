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
                        'r' => "31",
                        'g' => "32",
                        'y' => "33",
                        'b' => "34",
                        'm' => "35",
                        'c' => "36",
                        'w' => "37",
                        'k' => "30",
                        _ => "",
                    };
                    if !col.is_empty() {
                        it.next();
                        attr.push(col)
                    }
                }
            },
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
///     1. b -> bold
///     2. u -> underline
///     3. i -> italic
///
/// #### Color
/// Color tokens start with an `F` (for foreground)
///
///     1. Fb -> blue
///     2. Fr -> red
///     3. Fg -> green
///     4. Fy -> yellow
///     5. Fm -> magenta
///     6. Fc -> cyan
///     7. Fw -> white
///     8. Fk -> black
///
/// #### Special Newline Token
/// If you want to add a newline  within the string, include a `N` token at the start
/// of the word(s) you wish to be on the newline.
///
/// **Adding the actual `\n` character will cause issues, use the token!!**
///
///
/// Example -
/// ```
/// let color_string = colorize!(
///     b->"Hello", // First line
///     Nb->"world, it's me!" // "world..." will be on the new line
/// )
/// ```
///
/// ### Examples
/// ```
/// // Returns "Hello" in bold green
/// let color_string = colorize!(Fgb->"Hello");
/// assert_eq!(String::from("\x1b[1;32mHello\x1b[0m"), color_string);
///
/// // Returns "Hello" in italic blue and "World" underlined in magenta
/// // ", it's me" will be unformatted
/// let color_string = colorize!(iFb->"Hello", Fmu->"world", ", it's me!");
/// assert_eq!(String::from("\x1b[3;34mHello\x1b[0m \x1b[4;35mworld\x1b[0m , it's me!"), color_string);
/// ```
#[macro_export]
macro_rules! colorize {

    () => {String::new()};

    ( [ $($acc:tt)* ]; $tag:ident -> $msg:expr, $($rest:tt)* ) => {
        {
            let color = $crate::color_str( $msg , stringify!($tag));
            colorize!([ $($acc)* color, ]; $($rest)* )
        }
    };

    ( [ $($acc:tt)* ]; $msg:expr, $($rest:tt)* ) => {colorize!([$($acc)* $msg.to_string() ,]; $($rest)*)};

    ( [ $($acc:tt)* ]; $tag:ident -> $msg:expr ) => {colorize!([$($acc)*]; $tag -> $msg , )};

    ( [ $($acc:tt)* ]; $msg:expr ) => {colorize!([$($acc)* $msg.to_string() ,]; )};

    ( [ $($acc:tt)* ]; ) =>  { [$($acc)*].join(" ") };

    ( $($any:tt)* ) => { colorize!([]; $($any)* ) };
}

/// `println` using the [`colorize!`] macro
///
///
/// See [`colorize!`] for more details
///
/// ## Usage
/// ```
/// // Will println to the console with "Hello" bold and green, world will be unformatted
/// print_color!(Fgb->"Hello", "world")
/// ```
#[macro_export]
macro_rules! print_color {
    () => (println!(""));
    ( $($any:tt)* ) => ( println!("{}", colorize!([]; $($any)*)) );
}

#[cfg(test)]
mod tests {
    use std::str::FromStr;

    use super::*;

    #[test]
    fn test_print_color() {
        print_color!(Fr->"testing", Fbbi->"testing1", b->"testing2", x->"testing3", "testing4", Fgbu->"testing5");
        print_color!("hello");
    }

    #[test]
    fn test_colorize() {
        let col_str =
            colorize!(Fgb->"hello again", N->"hello", "and", FrFb->"goodbye", b->"again" );
        assert_eq!(
            String::from("\x1b[32;1mhello again\x1b[0m \n\x1b[mhello\x1b[0m and \x1b[31;34mgoodbye\x1b[0m \x1b[1magain\x1b[0m"),
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
