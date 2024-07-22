This rust crate is for creating a string between two other strings, that is lexicographically halfway between them.

```
// "aaa" |
//       | -> creates "aan"
// "aaz" |
```


This is useful for re-ordering items in the database with only one update. Get the left and right key strings (that is used for ordering), then create the new key string with `mid_string()` function. Then assign this key string to the new item that is placed in between the previous two items. If you sort by this key, it should be ordered correctly.

**Important! For now, this works only for lower case English ascii characters: [a ~ z]. Other characters will result in unpredictable behaviour.**

This library is based on the answer provided by "m69" to the following stackoverflow.com question:

[https://stackoverflow.com/questions/38923376/return-a-new-string-that-sorts-between-two-given-strings](https://stackoverflow.com/questions/38923376/return-a-new-string-that-sorts-between-two-given-strings)

Question: Return a new string that sorts between two given strings

Answer provided by "m69 snarky and unwelcoming":
[https://stackoverflow.com/a/38927158/1762976](https://stackoverflow.com/a/38927158/1762976)

# Examples

```
use midstring::mid_string;

// create new strings from empty strings
println!("_, i => {}", mid_string("", "i")); // -> e
println!("_, b => {}", mid_string("", "b")); // -> an
println!("_, _ => {}", mid_string("", "")); // -> n

assert_eq!(mid_string("", "i"), "e");
assert_eq!(mid_string("", "b"), "an");
assert_eq!(mid_string("", ""), "n");

// create new strings between other strings
assert_eq!(mid_string("aaa", "aaz"), "aan");
assert_eq!(mid_string("abc", "abcab"), "abcaan");

assert_eq!(mid_string(&String::from("abcde"), "abchi"), String::from("abcf"));

let left = String::from("abc");
let right = "abcab".to_string();
let should_be = String::from("abcaan");
assert_eq!(mid_string(&left, &right), should_be);
```

