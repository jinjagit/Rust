fn main() {
    let s1 = gives_ownership();         // gives_ownership moves its return
                                        // value into s1

    println!("s1 = {}", s1);
                                    
    let s2 = String::from("string2");     // s2 comes into scope

    println!("s2 = {}", s2);

    let s3 = takes_and_gives_back(s2);  // s2 is moved into
                                        // takes_and_gives_back, which also
                                        // moves its return value into s3

    println!("s3 = {}", s3);
} // Here, s3 goes out of scope and is dropped. s2 goes out of scope but was
  // moved, so nothing happens. s1 goes out of scope and is dropped.

fn gives_ownership() -> String {             // gives_ownership will move its
                                             // return value into the function
                                             // that calls it

    let some_string = String::from("goodbye"); // some_string comes into scope

    some_string                              // some_string is returned and
                                             // moves out to the calling
                                             // function
}

// takes_and_gives_back will take a String and return one
fn takes_and_gives_back(a_string: String) -> String { // a_string comes into
                                                      // scope

    a_string  // a_string is returned and moves out to the calling function
}


// It’s possible to return multiple values using a tuple, as shown:

// Filename: src/main.rs

// fn main() {
//     let s1 = String::from("hello");

//     let (s2, len) = calculate_length(s1);

//     println!("The length of '{}' is {}.", s2, len);
// }

// fn calculate_length(s: String) -> (String, usize) {
//     let length = s.len(); // len() returns the length of a String

//     (s, length)
// }

// But this is too much ceremony and a lot of work for a concept
// that should be common. Luckily for us, Rust has a feature for
// this concept, called 'references'.