use colorize::colorize;

#[test]
fn escaped_braces() {
    assert_eq!(
        colorize!("{{literal}} {}", Fg->"x"),
        "{literal} \x1b[32mx\x1b[0m"
    );
}

#[test]
fn inline_named_arg() {
    let n = 5;
    assert_eq!(colorize!("{n} {}", Fg->"x"), "5 \x1b[32mx\x1b[0m");
}

#[test]
fn inline_named_arg_with_format_spec() {
    let name = "n";
    assert_eq!(colorize!("{name:?} {}", Fg->"x"), "\"n\" \x1b[32mx\x1b[0m");
}

#[test]
fn inline_named_arg_gets_shared_style() {
    let n = 5;
    assert_eq!(
        colorize!("{n} {}", b => Fg->"x"),
        "\x1b[1m5\x1b[0m \x1b[1;32mx\x1b[0m"
    );
}

#[test]
fn explicit_index_follows_argument() {
    assert_eq!(
        colorize!("{1} {0} {1}", Fg->"a", Fb->"b"),
        "\x1b[34mb\x1b[0m \x1b[32ma\x1b[0m \x1b[34mb\x1b[0m"
    );
}

#[test]
fn star_precision() {
    assert_eq!(
        colorize!("{:.*} {}", 2, Fg->1.2345, "x"),
        "\x1b[32m1.23\x1b[0m x"
    );
}
