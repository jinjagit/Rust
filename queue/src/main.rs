fn main() {

    let mut arr: Vec<u32> = vec![1, 2, 3, 4, 5];

    arr = queue(arr, 99);

    println!("arr: {:?}", arr);
}

fn queue(mut arry: Vec<u32>, new_elem: u32) -> Vec<u32> {
    arry.push(new_elem); // add element to end of vector
    arry.remove(0); // remove element at given index

    arry
}
