fn main() {
    // Here is how you would define and use a calculate_length function that
    // has a reference to an object as a parameter instead of taking ownership
    // of the value:
    let s1 = String::from("hello");

    // &s1 is a reference - basically, a pointer to the s1 pointer
    // Because it does not own it, the value it points to will not be dropped
    // when the reference goes out of scope.
    let len = calculate_length(&s1);

    println!("The length of '{}' is {}.", s1, len);

    // Here, we set s to be mut (mutable)
    let mut s = String::from("hello");

    // Here, we set a mutable reference to s
    change(&mut s);

    println!("s = {}", s);


    let new_string = no_dangle();

    println!("new_string = {}", new_string);
}

fn calculate_length(s: &String) -> usize { // s is a reference to a String
    s.len()
} // Here, s goes out of scope. But because it does not have ownership of what
  // it refers to, nothing happens.
  // Note that although we can use the value of s here, since it is 'borrowed'
  // we still cannot modify it here (it is not a mutable reference).

  fn change(some_string: &mut String) { // some_string is a mutable reference to a mutable (String) variable
    some_string.push_str(", world");
} // Note that we can now modify the variable and this modifies the variable it 
  // references without any further value assignment in main (where we then
  // print out the new variable value)

  fn no_dangle() -> String {
    let s = String::from("hello");

    s
} // this does not attempt to create a dangling pointer (= good!)
  // See last example, below, for explanation of dangling pointer errors

// Mutable references have one big restriction: you can have only one mutable
// reference to a particular piece of data in a particular scope.
// This code will fail:

//   let mut s = String::from("hello");

//     let r1 = &mut s;
//     let r2 = &mut s;

//     println!("{}, {}", r1, r2);

// error[E0499]: cannot borrow `s` as mutable more than once at a time

// this prevents data races.


// We can use curly brackets to create a new scope, allowing for multiple
// mutable references, just not simultaneous ones:

// let mut s = String::from("hello");

//     {
//         let r1 = &mut s;
//     } // r1 goes out of scope here, so we can make a new reference with no problems.

//     let r2 = &mut s;


// A similar rule exists for combining mutable and immutable references.
// This code results in an error:

// let mut s = String::from("hello");

//     let r1 = &s; // no problem
//     let r2 = &s; // no problem
//     let r3 = &mut s; // BIG PROBLEM

//     println!("{}, {}, and {}", r1, r2, r3);

// error[E0502]: cannot borrow `s` as mutable because it is also borrowed as immutable


// We also cannot have a mutable reference while we have an immutable one.


// Note that a reference’s scope starts from where it is introduced and
// continues through the last time that reference is used. For instance,
// this code will compile because the last usage of the immutable references
// occurs before the mutable reference is introduced:

// let mut s = String::from("hello");

//     let r1 = &s; // no problem
//     let r2 = &s; // no problem
//     println!("{} and {}", r1, r2);
//     // r1 and r2 are no longer used after this point

//     let r3 = &mut s; // no problem
//     println!("{}", r3);


// Dangling references. Rust prevents creation of a dangling pointer, a pointer
// that references a location in memory that may have been given to someone
// else, by freeing some memory while preserving a pointer to that memory.

// For example, this code raises an error:

// n main() {
//     let reference_to_nothing = dangle();
// }

// fn dangle() -> &String { // dangle returns a reference to a String
//     let s = String::from("hello"); // s is a new String

//     &s // we return a reference to the String, s
// } // Here, s goes out of scope, and is dropped. Its memory goes away.
     // Danger!