fn main() {
    // s comes into scope
    let s = String::from("hello");
    // s has move into s1
    // let s1 = s;
    //  and s is not valid anymore after move

    // s's value move into the function... and is no longer valid here
    takes_ownership(s);
    // you can not use s anymore

    // so passing a complex variable which does not impl copy trait
    // to a function will move, just as assignment does

    // x comes into scope
    let x = 5;
    // x would move into the function, but i32 is Copy, so it's okay to still use x afterward
    makes_copy(x);
    println!("{x}")
} // here, x goes out of scope, then s, but cuz s's value was moved, nothing special happens

fn takes_ownership(some_string: String) {
    // some_string comes into this scope
    println!("{}", some_string)
} // here, some_string goes out of the scope and 'drop' is called, the backing memory is freed.

fn makes_copy(some_integer: i32) {
    // some_integer comes into this scope
    println!("{some_integer}")
} // here, some_string goes out of the scope. Nothing special happens
