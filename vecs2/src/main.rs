// Task: Create a vec in main, pass a ref to the vec (or as a slice) to a fn
// which returns a mutated version of the vec & set the vec to new value in main.

fn main() {
    let mut my_vec = vec![1, 2, 3];

    my_vec = mutate_vec(&my_vec);

    println!("my_vec: {:?}", my_vec);
}

fn mutate_vec(vec: &Vec<u32>) -> Vec<u32> {
    println!("vec: {:?}", vec);

    let mut new_vec = vec.to_vec();

    new_vec[2] = 777;

    new_vec
}
