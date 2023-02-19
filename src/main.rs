fn parse(s: &str) -> Vec<&str> {
    return s
        .strip_prefix("(")
        .unwrap()
        .strip_suffix(")")
        .unwrap()
        .split_whitespace()
        .collect();
}

fn main() {
    let s = "(+ 1 2)";

    println!("{:?}", parse(s));
}
