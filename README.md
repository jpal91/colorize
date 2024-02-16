# colorize

A set of Rust macros to assist in turning text into colors for printing on the terminal. 

## Purpose

As I was working with another command line utility, I wanted the ability to convert regular text into ANSI color formatted text more easily, so I wrote a series of macros to help with formatting and/or printing that could be reusable.


## Usage
```rust
// Println "Hello world" in bold green
print_color!(Fgb->"Hello world");

// Returns "Hello" in italic blue and "World" underlined in magenta
let color_string = colorize!(iFb->"Hello", Fmu->"World");
assert_eq!(
    String::from("\x1b[3;34mHello\x1b[0m \x1b[4;35mWorld\x1b[0m"), 
    color_string
);
```

See the [colorize macro](https://docs.rs/colorize-macros/latest/colorize/macro.colorize.html) docs for further style specs.

## Development
- [x] Add background color
- [ ] Rework the `colorize!` macro or create a new macro so it acts more like `format!`
- [ ] Add ability to format multiple arguments with the same input (ie `colorize!(b => "Hello", Fg-> "world")` where "Hello" and "world" are both bold but "world" is the only word that's green)
- [ ] Integrate a color set of log macros from the [log](https://docs.rs/log/latest/log/) crate

## Special Thanks
This crate was originally inspired by the [row](https://github.com/phsym/prettytable-rs/blob/master/src/row.rs) macro in [prettytable](https://github.com/phsym/prettytable-rs).