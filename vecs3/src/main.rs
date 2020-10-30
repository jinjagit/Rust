// Task: create slice from vec of u32s, then convert slice to slice of usizes.
// confirm by using slice item as index ref.

fn main() {
    let my_vec: Vec<u32> = vec![1, 2, 3 ,4];

    let slice: [usize] = &my_vec[0..2] as &[usize];

    println!("slice: {:?}", slice);
}
