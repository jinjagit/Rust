fn main() {
    let my_string = String::from("hello world");

    // first_word works on slices of `String`s
    let word = first_word(&my_string[..]);

    println!("word = {}", word);

    let my_string_literal = "hello world";

    // first_word works on slices of string literals
    let word = first_word(&my_string_literal[..]);

    println!("word = {}", word);

    // Because string literals *are* string slices already,
    // this works too, without the slice syntax!
    let word = first_word(my_string_literal);

    println!("word = {}", word);

    let a = [1, 2, 3, 4, 5];

    let slice = &a[1..4]; // from index 1 to index 4, NOT including index 4

    println!("slice = {:?}", slice);
}

fn first_word(s: &str) -> &str { // return type &str is an immutable reference
    let bytes = s.as_bytes();

    for (i, &item) in bytes.iter().enumerate() {
        if item == b' ' {
            return &s[0..i];
        }
    }

    &s[..]
}