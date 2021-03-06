fn main() {
    fib(185);
}

fn fib(n: u8) { // max n = 185
    let mut counter = n;
    let mut a: u128 = 0;
    let mut b: u128 = 1;
    let mut temp: u128;

    while counter != 0 {
        temp = b;
        b = a + b;
        a = temp;

        println!("{}", b);

        counter -= 1;
    }
}
