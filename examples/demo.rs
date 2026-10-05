//! Showcase of `colorize!` and `print_color!` output.
//!
//! Run with `cargo run --example demo`.

use std::path::PathBuf;

use colorize::{colorize, print_color};

fn main() {
    // Single styled arguments
    print_color!("{} {} {}", Fgb->"bold green", iFb->"italic blue", Fmu->"underlined magenta");

    // Background colors
    print_color!("{} {} {}", BrFw->" error ", ByFk->" warn ", BgFk->" ok ");

    // Apply a token to every argument with `=>`
    print_color!("{}, {}!", b => Fc->"Hello", Fy->"world");

    // Any `Debug` or `Display` value works, just like `format!`
    let src = PathBuf::from("/home/color/my_new_file.txt");
    let dest = PathBuf::from("/home/new_color_dir/my_second_file.txt");
    print_color!("{} {:?} {} {:?}", b => "Moving", Fy->src, "to", Fg->dest);

    // `colorize!` returns a `String` instead of printing
    let status = colorize!("[{}] {}", Fgb->"PASS", "all checks green");
    println!("{status}");
}
