use std::ops::Deref;

// The Deref trait, provided by the standard library, requires us to implement
// one method named deref that borrows self and returns a reference to the inner
// data.
impl<T> Deref for MyBox<T> {
    type Target = T;

    fn deref(&self) -> &T {
        &self.0
    }
}

struct MyBox<T>(T);

impl<T> MyBox<T> {
    fn new(x: T) -> MyBox<T> {
        MyBox(x)
    }
}

fn main() {
    let x = 5;
    let y = &x;

    println!("x = {}", x);

    // Comparing a number and a reference to a number isn’t allowed because
    // they’re different types. We must use the dereference operator to follow
    // the reference to the value it’s pointing to.
    println!("x = {}", *y);

    assert_eq!(5, x);
    assert_eq!(5, *y);

    // Our MyBox<T> type can’t be dereferenced because we haven’t implemented
    // that ability on our type. We have enabled dereferencing with the
    // * operator by implementing the Deref trait (see impl, above).
    let z = MyBox::new(x);

    println!("z = {}", *z);
}
