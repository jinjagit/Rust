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
}
