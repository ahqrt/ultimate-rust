use std::ops::Deref;

enum List {
    Cons(i32, Box<List>),
    Nil,
}

struct MyBox<T>(T);

impl<T> MyBox<T> {
    fn new(x: T) -> MyBox<T> {
        MyBox(x)
    }
}

impl<T> Deref for MyBox<T> {
    type Target = T;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

fn main() {
    // let list = List::Cons(
    //     1,
    //     Box::new(List::Cons(2, Box::new(List::Cons(3, Box::new(List::Nil))))),
    // );
}

#[cfg(test)]
mod tests {
    use crate::MyBox;

    #[test]
    fn test() {
        let x = 5;
        let y = MyBox::new(x);
        assert_eq!(x, 5);
        assert_eq!(5, *y);
    }
}
