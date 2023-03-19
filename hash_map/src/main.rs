use std::collections::HashMap;

fn main() {
    let mut scores = HashMap::new();
    scores.insert(String::from("blue"), 10);
    scores.insert(String::from("red"), 12);

    let team_name = String::from("blue");
    let score = scores.get(&team_name).copied().unwrap_or(0);
    println!("{score}");

    for (key, value) in &scores {
        println!("{key}: {value}")
    }

    let filed_name = String::from("favorite color");
    let filed_value = String::from("Blue");

    let mut map = HashMap::new();
    // if the type impl the Copy trait, the values will copied into the hash map
    // for owned values like String the values will be moved and the hash map will be the owner
    // map.insert(filed_name, filed_value);
    //  this will work
    map.insert(&filed_name, &filed_value);
    // so the filed_name and the filed_value is not available here
    println!("{filed_name}, {filed_value}");
    let value = map.get(&filed_name).expect("");
    println!("{}", **value);

    // adding a key and value only if a key is not present
    let mut scores = HashMap::new();
    scores.insert(String::from("blue"), 10);
    scores.entry(String::from("Yellow")).or_insert(10);

    let text = "hello world wonderful world";

    let mut map = HashMap::new();

    for word in text.split_whitespace() {
        let count = map.entry(word).or_insert(0);
        *count += 1;
    }

    println!("{:?}", map);
}
