// The implementation of Rc::clone doesn’t make a deep copy of all the data like
// most types’ implementations of clone do. The call to Rc::clone only
// increments the reference count, which doesn’t take much time.

enum List {
    Cons(i32, Rc<List>),
    Nil,
}

use crate::List::{Cons, Nil};
use std::rc::Rc;

fn main() {
    let a = Rc::new(Cons(5, Rc::new(Cons(10, Rc::new(Nil)))));
    println!("count after creating a = {}", Rc::strong_count(&a));
    let _b = Cons(3, Rc::clone(&a));
    println!("count after creating b = {}", Rc::strong_count(&a));
    {
        let _c = Cons(4, Rc::clone(&a));
        println!("count after creating c = {}", Rc::strong_count(&a));
    }

    // We don’t have to call a function to decrease the reference count like we
    // have to call Rc::clone to increase the reference count: the
    // implementation of the Drop trait decreases the reference count
    // automatically when an Rc<T> value goes out of scope.
    println!("count after c goes out of scope = {}", Rc::strong_count(&a));
}