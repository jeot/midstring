//! This 'midstring' library is for creating a string between two other strings, that is
//! lexicographically halfway between them.
//! ```
//! // "aaa" |
//! //       | -> creates "aan"
//! // "aaz" |
//! ```
//!
//! This is useful for re-ordering items in the database with only one update. Get the left and
//! right key strings (that is used for ordering), then create the new key string with
//! `mid_string()` function. Then assign this key string to the new item that is placed in between
//! the previous two items. If you sort by this key, it should be ordered correctly.
//!
//! This library is based on the answer provided by "m69" to the following stackoverflow.com question:
//!
//! [https://stackoverflow.com/questions/38923376/return-a-new-string-that-sorts-between-two-given-strings](https://stackoverflow.com/questions/38923376/return-a-new-string-that-sorts-between-two-given-strings)
//!
//! Question: Return a new string that sorts between two given strings
//!
//! Answer provided by "m69 snarky and unwelcoming":
//! [https://stackoverflow.com/a/38927158/1762976](https://stackoverflow.com/a/38927158/1762976)
//!
//! # Examples
//!
//! ```
//! use midstring::mid_string;
//!
//! // create new strings from empty strings
//! println!("_, i => {}", mid_string("", "i")); // -> e
//! println!("_, b => {}", mid_string("", "b")); // -> an
//! println!("_, _ => {}", mid_string("", "")); // -> n
//!
//! assert_eq!(mid_string("", "i"), "e");
//! assert_eq!(mid_string("", "b"), "an");
//! assert_eq!(mid_string("", ""), "n");
//!
//! // create new strings between other strings
//! assert_eq!(mid_string("aaa", "aaz"), "aan");
//! assert_eq!(mid_string("abc", "abcab"), "abcaan");
//!
//! assert_eq!(mid_string(&String::from("abcde"), "abchi"), String::from("abcf"));
//!
//! let left = String::from("abc");
//! let right = "abcab".to_string();
//! let should_be = String::from("abcaan");
//! assert_eq!(mid_string(&left, &right), should_be);
//! ```
//!

// The original C code provided by m69:
// https://stackoverflow.com/a/38927158/1762976
/*
int midstring(const char *prev, const char *next, char *buf) {
    char p = 0, n = 0;
    int len = 0;
    while (p == n) {                                           // copy identical part
        p = prev[len] ? prev[len] : 'a' - 1;
        n = next[len] ? next[len] : 'z' + 1;
        if (p == n) buf[len++] = p;
    }
    if (p == 'a' - 1) {                                        // end of left string
        while (n == 'a') {                                     // handle a's
            buf[len++] = 'a';
            n = next[len] ? next[len] : 'z' + 1;
        }
        if (n == 'b') {                                        // handle b
            buf[len++] = 'a';
            n = 'z' + 1;
        }
    }
    else if (p + 1 == n) {                                     // consecutive characters
        n = 'z' + 1;
        buf[len++] = p;
        while ((p = prev[len] ? prev[len] : 'a' - 1) == 'z') { // handle z's
            buf[len++] = 'z';
        }
    }
    buf[len++] = n - (n - p) / 2;                              // append middle character
    buf[len] = '\0';
    return len;
}
*/

///
/// Some constants for representing ascii characters
///
const A: u8 = 0x61; // 'a'
const B: u8 = 0x62; // 'b'
const _N: u8 = 0x6E; // 'n'
const Z: u8 = 0x7A; // 'z'

/// Create a string that is lexicographically halfway between the left and right strings
///
/// # Examples
///
/// ```
/// use midstring::mid_string;
///
/// assert_eq!(mid_string("aaa", "aaz"), "aan");
/// assert_eq!(mid_string("abc", "abcab"), "abcaan");
///
/// assert_eq!(mid_string(&String::from("abcde"), "abchi"), String::from("abcf"));
///
/// let left = String::from("abc");
/// let right = "abcab".to_string();
/// let should_be = String::from("abcaan");
/// assert_eq!(mid_string(&left, &right), should_be);
/// ```
///
pub fn mid_string(prev: &str, next: &str) -> String {
    let prev_bytes = prev.to_string().as_bytes().to_vec();
    let next_bytes = next.to_string().as_bytes().to_vec();
    let buf_bytes = the_original_algorith_with_ascii_digits(prev_bytes, next_bytes);
    String::from_utf8(buf_bytes).unwrap()
}

/// The original code provided by "m69 snarky and unwelcoming"
fn the_original_algorith_with_ascii_digits(prev: Vec<u8>, next: Vec<u8>) -> Vec<u8> {
    // first add the null pointer at the end
    let mut prev = prev.clone();
    let mut next = next.clone();
    prev.push(0);
    next.push(0);
    let mut p: u8 = 0;
    let mut n: u8 = 0;
    let mut len: usize = 0;
    let mut buf: Vec<u8> = Vec::new();

    while p == n {
        // copy identical part
        p = if prev[len] != 0 { prev[len] } else { A - 1 };
        n = if next[len] != 0 { next[len] } else { Z + 1 };
        if p == n {
            buf.push(p);
            len += 1;
        }
    }

    if p == (A - 1) {
        // end of left string
        while n == A {
            // handle a's
            buf.push(A);
            len += 1;
            n = if next[len] != 0 { next[len] } else { Z + 1 };
        }
        if n == B {
            // handle b
            buf.push(A);
            len += 1;
            n = Z + 1;
        }
    } else if (p + 1) == n {
        // consecutive characters
        n = Z + 1;
        buf.push(p);
        len += 1;
        p = if prev[len] != 0 { prev[len] } else { A - 1 };
        let mut check: bool = p == Z;
        while check {
            // handle z's
            buf.push(Z);
            len += 1;
            p = if prev[len] != 0 { prev[len] } else { A - 1 };
            check = p == Z;
        }
    }
    let middle_char = n - (n - p) / 2; // append middle character
    buf.push(middle_char);
    // buf.push(0);
    buf
}

#[cfg(test)]
mod tests {
    use super::*;

    // Simple test
    // aaa ~ aaz  ->  aan
    #[test]
    fn test_simple() {
        assert_eq!(mid_string("aaa", "aaz"), "aan");
    }

    // Basic case
    // abcde ~ abchi  ->  abc  +  d ~ h  ->  abcf
    // abc   ~ abchi  ->  abc  +  _ ~ h  ->  abcd
    #[test]
    fn test_basic_case() {
        assert_eq!(mid_string("abcde", "abchi"), "abcf");
        assert_eq!(mid_string("abc", "abchi"), "abcd");
    }

    // Consecutive characters
    // abhs ~ abit  ->  ab  +  h ~ i  ->  abh  +  s ~ _  ->  abhw
    // abh  ~ abit  ->  ab  +  h ~ i  ->  abh  +  _ ~ _  ->  abhn
    #[test]
    fn test_consecutive_characters_1() {
        assert_eq!(mid_string("abhs", "abit"), "abhw");
        assert_eq!(mid_string("abh", "abit"), "abhn");
        assert_eq!(
            mid_string(&String::from("abh"), &String::from("abit")),
            String::from("abhn")
        );
    }

    //abhz   ~ abit  ->  ab  +  h ~ i  ->  abh  +  z ~ _  ->  abhz  +  _ ~ _  ->  abhzn
    //abhzs  ~ abit  ->  ab  +  h ~ i  ->  abh  +  z ~ _  ->  abhz  +  s ~ _  ->  abhzw
    //abhzz  ~ abit  ->  ab  +  h ~ i  ->  abh  +  z ~ _  ->  ... ->  abhzz  +  _ ~ _  ->  abhzzn
    #[test]
    fn test_consecutive_characters_2() {
        assert_eq!(mid_string("abhz", "abit"), "abhzn");
        assert_eq!(mid_string("abhzs", "abit"), "abhzw");
        assert_eq!(mid_string("abhzz", "abit"), "abhzzn");
        assert_eq!(
            mid_string(&String::from("abhzz"), &String::from("abit")),
            String::from("abhzzn")
        );
    }

    // Right character is a or b
    //abc  ~ abcah   ->  abc  +  _ ~ a  ->  abca  +  _ ~ h  ->  abcad
    //abc  ~ abcab   ->  abc  +  _ ~ a  ->  abca  +  _ ~ b  ->  abcaa  +  _ ~ _  ->  abcaan
    //abc  ~ abcaah  ->  abc  +  _ ~ a  ->  abca  +  _ ~ a  ->  abcaa  +  _ ~ h  ->  abcaad
    //abc  ~ abcb    ->  abc  +  _ ~ b  ->  abca  +  _ ~ _  ->  abcan
    #[test]
    fn test_right_character_is_a_or_b() {
        assert_eq!(mid_string("abc", "abcah"), "abcad");
        assert_eq!(mid_string("abc", "abcab"), "abcaan");
        assert_eq!(mid_string("abc", "abcaah"), "abcaad");
        assert_eq!(mid_string("abc", "abcb"), "abcan");
        assert_eq!(
            mid_string(&String::from("abc"), &String::from("abcb")),
            String::from("abcan")
        );
    }
}
