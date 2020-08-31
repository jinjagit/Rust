fn main() {
    fib(9);
}

fn fib(n: u16) {
    let mut counter = n;
    let mut a: u32 = 0;
    let mut b: u32 = 1;
    let mut temp: u32;

    while counter != 0 {
        temp = b;
        b = a + b;
        a = temp;

        print!("{} ", b);

        counter -= 1;
    }

    println!("")
}
