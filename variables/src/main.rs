fn main() {
    let x = 5;

    // shadowing:
    let x = x + 1;

    let x = x * 2;

    println!("The value of x is: {}", x);


    // Tuples have a fixed length:
    // once declared, they cannot grow or shrink in size.
    let tup: (i32, f64, u8) = (500, 6.4, 1);

    println!("The tuple is {:?}", tup);

    println!("The tuple contains {}, {}, {}", tup.0, tup.1, tup.2);

    let (x, y, z) = tup;

    println!("The tuple contains {}, {}, {}", x, y, z);

    // Arrays must contain elements of the same type
    // They are also fixed size.
    // Arrays are allocated to the stack (not the heap)
    let a: [i32; 5] = [1, 2, 3, 4, 5];

    println!("The 3rd array element is {}", a[2]);
}