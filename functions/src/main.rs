fn main() {
    another_function(5);

    // prefix unused vars with an undescore
    let _x = 5;

    let y = {
        let x = 3;
        // note that expressions do not end with semi-colon
        // if a semi-colon is added, it will become a statement
        // and will not return a value
        x + 1
    };

    println!("The value of y is: {}", y);

    let x = five();

    println!("The value of x is: {}", x);

    let x = plus_one(5);

    println!("The value of x is: {}", x);
}

// In function signatures, you must declare the type of each parameter.
fn another_function(x: i32) {
    println!("The value of x is: {}", x);
}

// declare function return value type with ->
fn five() -> i32 {
    5
}

fn plus_one(x: i32) -> i32 {
    x + 1
}