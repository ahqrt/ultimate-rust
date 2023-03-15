// fn main() {
//     // s comes into scope
//     let s = String::from("hello");
//     // s has move into s1
//     // let s1 = s;
//     //  and s is not valid anymore after move

//     // s's value move into the function... and is no longer valid here
//     takes_ownership(s);
//     // you can not use s anymore

//     // so passing a complex variable which does not impl copy trait
//     // to a function will move, just as assignment does

//     // x comes into scope
//     let x = 5;
//     // x would move into the function, but i32 is Copy, so it's okay to still use x afterward
//     makes_copy(x);
//     println!("{x}")
// } // here, x goes out of scope, then s, but cuz s's value was moved, nothing special happens

// fn takes_ownership(some_string: String) {
//     // some_string comes into this scope
//     println!("{}", some_string)
// } // here, some_string goes out of the scope and 'drop' is called, the backing memory is freed.

// fn makes_copy(some_integer: i32) {
//     // some_integer comes into this scope
//     println!("{some_integer}")
// } // here, some_string goes out of the scope. Nothing special happens

// return values and scope

fn main() {
    let s1 = gives_ownership(); // gives_ownership moves its return
                                // value into s1

    let s2 = String::from("hello"); // s2 comes into scope

    let s3 = takes_and_gives_back(s2); // s2 is moved into
                                       // takes_and_gives_back, which also
                                       // moves its return value into s3
} // Here, s3 goes out of scope and is dropped. s2 was moved, so nothing
  // happens. s1 goes out of scope and is dropped.

fn gives_ownership() -> String {
    // gives_ownership will move its
    // return value into the function
    // that calls it

    let some_string = String::from("yours"); // some_string comes into scope

    some_string // some_string is returned and
                // moves out to the calling
                // function
}

// This function takes a String and returns one
fn takes_and_gives_back(a_string: String) -> String {
    // a_string comes into
    // scope

    a_string // a_string is returned and moves out to the calling function
}
