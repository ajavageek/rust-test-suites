fn main() {
    println!("{}", hello());
}

#[cfg(not(test))]
fn hello() -> &'static str {
    return "Hello world";
}

#[cfg(test)]
fn hello() -> &'static str {
    return "Hello test";
}

#[test]
fn test_hello() {
    assert_eq!(hello(), "Hello test");
}
