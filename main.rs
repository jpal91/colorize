#![allow(unused)]
use proc_colorize::colorize;

fn main() {
    let my_str = "hello";
    let res = colorize!(Fg->"none", b->"some", my_str, BgFg->String::from("good"));
    println!("{}", res);
}
