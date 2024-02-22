#![allow(unused)]
use proc_colorize::colorize;

fn main() {
    let my_str = "hello";
    let res = colorize!(Fn->"none", "some", my_str, String::from("good"));
    println!("{}", res);
}
