fn main() {
    let number = 6;

    if number % 4 == 0 {
        println!("number is divisible by 4");
    } else if number % 3 == 0 {
        println!("number is divisible by 3");
    } else if number % 2 == 0 {
        println!("number is divisible by 2");
    } else {
        println!("number is not divisible by 4, 3, or 2");
    }

    let condition = true;

    // the types of results must match (as they do here)
    let number = if condition { 5 } else { 6 };

    println!("The value of number is: {}", number);


    // One of the uses of a loop is to retry an operation you know might fail,
    // such as checking whether a thread has completed its job.
    // However, you might need to pass the result of that operation to the rest
    // of your code.

    // Here, we use the 'break' keyword with the value 'counter * 2'.
    // After the loop, we use a semicolon to end the statement that assigns
    // the value to result.
    let mut counter = 0;

    let result = loop {
        counter += 1;

        if counter == 10 {
            break counter * 2;
        }
    };

    println!("The result is {}", result);


    // It’s often useful for a program to evaluate a condition within a loop.
    // While the condition is true, the loop runs. When the condition ceases
    // to be true, the program calls 'break', stopping the loop.
    let mut number = 3;

    while number != 0 {
        println!("{}!", number);

        number -= 1;
    }

    println!("LIFTOFF!!!");


    // For loops:
    let a = [10, 20, 30, 40, 50];

    for element in a.iter() {
        println!("the value is: {}", element);
    }

    // Note the use of a range with '.rev' to reverse the range.
    for number in (1..4).rev() {
        println!("{}!", number);
    }
    println!("LIFTOFF!!!");

}