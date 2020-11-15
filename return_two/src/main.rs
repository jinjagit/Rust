fn main() {
    let mut a: i32 = 1;
    let mut b: i32 = 2;

    for _ in 0..3 {
        let (a, b) = return_two((a, b));
    }
    
    println!("a: {}, b: {}", a, b);
}

fn return_two(pair: (i32, i32)) -> (i32, i32) {
    let (a, b) = pair;

    return (a * 2, b *3);
}
