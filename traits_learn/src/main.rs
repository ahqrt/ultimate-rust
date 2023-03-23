use std::collections::HashMap;

fn main() {
    let mut map = HashMap::new();
    map.insert(String::from("Hello"), 10);

    println!("{:?}", map);
}
