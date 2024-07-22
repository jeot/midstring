use midstring::mid_string;

fn main() {
    println!("Hello, world!");

    println!("_, i => {}", mid_string("", "i")); // -> e
    println!("_, b => {}", mid_string("", "b")); // -> an
    println!("_, _ => {}", mid_string("", "")); // -> n

    assert_eq!(mid_string("aaa", "aaz"), "aan");

    let left = String::from("abc");
    let right = "abcab".to_string();
    let should_be = String::from("abcaan");
    assert_eq!(mid_string(&left, &right), should_be);

    println!("Goodbye, world!");
}
