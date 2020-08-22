// Each value in Rust has a variable that’s called its owner.
// There can only be one owner at a time.
// When the owner goes out of scope, the value will be dropped.

fn main() {
    let l = "Hello";

    println!("{}: I am a string literal. I live on the stack.", l);

    let mut s = String::from("Hi");

    println!("{}: I am a mutable string. I live on the heap.", s);

    s.push_str(", world!"); // push_str() appends a literal to a String

    println!("{}", s); // This will print `hello, world!`

    // Bind the value 5 to x;
    // then make a copy of the value in x and bind it to y.
    // Note that integers have a known size and are stored
    // entirely on the stack.
    let x = 5;
    let y = x;

    println!("x = {}, y = {}", x, y);

    // This would cause a double-free error, if it were permitted.
    // Rust will move s1 into s2.
    // (actually it deletes s1's stack data and its pointer)
    let s1 = String::from("Bibble");
    let s2 = s1;

    // error[E0382]: borrow of moved value: `s1`
    // println!("s1 = {}, s2 = {}", s1, s2);
    println!("s2 = {}", s2);

    // If we do want to deeply copy the heap data, not just the stack data,
    // we can use a common method called clone (relatively expensive).
    let s1 = String::from("Bobble");
    let s2 = s1.clone();

    println!("s1 = {}, s2 = {}", s1, s2);


    // Passing a variable to a function will move or copy,
    // just as assignment does.
    let s = String::from("hello");  // s comes into scope

    takes_ownership(s);             // s's value moves into the function...
                                    // ... and so is no longer valid here

    // println!("{}", s); // error[E0382]: borrow of moved value: `s`

    let x = 5;                      // x comes into scope

    makes_copy(x);                  // x would move into the function,
                                    // but i32 is Copy, so it’s okay to still
                                    // use x afterward

    println!("{}", x); // perfectly fine - x is still in scope
    
} // Here, x goes out of scope, then s. But because s's value was moved, nothing
  // special happens.

fn takes_ownership(some_string: String) { // some_string comes into scope
    println!("{}", some_string);
} // Here, some_string goes out of scope and `drop` is called. The backing
  // memory is freed.

fn makes_copy(some_integer: i32) { // some_integer comes into scope
    println!("{}", some_integer);
} // Here, some_integer goes out of scope. Nothing special happens.

