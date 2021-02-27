fn main() {
    let mut guess: i64 = 1;

    let mut result: i64 = 1;

    loop {
        result = result * (guess + 1 );

        if result > 1000000 {
            break;
        } else {
            guess += 1;
        }
    }

    println!("solution: {}", guess);
}
