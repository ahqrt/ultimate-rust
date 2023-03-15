fn main() {
    // slices let you reference a contiguous sequence of elements in a collection
    // rather than the whole collection.
    // a slice is a kind of reference. so it does not have ownership.

    let mut s = String::from("Hello world");
    let word = first_word(&s);
    // s.clear();
    // this empties the string, making it equal to ""
    // we could not meaningfully use the value with word .word is now totally invalid
    println!("{word}");

    // string slice
    let s = String::from("Hello world");
    let hello = &s[0..5];
    let word = &s[6..=10];
    println!("{hello}, {word}")
}

fn first_word(s: &String) -> &str {
    let bytes = s.as_bytes();
    for (i, &item) in bytes.iter().enumerate() {
        if item == b' ' {
            return &s[0..i];
        }
    }
    &s[..]
}
