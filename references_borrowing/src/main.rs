fn main() {
    let mut s = String::from("Hello");

    // & reference does not take the ownership
    let len = calculate_length(&s);
    println!("The length of '{s}' is {len}");
    change(&mut s);
    let len = calculate_length(&s);
    println!("The length of '{s}' is {len}");

    // Mutable references have one big restriction: if you have a mutable reference to a value,
    // you can have no other references to that value.
    //  This code that attempts to create two mutable references to s will fail:
    let mut s = String::from("hello");

    let r1 = &mut s;
    // let r2 = &mut s;
    // println!("{}, {}", r1, r2);

    // but we can use this way
    {
        let r2 = &mut s;
        // and do something with r2
    }

    let s1 = &s;
    let s2 = &s;
    let s3 = &mut s;
    println!("{}, {}, and {}", s1, s2, s3);
    // above error tell us that we also can not have a mutable reference while we
    // have an immutable on to the same value

    //  this will work
    let mut s = String::from("hello");
    let r1 = &s; // no problem
    let r2 = &s; // no problem
    println!("{} and {}", r1, r2);
    // variables r1 and r2 will not be used after this point

    let r3 = &mut s; // no problem
    println!("{}", r3);
}

fn calculate_length(s: &String) -> usize {
    s.len()
}

fn change(some_string: &mut String) {
    some_string.push_str("world")
}

fn dangle() -> &String {
    let s = String::from("hello");
    &s
}
