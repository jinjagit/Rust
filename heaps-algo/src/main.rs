// https://en.wikipedia.org/wiki/Heap's_algorithm
// My adaptations of the wiki code.

fn main() {
    let permutations: Vec<Vec<i32>> = find_permutations(vec![0, 1, 2, 3, 4]);

    for p in permutations {
        println!("{:?}", p);
    }   
}

// Non-recursive version of Heap's Algorithm, adapted from en.wikipedia.org/wiki/Heap's_algorithm
fn find_permutations(mut array: Vec<i32>) -> Vec<Vec<i32>> {
    let n: usize = array.iter().count();
    let mut permutations: Vec<Vec<i32>> = vec![];

    //c is an encoding of the stack state. c[k] encodes the for-loop counter for when generate(k - 1, A) is called
    let mut c: Vec<usize> = vec![];

    for _ in 0..n {
        c.push(0);
    }

    permutations.push(array.clone());

    //i acts similarly to the stack pointer
    let mut i: usize = 0;
    while i < n {
        if c[i] < i {
            if i % 2 == 0 {
                let temp = array[0];
                array[0] = array[i];
                array[i] = temp;
            } else {
                let temp = array[c[i]];
                array[c[i]] = array[i];
                array[i] = temp;
            }

            permutations.push(array.clone());
            //Swap has occurred ending the for-loop. Simulate the increment of the for-loop counter
            c[i] += 1;
            //Simulate recursive call reaching the base case by bringing the pointer to the base case analog in the array
            i = 0;
        } else {
            //Calling generate(i+1, A) has ended as the for-loop terminated. Reset the state and simulate popping the stack by incrementing the pointer.
            c[i] = 0;
            i += 1;
        }
    }

    permutations
}

// This recursive form works, but I don't know how to return the permutations as they are found.
#[allow(dead_code)]
fn find_permutations_recursive(k: usize, mut array: Vec<i32>) {
    if k == 1 {
        // all permutations produced here, but I have no idea how to return them!
        println!("{:?}", array);
    } else {
        // Generate permutations with kth unaltered
        // Initially k == length(A)
        find_permutations_recursive(k - 1, array.clone());

        // Generate permutations for kth swapped with each k-1 initial
        for i in 0..(k - 1) {
            // Swap choice dependent on parity of k (even or odd)
            if k % 2 == 0 {
                let temp: i32 = array[i];
                array[i] = array[k - 1];
                array[k - 1] = temp; // zero-indexed, the kth is at k-1
            } else {
                let temp: i32 = array[0];
                array[0] = array[k - 1];
                array[k - 1] = temp;
            }

            find_permutations_recursive(k - 1, array.clone());
        }
    }
}
