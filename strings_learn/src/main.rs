fn main() {
    let mut s = String::new();

    let data = "initial contents";
    let s = data.to_string();

    let s = String::from(data);

    let mut s = String::from("foo");
    s.push_str("bar");

    s.push('a');
    println!("{s}");

    let s1 = String::from("Hello");
    let s2 = String::from("World ");
    let s3 = String::from("") + &s1 + &s2; // note s1 has been moved here and can no longer be used
    println!("{s3}, {s1}, {s2}");
    // also can use format!
    let s3 = format!("{s3}, {s1}, {s2}");
    println!("{s3}, {s1}, {s2}");

    // indexing to strings
    let s1 = String::from("hello");
    // let h = s1[0];  error: string can not be indexing
    println!("{}", s1.len());
    let hello = String::from("Здравствуйте");
    println!("{}", hello.len());
    let hello = String::from("你好");
    println!("{}", hello.len());

    for c in "Зд".chars() {
        println!("{c}");
    }

    for b in "Зд".bytes() {
        println!("{b}");
    }
}
