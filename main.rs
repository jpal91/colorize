#![allow(unused)]
// use colorize::colorize;
// use proc_colorize::colorize;
use colorize_proc_macro::colorize;

fn main() {
    let my_str = "hello";
    // let res = colorize!(Fg->"none", b->"some", my_str, BgFg->String::from("good"));
    let res2 =
        colorize!("{}{:#?}{}", b => Fy -> "some", std::path::PathBuf::from("/tmp"), NFru -> "else");
    println!("{}", res2);
}

// fn main() {}
