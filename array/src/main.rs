use std::io;

fn main() {
    let a = [1, 2, 3, 4, 5];
    println!("Please enter an array index.");
    let mut index = String::new();

    io::stdin().read_line(&mut index).unwrap();

    let index: usize = index.trim().parse().unwrap();

    let element = a[index];

    println!("The value of the element at index {index} is {element}")
}
